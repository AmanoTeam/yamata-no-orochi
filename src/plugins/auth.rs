// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The auth plugin.

use base64::Engine;
use ferogram::{Result, Router, filter, handler};
use grammers_client::{
    InputMessage, Update, button, reply_markup,
    types::{Chat, Message},
};
use maplit::hashmap;
use serde::{Deserialize, Serialize};

use crate::{
    Config,
    models::{UpdateUser, User},
    resources::{Database, I18n},
};

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router
        .register(
            handler::new_message(filter::command("auth").description("Authenticate with AniList."))
                .then(auth),
        )
        .register(
            handler::new_update(filter::always).then(|update: Update| async move {
                println!("{:?}", update);

                Ok(())
            }),
        )
}

/// The auth handler.
async fn auth(message: Message, db: Database, i18n: I18n, config: Config) -> Result<()> {
    let t = |key: &str| i18n.translate(key);
    let t_a = |key: &str, args| i18n.translate_with_args(key, args);
    let pool = db.pool();

    let args = message
        .text()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>();
    let sender = message.sender();

    if let Some(Chat::User(u)) = sender {
        if let Some(user) = User::get_by_id(pool, &u.id()).await? {
            if user.anilist_token.is_some() {
                message
                    .reply(InputMessage::html(t("already_authenticated")).reply_markup(
                        &reply_markup::inline(vec![vec![
                            button::inline(t("disconnect_btn"), "auth revoke"),
                            button::inline(
                                t("profile_btn"),
                                format!("user {}", user.anilist_id.unwrap_or(0)),
                            ),
                        ]]),
                    ))
                    .await?;
            } else {
                if args.is_empty() {
                    message.reply(InputMessage::html(t("authenticate")).reply_markup(
                        &reply_markup::inline(vec![vec![button::webview(
                            t("authenticate_btn"),
                            format!("https://anilist.co/api/v2/oauth/authorize?client_id={0}&response_type=code&redirect_uri=https://yamata-no-orochi.vercel.app/auth", config.anilist.client_id),
                        )]]),
                    ))
                    .await?;
                } else {
                    message.delete().await?;

                    let code = args[0];
                    let mut response = surf::post("https://anilist.co/api/v2/oauth/token")
                        .header("content-type", "application/json")
                        .header("accept", "application/json")
                        .body_json(&Body {
                            grant_type: "authorization_code".to_string(),
                            client_id: config.anilist.client_id.clone(),
                            client_secret: config.anilist.client_secret.clone(),
                            redirect_uri: "https://yamata-no-orochi.vercel.app/auth".to_string(),
                            code: code.to_string(),
                        })?
                        .await?;

                    let ani_res = response.body_json::<Response>().await?;

                    if response.status().is_success() {
                        if let Some(token) = ani_res.access_token {
                            let splitted_token = token.split(".").collect::<Vec<_>>();

                            let body = splitted_token.get(1).expect("failed to get token body");
                            let body = base64::engine::general_purpose::STANDARD
                                .decode(body)
                                .expect("failed to decode token body");
                            let claims = serde_json::from_slice::<Claims>(&body)
                                .expect("failed to parse token body");

                            let ani_id = claims
                                .sub
                                .parse::<i32>()
                                .expect("failed to parse user's AniList ID");

                            message
                                .reply(
                                    InputMessage::html(t("authentication_success")).reply_markup(
                                        &reply_markup::inline(vec![vec![button::inline(
                                            t("profile_btn"),
                                            format!("user {}", ani_id),
                                        )]]),
                                    ),
                                )
                                .await?;

                            let mut update_user: UpdateUser = user.into();
                            update_user.anilist_id = Some(ani_id);
                            update_user.anilist_token = Some(token);
                            update_user.update(pool).await?;
                        } else {
                            message
                                .reply(InputMessage::html(t_a("authentication_failed", hashmap! { "error" => "No token received from AniList".to_string()})))
                                .await?;
                        }
                    } else {
                        message
                            .reply(InputMessage::html(t_a("authentication_failed", hashmap! { "error" => ani_res.error.unwrap_or("Unknown error".to_string())})))
                            .await?;
                    }
                }
            }
        }
    } else {
        message
            .reply(InputMessage::html(t("only_user_command")))
            .await?;
    }

    Ok(())
}

pub fn authenticate_anilist_account() {}

/// The body of the request to the AniList API.
#[derive(Serialize)]
struct Body {
    /// The grant type of the request.
    grant_type: String,
    /// The client ID of the AniList API.
    client_id: i32,
    /// The client secret of the AniList API.
    client_secret: String,
    /// The redirect URI of the AniList API.
    redirect_uri: String,
    /// The code of the request.
    code: String,
}

/// The response from the AniList API.
#[derive(Deserialize)]
struct Response {
    /// The error message from the AniList API.
    #[serde(rename = "hint")]
    pub error: Option<String>,
    /// The access token from the AniList API.
    pub access_token: Option<String>,
}

/// The claims of the JWT token.
#[derive(Debug, Deserialize)]
struct Claims {
    /// The user's AniList ID.
    sub: String,
}
