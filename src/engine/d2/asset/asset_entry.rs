/// Specifies all supported asset formats

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AssetFormat {
    // Images
    WEBP,
    JXR,
    PNG,
    JPG,
    GIF,

    // Compressed textures
    DDS,
    PVR,
    PKM,

    // Audio
    MP3,
    M4A,
    OPUS,
    OGG,
    WAV,

    // Raw text/data
    BLOB,
}

impl Default for AssetFormat {
    fn default() -> Self {
        Self::BLOB
    }
}

/// Defines an asset that will be loaded.

#[derive(Default, Clone, Debug)]
pub struct AssetEntry {
    /// The name of this asset.
    pub name: String,

    /// The URL or file path this asset will be loaded from. Will be appended to `Manifest.localBase`
    /// or `Manifest.remoteBase` to get the actual URL to load from.
    pub url: String,

    /// The format this asset will be loaded as.
    pub format: AssetFormat,

    /// The size of this asset in bytes, or 0 if unknown.
    pub bytes: usize,
}

impl AssetEntry {
    pub fn new(name: String, url: String, format: AssetFormat, bytes: usize) -> Self {
        Self {
            name,
            url,
            format,
            bytes,
        }
    }
}
