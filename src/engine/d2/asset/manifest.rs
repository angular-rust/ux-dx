use crate::engine::d2::{asset::AssetEntry, util::StringExtensions, System};

use super::AssetFormat;

/// An asset manifest contains all the information needed to load an asset pack. A manifest is
/// usually created with `Manifest.fromAssets("directory")`, but manifests can also be assembled
/// programmatically.

#[derive(Default, Debug)]
pub struct Manifest {
    pub entries: Vec<AssetEntry>,

    /// A base path on the current domain to load this manifest's assets from, or None.
    pub local_base: Option<String>,
    /// A base URL on another domain to load this manifest's assets from, or None. May be used to
    /// load assets from a CDN, in browsers that support cross-domain requests. If not supported or
    /// unset, will fallback to using localBase.
    pub remote_base: Option<String>,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            remote_base: None,
            local_base: None,
        }
    }

    /// Gets the manifest of a pack in the asset directory, that was processed at build-time.
    /// @param packName The folder name in your assets/ directory.
    /// @param required When true and this pack was not found, throw an error. Otherwise None is
    ///   returned.
    //  static
    // required :bool = true
    pub fn from_assets(pack_name: String, required: bool) -> Option<Manifest> {
        // let packData: Vec<Vec<String>> = Reflect::field(Meta::getType(Manifest).assets[0], packName);
        // if packData.is_none() {
        //     if required {
        //         panic!("Missing asset pack ["name", {}]", packName);
        //     }
        //     return None;
        // }

        // let manifest = Manifest::new();
        // manifest.localBase = "assets";

        // for asset in packData {
        //     let name = asset.name;
        //     let path = packName + "/" + name + "?v=" + asset.md5;

        //     let format = Self::inferFormat(name);
        //     if format != AssetFormat::BLOB {
        //         // If this an asset that not all platforms may support, trim the extension from
        //         // the name. We'll only load one of the assets if this creates a name collision.
        //         name = name.remove_file_extension();
        //     }

        //     manifest.add(name, path, asset.bytes, format);
        // }

        // return manifest;
        unimplemented!()
    }

    /// Tries to find a pack suffixed with the closest available variant of the locale. For example,
    /// fromAssetsLocalized("foo", "pt-BR") will first try to load foo_pt-BR, then foo_pt, then just
    /// foo.
    /// @param packName The folder name in your assets/ directory.
    /// @param locale An RFC 4646 language tag, or None to use the system language.
    /// @param required When true and this pack was not found, throw an error. Otherwise None is
    ///   returned.
    //  static
    // locale :String = None, required :bool = true
    pub fn from_assets_localized(
        pack_name: String,
        locale: Option<String>,
        required: bool,
    ) -> Option<Manifest> {
        let locale = if locale.is_none() {
            System::locale()
        } else {
            locale
        };

        if let Some(locale) = locale {
            let mut parts: Vec<String> = locale.split("-").map(String::from).collect();

            while parts.len() > 0 {
                let path = format!("{}_{}", pack_name, parts.join("-"));
                let manifest = Self::from_assets(path, false);
                if manifest.is_some() {
                    return manifest;
                }
                parts.pop();
            }
        }

        Self::from_assets(pack_name, required)
    }

    /// Returns true if the given named pack was included in the asset directory at build-time.
    //  static
    pub fn exists(pack_name: String) -> bool {
        // return Reflect::hasField(Meta::getType(Manifest).assets[0], packName);
        unimplemented!()
    }

    /// Adds an asset entry to this manifest.
    /// @param name The name of the asset.
    /// @param url The URL this asset will be downloaded from.
    /// @param bytes The size in bytes.
    /// @param format Optionally specified content format, by default infer it from the URL.
    //  bytes: i32 = 0, ?format :AssetFormat
    pub fn add(
        &mut self,
        name: String,
        url: String,
        bytes: usize,
        format: Option<AssetFormat>,
    ) -> AssetEntry {
        let format = format.unwrap_or(Self::infer_format(url.as_str()));

        let entry = AssetEntry::new(name, url, format, bytes);
        self.entries.push(entry.clone());

        entry
    }

    /// Iterates over all the assets defined in this manifest.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &AssetEntry> + '_ {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get the full URL to load an asset from. Will prepend localBase or remoteBase depending on
    /// cross-domain support.
    pub fn full_url(&self, entry: AssetEntry) -> String {
        const SUPPORT_CROSS_ORIGIN: bool = true;
        let base_path = if self.remote_base.is_some() && SUPPORT_CROSS_ORIGIN {
            &self.remote_base
        } else {
            &self.local_base
        };

        if let Some(path) = base_path {
            path.join_path(entry.url)
        } else {
            entry.url
        }
    }

    fn local_base(&self) -> Option<String> {
        self.local_base.clone()
    }

    fn set_local_base(&mut self, local_base: Option<String>) {
        if let Some(ref local_base) = local_base {
            assert!(
                !local_base.starts_with("http://") && !local_base.starts_with("https://"),
                "localBase must be a path on the same domain, NOT starting with http(s)://",
            );
        }

        self.local_base = local_base;
    }

    fn remote_base(&self) -> Option<String> {
        self.remote_base.clone()
    }

    fn set_remote_base(&mut self, remote_base: Option<String>) {
        if let Some(ref remote_base) = remote_base {
            assert!(
                remote_base.starts_with("http://") || remote_base.starts_with("https://"),
                "remoteBase must be on a remote domain, starting with http(s)://",
            );
        }
        self.remote_base = remote_base;
    }

    // static
    fn infer_format(url: &str) -> AssetFormat {
        if let Some(extension) = url.url_extension() {
            return match extension.to_lowercase().as_str() {
                "gif" => AssetFormat::GIF,
                "jpg" | "jpeg" => AssetFormat::JPG,
                "jxr" | "wdp" => AssetFormat::JXR,
                "png" => AssetFormat::PNG,
                "webp" => AssetFormat::WEBP,

                "dds" => AssetFormat::DDS,
                "pvr" => AssetFormat::PVR,
                "pkm" => AssetFormat::PKM,

                "m4a" => AssetFormat::M4A,
                "mp3" => AssetFormat::MP3,
                "ogg" => AssetFormat::OGG,
                "opus" => AssetFormat::OPUS,
                "wav" => AssetFormat::WAV,
                _ => AssetFormat::BLOB,
            };
        } else {
            // log::warn!("No file extension for asset, it will be loaded as data [url, {}]", url);
        }
        return AssetFormat::BLOB;
    }

    // // Whether the environment fully supports loading assets from another domain
    // // static
    // let _supportsCrossOrigin = (|| -> bool {
    //     let detected =
    // #[cfg(feature = "html")]
    //     (fn () {
    //         // CORS in the stock Android browser is buggy. If your game is contained in an iframe, XHR
    //         // will work the first time. If the response had an Expires header, on subsequent page loads
    //         // instead of retrieving it from the cache, it will fail with error code 0.
    //         // http://stackoverflow.com/questions/6090816/android-cors-requests-work-only-once
    //         if Browser::navigator.userAgent.iter().position(|x| x == "Linux; U; Android").is_some() {
    //             return false;
    //         }

    //         let xhr :Dynamic = __new__("XMLHttpRequest");
    //         return (xhr.withCredentials.is_some());
    //     })();
    // #else
    //         true; // Assumes you have a valid crossdomain.xml
    // #end
    //     if (!detected) {
    //         log::warn!("This browser does not support cross-domain asset loading, any Manifest.remoteBase setting will be ignored.");
    //     }
    //     return detected;
    // })();
}

impl PartialEq for Manifest {
    fn eq(&self, other: &Self) -> bool {
        self.local_base == other.local_base && self.remote_base == other.remote_base
    }
}
