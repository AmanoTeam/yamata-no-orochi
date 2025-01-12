// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utility functions.

use grammers_client::button::{self, Inline};
use rust_anilist::models::{Anime, Character, Format, Gender, Manga, Status, User};

use crate::resources::i18n::I18n;

/// Escapes special HTML characters in a given text to their corresponding HTML entities.
///
/// The following replacements are made:
/// - `&` to `&amp;`
/// - `<` to `&lt;`
/// - `>` to `&gt;`
/// - `"` to `&quot;`
/// - `'` to `&#x27;`
/// - `/` to `&#x2F;`
///
/// # Arguments
///
/// * `text` - The text to be escaped. It can be any type that implements the `Into<String>` trait.
pub fn escape_html(text: impl Into<String>) -> String {
    text.into()
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace(r"\", "&quot;")
        .replace("'", "&#x27;")
        .replace("/", "&#x2F;")
}

/// Removes specific HTML tags from the given text.
///
/// This function takes a string input and removes the following HTML tags and chars:
/// `<i>`, `</i>`, `<p>`, `</p>`, `<br>`, `<br/>`, `<br />`, `<em>`, `</em>`, `<li>`, `</li>`,
/// `<ol>`, `</ol>`, `<ul>`, `</ul>`, `<`, `>`, `&quot;`, `&#x27;`, `&#x2F;`.
///
/// # Arguments
///
/// * `text` - A value that can be converted into a `String`.
pub fn remove_html(text: impl Into<String>) -> String {
    text.into()
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<p>", "")
        .replace("</p>", "")
        .replace("<br>", "")
        .replace("<br/>", "")
        .replace("<br />", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("<li>", "• ")
        .replace("</li>", "")
        .replace("<ol>", "")
        .replace("</ol>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#x2F;", "/")
}

/// Shortens a given text to a specified maximum length, appending "..." if truncated.
///
/// # Arguments
///
/// * `text` - The text to be shortened. It can be any type that implements the `ToString` trait.
/// * `max_length` - The maximum length of the resulting string, including the ellipsis.
pub fn shorten_text<T: ToString>(text: T, mut max_length: usize) -> String {
    let text = text.to_string();
    max_length -= 3;

    if text.len() > max_length {
        format!("{}...", text.chars().take(max_length).collect::<String>())
    } else {
        text.to_string()
    }
}

/// Generates a formatted string containing detailed information about an anime.
///
/// # Arguments
///
/// * `anime` - A reference to an `Anime` struct containing the anime details.
/// * `i18n` - A reference to an `I18n` struct containing the translations.
pub fn gen_anime_info(anime: &Anime, i18n: &I18n) -> String {
    let t = |key: &str| i18n.translate(key);

    let mut text = format!(
        "↓ <code>{0}</code> → <b>{1}</b>\n\n",
        anime.id,
        anime.title.romaji(),
    );

    text.push_str(&format!(
        "{0} | <b>{1}</b>: <i>{2}</i>\n",
        match anime.status {
            Status::Hiatus => "🕰",
            Status::Paused => "⏸",
            Status::Current => "✔",
            Status::Dropped => "❌",
            Status::Planning => "📅",
            Status::Finished => "🏁",
            Status::Cancelled => "❌",
            Status::Completed => "🏁",
            Status::Releasing => "📆",
            Status::Repeating => "🔁",
            Status::NotYetReleased => "🔜",
        },
        t("status"),
        anime.status
    ));

    text.push_str(&format!(
        "{0} | <b>{1}</b>: <i>{2}</i>\n",
        match anime.format {
            Format::Tv => "📺",
            Format::Ona => "🎞",
            Format::Ova => "🎞",
            Format::Movie => "🎥",
            Format::Music => "🎵",
            Format::OneShot => "📖",
            Format::Special => "🎌",
            Format::TvShort => "📺",
            _ => "📖",
        },
        t("format"),
        anime.format
    ));

    if let Some(genres) = anime.genres.as_ref() {
        text.push_str(&format!(
            "🎭 | <b>{0}</b>: <i>{1}</i>\n",
            t("genres"),
            genres.join(", ")
        ));
    }

    if let Some(date) = anime.start_date.as_ref() {
        if date.is_valid() {
            text.push_str(&format!(
                "📅 | <b>{0}</b>: <i>{1}</i>\n",
                t("start_date"),
                date.format("{dd}/{mm}/{yyyy}")
            ));
        }
    }
    if let Some(date) = anime.end_date.as_ref() {
        if date.is_valid() {
            text.push_str(&format!(
                "📆 | <b>{0}</b>: <i>{1}</i>\n",
                t("end_date"),
                date.format("{dd}/{mm}/{yyyy}")
            ));
        }
    }

    if !anime.description.is_empty() {
        text.push_str(&format!(
            "\n<blockquote><i>{}</i></blockquote>\n",
            shorten_text(remove_html(&anime.description), 500).as_str()
        ));
    }

    text.push_str(&format!("\n🔗 | <a href=\"{}\">AniList</a>", anime.url));
    if let Some(id) = anime.id_mal {
        text.push_str(&format!(
            " ↭ <a href=\"https://myanimelist.net/anime/{}\">MyAnimeList</a>",
            id
        ));
    }

    text
}

/// Generates a formatted string containing detailed information about a manga.
///
/// # Arguments
///
/// * `manga` - A reference to an `Manga` struct containing the manga details.
/// * `i18n` - A reference to an `I18n` struct containing the translations.
pub fn gen_manga_info(manga: &Manga, i18n: &I18n) -> String {
    let t = |key: &str| i18n.translate(key);

    let mut text = format!(
        "↓ <code>{0}</code> → <b>{1}</b>\n\n",
        manga.id,
        manga.title.romaji(),
    );

    if let Some(average_score) = manga.average_score {
        text.push_str(&format!(
            "🌟 | <b>{0}</b>: <i>{1:02}%</i>\n",
            t("score"),
            average_score
        ));
    }

    text.push_str(&format!(
        "{0} | <b>{1}</b>: <i>{2}</i>\n",
        match manga.status {
            Status::Hiatus => "🕰",
            Status::Paused => "⏸",
            Status::Current => "✔",
            Status::Dropped => "❌",
            Status::Planning => "📅",
            Status::Finished => "🏁",
            Status::Cancelled => "❌",
            Status::Completed => "🏁",
            Status::Releasing => "📆",
            Status::Repeating => "🔁",
            Status::NotYetReleased => "🔜",
        },
        t("status"),
        manga.status
    ));

    text.push_str(&format!(
        "{0} | <b>{1}</b>: <i>{2}</i>\n",
        match manga.format {
            Format::Novel => "📖",
            Format::Manga => "📚",
            Format::Music => "🎵",
            Format::OneShot => "📖",
            Format::Special => "🎌",
            _ => "🎥",
        },
        t("format"),
        manga.format
    ));

    if let Some(genres) = manga.genres.as_ref() {
        text.push_str(&format!(
            "🎭 | <b>{0}</b>: <i>{1}</i>\n",
            t("genres"),
            genres.join(", ")
        ));
    }

    if let Some(chapters) = manga.chapters {
        text.push_str(&format!(
            "🔢 | <b>{0}</b>: <i>{1}</i>\n",
            t("chapters"),
            chapters
        ));
    }

    if let Some(volumes) = manga.volumes {
        text.push_str(&format!(
            "📚 | <b>{0}</b>: <i>{1}</i>\n",
            t("volumes"),
            volumes
        ));
    }

    if let Some(date) = manga.start_date.as_ref() {
        if date.is_valid() {
            text.push_str(&format!(
                "📅 | <b>{0}</b>: <i>{1}</i>\n",
                t("start_date"),
                date.format("{dd}/{mm}/{yyyy}")
            ));
        }
    }
    if let Some(date) = manga.end_date.as_ref() {
        if date.is_valid() {
            text.push_str(&format!(
                "📆 | <b>{0}</b>: <i>{1}</i>\n",
                t("end_date"),
                date.format("{dd}/{mm}/{yyyy}")
            ));
        }
    }

    if !manga.description.is_empty() {
        text.push_str(&format!(
            "\n<blockquote><i>{}</i></blockquote>\n",
            shorten_text(remove_html(&manga.description), 350).as_str()
        ));
    }

    text.push_str(&format!("\n🔗 | <a href=\"{}\">AniList</a>", manga.url));
    if let Some(id) = manga.id_mal {
        text.push_str(&format!(
            " ↭ <a href=\"https://myanimelist.net/manga/{}\">MyAnimeList</a>",
            id
        ));
    }

    text
}

/// Generates a formatted string containing detailed information about a user.
///
/// # Arguments
///
/// * `user` - A reference to an `User` struct containing the user details.
pub fn gen_user_info(user: &User) -> String {
    let mut text = format!("↓ <code>{0}</code> → <b>{1}</b>\n", user.id, user.name);

    if let Some(about) = user.about.as_ref() {
        text.push_str(&format!(
            "\n<blockquote>{}</blockquote>\n",
            shorten_text(about, 250).as_str()
        ));
    }

    text.push_str(&format!(
        "\n🔗 | <a href=\"https://anilist.co/user/{}\">AniList</a>",
        user.id
    ));

    text
}

/// Generates a list of characters with pagination and internationalization support.
///
/// # Arguments
///
/// * `characters` - A slice of `Character` structs to be displayed.
/// * `page` - The current page number for pagination.
/// * `per_page` - The number of characters per page.
/// * `i18n` - A reference to the `I18n` struct for internationalization.
pub fn gen_char_list(
    characters: &[Character],
    page: usize,
    per_page: usize,
    i18n: &I18n,
) -> String {
    let t = |key: &str| i18n.translate(key);

    let mut text = format!("👥 <b>{}</b>:\n", t("characters"));

    let offset = (page - 1) * per_page;

    for character in characters.iter().skip(offset).take(per_page) {
        text.push_str(&format!(
            "{0} | <code>{1}</code>. <b>{2}</b>\n",
            match character.gender.clone().unwrap_or_default() {
                Gender::Male => "👨",
                Gender::Female => "👩",
                Gender::NonBinary => "👨‍👧‍👦",
                Gender::Other(_) => "👨‍👩‍👧‍👦",
            },
            character.id,
            character.name.full()
        ));

        if let Some(role) = character.role.as_ref() {
            text.push_str(&format!("🎭 | <i>{}</i>\n", role));
        }
    }

    text
}

pub fn gen_pagination_buttons(callback: &str, page: usize, max_pages: usize) -> Vec<Inline> {
    let mut buttons = Vec::new();

    for i in 1..=max_pages {
        if (page > 1 && i < (page - 2)) || i > (page + 2) {
            continue;
        }

        buttons.push(button::inline(
            if i < page {
                format!("⬅️ {0}", i)
            } else if i > page {
                format!("{0} ➡️", i)
            } else {
                format!("· {0} ·", i)
            },
            format!("{0} {1}", callback, i),
        ));
    }

    buttons
}
