// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The start plugin.

use ferogram::{filter, handler, Context, Result, Router};
use grammers_client::InputMessage;

use crate::resources::I18n;

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router.register(
        handler::new_message(filter::command("start").description("Start the bot.")).then(start),
    )
}

/// The start command handler.
async fn start(ctx: Context, i18n: I18n) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    ctx.reply(InputMessage::html(t("start"))).await?;

    Ok(())
}
