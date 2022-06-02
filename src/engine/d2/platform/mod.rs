//! Platform

pub mod shader;

mod basic_asset_pack_loader;
pub use self::basic_asset_pack_loader::*;

mod basic_asset;
pub use self::basic_asset::*;

mod basic_file;
pub use self::basic_file::*;

mod basic_keyboard;
pub use self::basic_keyboard::*;

mod basic_mouse;
pub use self::basic_mouse::*;

mod basic_pointer;
pub use self::basic_pointer::*;

mod basic_texture;
pub use self::basic_texture::*;

mod basic_touch;
pub use self::basic_touch::*;

mod wamp_client;
pub use self::wamp_client::*;

mod component_builder;
pub use self::component_builder::*;

mod debug_logic;
pub use self::debug_logic::*;

mod dummy_external;
pub use self::dummy_external::*;

mod dummy_keyboard;
pub use self::dummy_keyboard::*;

mod dummy_motion;
pub use self::dummy_motion::*;

mod dummy_mouse;
pub use self::dummy_mouse::*;

mod dummy_sound;
pub use self::dummy_sound::*;

mod dummy_storage;
pub use self::dummy_storage::*;

mod dummy_touch;
pub use self::dummy_touch::*;

mod dummy_web;
pub use self::dummy_web::*;

mod event_group;
pub use self::event_group::*;

mod heavy_signal1;
pub use self::heavy_signal1::*;

mod key_codes;
pub use self::key_codes::*;

mod main_loop;
pub use self::main_loop::*;

mod manifest_builder;
pub use self::manifest_builder::*;

mod math_util;
pub use self::math_util::*;

mod mouse_codes;
pub use self::mouse_codes::*;

mod overdraw_graphics;
pub use self::overdraw_graphics::*;

mod platform;
pub use self::platform::*;

mod texture_root;
pub use self::texture_root::*;

mod tickable;
pub use self::tickable::*;

// stubs

#[derive(Default, Clone, Copy, Debug)]
pub struct Dynamic; // DV

// native impls
#[derive(Default, Clone, Debug)]
pub struct IEventDispatcher; // DV

#[derive(Default, Clone, Debug)]
pub struct Event; // DV
