use crate::engine::d2::util::{Disposable, Value};

/// A fully loaded asset.
pub trait Asset: Disposable {
    /// The number of times this asset has been live-reloaded. Asset reloading is only enabled in
    /// debug builds, and only from the /assets directory.
    fn reload_count(&self) -> usize;

    // /// Frees up the underlying resources used by this asset. An asset must not be used after it has
    // /// been disposed.
    // fn dispose(&self);
}
