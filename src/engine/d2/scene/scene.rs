use crate::engine::d2::{util::Signal0, Component};

/// Optional, extra functionality for scene entities that are added to a Director.
#[derive(Default, Clone, Debug)]
pub struct Scene2D {
    pub inner: Component,
    /// Emitted by the Director when this scene becomes the top scene.
    pub shown: Signal0,

    /// Emitted by the Director when this scene is no longer the top scene.
    pub hidden: Signal0,

    /// When true, hints that scenes below this one don't need to be rendered. Scenes that don't fill
    /// the entire stage or have a transparent background should set this to false.
    pub opaque: bool,
}

impl Scene2D {
    // opaque :bool = true
    pub fn new(opaque: bool) -> Self {
        Self {
            inner: Component::default(),
            opaque,
            shown: Signal0::new(None),
            hidden: Signal0::new(None),
        }
    }
}

impl AsRef<Component> for Scene2D {
    fn as_ref(&self) -> &Component {
        &self.inner
    }
}
