// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The AniList resource.

use std::{sync::Arc, time::Duration};

use rust_anilist::{
    models::{Anime, Manga, User},
    Client, Error,
};

/// AniList module.
#[derive(Clone)]
pub struct AniList {
    client: Arc<rust_anilist::Client>,
}

impl AniList {
    /// Creates a new instance of the AniList resource.
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::with_timeout(Duration::from_secs(15))),
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

    /// Gets a user by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The user ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the user could not be retrieved.
    pub async fn get_user(&self, id: i32) -> Result<User, Error> {
        self.client.get_user(id).await
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
        self.client.search_anime(title, 1, 6).await
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
    pub async fn search_manga(&self, title: &str) -> Option<Vec<Manga>> {
        self.client.search_manga(title, 1, 6).await
    }

    /// Searches for users by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The user name.
    ///
    /// # Errors
    ///
    /// Returns an error if the user could not be retrieved.
    pub async fn search_user(&self, name: &str) -> Option<Vec<User>> {
        self.client.search_user(name, 1, 6).await
    }
}
