// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The ping plugin.

use std::time::Instant;

use ferogram::{filter, handler, Result, Router};
use grammers_client::{grammers_tl_types as tl, types::Message, Client, InputMessage};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router.register(
        handler::new_message(filter::command("ping").description("Ping the bot.")).then(ping),
    )
}

/// The ping command handler.
async fn ping(client: Client, message: Message) -> Result<()> {
    let sent = message.reply(InputMessage::html("<b>Ping</b>...")).await?;

    let start = Instant::now();
    client
        .invoke(&tl::functions::Ping {
            ping_id: rand::random(),
        })
        .await?;
    let elapsed = start.elapsed().as_millis();

    sent.edit(InputMessage::html(format!(
        "<b>Ping</b>... <b>Pong</b>! <code>{}</code>ms.",
        elapsed
    )))
    .await?;

    Ok(())
}
