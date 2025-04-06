// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Plugins.

use ferogram::Router;

mod anime;
mod auth;
mod character;
mod inline;
mod language;
mod manga;
mod ping;
mod start;
mod user;

/// The plugins setup.
pub fn setup(router: Router) -> Router {
    router
        .extend(ping::setup)
        .extend(start::setup)
        .extend(language::setup)
        .extend(anime::setup)
        .extend(manga::setup)
        .extend(user::setup)
        .extend(character::setup)
        .extend(inline::setup)
        .extend(auth::setup)
}
