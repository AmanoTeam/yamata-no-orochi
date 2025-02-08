// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The character plugin.

use std::time::Duration;

use ferogram::{filter, handler, utils::split_btns_into_columns, Context, Result, Router};
use grammers_client::{
    button, reply_markup,
    types::{inline, InlineQuery},
    InputMessage,
};
use maplit::hashmap;
use rust_anilist::models::Character;

use crate::{
    resources::{AniList, I18n},
    utils::{self, remove_html, shorten_text},
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .register(
            handler::new_message(
                filter::commands(&["c", "char", "p", "perso"])
                    .description("Search for characters."),
            )
            .then(character),
        )
        .register(handler::callback_query(filter::regex(r"^char (\d+) (\d+)")).then(character))
        .register(handler::inline_query(filter::regex(r"^[\.!]?(c|p) (.+)")).then(character_inline))
}

/// The character handler.
async fn character(ctx: Context, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let t_a = |key: &str, args| i18n.translate_with_args(key, args);

    let text = if ctx.is_callback_query() {
        ctx.query()
    } else {
        ctx.text()
    }
    .unwrap();
    let mut args = text.split_whitespace().skip(1).collect::<Vec<_>>();

    let sender = ctx.sender().unwrap();

    if let Some(query) = ctx.callback_query() {
        let sender_id = args.pop().unwrap().parse::<i64>().unwrap();

        if sender.id() != sender_id {
            query
                .answer()
                .cache_time(Duration::from_secs(120))
                .alert(t("not_allowed"))
                .send()
                .await?;
            return Ok(());
        }
    }

    if args.is_empty() {
        ctx.reply(
            InputMessage::html(t("character_usage")).reply_markup(&reply_markup::inline(vec![
                vec![button::switch_inline(t("search_btn"), "!c ")],
            ])),
        )
        .await?;
    } else {
        if let Ok(id) = args[0].parse::<i64>() {
            if let Ok(char) = ani.get_char(id).await {
                send_char_info(char, ctx, &i18n).await?;
            } else {
                ctx.reply(InputMessage::html(t("not_found"))).await?;
            }
        } else {
            let title = args.join(" ");

            if let Some(result) = ani.search_char(&title, 1, 6).await {
                if result.is_empty() {
                    ctx.reply(InputMessage::html(t("no_results_text")).reply_markup(
                        &reply_markup::inline(vec![vec![button::switch_inline(
                            t("search_again_btn"),
                            format!("!c {}", title),
                        )]]),
                    ))
                    .await?;
                    return Ok(());
                } else if result.len() == 1 {
                    return send_char_info(result[0].clone(), ctx, &i18n).await;
                }

                let buttons = result
                    .into_iter()
                    .map(|char| {
                        vec![button::inline(
                            char.name.full(),
                            format!("char {0} {1}", char.id, sender.id()),
                        )]
                    })
                    .collect::<Vec<_>>();

                ctx.reply(
                    InputMessage::html(t_a("search_results", hashmap! { "search" => title }))
                        .reply_markup(&reply_markup::inline(buttons)),
                )
                .await?;
            } else {
                ctx.reply(InputMessage::html(t("no_results_text")).reply_markup(
                    &reply_markup::inline(vec![vec![button::switch_inline(
                        t("search_again_btn"),
                        format!("!c {}", title),
                    )]]),
                ))
                .await?;
            }
        }
    }

    Ok(())
}

/// Sends the char info to the user.
async fn send_char_info(char: Character, ctx: Context, i18n: &I18n) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    let text = utils::gen_char_info(&char, i18n);
    let image_url = char.image.largest();
    let mut buttons = Vec::new();

    let sender = ctx.sender().unwrap();

    if char.voice_actors.is_some() {
        buttons.push(button::inline(
            t("voice_actors_btn"),
            format!("char voice_actors {} {}", char.id, sender.id()),
        ));
    }

    let mut buttons = split_btns_into_columns(buttons, 2);
    buttons.push(vec![button::inline(
        t("medias_btn"),
        format!("char medias {} {}", char.id, sender.id()),
    )]);

    let markup = reply_markup::inline(buttons);

    if ctx.is_callback_query() {
        ctx.edit(
            InputMessage::html(format!("<a href=\"{}\">⁠</a>", image_url) + &text)
                .link_preview(true)
                .photo_url(image_url)
                .reply_markup(&markup),
        )
        .await?;
    } else {
        ctx.reply(
            InputMessage::html(text)
                .photo_url(image_url)
                .reply_markup(&markup),
        )
        .await?;
    }

    Ok(())
}

/// The character inline query handler.
async fn character_inline(query: InlineQuery, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    let arg = query
        .text()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    let offset = query.offset().parse::<u16>().unwrap_or(1);
    let mut results = Vec::new();

    if let Some(result) = ani.search_char(&arg, offset, 10).await {
        for char in result {
            let article = gen_char_article(&query, char, &i18n);
            results.push(article);
        }
    }

    if results.is_empty() {
        if offset == 1 {
            results.push(
                inline::query::Article::new(
                    t("no_results"),
                    InputMessage::html(t("no_results_text")).reply_markup(&reply_markup::inline(
                        vec![vec![button::switch_inline(
                            t("search_again_btn"),
                            format!("!c {}", arg),
                        )]],
                    )),
                )
                .description(t("click_for_more_info")),
            );
        } else {
            results.push(
                inline::query::Article::new(
                    t("no_more_results"),
                    InputMessage::html(t("no_more_results_text")).reply_markup(
                        &reply_markup::inline(vec![vec![button::switch_inline(
                            t("search_again_btn"),
                            format!("!c {}", arg),
                        )]]),
                    ),
                )
                .description(t("click_for_more_info")),
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

/// Generates an inline query article for a character.
fn gen_char_article(query: &InlineQuery, char: Character, i18n: &I18n) -> inline::query::Article {
    let t = |key: &str| i18n.translate(key);

    let text = utils::gen_char_info(&char, &i18n);
    let image_url = char.image.largest();

    let sender = query.sender();

    let mut article = inline::query::Article::new(
        char.name.full(),
        InputMessage::html(format!("<a href=\"{}\">⁠</a>", image_url) + &text)
            .link_preview(true)
            .reply_markup(&reply_markup::inline(vec![vec![button::inline(
                t("load_more_btn"),
                format!("char {0} {1}", char.id, sender.id()),
            )]])),
    )
    .description(shorten_text(remove_html(char.description), 150));

    if !image_url.is_empty() {
        article = article.thumb_url(image_url);
    }

    article
}
