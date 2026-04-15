// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Mirror Manager resource.

use std::sync::Arc;

use ferogram::Result;
use tokio::sync::RwLock;

use crate::models::Mirror;

/// Mirror Manager module.
#[derive(Clone)]
pub struct MirrorManager {
    /// The mirrors.
    pub mirrors: Arc<RwLock<Vec<Mirror>>>,
}

impl MirrorManager {
    /// Creates a new instance of the Mirror Manager resource.
    pub fn new(mirrors: Vec<Mirror>) -> Self {
        Self {
            mirrors: Arc::new(RwLock::new(mirrors)),
        }
    }

    /// Pushes a new mirror into the manager.
    pub async fn push_mirror(&self, mirror: Mirror) {
        let mut mirrors = self.mirrors.write().await;
        mirrors.push(mirror);
    }

    /// Validates a bot token.
    pub async fn validate_token(&self, token: &str) -> Result<bool> {
        let url = format!("https://api.telegram.org/bot{}/getMe", token);
        let response = reqwest::get(&url).await?;

        Ok(response.status().is_success())
    }
}
