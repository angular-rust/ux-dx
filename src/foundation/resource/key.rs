//! Key type used to index resources.

use std::{fmt, ops::Deref, path::Path};

use warmy::SimpleKey;

/// Type of key used to index resources.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct StoreKey(pub warmy::SimpleKey);

impl fmt::Display for StoreKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl warmy::Key for StoreKey {
    fn prepare_key(self, root: &Path) -> Self {
        StoreKey(self.0.prepare_key(root))
    }
}

impl Deref for StoreKey {
    type Target = warmy::SimpleKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StoreKey {
    pub fn from_path(path: &str) -> Self {
        Self(SimpleKey::from_path(path))
    }
}
