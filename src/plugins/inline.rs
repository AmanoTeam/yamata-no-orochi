// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The start plugin.

use ferogram::{Result, Router, filter, handler};
use grammers_client::{
    InputMessage,
    types::{InlineQuery, inline},
};

use crate::resources::I18n;

/// The plugin setup.
pub fn setup(router: Router) -> Router {
    router.register(handler::inline_query(filter::always).then(inline))
}

/// The inline handler.
async fn inline(query: InlineQuery, i18n: I18n) -> Result<()> {
    let t = |key: &str| i18n.translate(key);

    query
        .answer(vec![
            inline::query::Article::new(
                t("how_to_use_inline"),
                InputMessage::html(t("how_to_use_inline_text")),
            )
            .description(t("click_for_more_info")),
        ])
        .cache_time(60)
        .private()
        .send()
        .await?;

    Ok(())
}
