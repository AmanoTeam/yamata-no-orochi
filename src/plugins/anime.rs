// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The anime plugin.

use ferogram::{filter, handler, Context, Result, Router};
use grammers_client::{button, reply_markup, InputMessage};
use maplit::hashmap;

use crate::{
    resources::{anilist::AniList, i18n::I18n},
    utils,
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .handler(handler::new_message(filter::commands(&["a", "anime"])).then(anime))
        .handler(handler::callback_query(filter::regex(r"^anime (\d+)")).then(anime))
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
    let args = text.split_whitespace().skip(1).collect::<Vec<&str>>();

    if args.is_empty() {
        ctx.reply(InputMessage::html(t("anime_usage"))).await?;
    } else {
        if let Ok(id) = args[0].parse::<i64>() {
            if let Ok(anime) = ani.get_anime(id).await {
                let mut text = utils::gen_anime_info(&anime);
                let image_url = format!("https://img.anili.st/media/{}", anime.id);

                if ctx.is_callback_query() && !ctx.has_photo().await {
                    text.push_str(&format!("<a href='{}'> </a>", image_url));
                    ctx.edit(InputMessage::html(text).link_preview(true))
                        .await?;
                } else {
                    ctx.reply(InputMessage::html(text).photo_url(image_url))
                        .await?;
                }
            } else {
                ctx.reply(InputMessage::html(t("anime_not_found"))).await?;
            }
        } else {
            let title = args.join(" ");

            if let Some(result) = ani.search_anime(&title).await {
                let buttons = result
                    .into_iter()
                    .map(|anime| {
                        vec![button::inline(
                            anime.title.romaji.unwrap_or(anime.title.native),
                            format!("anime {}", anime.id),
                        )]
                    })
                    .collect::<Vec<_>>();

                if buttons.is_empty() {
                    ctx.reply(InputMessage::html(t("no_results"))).await?;
                    return Ok(());
                }

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
