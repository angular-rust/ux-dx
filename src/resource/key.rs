//! Key type used to index resources.

use std::{fmt, ops::Deref, path::Path};

/// Type of key used to index resources.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Key(warmy::SimpleKey);

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl warmy::Key for Key {
    fn prepare_key(self, root: &Path) -> Self {
        Key(self.0.prepare_key(root))
    }
}

impl Deref for Key {
    type Target = warmy::SimpleKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
