// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Resources.

pub mod anilist;
pub mod cache;
pub mod database;
pub mod i18n;

pub use anilist::AniList;
pub use cache::Cache;
pub use database::Database;
pub use i18n::I18n;
