// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The manga plugin.

use ferogram::{filter, handler, Context, Result, Router};
use grammers_client::{
    button, reply_markup,
    types::{inline, InlineQuery},
    InputMessage,
};
use maplit::hashmap;
use rust_anilist::models::Manga;

use crate::{
    resources::{anilist::AniList, i18n::I18n},
    utils::{self, remove_html, shorten_text},
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .handler(
            handler::new_message(
                filter::commands(&["m", "manga"]).description("Search for mangas."),
            )
            .then(manga),
        )
        .handler(handler::callback_query(filter::regex(r"^manga (\d+)")).then(manga))
        .handler(
            handler::inline_query(filter::regex(r"^[\.!]?m(a(n(g(a)?)?)?)? (.+)"))
                .then(manga_inline),
        )
}

/// The manga command handler.
async fn manga(ctx: Context, i18n: I18n, ani: AniList) -> Result<()> {
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
            InputMessage::html(t("manga_usage")).reply_markup(&reply_markup::inline(vec![vec![
                button::switch_inline(t("search_btn"), "!m "),
            ]])),
        )
        .await?;
    } else {
        if let Ok(id) = args[0].parse::<i64>() {
            if let Ok(manga) = ani.get_manga(id).await {
                send_manga_info(manga, ctx, &i18n).await?;
            } else {
                ctx.reply(InputMessage::html(t("manga_not_found"))).await?;
            }
        } else {
            let title = args.join(" ");

            if let Some(result) = ani.search_manga(&title, 1, 6).await {
                if result.is_empty() {
                    ctx.reply(InputMessage::html(t("no_results"))).await?;
                    return Ok(());
                } else if result.len() == 1 {
                    return send_manga_info(result[0].clone(), ctx, &i18n).await;
                }

                let buttons = result
                    .into_iter()
                    .map(|manga| {
                        vec![button::inline(
                            manga.title.romaji(),
                            format!("manga {}", manga.id),
                        )]
                    })
                    .collect::<Vec<_>>();

                ctx.reply(
                    InputMessage::html(t_a("search_results", hashmap! { "search" => title }))
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

/// Sends the manga info to the user.
async fn send_manga_info(manga: Manga, ctx: Context, i18n: &I18n) -> Result<()> {
    let mut text = utils::gen_manga_info(&manga, i18n);
    let image_url = manga.banner.or(manga.cover.largest().map(String::from));

    if ctx.is_callback_query() {
        if let Some(image_url) = image_url.as_ref() {
            text = format!("<a href=\"{}\">⁠</a>", image_url) + &text;
        }

        ctx.edit(InputMessage::html(text).link_preview(true))
            .await?;
    } else {
        ctx.reply(InputMessage::html(text).photo_url(image_url.unwrap_or_default()))
            .await?;
    }

    Ok(())
}

/// The manga inline query handler.
async fn manga_inline(query: InlineQuery, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    let arg = query
        .text()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    let offset = query.offset().parse::<u16>().unwrap_or(1);
    let mut results = Vec::new();

    if let Ok(id) = arg.parse::<i64>() {
        if let Ok(manga) = ani.get_manga(id).await {
            let article = gen_manga_article(manga, &i18n);
            results.push(article.into());
        }
    } else {
        if let Some(result) = ani.search_manga(&arg, offset, 10).await {
            for manga in result {
                let article = gen_manga_article(manga, &i18n);
                results.push(article.into());
            }
        }
    }

    if results.is_empty() {
        if offset == 1 {
            results.push(
                inline::query::Article::new(t("no_results"), InputMessage::html(t("no_results")))
                    .into(),
            );
        } else {
            results.push(
                inline::query::Article::new(
                    t("no_more_results"),
                    InputMessage::html(t("no_more_results")),
                )
                .into(),
            );
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

/// Generates an inline query article for a manga.
fn gen_manga_article(manga: Manga, i18n: &I18n) -> inline::query::Article {
    let t = |key: &str| i18n.translate(key);

    let mut text = utils::gen_manga_info(&manga, &i18n);
    let image_url = manga.banner.or(manga.cover.largest().map(String::from));

    if let Some(image_url) = image_url.as_ref() {
        text = format!("<a href=\"{}\">⁠</a>", image_url) + &text;
    }

    let mut article = inline::query::Article::new(
        manga.title.romaji(),
        InputMessage::html(text)
            .reply_markup(&reply_markup::inline(vec![vec![button::inline(
                t("load_more_btn"),
                format!("manga {}", manga.id),
            )]]))
            .link_preview(true),
    )
    .description(shorten_text(remove_html(manga.description), 150));

    if let Some(image_url) = image_url {
        article = article.thumb_url(image_url);
    }

    article
}
