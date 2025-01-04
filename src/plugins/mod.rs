// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Plugins.

use ferogram::Router;

pub mod anime;
pub mod manga;
pub mod start;
pub mod user;

/// The plugins setup.
pub fn setup(router: Router) -> Router {
    router
        .router(start::setup)
        .router(anime::setup)
        .router(manga::setup)
        .router(user::setup)
}
