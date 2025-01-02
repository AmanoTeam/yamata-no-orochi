// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The bot configuration.

use std::io::Read;

use ferogram::Result;
use serde::{Deserialize, Serialize};

/// The path to the configuration file.
const PATH: &str = "./assets/config.toml";

/// The bot configuration.
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Application-related settings.
    pub app: App,
    /// Telegram-related settings.
    pub telegram: Telegram,
}

impl Config {
    /// Load the configuration from the file.
    pub fn load() -> Result<Self> {
        let mut file = std::fs::File::open(PATH)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(toml::from_str::<Self>(&content)?)
    }
}

#[derive(Deserialize, Serialize)]
pub struct App {
    /// The log level.
    pub log_level: String,
    /// The session file path.
    pub session_file: String,
}

/// Telegram-related settings.
#[derive(Deserialize, Serialize)]
pub struct Telegram {
    /// The Telegram API ID.
    pub api_id: i32,
    /// The Telegram API hash>
    pub api_hash: String,
    /// The Telegram bot token.
    pub bot_token: String,
    /// Whether try to get messages received when the bot was offline.
    pub catch_up: bool,
    /// The flood sleep threshold.
    pub flood_sleep_threshold: u32,
}
