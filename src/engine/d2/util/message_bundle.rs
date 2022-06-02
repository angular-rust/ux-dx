use crate::engine::d2::util::StringExtensions;

use super::{Config, Signal1};

/// Stupid-simple text localization.
pub struct MessageBundle {
    pub config: Config,

    /// Emitted when a translation is requested that this bundle doesn't provide.
    pub missing_translation: Signal1<String>,
}

impl MessageBundle {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            missing_translation: Signal1::new(None),
        }
    }

    // static
    #[inline]
    pub fn parse(&self, text: String) -> MessageBundle {
        MessageBundle::new(Config::parse(text))
    }

    /// Fetch a translation from the config, and substitute in params with Strings.substitute. If
    /// the path doesn't exist in the config, missingTranslation is emitted and the original path is
    /// returned.
    pub fn get(&self, path: &String, params: Option<Vec<String>>) -> String {
        match self.config.get(path) {
            Some(value) => {
                if let Some(params) = params {
                    value.substitute(params)
                } else {
                    value.clone()
                }
            }
            None => {
                // log::warn!("Requested a missing translation from bundle [path: {}]", path);
                self.missing_translation.emit(path.clone());
                // Return the best we can
                path.clone()
            }
        }
    }
}
