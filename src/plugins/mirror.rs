// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The mirror plugin.

use std::time::Duration;

use ferogram::{Context, Result, Router, filter, handler};
use grammers_client::{
    InputMessage, button, reply_markup,
    types::{CallbackQuery, Chat},
};
use maplit::hashmap;

use crate::{
    Config,
    models::{Mirror, NewMirror, User, mirror::State},
    plugins,
    resources::{Database, I18n, MirrorManager},
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .register(
            handler::new_message(filter::command("mirror").description("Manage your mirrors."))
                .then(mirror),
        )
        .register(handler::callback_query(filter::regex("mirror create")).then(mirror_create))
}

/// The mirror command handler.
async fn mirror(ctx: Context, db: Database, i18n: I18n, config: Config) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let t_a = |key: &str, args| i18n.translate_with_args(key, args);
    let pool = db.pool();

    if let Some(Chat::User(u)) = ctx.sender() {
        if !ctx.is_private() {
            ctx.reply(InputMessage::html(t("only_private"))).await?;
            return Ok(());
        }

        if let Some(user) = User::get_by_id(pool, &u.id()).await? {
            if user.anilist_token.is_some() {
                let mirrors = Mirror::get_by_owner_id(pool, &u.id()).await?;

                let buttons = mirrors
                    .iter()
                    .map(|mirror| {
                        button::inline(
                            format!(
                                "ℹ️ {} {}",
                                mirror.id,
                                match mirror.state {
                                    State::Active => "✔️",
                                    State::Restricted => "🔒",
                                    State::Disabled => "❌",
                                }
                            ),
                            format!("mirror info {}", mirror.id),
                        )
                    })
                    .collect::<Vec<_>>();
                let mut buttons = vec![buttons];

                buttons.push(vec![button::inline(t("create_btn"), "mirror create")]);

                ctx.edit_or_reply(
                    InputMessage::html(t_a("mirrors_owned", hashmap! {"count" => mirrors.len()}))
                        .reply_markup(&reply_markup::inline(buttons)),
                )
                .await?;
            } else {
                if let Some(message) = ctx.message().await {
                    return plugins::auth::auth(message, db, i18n, config).await;
                }
            }
        }
    }

    Ok(())
}

/// The mirror create callback handler.
async fn mirror_create(
    ctx: Context,
    callback: CallbackQuery,
    db: Database,
    i18n: I18n,
    config: Config,
    mirror_manager: MirrorManager,
) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let t_a = |key: &str, args| i18n.translate_with_args(key, args);
    let pool = db.pool();

    let sender = callback.sender();
    let message = callback.load_message().await?;

    if let Some(user) = User::get_by_id(pool, &sender.id()).await? {
        if user.anilist_token.is_some() {
            let sent = message
                .reply(InputMessage::html(t("send_mirror_token")))
                .await?;

            if let Ok(reply) = ctx.wait_for_message(None).await {
                let token = reply.text();
                sent.edit(InputMessage::html(t_a(
                    "validating_mirror_token",
                    hashmap! {"token" => token},
                )))
                .await?;

                reply.delete().await?;

                if mirror_manager.validate_token(token).await? {
                    let new_mirror = NewMirror::new(sender.id(), token.to_string());

                    if let Ok(m) = new_mirror.create(pool).await {
                        let id = m.id.to_string();

                        mirror_manager.push_mirror(m).await;

                        sent.edit(InputMessage::html(t_a(
                            "mirror_token_validated",
                            hashmap! {"id" => id.as_str()},
                        )))
                        .await?;

                        tokio::time::sleep(Duration::from_secs(3)).await;
                        sent.delete().await?;

                        return mirror(ctx, db, i18n, config).await;
                    } else {
                        sent.edit(InputMessage::html(t("failed_to_create_mirror")))
                            .await?;

                        tokio::time::sleep(Duration::from_secs(3)).await;
                        sent.delete().await?;
                    }
                } else {
                    sent.edit(InputMessage::html(t("failed_to_validate_mirror_token")))
                        .await?;

                    tokio::time::sleep(Duration::from_secs(3)).await;
                    sent.delete().await?;
                }
            } else {
                sent.delete().await?;
                callback.answer().alert(t("times_up")).send().await?;
            }
        } else {
            callback
                .answer()
                .text(t("not_authenticated"))
                .send()
                .await?;
        }
    }

    callback.answer().send().await?;

    Ok(())
}
