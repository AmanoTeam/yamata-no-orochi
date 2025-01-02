// Copyright 2025 - Andriel Ferreira
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The internationalization (i18n) resource.

use std::{collections::HashMap, sync::Arc};

use ferogram::Result;
use serde_json::Value;
use tokio::sync::Mutex;

/// The path to the locales directory.
const PATH: &str = "./assets/locales/";

/// Internationalization module.
#[derive(Clone)]
pub struct I18n {
    /// The current locale.
    current_locale: Arc<Mutex<String>>,
    /// The default locale.
    default_locale: String,

    /// The locales.
    locales: HashMap<String, Value>,
}

#[allow(dead_code)]
impl I18n {
    /// Creates a new instance of the I18n resource.
    ///
    /// # Arguments
    ///
    /// * `locale` - The default locale.
    pub fn with_locale<L: ToString>(locale: L) -> Self {
        let default_locale = locale.to_string();

        Self {
            current_locale: Arc::new(Mutex::new(default_locale.clone())),
            default_locale,

            locales: HashMap::new(),
        }
    }

    /// Loads the locales.
    ///
    /// # Errors
    ///
    /// Returns an error if the locales could not be loaded.
    pub fn load(&mut self) -> Result<()> {
        let locales = std::fs::read_dir(PATH)?
            .map(|entry| entry.expect("failed to read entry"))
            .map(|entry| {
                let path = entry.path();
                let locale = path.file_stem().unwrap().to_str().unwrap().to_string();
                let content = std::fs::read_to_string(path).unwrap();
                let value: Value = serde_json::from_str(&content).unwrap();
                (locale, value)
            })
            .collect::<HashMap<String, Value>>();
        self.locales = locales;

        log::debug!("locales loaded: {:?}", self.locales.keys());

        Ok(())
    }

    /// Gets the current locale.
    ///
    /// # Errors
    ///
    /// Returns an error if the current locale could not be retrieved.
    pub fn locale(&self) -> String {
        let current_locale = self.current_locale.try_lock().unwrap();
        current_locale.clone()
    }

    /// Sets the current locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to set.
    ///
    /// # Errors
    ///
    /// Returns an error if the locale could not be set.
    pub fn set_locale<L: ToString>(&self, locale: L) {
        let mut current_locale = self.current_locale.try_lock().unwrap();
        *current_locale = locale.to_string();
    }

    /// Translates a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to translate.
    ///
    /// # Errors
    ///
    /// Returns an error if the key could not be translated.
    pub fn translate<K: ToString>(&self, key: K) -> String {
        let locale = self.locale();
        self.translate_from_locale(key, &locale)
    }

    /// Translates a key with arguments.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to translate.
    /// * `args` - The arguments to replace in the translation.
    ///
    /// # Errors
    ///
    /// Returns an error if the key could not be translated.
    pub fn translate_with_args<K: ToString, A: ToString>(
        &self,
        key: K,
        args: HashMap<&str, A>,
    ) -> String {
        let locale = self.locale();
        self.translate_from_locale_with_args(key, &locale, args)
    }

    /// Translates a key from a locale.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to translate.
    /// * `locale` - The locale to translate from.
    ///
    /// # Errors
    ///
    /// Returns an error if the key could not be translated.
    pub fn translate_from_locale<L: ToString, K: ToString>(&self, key: K, locale: L) -> String {
        let key = key.to_string();
        let locale = locale.to_string();

        let object = self
            .locales
            .get(&locale)
            .or_else(|| {
                Some(
                    self.locales
                        .get(&self.default_locale)
                        .expect("default locale not found"),
                )
            })
            .unwrap();
        let value = object.get(&key).map_or("KEY_NOT_FOUND", |value| {
            value.as_str().expect("value not found")
        });

        value.to_string()
    }

    /// Translates a key from a locale with arguments.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to translate.
    /// * `locale` - The locale to translate from.
    /// * `args` - The arguments to replace in the translation.
    ///
    /// # Errors
    ///
    /// Returns an error if the key could not be translated.
    pub fn translate_from_locale_with_args<L: ToString, K: ToString, A: ToString>(
        &self,
        key: K,
        locale: L,
        args: HashMap<&str, A>,
    ) -> String {
        let mut result = self.translate_from_locale(key, locale);

        for (key, value) in args.into_iter() {
            result = result.replace(&format!("${{{}}}", key), &value.to_string());
        }

        result
    }
}
