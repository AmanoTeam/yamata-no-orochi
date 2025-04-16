// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! AniList middleware.

use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use ferogram::{
    Context, Injector, Middleware,
    flow::{self, Flow},
};
use grammers_client::{Client, Update};

use crate::{
    models::User,
    resources::{AniList, Cache, Database},
};

/// The middleware to update the Anilist client token.
#[derive(Clone)]
pub struct AuthenticateAniList {
    clients: Cache<i64, Arc<rust_anilist::Client>>,
}

impl AuthenticateAniList {
    /// Creates a new instance of the middleware.
    pub fn new() -> Self {
        Self {
            clients: Cache::with_capacity(50),
        }
    }
}

#[async_trait]
impl Middleware for AuthenticateAniList {
    async fn handle(&mut self, _: &Client, _: &Update, injector: &mut Injector) -> Flow {
        let mut ani = (*injector.take::<AniList>().unwrap()).clone();

        let db = injector.get::<Database>().unwrap();
        let ctx = injector.get::<Context>().unwrap();

        let pool = db.pool();
        if let Some(sender) = ctx.sender() {
            if let Ok(Some(user)) = User::get_by_id(pool, &sender.id()).await {
                if let Some(client) = self.clients.get(&user.id) {
                    ani.client = client.clone();
                } else {
                    log::debug!("creating a new Anilist client for user {:?}", user.id);

                    let client = Arc::new(if let Some(token) = user.anilist_token {
                        rust_anilist::Client::with_token(&token).timeout(Duration::from_secs(15))
                    } else {
                        rust_anilist::Client::with_timeout(Duration::from_secs(15))
                    });

                    self.clients.insert(user.id, Arc::clone(&client)).await;
                    ani.client = client;
                }
            }
        } else {
            log::debug!("creating a new Anilist client for anonymous user");

            ani.client = Arc::new(rust_anilist::Client::with_timeout(Duration::from_secs(15)));
        }

        injector.insert(ani);

        flow::continue_now()
    }
}
