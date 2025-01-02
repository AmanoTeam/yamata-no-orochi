// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The manga plugin.

use ferogram::{filter, handler, Context, Router};
use grammers_client::{button, reply_markup, InputMessage};
use maplit::hashmap;

use crate::{
    resources::{anilist::AniList, i18n::I18n},
    utils,
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router.handler(handler::new_message(filter::command("manga")).then(manga))
}

/// The anime command handler.
async fn manga(ctx: Context, i18n: I18n, ani: AniList) -> ferogram::Result<()> {
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
        ctx.reply(InputMessage::html(t("manga_usage"))).await?;
    } else {
        if let Ok(id) = args[0].parse::<i64>() {
            if let Ok(manga) = ani.get_manga(id).await {
                let mut text = utils::gen_manga_info(&manga);
                let image_url = manga.banner.unwrap_or(
                    manga.cover.extra_large.unwrap_or(
                        manga
                            .cover
                            .large
                            .unwrap_or(manga.cover.medium.unwrap_or(String::new())),
                    ),
                );

                if ctx.is_callback_query() {
                    text.push_str(&format!("\n<a href='{}'>ㅤ</a>", image_url));
                    ctx.edit(InputMessage::html(text).link_preview(true))
                        .await?;
                } else {
                    ctx.reply(InputMessage::html(text).photo_url(image_url))
                        .await?;
                }
            } else {
                ctx.reply(InputMessage::html(t("manga_not_found"))).await?;
            }
        } else {
            let title = args.join(" ");

            if let Some(result) = ani.search_manga(&title).await {
                let buttons = result
                    .into_iter()
                    .take(6)
                    .map(|manga| {
                        vec![button::inline(
                            manga.title.romaji.unwrap_or(manga.title.native),
                            format!("manga {}", manga.id),
                        )]
                    })
                    .collect::<Vec<_>>();
                ctx.reply(
                    InputMessage::html(t_a("search_results", hashmap! { "title" => title }))
                        .reply_markup(&reply_markup::inline(buttons)),
                )
                .await?;
            } else {
                ctx.reply(InputMessage::html(t("manga_not_found"))).await?;
            }
        }
    }

    Ok(())
}
