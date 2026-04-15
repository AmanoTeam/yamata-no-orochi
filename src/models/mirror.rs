// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The mirror model.

use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Postgres, Type};
use tiny_orm::Table;
use uuid::Uuid;

/// The mirror model.
#[derive(Clone, Debug, FromRow, Table)]
#[tiny_orm(table_name = "mirrors")]
pub struct Mirror {
    /// The mirror's ID.
    #[tiny_orm(primary_key)]
    pub id: Uuid,
    /// The mirror's owner ID.
    pub owner_id: i64,
    /// The mirror's bot token.
    pub token: String,
    /// The mirror's state.
    pub state: State,
    /// The mirror's created at date.
    pub created_at: DateTime<Utc>,
    /// The mirror's updated at date.
    pub updated_at: DateTime<Utc>,
}

impl Mirror {
    /// Gets the mirrors by its owner ID.
    pub async fn get_by_owner_id(db: &PgPool, owner_id: &i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<Postgres, Self>(&format!(
            "SELECT * FROM mirrors WHERE owner_id = {}",
            owner_id
        ))
        .fetch_all(db)
        .await
    }
}

/// The new mirror model.
#[derive(Debug, FromRow, Table, Clone)]
#[tiny_orm(table_name = "mirrors")]
pub struct NewMirror {
    /// The mirror's owner ID.
    pub owner_id: i64,
    /// The mirror's bot token.
    pub token: String,
    /// The mirror's state.
    pub state: State,
}

impl NewMirror {
    /// Creates a new mirror.
    ///
    /// # Arguments
    ///
    /// * `owner_id` - The mirror's owner ID.
    /// * `token` - The mirror's bot token.
    pub fn new(owner_id: i64, token: String) -> Self {
        Self {
            owner_id,
            token,
            state: State::Disabled,
        }
    }
}

/// The mirror's state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum State {
    /// The active state.
    Active,
    /// The restricted state.
    Restricted,
    /// The disabled state.
    Disabled,
}
