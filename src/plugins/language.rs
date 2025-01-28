// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Language plugin.

use ferogram::{
    filter, handler,
    utils::{bytes_to_string, split_btns_into_columns},
    Context, Filter, Result, Router,
};
use grammers_client::{button, reply_markup, types::Chat, InputMessage};
use maplit::hashmap;

use crate::{
    models::{group::UpdateGroup, Group, UpdateUser, User},
    resources::{Database, I18n},
};

/// Language plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .handler(
            handler::new_message(
                filter::commands(&["lang", "language"])
                    .description("Change the bot language.")
                    .and(filter::administrator),
            )
            .then(language),
        )
        .handler(
            handler::callback_query(filter::regex("^language$").and(filter::administrator))
                .then(language),
        )
        .handler(
            handler::callback_query(
                filter::regex(r"^language set (\w+)$").and(filter::administrator),
            )
            .then(language_set),
        )
}

/// The language command handler.
async fn language(ctx: Context, i18n: I18n) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    let locales = i18n.locales();

    let buttons = locales
        .iter()
        .map(|locale| {
            button::inline(
                format!(
                    "{0} {1} {2}",
                    i18n.translate_from_locale("_FLAG", locale),
                    i18n.translate_from_locale("_NAME", locale),
                    if *locale == i18n.locale() { "✔" } else { "" },
                ),
                format!("language set {}", locale),
            )
        })
        .collect::<Vec<_>>();
    let buttons = split_btns_into_columns(buttons, 2);

    ctx.edit_or_reply(
        InputMessage::html(t("language")).reply_markup(&reply_markup::inline(buttons)),
    )
    .await?;

    Ok(())
}

/// The language set callback handler.
async fn language_set(ctx: Context, db: Database, i18n: I18n) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let t_a = |key: &str, args| i18n.translate_with_args(key, args);
    let pool = db.pool();

    let query = ctx.callback_query().unwrap();

    let chat = query.chat();
    let data = bytes_to_string(query.data());
    let args = data.split_whitespace().skip(2).collect::<Vec<_>>();

    let language_code = args[0];
    if language_code == i18n.locale() {
        query
            .answer()
            .alert(t_a(
                "already_language",
                hashmap! { "language" => i18n.translate_from_locale("_NAME", language_code) },
            ))
            .send()
            .await?;
        return Ok(());
    }

    let mut success = false;
    if let Chat::User(_) = chat {
        if let Some(user) = User::get_by_id(pool, &chat.id()).await? {
            let mut update_user: UpdateUser = user.into();
            update_user.language_code = language_code.to_string();
            update_user.update(pool).await?;

            success = true;
        } else {
            log::warn!("user not found: {}", chat.id());
        }
    } else {
        if let Some(group) = Group::get_by_id(pool, &chat.id()).await? {
            let mut update_group: UpdateGroup = group.into();
            update_group.language_code = language_code.to_string();
            update_group.update(pool).await?;

            success = true;
        } else {
            log::warn!("group not found: {}", chat.id());
        }
    }

    if success {
        query
            .answer()
            .edit(InputMessage::html(t_a(
                "new_language",
                hashmap! { "new_language" => i18n.translate_from_locale("_NAME", language_code) },
            )).reply_markup(&reply_markup::inline(vec![vec![button::inline(t("back_btn"), "language")]])))
            .await?;
        i18n.set_locale(language_code);
    }

    Ok(())
}
