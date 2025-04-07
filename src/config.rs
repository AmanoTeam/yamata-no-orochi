// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The bot configuration.

use std::io::{Read, Write};

use ferogram::{Result, utils::prompt};
use serde::{Deserialize, Serialize};

/// The path to the configuration file.
const PATH: &str = "./assets/config.toml";

/// The configuration.
#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    /// Application-related settings.
    pub app: App,
    /// Anilist-related settings.
    pub anilist: Anilist,
    /// Telegram-related settings.
    pub telegram: Telegram,
}

impl Config {
    /// Load the configuration from the file.
    pub fn load() -> Result<Self> {
        if let Ok(mut file) = std::fs::File::open(PATH) {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .expect("failed to read config file");

            Ok(toml::from_str::<Self>(&content).expect("failed to parse config file"))
        } else {
            let answer = prompt("Config file not found. Create a new one? (y/N) ", false)
                .expect("failed to read input");

            match answer.to_lowercase().trim() {
                "y" | "yes" => {
                    println!("Creating a new config file at {:?}", PATH);

                    let mut file =
                        std::fs::File::create(PATH).expect("failed to create config file");

                    let config = Self {
                        app: App {
                            log_level: "trace".to_string(),
                            database_url: "postgres://username:password@host:port/database"
                                .to_string(),
                            session_file: "./assets/bot.session".to_string(),
                        },
                        anilist: Anilist {
                            client_id: 12345,
                            client_secret: "YOUR_CLIENT_SECRET_HERE".to_string(),
                        },
                        telegram: Telegram {
                            api_id: 1234567,
                            api_hash: "YOUR_API_HASH_HERE".to_string(),
                            bot_token: "YOUR_BOT_TOKEN_HERE".to_string(),
                            catch_up: false,
                            flood_sleep_threshold: 180,
                        },
                    };
                    let content = toml::to_string_pretty(&config).expect("failed to serialize");
                    file.write_all(content.as_bytes())
                        .expect("failed to write config file");

                    println!("Config file created. Please edit it and run the bot again.");

                    std::process::exit(0);
                }
                _ => {
                    eprintln!("Aborting.");
                    std::process::exit(1);
                }
            }
        }
    }
}

/// Application-related settings.
#[derive(Clone, Deserialize, Serialize)]
pub struct App {
    /// The log level.
    pub log_level: String,
    /// The database URL.
    pub database_url: String,
    /// The session file path.
    pub session_file: String,
}

/// Anilist-related settings.
#[derive(Clone, Deserialize, Serialize)]
pub struct Anilist {
    /// The Anilist client ID.
    pub client_id: i32,
    /// The Anilist client secret.
    pub client_secret: String,
}

/// Telegram-related settings.
#[derive(Clone, Deserialize, Serialize)]
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
