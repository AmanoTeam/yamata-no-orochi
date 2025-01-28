// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The group model.

use chrono::{DateTime, Utc};
use sqlx::{FromRow, Row};
use tiny_orm::Table;

/// The group model.
#[derive(Debug, FromRow, Table, Clone)]
#[tiny_orm(table_name = "groups")]
pub struct Group {
    /// The group's ID.
    #[tiny_orm(primary_key)]
    pub id: i64,
    /// The group's langauge code.
    pub language_code: String,
    /// The group's created at date.
    pub created_at: DateTime<Utc>,
    /// The group's updated at date.
    pub updated_at: DateTime<Utc>,
}

/// The new group model.
#[derive(Debug, FromRow, Table, Clone)]
#[tiny_orm(table_name = "groups")]
pub struct NewGroup {
    /// The group's ID.
    pub id: i64,
    /// The group's langauge code.
    pub language_code: String,
}

impl NewGroup {
    /// Creates a new group.
    ///
    /// # Arguments
    ///
    /// * `id` - The group's ID.
    /// * `language_code` - The group's language code.
    pub fn new(id: i64, language_code: String) -> Self {
        Self { id, language_code }
    }
}

/// The update group model.
#[derive(Debug, FromRow, Table, Clone)]
#[tiny_orm(table_name = "groups")]
pub struct UpdateGroup {
    /// The group's ID.
    pub id: i64,
    /// The group's langauge code.
    pub language_code: String,
}

impl From<Group> for UpdateGroup {
    fn from(group: Group) -> Self {
        Self {
            id: group.id,
            language_code: group.language_code,
        }
    }
}
