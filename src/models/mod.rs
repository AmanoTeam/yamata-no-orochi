// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Database models.

pub mod group;
pub mod mirror;
pub mod user;

pub use group::{Group, NewGroup, UpdateGroup};
pub use mirror::{Mirror, NewMirror};
pub use user::{NewUser, UpdateUser, User};
