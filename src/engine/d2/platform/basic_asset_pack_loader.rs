use std::{collections::HashMap, rc::Rc};

use crate::engine::d2::{
    asset::{AssetEntry, AssetFormat, AssetPack, File, Manifest},
    display::Texture,
    sound::Sound,
    util::{Disposable, Promise, StringExtensions},
};

use super::{BasicAsset, DummySound, Platform};

pub struct BasicAssetPackLoader {
    pub promise: Promise<Box<dyn AssetPack>>,
    pub manifest: Manifest,

    platform: Box<dyn Platform>,

    // How many assets are still loading
    assets_remaining: usize,

    // How many bytes of each asset have been loaded
    bytes_loaded: HashMap<String, usize>,

    pack: Option<BasicAssetPack>,
}

impl BasicAssetPackLoader {
    pub fn new(platform: Box<dyn Platform>, manifest: Manifest) -> Self {
        let mut instance = Self {
            manifest,
            platform,
            promise: Promise::new(),
            bytes_loaded: HashMap::new(),
            pack: None,
            assets_remaining: 0,
        };

        // instance.pack = Some(BasicAssetPack::new(manifest, instance));
        if instance.manifest.len() == 0 {
            // There's nothing to load, just send them an empty pack
            instance.handle_success();
        } else {
            let mut groups: HashMap<String, Vec<AssetEntry>> = HashMap::new();

            // Group assets by name
            for entry in instance.manifest.iter() {
                match groups.get_mut(&entry.name) {
                    Some(group) => group.push(entry.clone()),
                    None => {
                        let mut group = Vec::new();
                        group.push(entry.clone());
                        groups.insert(entry.name.clone(), group);
                    }
                }
            }

            // Load the most suitable asset from each group
            instance.assets_remaining = groups.len();
            for group in groups.values() {
                // instance.pick_best_entry(group, |bestEntry: AssetEntry| {
                //     if bestEntry.is_some() {
                //         let url = manifest.get_full_url(bestEntry);
                //         // try {
                //         //     loadEntry(url, bestEntry);
                //         // } catch (error :Dynamic) {
                //         //     handleError(bestEntry, "Unexpected error: " + error);
                //         // }
                //         instance.promise.total += bestEntry.bytes;
                //     } else {
                //         let badEntry = group[0];
                //         if Self::is_audio(badEntry.format) {
                //             // Deal with missing audio files and bad browser support
                //             // log::warn!("Could not find a supported audio format to load [name: {}]", badEntry.name);
                //             instance.handle_load(badEntry, DummySound::get_instance());
                //         } else {
                //             instance.handle_error(badEntry, "Could not find a supported format to load");
                //         }
                //     }
                // });
            }
        }

        #[cfg(debug_assertions)]
        {
            if let Some(ref wamp) = instance.platform.wamp_client() {
                // wamp.add(&instance);
                unimplemented!()
            }
        }

        instance
    }

    /// Reload any asset that matches this URL (ignoring the ?v= query param).
    pub fn reload(&self, url: String) {
        // Find the AssetEntry that matches this url
        let base_url = Self::remove_url_params(url.as_str());
        // let found_entry = None;
        for entry in self.manifest.iter() {
            if base_url == Self::remove_url_params(entry.url.as_str()) {
                // found entry is entry
                // If the entry was found in this manifest, and is a supported format
                let entry = entry.clone();

                self.asset_formats(move |formats: Vec<AssetFormat>| {
                    if formats.iter().position(|&e| e == entry.format).is_some() {
                        // Dummy up a new AssetEntry based on the previous one, and reload it
                        let entry =
                            AssetEntry::new(entry.name.clone(), url.clone(), entry.format, 0);
                        self.load_entry(self.manifest.full_url(entry.clone()), entry);
                    }
                });

                break;
            }
        }
    }

    /// Called when this loader's AssetPack is disposed.
    pub fn on_disposed(&self) {
        #[cfg(debug_assertions)]
        {
            if let Some(ref wamp) = self.platform.wamp_client() {
                // wamp.remove(self);
                unimplemented!()
            }
        }
    }

    // static
    fn remove_url_params(url: &str) -> String {
        if let Some(query) = url.find("?") {
            url[..query].to_string()
        } else {
            url.to_string()
        }
    }

    /// Out of a list of asset entries with the same name, select the one best supported by this
    /// environment, or None if none of them are supported.
    fn pick_best_entry(&self, entries: &Vec<AssetEntry>, function: impl Fn(Option<&AssetEntry>)) {
        let on_formats_available = |formats: Vec<AssetFormat>| {
            for format in formats.iter() {
                for entry in entries.iter() {
                    if entry.format == *format {
                        function(Some(entry));
                        return;
                    }
                }
            }
            function(None); // This asset is not supported, we're boned
        };

        self.asset_formats(on_formats_available);
    }

    fn load_entry(&self, url: String, entry: AssetEntry) {
        panic!("See subclasses");
    }

    /// Gets the list of asset formats the environment supports, ordered by preference.
    fn asset_formats(&self, function: impl Fn(Vec<AssetFormat>)) {
        panic!("See subclasses");
    }

    fn handle_load<A /*:BasicAsset<A>*/>(&mut self, entry: AssetEntry, asset: A) {
        if let Some(ref pack) = self.pack {
            if pack.disposed {
                return; // Pack was disposed earlier, forget about it
            }
        }

        // Ensure this asset has been fully progressed
        self.handle_progress(&entry, entry.bytes);

        unimplemented!()
        // if let Some(ref pack) = self.pack {
        //     match entry.format {
        //         AssetFormat::WEBP
        //         | AssetFormat::JXR
        //         | AssetFormat::PNG
        //         | AssetFormat::JPG
        //         | AssetFormat::GIF
        //         | AssetFormat::DDS
        //         | AssetFormat::PVR
        //         | AssetFormat::PKM => {
        //             if let Some(ref pack) = self.pack {
        //                 // Allow some methods to get stripped in release builds, which don't allow reloading
        //                 #[cfg(debug_assertions)]
        //                 {
        //                     if let Some(old_asset) = pack.textures.get(&entry.name) {
        //                         log::info!("Reloaded asset url {}", entry.url);
        //                         old_asset.reload(asset);
        //                     } else {
        //                         pack.textures.insert(entry.name, Rc::new(asset));
        //                         self.assets_remaining -= 1;
        //                         if self.assets_remaining == 0 {
        //                             self.handle_success();
        //                         }
        //                     }
        //                 }
        //                 #[cfg(not(debug_assertions))]
        //                 {
        //                     pack.textures.insert(entry.name, Rc::new(asset));
        //                     self.assets_remaining -= 1;
        //                     if self.assets_remaining == 0 {
        //                         self.handle_success();
        //                     }
        //                 }
        //             }
        //         }
        //         AssetFormat::MP3 | AssetFormat::M4A | AssetFormat::OPUS | AssetFormat::OGG | AssetFormat::WAV => {
        //             if let Some(ref pack) = self.pack {
        //                 // Allow some methods to get stripped in release builds, which don't allow reloading
        //                 #[cfg(debug_assertions)]
        //                 {
        //                     if let Some(old_asset) = pack.sounds.get(&entry.name) {
        //                         log::info!("Reloaded asset url {}", entry.url);
        //                         old_asset.reload(asset);
        //                     } else {
        //                         pack.sounds.insert(entry.name, Rc::new(asset));
        //                         self.assets_remaining -= 1;
        //                         if self.assets_remaining == 0 {
        //                             self.handle_success();
        //                         }
        //                     }
        //                 }
        //                 #[cfg(not(debug_assertions))]
        //                 {
        //                     pack.sounds.insert(entry.name, Rc::new(asset));
        //                     self.assets_remaining -= 1;
        //                     if self.assets_remaining == 0 {
        //                         self.handle_success();
        //                     }
        //                 }
        //             }
        //         }
        //         AssetFormat::BLOB => {
        //             if let Some(ref pack) = self.pack {
        //                 // Allow some methods to get stripped in release builds, which don't allow reloading
        //                 #[cfg(debug_assertions)]
        //                 {
        //                     if let Some(old_asset) = pack.files.get(&entry.name) {
        //                         log::info!("Reloaded asset url {}", entry.url);
        //                         old_asset.reload(asset);
        //                     } else {
        //                         pack.files.insert(entry.name, Rc::new(asset));
        //                         self.assets_remaining -= 1;
        //                         if self.assets_remaining == 0 {
        //                             self.handle_success();
        //                         }
        //                     }
        //                 }
        //                 #[cfg(not(debug_assertions))]
        //                 {
        //                     pack.files.insert(entry.name, Rc::new(asset));
        //                     self.assets_remaining -= 1;
        //                     if self.assets_remaining == 0 {
        //                         self.handle_success();
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn handle_progress(&mut self, entry: &AssetEntry, bytes_loaded: usize) {
        self.bytes_loaded.insert(entry.name.clone(), bytes_loaded);

        let mut bytes_total = 0;
        for bytes in self.bytes_loaded.values() {
            bytes_total += *bytes;
        }
        self.promise.progress = bytes_total as f32;
    }

    fn handle_success(&self) {
        // self.promise.result = self.pack;
        unimplemented!()
    }

    fn handle_error<S: Into<String>>(&self, entry: &AssetEntry, message: S) {
        // log::warn!("Error loading asset pack [error: {}, url: {}]", message, entry.url);
        // self.promise
        //     .error
        //     .emit(format!("{} [url: {}]", message.into(), entry.url);
        unimplemented!()
    }

    fn handle_texture_error(&self, entry: &AssetEntry) {
        self.handle_error(
            entry,
            "Failed to create texture. Is the GPU context unavailable?",
        );
    }

    // static
    fn is_audio(format: AssetFormat) -> bool {
        match format {
            AssetFormat::MP3
            | AssetFormat::M4A
            | AssetFormat::OPUS
            | AssetFormat::OGG
            | AssetFormat::WAV => true,
            _ => false,
        }
    }
}

impl PartialEq for BasicAssetPackLoader {
    fn eq(&self, other: &Self) -> bool {
        self.manifest == other.manifest
    }
}

// A simple AssetPack backed by a Map
pub struct BasicAssetPack {
    pub loader: Rc<BasicAssetPackLoader>,

    pub textures: HashMap<String, Rc<dyn Texture>>,
    pub sounds: HashMap<String, Rc<dyn Sound>>,
    pub files: HashMap<String, Rc<dyn File>>,

    pub disposed: bool, // = false;

    manifest: Manifest,
}

impl BasicAssetPack {
    pub fn new(manifest: Manifest, loader: Rc<BasicAssetPackLoader>) -> Self {
        Self {
            manifest,
            loader,
            textures: HashMap::new(),
            sounds: HashMap::new(),
            files: HashMap::new(),
            disposed: false,
        }
    }

    #[inline]
    fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    #[inline]
    fn assert_not_disposed(&self) {
        assert!(
            !self.disposed,
            "AssetPack cannot be used after being disposed"
        );
    }

    // static
    fn warn_on_extension(path: &str) {
        if let Some(ref ext) = path.file_extension() {
            if ext.len() == 3 {
                log::warn!(
                    "Requested asset \"{}\" should not have a file extension, did you mean \"{}\"?",
                    path,
                    path.remove_file_extension()
                );
            }
        }
    }
}

impl AssetPack for BasicAssetPack {
    // required :bool = true
    fn texture(&self, name: String, required: bool) -> Option<Rc<dyn Texture>> {
        self.assert_not_disposed();

        #[cfg(debug_assertions)]
        Self::warn_on_extension(name.as_str());

        let texture = self.textures.get(&name).map(|item| item.clone());
        if texture.is_none() && required {
            panic!("Missing texture {}", name);
        }

        texture
    }

    // required :bool = true
    fn sound(&self, name: String, required: bool) -> Option<Rc<dyn Sound>> {
        self.assert_not_disposed();

        #[cfg(debug_assertions)]
        Self::warn_on_extension(name.as_str());

        let sound = self.sounds.get(&name).map(|item| item.clone());
        if sound.is_none() && required {
            panic!("Missing sound {}", name);
        }

        sound
    }

    // required :bool = true
    fn file(&self, name: String, required: bool) -> Option<Rc<dyn File>> {
        self.assert_not_disposed();

        let file = self.files.get(&name).map(|item| item.clone());
        if file.is_none() && required {
            panic!("Missing file {}", name);
        }

        file
    }
}

impl Disposable for BasicAssetPack {
    // Dispose all assets contained in this pack
    //
    // Disposes all the assets in this AssetPack. After calling self, any calls to get_texture,
    // get_sound, or get_file will assert.
    //
    fn dispose(&self) {
        // if !self.disposed {
        //     self.disposed = true;

        //     for texture in self.textures.values() {
        //         texture.dispose();
        //     }
        //     self.textures.clear();

        //     for sound in self.sounds.values() {
        //         sound.dispose();
        //     }
        //     self.sounds.clear();

        //     for file in self.files.values() {
        //         file.dispose();
        //     }
        //     self.files.clear();

        //     self.loader.on_disposed();
        // }
    }
}
