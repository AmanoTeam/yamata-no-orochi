// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Update chat language middleware.

use async_trait::async_trait;
use ferogram::{
    flow::{self, Flow},
    Context, Injector, Middleware,
};
use grammers_client::{Client, Update};

use crate::{
    models::{Group, NewGroup, NewUser, User},
    resources::{Database, I18n},
};

/// The middleware to update the language of the chat.
#[derive(Clone)]
pub struct UpdateChatLang;

#[async_trait]
impl Middleware for UpdateChatLang {
    async fn handle(&mut self, _: &Client, _: &Update, injector: &mut Injector) -> Flow {
        let db = injector.get::<Database>().unwrap();
        let ctx = injector.get::<Context>().unwrap();
        let i18n = injector.get::<I18n>().unwrap();

        let pool = db.pool();

        if ctx.is_private() {
            if let Some(sender) = ctx.sender() {
                match User::get_by_id(pool, &sender.id()).await {
                    Ok(Some(user)) => {
                        i18n.set_locale(user.language_code);
                    }
                    Ok(None) => {
                        let new_user = NewUser::new(sender.id(), "pt".to_string());
                        match new_user.create(pool).await {
                            Ok(user) => {
                                log::debug!("created a new user: {:?}", user)
                            }
                            Err(e) => {
                                log::error!(
                                    "failed to create a new user {:?} with error {:?}",
                                    new_user,
                                    e
                                )
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "failed to get user by id {:?} with error {:?}",
                            sender.id(),
                            e
                        )
                    }
                }
            }
        } else {
            if let Some(chat) = ctx.chat() {
                match Group::get_by_id(pool, &chat.id()).await {
                    Ok(Some(group)) => {
                        i18n.set_locale(group.language_code);
                    }
                    Ok(None) => {
                        let new_group = NewGroup::new(chat.id(), "pt".to_string());
                        match new_group.create(pool).await {
                            Ok(group) => {
                                log::debug!("created a new group: {:?}", group)
                            }
                            Err(e) => {
                                log::error!(
                                    "failed to create a new group {:?} with error {:?}",
                                    new_group,
                                    e
                                )
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "failed to get group by id {:?} with error {:?}",
                            chat.id(),
                            e
                        )
                    }
                }
            }
        }

        flow::continue_now()
    }
}
