// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utility functions.

use chrono::{DateTime, Local};
use chrono_humanize::{Accuracy, HumanTime, Tense};
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
/// `<ol>`, `</ol>`, `<ul>`, `</ul>`, `<center>`, `</center>`, `<strong>`, `</strong>`, `<`, `>`,
/// `&quot;`, `&#x27;`, `&#x2F;`.
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
        .replace("<center>", "")
        .replace("</center>", "")
        .replace("<strong>", "")
        .replace("</strong>", "")
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
        "<code>{0}</code> | <b>{1}</b>\n\n",
        anime.id,
        anime.title.romaji(),
    );

    if anime.start_date.is_some() || anime.end_date.is_some() {
        if let Some(date) = anime.start_date.as_ref() {
            if date.is_valid() {
                text.push_str(&format!(
                    "📅 | <b>{0}</b>: <i>{1}</i>",
                    t("date"),
                    date.format("{dd}/{mm}/{yyyy}")
                ));
            }
        }

        if let Some(date) = anime.end_date.as_ref() {
            if date.is_valid() {
                text.push_str(&format!(" - <i>{}</i>", date.format("{dd}/{mm}/{yyyy}")));
            }
        }

        text.push_str("\n");
    }

    if let Some(score) = anime.average_score {
        text.push_str(&format!(
            "🌟 | <b>{0}</b>: <i>{1:02}%</i>\n",
            t("score"),
            score
        ));
    }

    text.push_str(&format!(
        "{0} | <b>{1}</b>: <i>{2}</i>",
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

    if let Some(next_airing) = anime.next_airing_episode.as_ref() {
        let at = DateTime::from_timestamp(next_airing.at, 0)
            .expect("invalid timestamp")
            .time();
        let now = Local::now().time();
        let remaining = now - at;
        let human_time = HumanTime::from(remaining);
        text.push_str(&format!(
            " (<i>E<b>{0}</b> in {1}</i>)",
            next_airing.episode,
            human_time.to_text_en(Accuracy::Rough, Tense::Present)
        ));
    }

    text.push_str("\n");

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
            genres
                .iter()
                .map(|genre| format!("#{}", genre.replace("-", "")))
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }

    if let Some(episodes) = anime.episodes {
        text.push_str(&format!(
            "🎞 | <b>{0}</b>: <i>{1}</i>\n",
            t("episodes"),
            episodes
        ));
    }

    if !anime.description.is_empty() {
        text.push_str(&format!(
            "\n<blockquote expandable><i>{}</i></blockquote>\n",
            shorten_text(remove_html(&anime.description), 500).as_str()
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
        "<code>{0}</code> | <b>{1}</b>\n\n",
        manga.id,
        manga.title.romaji(),
    );

    if manga.start_date.is_some() || manga.end_date.is_some() {
        if let Some(date) = manga.start_date.as_ref() {
            if date.is_valid() {
                text.push_str(&format!(
                    "📅 | <b>{0}</b>: <i>{1}</i>",
                    t("date"),
                    date.format("{dd}/{mm}/{yyyy}")
                ));
            }
        }

        if let Some(date) = manga.end_date.as_ref() {
            if date.is_valid() {
                text.push_str(&format!(" - <i>{}</i>", date.format("{dd}/{mm}/{yyyy}")));
            }
        }

        text.push_str("\n");
    }

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
            genres
                .iter()
                .map(|genre| format!("#{}", genre.replace("-", "")))
                .collect::<Vec<_>>()
                .join(" ")
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

    if !manga.description.is_empty() {
        text.push_str(&format!(
            "\n<blockquote expandable><i>{}</i></blockquote>\n",
            shorten_text(remove_html(&manga.description), 350).as_str()
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
    let mut text = format!("<code>{0}</code> | <b>{1}</b>\n", user.id, user.name);

    if let Some(about) = user.about.as_ref() {
        text.push_str(&format!(
            "\n<blockquote expandable>{}</blockquote>\n",
            shorten_text(remove_html(about), 250)
        ));
    }

    text.push_str(&format!(
        "\n🔗 | <a href=\"https://anilist.co/user/{}\">AniList</a>",
        user.id
    ));

    text
}

/// Generates a formatted string containing detailed information about a character.
///
/// # Arguments
///
/// * `char` - A reference to an `Character` struct containing the character details.
/// * `i18n` - A reference to an `I18n` struct containing the translations.
pub fn gen_char_info(char: &Character, i18n: &I18n) -> String {
    let t = |key: &str| i18n.translate(key);

    let mut text = format!(
        "<code>{0}</code> | <b>{1}</b>\n\n",
        char.id,
        char.name.full()
    );

    if let Some(age) = char.age.as_ref() {
        text.push_str(&format!("🎂 | <b>{}</b>: <i>{}y</i>\n", t("age"), age));
    }

    if let Some(blood_type) = char.blood_type.as_ref() {
        text.push_str(&format!(
            "🩸 | <b>{}</b>: <i>{}</i>\n",
            t("blood_type"),
            blood_type
        ));
    }

    if let Some(date_of_birth) = char.date_of_birth.as_ref() {
        if date_of_birth.is_valid() {
            text.push_str(&format!(
                "📅 | <b>{}</b>: <i>{}</i>\n",
                t("date_of_birth"),
                date_of_birth.format("{dd}/{mm}/{yyyy}")
            ));
        }
    }

    if !char.description.is_empty() {
        text.push_str(&format!(
            "\n<blockquote expandable>{}</blockquote>\n",
            shorten_text(remove_html(&char.description), 250)
        ));
    }

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
