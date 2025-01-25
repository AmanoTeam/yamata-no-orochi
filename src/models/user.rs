// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The user model.

use chrono::{DateTime, Utc};
use sqlx::{FromRow, Row};
use tiny_orm::Table;

/// The user model.
#[derive(Debug, FromRow, Table, Clone)]
#[tiny_orm(table_name = "users")]
pub struct User {
    /// The user's ID.
    #[tiny_orm(primary_key)]
    pub id: i64,
    /// The user's Anilist ID.
    pub anilist_id: Option<i32>,
    /// The user's created at date.
    pub created_at: DateTime<Utc>,
    /// The user's updated at date.
    pub updated_at: DateTime<Utc>,
}

/// The new user model.
#[derive(Debug, FromRow, Table, Clone)]
pub struct NewUser {
    /// The user's ID.
    pub id: i64,
    /// The user's Anilist ID.
    pub anilist_id: Option<i32>,
}

impl NewUser {
    /// Creates a new user.
    ///
    /// # Arguments
    ///
    /// * `id` - The user's ID.
    /// * `anilist_id` - The user's Anilist ID.
    pub fn new(id: i64, anilist_id: Option<i32>) -> Self {
        Self { id, anilist_id }
    }
}

/// The update user model.
#[derive(Debug, FromRow, Table, Clone)]
pub struct UpdateUser {
    /// The user's ID.
    pub id: i64,
    /// The user's Anilist ID.
    pub anilist_id: Option<i32>,
}

impl From<User> for UpdateUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            anilist_id: user.anilist_id,
        }
    }
}
