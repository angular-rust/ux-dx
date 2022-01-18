use once_cell::sync::OnceCell;
use std::sync::Mutex;

use crate::engine::d2::{
    animation::AnimatedFloat,
    asset::{AssetPack, Manifest},
    platform::Platform,
    subsystem::{
        ExternalSystem, KeyboardSystem, MotionSystem, MouseSystem, PointerSystem, RendererSystem, StageSystem,
        StorageSystem, TouchSystem, WebSystem,
    },
    util::{Promise, Signal1, Value},
};

use super::Entity;

struct SystemProps {
    platform: Box<dyn Platform>, // =
    // #[cfg(not(feature = "html"))]
    // SOME IMPLEMENTATION FROM PALTFORM
    // #[cfg(feature = "html")]
    // SOME IMPLEMENTATION FROM PALTFORM

    called_init: bool,
}

unsafe impl Send for SystemProps {}

/// Provides access to all the different subsystems implemented on each platform.
pub struct System {
    props: Mutex<SystemProps>,
}

impl System {
    pub fn new() -> Self {
        // Self {
        // props: RwLock::new(Default::default()),
        //     platform: None,
        //     called_init: false,
        // }
        unimplemented!()
    }
    /// Starts up d2, this should usually be the first thing a game does.
    //  static
    pub fn init() {
        match Self::global().props.lock() {
            Ok(mut props) => {
                if !props.called_init {
                    props.platform.init();
                    props.called_init = true;
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// Retrieves the singleton instance of `Settings`
    ///
    /// # Returns
    ///
    /// the instance of `Settings`. The
    ///  returned object is owned by internals and it should not be unreferenced
    ///  directly
    fn global() -> &'static Self {
        static SYSTEM_INSTANCE: OnceCell<System> = OnceCell::new();
        SYSTEM_INSTANCE.get_or_init(Self::new)
    }

    /// Request to load an asset pack described by the given manifest.
    //  static
    #[inline]
    pub fn load_asset_pack(manifest: Manifest) -> Promise<Box<dyn AssetPack>> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.load_asset_pack(manifest),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The entity at the root of the hierarchy
    #[inline]
    pub fn root() -> Entity {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        // system.platform. ...
        unimplemented!()
    }

    /// The global volume applied to all sounds, defaults to 1
    #[inline]
    pub fn volume() -> AnimatedFloat {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        // system.platform. ...
        unimplemented!()
    }

    /// Gets the current clock time, in <b>seconds</b> since January 1, 1970.
    /// Depending on the platform, this may be slightly more efficient than Date.now().getTime().
    #[inline]
    pub fn time() -> f32 {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.time(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Stage subsystem, for controlling the display viewport
    #[inline]
    pub fn stage() -> Box<dyn StageSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.stage(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Storage subsystem, for persisting values
    #[inline]
    pub fn storage() -> Box<dyn StorageSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.storage(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Pointer subsystem, for unified mouse/touch events
    #[inline]
    pub fn pointer() -> Box<dyn PointerSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.pointer(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Mouse subsystem, for direct access to the mouse
    #[inline]
    pub fn mouse() -> Box<dyn MouseSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.mouse(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Touch subsystem, for direct access to the multi-touch
    #[inline]
    pub fn touch() -> Box<dyn TouchSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.touch(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Keyboard subsystem, for keyboard events
    #[inline]
    pub fn keyboard() -> Box<dyn KeyboardSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.keyboard(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Web subsystem, for using the device's web browser
    #[inline]
    pub fn web() -> Box<dyn WebSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.web(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The External subsystem, for interaction with external code
    #[inline]
    pub fn external() -> Box<dyn ExternalSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.external(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// Gets the RFC 4646 language tag of the environment.
    /// For example, "en-US", "pt", or None if the locale is unknown
    #[inline]
    pub fn locale() -> Option<String> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.locale(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    // TODO: Subsystems for gamepads, haptic, geolocation, video, textInput

    /// The Motion subsystem, for events from the device's motion sensors
    #[inline]
    pub fn motion() -> Box<dyn MotionSystem> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.motion(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// The Renderer subsystem, for creating textures and accessing the GPU
    #[inline]
    pub fn renderer() -> Box<dyn RendererSystem> {
        // RendererSystem<BitmapData>,
        // RendererSystem<html.Element>,
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        match system.props.lock() {
            Ok(props) => props.platform.renderer(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// True when the app is not currently visible, such as when minimized or placed in a background
    /// browser tab. While hidden, frame updates may be paused or throttled.
    #[inline]
    pub fn is_hidden() -> Value<bool> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        // system.platform. ...
        unimplemented!()
    }

    /// Emitted when an uncaught exception occurs, if the platform supports it
    #[inline]
    pub fn uncaught_error() -> Signal1<String> {
        let system = Self::global();

        #[cfg(debug_assertions)]
        system.assert_called_init();

        // system.platform. ...
        unimplemented!()
    }

    // static
    fn assert_called_init(&self) {
        match self.props.lock() {
            Ok(props) => {
                assert!(props.called_init, "You must call System::init() first");
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }
}
