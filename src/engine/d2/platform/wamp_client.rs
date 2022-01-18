use super::BasicAssetPackLoader;

/// Handles communication with the Wamp server run by `cargo ux serve`, for live reloading.
pub struct WampClient {
    loaders: Vec<BasicAssetPackLoader>,
}

impl WampClient {
    fn new() -> Self {
        Self { loaders: Vec::new() }
    }

    pub fn add(&mut self, loader: BasicAssetPackLoader) {
        // Only care about packs loaded from the assets directory
        #[cfg(not(feature = "disable_reloading"))]
        {
            if let Some(ref local_base) = loader.manifest.local_base {
                if local_base.as_str() == "assets" {
                    self.loaders.push(loader);
                }
            }
        }
    }

    pub fn remove(&mut self, loader: &BasicAssetPackLoader) {
        self.loaders.retain(|x| x != loader);
    }

    fn on_error(&self, cause: String) {
        log::warn!("Unable to connect to Wamp {}", cause);
    }

    fn on_message(&self, message: String) {
        // let message = Json::parse(message);
        // match message.type_ {
        //     "file_changed" => {
        //         let url = message.name + "?v=" + message.md5;
        //         url = url.replace("\\", "/"); // Handle backslash paths in Windows
        //         for loader in self._loaders {
        //             loader.reload(url);
        //         }
        //     }
        //     "restart" => {
        //         self.on_restart();
        //     }
        // }
        unimplemented!()
    }

    fn on_restart(&self) {
        panic!("See subclasses");
    }
}
