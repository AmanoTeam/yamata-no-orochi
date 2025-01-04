// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The user plugin.

use ferogram::{filter, handler, Context, Result, Router};
use grammers_client::{button, reply_markup, InputMessage};
use maplit::hashmap;
use rust_anilist::models::User;

use crate::{
    resources::{anilist::AniList, i18n::I18n},
    utils,
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .handler(handler::new_message(filter::commands(&["u", "user"])).then(user))
        .handler(handler::callback_query(filter::regex(r"^user (\d+)")).then(user))
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
        ctx.reply(InputMessage::html(t("user_usage"))).await?;
    } else {
        if let Ok(id) = args[0].parse::<i32>() {
            if let Ok(user) = ani.get_user(id).await {
                send_user_info(&user, ctx).await?;
            } else {
                ctx.reply(InputMessage::html(t("user_not_found"))).await?;
            }
        } else {
            let name = args.join(" ");

            if let Some(result) = ani.search_user(&name).await {
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
    let mut text = utils::gen_user_info(&user);
    let mut image_url = format!("https://img.anili.st/user/{}", user.id);

    if ctx.is_callback_query() && !ctx.has_photo().await {
        text.push_str(&format!("<a href='{}'>ㅤ</a>", image_url));
        ctx.edit(InputMessage::html(text).link_preview(true))
            .await?;
    } else {
        image_url.push_str(&format!("?u={}", rand::random::<u32>()));
        ctx.reply(InputMessage::html(text).photo_url(image_url))
            .await?;
    }

    Ok(())
}
