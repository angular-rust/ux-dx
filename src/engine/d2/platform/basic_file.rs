use std::fmt;

use crate::engine::d2::{
    asset::{Asset, File},
    util::{Disposable, Value},
};

use super::BasicAsset;

pub struct BasicFile {
    pub inner: BasicAsset<BasicFile>,
    content: Option<String>,
}

impl BasicFile {
    pub fn new(content: Option<String>) -> Self {
        Self {
            inner: BasicAsset::new(),
            content,
        }
    }

    // override
    fn copy_from(&mut self, that: BasicFile) {
        self.content = that.content;
    }

    // override
    fn on_disposed(&mut self) {
        self.content = None;
    }
}

impl AsRef<BasicAsset<BasicFile>> for BasicFile {
    fn as_ref(&self) -> &BasicAsset<BasicFile> {
        &self.inner
    }
}

impl File for BasicFile {}

impl Asset for BasicFile {
    fn reload_count(&self) -> usize {
        self.inner.reload_count()
    }
}

impl Disposable for BasicFile {
    fn dispose(&self) {
        self.inner.dispose()
    }
}

impl fmt::Display for BasicFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.assert_not_disposed();
        write!(
            f,
            "{}",
            self.content.clone().unwrap_or(String::from("None"))
        )
    }
}

impl fmt::Debug for BasicFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BasicFile")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
