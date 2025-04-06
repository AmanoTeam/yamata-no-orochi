// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The auth plugin.

use ferogram::{Context, Result, Router, filter, handler};
use grammers_client::{InputMessage, button, reply_markup, types::Chat};

use crate::{
    Config,
    models::User,
    resources::{Database, I18n},
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router.register(
        handler::new_message(filter::command("auth").description("Authenticate with AniList."))
            .then(auth),
    )
}

/// The auth handler.
async fn auth(ctx: Context, db: Database, i18n: I18n, config: Config) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let pool = db.pool();

    let sender = ctx.sender();
    if let Some(Chat::User(u)) = sender {
        if let Some(user) = User::get_by_id(pool, &u.id()).await? {
            if user.anilist_token.is_some() {
                ctx.reply(InputMessage::html(t("already_authenticated")).reply_markup(
                    &reply_markup::inline(vec![vec![
                        button::inline(t("disconnect_btn"), "auth revoke"),
                        button::inline(t("profile_btn"), format!("user {}", user.id)),
                    ]]),
                ))
                .await?;
            } else {
                ctx.reply(InputMessage::html(t("authenticate")).reply_markup(
                    &reply_markup::inline(vec![vec![button::url(
                        t("authenticate_btn"),
                        format!("https://anilist.co/api/v2/oauth/authorize?client_id={0}&response_type=code&redirect_uri=http://amanoteam.github.io/yamata-no-orochi", config.anilist.client_id),
                    )]]),
                ))
                .await?;
            }
        }
    } else {
        ctx.reply(InputMessage::html(t("only_user_command")))
            .await?;
    }

    Ok(())
}
