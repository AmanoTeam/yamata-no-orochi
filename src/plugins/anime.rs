// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The anime plugin.

use std::time::Duration;

use chrono::DateTime;
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
    resources::{anilist::AniList, i18n::I18n},
    utils::{self, gen_char_list, gen_pagination_buttons, remove_html, shorten_text},
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
        .handler(handler::callback_query(filter::regex(r"^anime (\d+) (\d+)")).then(anime))
        .handler(
            handler::callback_query(filter::regex(r"^anime (studios|synonyms|episodes|next_airing|staff|chars|tags|links) (\d+) (\d+)")).then(anime_info)
        )
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
                    return send_anime_info(result[0].clone(), ctx, &i18n).await;
                }

                let buttons = result
                    .into_iter()
                    .map(|anime| {
                        vec![button::inline(
                            if anime.is_adult {
                                format!("🔞 {}", anime.title.romaji())
                            } else {
                                anime.title.romaji().to_string()
                            },
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

    if anime.synonyms.is_some() {
        buttons.push(button::inline(
            t("synonyms_btn"),
            format!("anime synonyms {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.episodes.is_some() {
        buttons.push(button::inline(
            t("episodes_btn"),
            format!("anime episodes {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.next_airing_episode.is_some() {
        buttons.push(button::inline(
            t("next_airing_btn"),
            format!("anime next_airing {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.staff.is_some() {
        buttons.push(button::inline(
            t("staff_btn"),
            format!("anime staff {0} {1}", anime.id, sender.id()),
        ));
    }

    if !anime.characters().is_empty() {
        buttons.push(button::inline(
            t("characters_btn"),
            format!("anime chars {0} {1}", anime.id, sender.id()),
        ));
    }

    if anime.tags.is_some() {
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

    let relations = anime.relations();
    if !relations.is_empty() {
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
                format!("anime {} {}", prequel.media().id(), sender.id()),
            ));
        }
        if let Some(sequel) = sequel {
            relations_buttons.push(button::inline(
                t("next_btn"),
                format!("anime {} {}", sequel.media().id(), sender.id()),
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
            "↓ <code>{0}</code> → <b>{1}</b>\n\n",
            anime.id,
            anime.title.romaji()
        );

        match info {
            "studios" => {}
            "synonyms" => {
                let synonyms = anime
                    .synonyms
                    .unwrap_or_else(|| vec![t("no_synonyms")])
                    .join("; ");

                if let Err(e) = query
                    .answer()
                    .cache_time(Duration::from_secs(120))
                    .alert(format!("{}", synonyms))
                    .send()
                    .await
                {
                    if e.is("MESSAGE_TOO_LONG") {
                        text.push_str(&format!(
                            "📚 | <b>{0}</b>: <i>{1}</i>",
                            t("synonyms"),
                            synonyms
                        ));

                        query
                            .answer()
                            .edit(InputMessage::html(text).reply_markup(&reply_markup::inline(
                                vec![vec![button::inline(
                                    t("back_btn"),
                                    format!("anime {0} {1}", anime_id, sender_id),
                                )]],
                            )))
                            .await?;
                    }
                }
            }
            "episodes" => {}
            "next_airing" => {
                if let Some(next_airing) = anime.next_airing_episode.as_ref() {
                    let at =
                        DateTime::from_timestamp(next_airing.at, 0).expect("invalid timestamp");
                    let time_until = {
                        let mut text = String::new();
                        let time = Duration::from_secs(next_airing.time_until);

                        // Days
                        if time.as_secs() >= 86400 {
                            text.push_str(&format!("{}d, ", time.as_secs() / 86400));
                        }

                        // Hours
                        if time.as_secs() >= 3600 {
                            text.push_str(&format!("{}h, ", (time.as_secs() % 86400) / 3600));
                        }

                        // Minutes
                        if time.as_secs() >= 60 {
                            text.push_str(&format!("{}m and ", (time.as_secs() % 3600) / 60));
                        }

                        // Seconds
                        if time.as_secs() >= 1 {
                            text.push_str(&format!("{}s", time.as_secs() % 60));
                        }

                        text
                    };

                    text.push_str(&format!(
                        "🔁 <b>{0}</b>:\n📺 | <b>EP</b>: <i>{1:02}</i>\n📅 | <b>{2}</b>: <i>{3}</i>\n📆 | <b>{4}</b>: <i>{5}</i>",
                        t("next_airing"),
                        next_airing.episode,
                        t("date"),
                        at.format("%d/%m/%Y %H:%M:%S"),
                        t("time_until"),
                        time_until,
                    ));

                    query
                        .answer()
                        .edit(
                            InputMessage::html(text).reply_markup(&reply_markup::inline(vec![
                                vec![button::inline(
                                    t("reload_btn"),
                                    format!("anime next_airing {0} {1}", anime_id, sender_id),
                                )],
                                vec![button::inline(
                                    t("back_btn"),
                                    format!("anime {0} {1}", anime_id, sender_id),
                                )],
                            ])),
                        )
                        .await?;
                } else {
                    query.answer().alert(t("not_available")).send().await?;
                }
            }
            "staff" => {}
            "chars" => {
                let page = args
                    .get(3)
                    .unwrap_or(&1.to_string())
                    .parse::<usize>()
                    .unwrap();
                let characters = anime.characters();

                let per_page = 10;
                let max_pages = (characters.len() / 15) + 1;

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
                if let Some(tags) = anime.tags.as_mut() {
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

    if let Ok(id) = arg.parse::<i64>() {
        if let Ok(anime) = ani.get_anime(id).await {
            let article = gen_anime_article(&query, anime, &i18n);
            results.push(article.into());
        }
    } else {
        if let Some(result) = ani.search_anime(&arg, offset, 10).await {
            for anime in result {
                let article = gen_anime_article(&query, anime, &i18n);
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
fn gen_anime_article(query: &InlineQuery, anime: Anime, i18n: &I18n) -> inline::query::Article {
    let t = |key: &str| i18n.translate(key);

    let text = utils::gen_anime_info(&anime, &i18n);
    let image_url = ANILIST_BANNER_URL.to_owned() + &anime.id.to_string();

    let sender = query.sender();

    let mut article = inline::query::Article::new(
        if anime.is_adult {
            format!("🔞 {}", anime.title.romaji())
        } else {
            anime.title.romaji().to_string()
        },
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
