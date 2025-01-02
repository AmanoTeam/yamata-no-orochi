// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utility functions.

use rust_anilist::models::{Anime, Format, Manga, Status};

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
        format!("{}...", &text[..max_length])
    } else {
        text.to_string()
    }
}

/// Generates a formatted string containing detailed information about an anime.
///
/// # Arguments
///
/// * `anime` - A reference to an `Anime` struct containing the anime details.
pub fn gen_anime_info(anime: &Anime) -> String {
    let mut text = format!(
        "↓ <code>{0}</code> → <b>{1}</b>\n\n",
        anime.id,
        anime.title.romaji.as_ref().unwrap_or(&anime.title.native),
    );

    text.push_str(&format!(
        "{0} | <b>Status</b>: <i>{1:?}</i>\n",
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
        anime.status
    ));

    text.push_str(&format!(
        "{0} | <b>Format</b>: <i>{1:?}</i>\n",
        match anime.format {
            Format::Tv => "📺",
            Format::Ona => "🎞",
            Format::Ova => "🎞",
            Format::Novel => "📖",
            Format::Manga => "📚",
            Format::Movie => "🎞",
            Format::Music => "🎵",
            Format::OneShot => "📖",
            Format::Special => "🎌",
            Format::TvShort => "📺",
        },
        anime.format
    ));

    if let Some(genres) = anime.genres.as_ref() {
        text.push_str(&format!(
            "🌀 | <b>Genres</b>: <i>{}</i>\n",
            genres.join(", ")
        ));
    }

    text.push_str(&format!(
        "👤 | <b>Popularity</b>: <i>{}</i>\n",
        anime.popularity.unwrap_or(0)
    ));

    if let Some(start_date) = anime.start_date.as_ref() {
        let mut date = String::new();

        if let Some(day) = start_date.day {
            date.push_str(&format!("{:0>2}", day));
        }
        if let Some(month) = start_date.month {
            date.push_str(&format!("/{:0>2}", month));
        }
        if let Some(year) = start_date.year {
            date.push_str(&format!("/{}", year));
        }

        if !date.is_empty() {
            text.push_str(&format!("📅 | <b>Start Date</b>: <i>{}</i>\n", date));
        }
    }
    if let Some(end_date) = anime.end_date.as_ref() {
        let mut date = String::new();

        if let Some(day) = end_date.day {
            date.push_str(&format!("{:0>2}", day));
        }
        if let Some(month) = end_date.month {
            date.push_str(&format!("/{:0>2}", month));
        }
        if let Some(year) = end_date.year {
            date.push_str(&format!("/{}", year));
        }

        if !date.is_empty() {
            text.push_str(&format!("📆 | <b>End Date</b>: <i>{}</i>\n", date));
        }
    }

    text.push_str(&format!(
        "\n<blockquote>{}</blockquote>\n",
        shorten_text(&anime.description, 550).as_str()
    ));

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
pub fn gen_manga_info(manga: &Manga) -> String {
    let mut text = format!(
        "↓ <code>{0}</code> → <b>{1}</b>\n\n",
        manga.id,
        manga.title.romaji.as_ref().unwrap_or(&manga.title.native),
    );

    text.push_str(&format!(
        "🌟 | <b>Score</b>: <i>{}</i>\n",
        manga.average_score.unwrap_or(0)
    ));

    text.push_str(&format!(
        "{0} | <b>Status</b>: <i>{1:?}</i>\n",
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
        manga.status
    ));

    text.push_str(&format!(
        "{0} | <b>Format</b>: <i>{1:?}</i>\n",
        match manga.format {
            Format::Tv => "📺",
            Format::Ona => "🎞",
            Format::Ova => "🎞",
            Format::Novel => "📖",
            Format::Manga => "📚",
            Format::Movie => "🎞",
            Format::Music => "🎵",
            Format::OneShot => "📖",
            Format::Special => "🎌",
            Format::TvShort => "📺",
        },
        manga.format
    ));

    if let Some(genres) = manga.genres.as_ref() {
        text.push_str(&format!(
            "🌀 | <b>Genres</b>: <i>{}</i>\n",
            genres.join(", ")
        ));
    }

    text.push_str(&format!(
        "🔢 | <b>Chapters</b>: <i>{0}</i>\n",
        manga.chapters.unwrap_or(0),
    ));

    text.push_str(&format!(
        "👤 | <b>Popularity</b>: <i>{}</i>\n",
        manga.popularity.unwrap_or(0)
    ));

    if let Some(start_date) = manga.start_date.as_ref() {
        let mut date = String::new();

        if let Some(day) = start_date.day {
            date.push_str(&format!("{:0>2}", day));
        }
        if let Some(month) = start_date.month {
            date.push_str(&format!("/{:0>2}", month));
        }
        if let Some(year) = start_date.year {
            date.push_str(&format!("/{}", year));
        }

        if !date.is_empty() {
            text.push_str(&format!("📅 | <b>Start Date</b>: <i>{}</i>\n", date));
        }
    }
    if let Some(end_date) = manga.end_date.as_ref() {
        let mut date = String::new();

        if let Some(day) = end_date.day {
            date.push_str(&format!("{:0>2}", day));
        }
        if let Some(month) = end_date.month {
            date.push_str(&format!("/{:0>2}", month));
        }
        if let Some(year) = end_date.year {
            date.push_str(&format!("/{}", year));
        }

        if !date.is_empty() {
            text.push_str(&format!("📆 | <b>End Date</b>: <i>{}</i>\n", date));
        }
    }

    text.push_str(&format!(
        "\n<blockquote>{}</blockquote>\n",
        shorten_text(&manga.description, 250).as_str()
    ));

    text.push_str(&format!("\n🔗 | <a href=\"{}\">AniList</a>", manga.url));
    if let Some(id) = manga.id_mal {
        text.push_str(&format!(
            " ↭ <a href=\"https://myanimelist.net/manga/{}\">MyAnimeList</a>",
            id
        ));
    }

    text
}
