// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The user plugin.

use ferogram::{filter, handler, Context, Result, Router};
use grammers_client::{
    button, reply_markup,
    types::{inline, InlineQuery},
    InputMessage,
};
use maplit::hashmap;
use rust_anilist::models::User;

use crate::{
    resources::{AniList, I18n},
    utils,
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .register(
            handler::new_message(filter::commands(&["u", "user"]).description("Search for users."))
                .then(user),
        )
        .register(handler::callback_query(filter::regex(r"^user (\d+)")).then(user))
        .register(handler::inline_query(filter::regex(r"^[\.!]?u (.+)")).then(user_inline))
}

/// The user handler.
async fn user(ctx: Context, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let t_a = |key: &str, args| i18n.translate_with_args(key, args);

    let text = if ctx.is_callback_query() {
        ctx.query()
    } else {
        ctx.text()
    }
    .unwrap();
    let args = text.split_whitespace().skip(1).collect::<Vec<&str>>();

    if args.is_empty() {
        ctx.reply(
            InputMessage::html(t("user_usage")).reply_markup(&reply_markup::inline(vec![vec![
                button::switch_inline(t("search_btn"), "!u "),
            ]])),
        )
        .await?;
    } else {
        if let Ok(id) = args[0].parse::<i32>() {
            if let Ok(user) = ani.get_user(id).await {
                send_user_info(&user, ctx).await?;
            } else {
                ctx.reply(InputMessage::html(t("user_not_found"))).await?;
            }
        } else {
            let name = args.join(" ");

            if let Some(result) = ani.search_user(&name, 1, 6).await {
                if result.is_empty() {
                    ctx.reply(InputMessage::html(t("no_results"))).await?;
                    return Ok(());
                } else if result.len() == 1 {
                    return send_user_info(&result[0], ctx).await;
                }

                let buttons = result
                    .into_iter()
                    .map(|user| vec![button::inline(user.name, format!("user {}", user.id))])
                    .collect::<Vec<_>>();

                ctx.reply(
                    InputMessage::html(t_a("search_results", hashmap! { "search" => name }))
                        .reply_markup(&reply_markup::inline(buttons)),
                )
                .await?;
            } else {
                ctx.reply(InputMessage::html(t("no_results"))).await?;
            }
        }
    }

    Ok(())
}

/// Sends the user info to the user.
async fn send_user_info(user: &User, ctx: Context) -> Result<()> {
    let text = utils::gen_user_info(&user);
    let mut image_url = format!("https://img.anili.st/user/{}", user.id);

    if ctx.is_callback_query() {
        ctx.edit(
            InputMessage::html(format!("<a href=\"{}\">⁠</a>", image_url) + &text)
                .link_preview(true),
        )
        .await?;
    } else {
        image_url.push_str(&format!("?u={}", rand::random::<u32>()));
        ctx.reply(InputMessage::html(text).photo_url(image_url))
            .await?;
    }

    Ok(())
}

/// Generates an inline query article for a user.
async fn user_inline(query: InlineQuery, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    let arg = query
        .text()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    let offset = query.offset().parse::<u16>().unwrap_or(1);
    let mut results = Vec::new();

    if let Some(result) = ani.search_user(&arg, offset, 10).await {
        for user in result {
            let article = gen_user_article(user);
            results.push(article);
        }
    }

    if results.is_empty() {
        if offset == 1 {
            results.push(inline::query::Article::new(
                t("no_results"),
                InputMessage::html(t("no_results")),
            ));
        } else {
            results.push(inline::query::Article::new(
                t("no_more_results"),
                InputMessage::html(t("no_more_results")),
            ));
        }
    }

    query
        .answer(results)
        .cache_time(120)
        .next_offset((offset + 1).to_string())
        .send()
        .await?;

    Ok(())
}

/// Generates an inline query article for a user.
fn gen_user_article(user: User) -> inline::query::Article {
    let text = utils::gen_user_info(&user);
    let image_url = format!("https://img.anili.st/user/{}", user.id);

    let mut article = inline::query::Article::new(
        user.name.clone(),
        InputMessage::html(format!("<a href=\"{}\">⁠</a>", image_url) + &text).link_preview(true),
    );

    let image_url = user.banner.or(user.avatar.map(|a| a.largest().to_string()));
    if let Some(image_url) = image_url {
        article = article.thumb_url(image_url);
    }

    article
}
