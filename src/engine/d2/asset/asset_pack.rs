use std::rc::Rc;

use crate::engine::d2::{display::Texture, sound::Sound, util::Disposable};

use super::File;

/// Represents a collection of fully loaded assets.
pub trait AssetPack: Disposable {
    /// The manifest that was used to load this asset pack.
    // let manifest (get, None) ->Manifest;

    /// Gets a texture by name from the asset pack. The name must NOT contain a filename extension.
    /// Textures are cached, so it's safe to get the same texture multiple times.
    /// @param required If true and the asset was not found, an error is thrown.
    //  required :bool = true
    fn texture(&self, name: String, required: bool) -> Option<Rc<dyn Texture>>;

    /// Gets a sound by name from the asset pack. The name must NOT contain a filename extension.
    /// Sounds are cached, so it's safe to get the same sound multiple times.
    /// @param required If true and the asset was not found, an error is thrown.
    //  required :bool = true
    fn sound(&self, name: String, required: bool) -> Option<Rc<dyn Sound>>;

    /// Gets a file by name from the asset pack, returning its raw content. Files are cached, so it's
    /// safe to get the same file multiple times.
    /// @param required If true and the asset was not found, an error is thrown.
    //  required :bool = true
    fn file(&self, name: String, required: bool) -> Option<Rc<dyn File>>;
}
