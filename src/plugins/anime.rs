// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The anime plugin.

use std::time::Duration;

use ferogram::{
    filter, handler,
    utils::{bytes_to_string, split_btns_into_columns},
    Context, Result, Router,
};
use grammers_client::{
    button, reply_markup,
    types::{inline, CallbackQuery, InlineQuery},
    InputMessage,
};
use maplit::hashmap;
use rust_anilist::models::{Anime, RelationType};

use crate::{
    resources::{AniList, I18n},
    utils::{self, gen_char_list, gen_pagination_buttons, remove_html, shorten_text},
};

const ANILIST_BANNER_URL: &str = "https://img.anili.st/media/";

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .register(
            handler::new_message(
                filter::commands(&["a", "anime"]).description("Search for animes."),
            )
            .then(anime),
        )
        .register(handler::callback_query(filter::regex(r"^anime (\d+) (\d+)")).then(anime))
        .register(
            handler::callback_query(filter::regex(
                r"^anime (studios|episodes|staff|chars|tags|links) (\d+) (\d+)",
            ))
            .then(anime_info),
        )
        .register(handler::inline_query(filter::regex(r"^[\.!]?a (.+)")).then(anime_inline))
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
            InputMessage::html(t("anime_usage")).reply_markup(&reply_markup::inline(vec![vec![
                button::switch_inline(t("search_btn"), "!a "),
            ]])),
        )
        .await?;
    } else {
        if let Ok(id) = args[0].parse::<i64>() {
            if let Ok(anime) = ani.get_anime(id).await {
                send_anime_info(anime, ctx, &i18n).await?;
            } else {
                ctx.reply(InputMessage::html(t("anime_not_found"))).await?;
            }
        } else {
            let title = args.join(" ");

            if let Some(result) = ani.search_anime(&title, 1, 6).await {
                if result.is_empty() {
                    ctx.reply(InputMessage::html(t("no_results"))).await?;
                    return Ok(());
                } else if result.len() == 1 {
                    let anime = ani.get_anime(result[0].id).await.unwrap_or_default();
                    return send_anime_info(anime, ctx, &i18n).await;
                }

                let buttons = result
                    .into_iter()
                    .map(|anime| {
                        vec![button::inline(
                            if anime.is_adult { "🔞 " } else { "" }.to_string()
                                + &anime.title.romaji(),
                            format!("anime {0} {1}", anime.id, sender.id()),
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
    let t = |key: &str| i18n.translate(key);

    let text = utils::gen_anime_info(&anime, i18n);
    let image_url = ANILIST_BANNER_URL.to_owned() + &anime.id.to_string();
    let mut buttons = Vec::new();

    let sender = ctx.sender().unwrap();

    if anime.studios.is_some() {
        buttons.push(button::inline(
            t("studios_btn"),
            format!("anime studios {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.episodes.is_some() {
        buttons.push(button::inline(
            t("episodes_btn"),
            format!("anime episodes {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.staff.is_some() {
        buttons.push(button::inline(
            t("staff_btn"),
            format!("anime staff {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.characters().is_ok() {
        buttons.push(button::inline(
            t("characters_btn"),
            format!("anime chars {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.tags.as_ref().is_some_and(|tags| !tags.is_empty()) {
        buttons.push(button::inline(
            t("tags_btn"),
            format!("anime tags {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.external_links.is_some() {
        buttons.push(button::inline(
            t("links_btn"),
            format!("anime links {0} {1}", anime.id, sender.id()),
        ));
    }

    let mut buttons = split_btns_into_columns(buttons, 2);

    if let Ok(relations) = anime.relations() {
        let mut relations_buttons = Vec::new();

        let prequel = relations
            .iter()
            .filter(|r| matches!(r.relation_type, RelationType::Prequel))
            .last();
        let sequel = relations
            .iter()
            .filter(|r| matches!(r.relation_type, RelationType::Sequel))
            .last();

        if let Some(prequel) = prequel {
            relations_buttons.push(button::inline(
                t("previous_btn"),
                format!("anime {0} {1}", prequel.media().id(), sender.id()),
            ));
        }
        if let Some(sequel) = sequel {
            relations_buttons.push(button::inline(
                t("next_btn"),
                format!("anime {0} {1}", sequel.media().id(), sender.id()),
            ));
        }

        if !relations_buttons.is_empty() {
            buttons.push(relations_buttons);
        }
    }

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

/// The anime info handler.
async fn anime_info(query: CallbackQuery, i18n: I18n, ani: AniList) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    let data = query.data();
    let args = bytes_to_string(data)
        .split_whitespace()
        .skip(1)
        .map(String::from)
        .collect::<Vec<_>>();

    let info = args[0].as_str();
    let anime_id = args[1].parse::<i64>().unwrap();
    let sender_id = args[2].parse::<i64>().unwrap();

    let sender = query.sender();

    if sender.id() != sender_id {
        query
            .answer()
            .cache_time(Duration::from_secs(120))
            .alert(t("not_allowed"))
            .send()
            .await?;
        return Ok(());
    }

    if let Ok(mut anime) = ani.get_anime(anime_id).await {
        let mut text = format!(
            "<code>{0}</code> | <b>{1}</b>\n\n",
            anime.id,
            anime.title.romaji()
        );

        match info {
            "studios" => {}
            "episodes" => {}
            "staff" => {}
            "chars" => {
                let page = args
                    .get(3)
                    .unwrap_or(&1.to_string())
                    .parse::<usize>()
                    .unwrap();
                let characters = anime.characters().unwrap_or_default();

                let per_page = 10;
                let max_pages = (characters.len() as f32 / 15f32).round() as usize + 1;

                if characters.is_empty() {
                    query.answer().alert(t("not_available")).send().await?;
                    return Ok(());
                }

                text.push_str(&gen_char_list(&characters, page, per_page, &i18n));
                let buttons = gen_pagination_buttons(
                    &format!("anime chars {0} {1}", anime_id, sender_id),
                    page,
                    max_pages,
                );

                query
                    .answer()
                    .edit(
                        InputMessage::html(text).reply_markup(&reply_markup::inline(vec![
                            buttons,
                            vec![button::inline(
                                t("back_btn"),
                                format!("anime {0} {1}", anime_id, sender_id),
                            )],
                        ])),
                    )
                    .await?;
            }
            "tags" => {
                if let Some(tags) = anime.tags.as_mut().take_if(|tags| !tags.is_empty()) {
                    let tags = tags
                        .iter()
                        .map(|tag| {
                            if tag.is_adult {
                                format!("<s>{}</s>", tag.name)
                            } else if tag.is_general_spoiler || tag.is_media_spoiler {
                                format!("<details>{}</details>", tag.name)
                            } else {
                                tag.name.clone()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    text.push_str(&format!("🏷 | <b>{0}</b>: <i>{1}</i>", t("tags"), tags));

                    query
                        .answer()
                        .edit(
                            InputMessage::html(text).reply_markup(&reply_markup::inline(vec![
                                vec![button::inline(
                                    t("back_btn"),
                                    format!("anime {0} {1}", anime_id, sender_id),
                                )],
                            ])),
                        )
                        .await?;
                } else {
                    query
                        .answer()
                        .cache_time(Duration::from_secs(120))
                        .alert(t("not_available"))
                        .send()
                        .await?;
                }
            }
            "links" => {
                text.push_str(&format!("🖇 <b>{}</b>:\n", t("links")));

                if let Some(links) = anime.external_links.as_ref() {
                    for link in links.iter().filter(|l| l.is_disabled.is_none()) {
                        text.push_str(&format!(
                            "🔗 | <a href=\"{}\">{}</a>\n",
                            link.url, link.site
                        ));
                    }
                }

                text.push_str(&format!("🔗 | <a href=\"{}\">AniList</a>\n", anime.url));
                if let Some(id) = anime.id_mal {
                    text.push_str(&format!(
                        "🔗 | <a href=\"https://myanimelist.net/manga/{}\">MyAnimeList</a>",
                        id
                    ));
                }

                query
                    .answer()
                    .edit(
                        InputMessage::html(text).reply_markup(&reply_markup::inline(vec![vec![
                            button::inline(
                                t("back_btn"),
                                format!("anime {0} {1}", anime_id, sender_id),
                            ),
                        ]])),
                    )
                    .await?;
            }
            _ => {
                query
                    .answer()
                    .cache_time(Duration::from_secs(120))
                    .alert(t("not_implemented"))
                    .send()
                    .await?
            }
        }
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
        .collect::<Vec<_>>()
        .join(" ");
    let offset = query.offset().parse::<u16>().unwrap_or(1);
    let mut results = Vec::new();

    if let Some(result) = ani.search_anime(&arg, offset, 10).await {
        for anime in result {
            let article = gen_anime_article(&query, anime, &i18n);
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

/// Generates an inline query article for an anime.
fn gen_anime_article(query: &InlineQuery, anime: Anime, i18n: &I18n) -> inline::query::Article {
    let t = |key: &str| i18n.translate(key);

    let text = utils::gen_anime_info(&anime, &i18n);
    let image_url = ANILIST_BANNER_URL.to_owned() + &anime.id.to_string();

    let sender = query.sender();

    let mut article = inline::query::Article::new(
        if anime.is_adult { "🔞 " } else { "" }.to_string() + &anime.title.romaji(),
        InputMessage::html(format!("<a href=\"{}\">⁠</a>", image_url) + &text)
            .link_preview(true)
            .reply_markup(&reply_markup::inline(vec![vec![button::inline(
                t("load_more_btn"),
                format!("anime {0} {1}", anime.id, sender.id()),
            )]])),
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
