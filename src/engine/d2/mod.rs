//! 2D game-engine

#![allow(unused_imports)]
pub mod animation;

pub mod asset;

mod context;
pub use context::*;

pub mod debug;

pub mod display;

pub mod input;

pub mod math;

pub mod platform;

pub mod scene;

pub mod script;

pub mod sound;

pub mod subsystem;

pub mod swf;

pub mod util;

pub mod web;

mod component;
pub use self::component::*;

mod director;
pub use self::director::*;

mod disposer;
pub use self::disposer::*;

mod entity;
pub use self::entity::*;

mod speed_adjuster;
pub use self::speed_adjuster::*;

mod system;
pub use self::system::*;
