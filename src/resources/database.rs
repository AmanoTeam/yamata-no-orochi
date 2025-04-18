// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The database resource.

use std::path::Path;

use ferogram::Result;
use sqlx::{PgPool, migrate::Migrator};
use tokio::fs::read_dir;

/// Where the migrations are located.
const MIGRATIONS_PATH: &str = "./assets/migrations/";

/// Database module.
#[derive(Clone)]
pub struct Database {
    /// The database pool.
    pool: PgPool,
}

impl Database {
    /// Connects to the database.
    ///
    /// # Arguments
    ///
    /// * `database_url` - The connection string.
    pub async fn connect(database_url: &str) -> Self {
        log::info!("connecting to the database...");

        let pool = PgPool::connect(database_url)
            .await
            .expect("failed to connect to the database.");

        log::info!("database connected");

        Self { pool }
    }

    /// Gets the database pool.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Migrates the database.
    ///
    /// Search for migrations in the `assets/migrations` folder.
    ///
    /// # Errors
    ///
    /// Returns an error if the migration fails.
    pub async fn migrate(&self) -> Result<()> {
        log::debug!("searching migrations from: {:?}", MIGRATIONS_PATH);

        let mut dir = read_dir(MIGRATIONS_PATH)
            .await
            .expect("failed to read migrations directory");
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            files.push(child);
        }

        if files.is_empty() {
            log::warn!("no migrations found");
            return Ok(());
        } else {
            log::debug!(
                "found migrations: {}",
                files
                    .into_iter()
                    .map(|entry| entry
                        .file_name()
                        .into_string()
                        .expect("failed to parse file name"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        log::debug!("migrating the database...");

        let migrator = Migrator::new(Path::new(MIGRATIONS_PATH)).await?;
        let result = migrator.run(&self.pool).await.map_err(Into::into);
        if result.is_ok() {
            log::debug!("database migrated");
        }

        result
    }
}
