// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Middlewares.

mod update_chat_lang;

use update_chat_lang::UpdateChatLang;

use ferogram::MiddlewareStack;

/// The middlewares setup.
pub fn setup(stack: MiddlewareStack) -> MiddlewareStack {
    stack.before(UpdateChatLang)
}
