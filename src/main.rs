// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The bot.

mod config;
mod middlewares;
pub mod models;
mod plugins;
mod resources;
pub mod utils;

use ferogram::{Client, Injector, Result};
use grammers_client::{types::inline, InputMessage, Update};
use resources::{AniList, Database, I18n};

fn main() -> Result<()> {
    tokio_uring::start(async {
        // Load the configuration.
        let config = config::Config::load()?;

        // Set the log level if it is not set.
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var(
                "RUST_LOG",
                format!("yamata_no_orochi={}", config.app.log_level),
            );
        }

        // Initialize the logger.
        env_logger::init();

        // Initialize the client.
        log::info!("connecting to the telegram server...");

        let client = Client::bot(config.telegram.bot_token)
            .api_id(config.telegram.api_id)
            .api_hash(config.telegram.api_hash)
            .session_file(config.app.session_file)
            .catch_up(config.telegram.catch_up)
            .flood_sleep_threshold(config.telegram.flood_sleep_threshold)
            .set_bot_commands()
            .on_err(|_, update, err| async move {
                match update {
                    Update::NewMessage(message) | Update::MessageEdited(message) => {
                        message
                            .reply(InputMessage::html(format!(
                                "Ocorreu um erro enquanto processávamos sua mensagem:\n\n<blockquote>{}</blockquote>\n\nReporte em @Yonorochi.",
                                err
                            )))
                            .await?;
                    }
                    Update::CallbackQuery(query) => {
                        query
                            .answer()
                            .alert(format!(
                                "Erro:\n{}\n\nReporte em @Yonorochi.",
                                err
                            ))
                            .send()
                            .await?;
                    }
                    Update::InlineQuery(query) => {
                        query
                            .answer(vec![inline::query::Article::new("Erro", InputMessage::html(format!(
                                "Ocorreu um erro enquanto processávamos sua solicitação:\n\n<blockquote>{}</blockquote>\n\nReporte em @Yonorochi.",
                                err
                            ))).description("Ocorreu um erro enquanto processávamos sua solicitação.")])
                            .switch_pm("Reportar erro", "error_report")
                            .send()
                            .await?;
                    }
                    _ => {
                        log::debug!("A update error was not handled: {0}\n{1:?}", err, update);
                    },
                };

                log::error!("An error occurred: {:?}", err);

                Ok(())
            })
            .wait_for_ctrl_c()
            .build_and_connect()
            .await?;

        log::info!("telegram server connected");

        // Initialize the injector.
        let mut injector = Injector::default();

        // Initialize and register the i18n resource.
        let mut i18n = I18n::with_locale("pt");
        i18n.load()?;
        injector.insert(i18n);

        // Initialize and register the AniList resource.
        let anilist = AniList::new();
        injector.insert(anilist);

        // Initialize and register the database resource.
        let database = Database::connect(&config.app.database_url).await;
        database.migrate().await?;
        injector.insert(database);

        // Register the handlers and run the client.
        client
            .dispatcher(|dp| {
                dp.resources(|_| injector)
                    .router(plugins::setup)
                    .middlewares(middlewares::setup)
            })
            .run()
            .await?;

        Ok(())
    })
}
