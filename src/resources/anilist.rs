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

use super::cache::Cache;

/// AniList module.
#[derive(Clone)]
pub struct AniList {
    /// The AniList client.
    client: Arc<rust_anilist::Client>,
    /// The cache.
    cache: Arc<Cache<i64, Cached>>,
}

impl AniList {
    /// Creates a new instance of the AniList resource.
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::with_timeout(Duration::from_secs(15))),
            cache: Arc::new(Cache::new()),
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
        if let Some(Cached::Anime(anime)) = self.cache.get(&id).await {
            Ok(anime)
        } else {
            if let Ok(anime) = self.client.get_anime(id).await {
                self.cache.insert(id, Cached::Anime(anime.clone())).await;

                Ok(anime)
            } else {
                Err(Error::InvalidId)
            }
        }
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
        if let Some(Cached::Manga(manga)) = self.cache.get(&id).await {
            Ok(manga)
        } else {
            if let Ok(manga) = self.client.get_manga(id).await {
                self.cache.insert(id, Cached::Manga(manga.clone())).await;

                Ok(manga)
            } else {
                Err(Error::InvalidId)
            }
        }
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
        if let Some(Cached::User(user)) = self.cache.get(&(id as i64)).await {
            Ok(user)
        } else {
            if let Ok(user) = self.client.get_user(id).await {
                self.cache
                    .insert(id as i64, Cached::User(user.clone()))
                    .await;
                Ok(user)
            } else {
                Err(Error::InvalidId)
            }
        }
    }

    /// Searches for animes by its title.
    ///
    /// # Arguments
    ///
    /// * `title` - The anime title.
    /// * `page` - The page number.
    /// * `limit` - The number of results per page.
    ///
    /// # Errors
    ///
    /// Returns an error if the anime could not be retrieved.
    pub async fn search_anime(&self, title: &str, page: u16, limit: u16) -> Option<Vec<Anime>> {
        self.client.search_anime(title, page, limit).await
    }

    /// Searches for mangas by its title.
    ///
    /// # Arguments
    ///
    /// * `title` - The manga title.
    /// * `page` - The page number.
    /// * `limit` - The number of results per page.
    ///
    /// # Errors
    ///
    /// Returns an error if the manga could not be retrieved.
    pub async fn search_manga(&self, title: &str, page: u16, limit: u16) -> Option<Vec<Manga>> {
        self.client.search_manga(title, page, limit).await
    }

    /// Searches for users by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The user name.
    /// * `page` - The page number.
    /// * `limit` - The number of results per page.
    ///
    /// # Errors
    ///
    /// Returns an error if the user could not be retrieved.
    pub async fn search_user(&self, name: &str, page: u16, limit: u16) -> Option<Vec<User>> {
        self.client.search_user(name, page, limit).await
    }
}

/// A cached value.
#[derive(Clone)]
pub enum Cached {
    /// An anime.
    Anime(Anime),
    /// A manga.
    Manga(Manga),
    /// An user
    User(User),
}
