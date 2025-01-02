// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The AniList resource.

use rust_anilist::{
    models::{Anime, Manga},
    Error,
};

/// AniList module.
#[derive(Clone)]
pub struct AniList {
    client: rust_anilist::Client,
}

impl AniList {
    /// Creates a new instance of the AniList resource.
    pub fn new() -> Self {
        Self {
            client: rust_anilist::Client::default(),
        }
    }

    /// Gets an anime by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The anime ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the anime could not be retrieved.
    pub async fn get_anime(&self, id: i64) -> Result<Anime, Error> {
        self.client.get_anime(id).await
    }

    /// Gets a manga by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The manga ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the manga could not be retrieved.
    pub async fn get_manga(&self, id: i64) -> Result<Manga, Error> {
        self.client.get_manga(id).await
    }

    /// Searches for animes by its title.
    ///
    /// # Arguments
    ///
    /// * `title` - The anime title.
    ///
    /// # Errors
    ///
    /// Returns an error if the anime could not be retrieved.
    pub async fn search_anime(&self, title: &str) -> Option<Vec<Anime>> {
        self.client.search_anime(title, 1, 30).await
    }

    /// Searches for mangas by its title.
    ///
    /// # Arguments
    ///
    /// * `title` - The manga title.
    ///
    /// # Errors
    ///
    /// Returns an error if the manga could not be retrieved.
    pub async fn search_manga(&self, _title: &str) -> Option<Vec<Manga>> {
        unimplemented!();
        // self.client.search_manga(title, 1, 30).await
    }
}
