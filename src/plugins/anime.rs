// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The anime plugin.

use ferogram::{filter, handler, Context, Result, Router};
use grammers_client::{
    button, reply_markup,
    types::{inline, InlineQuery},
    InputMessage,
};
use maplit::hashmap;
use rust_anilist::models::Anime;

use crate::{
    resources::{anilist::AniList, i18n::I18n},
    utils::{self, remove_html, shorten_text},
};

const ANILIST_BANNER_URL: &str = "https://img.anili.st/media/";

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .handler(
            handler::new_message(
                filter::commands(&["a", "anime"]).description("Search for animes."),
            )
            .then(anime),
        )
        .handler(handler::callback_query(filter::regex(r"^anime (\d+)")).then(anime))
        .handler(
            handler::inline_query(filter::regex(r"^[\.!]?a(n(i(m(e)?)?)?)? (.+)"))
                .then(anime_inline),
        )
}

/// The anime command handler.
async fn anime(ctx: Context, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let t_a = |key: &str, args| i18n.translate_with_args(key, args);

    let text = if ctx.is_callback_query() {
        ctx.query()
    } else {
        ctx.text()
    }
    .unwrap();
    let arg = text
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ");

    if arg.is_empty() {
        ctx.reply(
            InputMessage::html(t("anime_usage")).reply_markup(&reply_markup::inline(vec![vec![
                button::switch_inline(t("search_btn"), "!a "),
            ]])),
        )
        .await?;
    } else {
        if let Ok(id) = arg.parse::<i64>() {
            if let Ok(anime) = ani.get_anime(id).await {
                send_anime_info(anime, ctx, &i18n).await?;
            } else {
                ctx.reply(InputMessage::html(t("anime_not_found"))).await?;
            }
        } else {
            let title = arg;

            if let Some(result) = ani.search_anime(&title, 1, 6).await {
                if result.is_empty() {
                    ctx.reply(InputMessage::html(t("no_results"))).await?;
                    return Ok(());
                } else if result.len() == 1 {
                    return send_anime_info(result[0].clone(), ctx, &i18n).await;
                }

                let buttons = result
                    .into_iter()
                    .map(|anime| {
                        vec![button::inline(
                            anime.title.romaji(),
                            format!("anime {}", anime.id),
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

/// Sends the anime info to the user.
async fn send_anime_info(anime: Anime, ctx: Context, i18n: &I18n) -> Result<()> {
    let text = utils::gen_anime_info(&anime, i18n);
    let image_url = ANILIST_BANNER_URL.to_owned() + &anime.id.to_string();

    if ctx.is_callback_query() {
        ctx.edit(
            InputMessage::html(format!("<a href=\"{}\">⁠</a>", image_url) + &text)
                .link_preview(true),
        )
        .await?;
    } else {
        ctx.reply(InputMessage::html(text).photo_url(image_url))
            .await?;
    }

    Ok(())
}

/// The anime inline query handler.
async fn anime_inline(query: InlineQuery, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    let arg = query
        .text()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ");
    let offset = query.offset().parse::<u16>().unwrap_or(1);
    let mut results = Vec::new();

    if let Ok(id) = arg.parse::<i64>() {
        if let Ok(anime) = ani.get_anime(id).await {
            let article = gen_anime_article(anime, &i18n);
            results.push(article.into());
        }
    } else {
        if let Some(result) = ani.search_anime(&arg, offset, 10).await {
            for anime in result {
                let article = gen_anime_article(anime, &i18n);
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

/// Generates an inline query article for an anime.
fn gen_anime_article(anime: Anime, i18n: &I18n) -> inline::query::Article {
    let t = |key: &str| i18n.translate(key);

    let text = utils::gen_anime_info(&anime, &i18n);
    let image_url = ANILIST_BANNER_URL.to_owned() + &anime.id.to_string();

    let mut article = inline::query::Article::new(
        anime.title.romaji(),
        InputMessage::html(format!("<a href=\"{}\">⁠</a>", image_url) + &text)
            .reply_markup(&reply_markup::inline(vec![vec![button::inline(
                t("load_more_btn"),
                format!("anime {}", anime.id),
            )]]))
            .link_preview(true),
    )
    .description(shorten_text(remove_html(anime.description), 150));

    let image_url = anime.banner.unwrap_or(
        anime
            .cover
            .largest()
            .map(String::from)
            .unwrap_or(String::new()),
    );
    if !image_url.is_empty() {
        article = article.thumb_url(image_url);
    }

    article
}
