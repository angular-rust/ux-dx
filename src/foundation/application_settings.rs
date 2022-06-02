#![allow(unused_variables)]
use crate::prelude::Renderable;

use super::DefaultPreloader;

/// Application settings
pub struct ApplicationSettings {
    /// The initial title of the application window.
    pub title: String,

    /// Width of the viewport.
    pub width: u32,

    /// Height of the viewport.
    pub height: u32,

    /// Shall the viewport be in fullscreen mode?
    pub fullscreen: bool,

    /// Called before initializing the application host, usually to clear the window.
    pub before: Box<dyn FnOnce(u32, u32)>,

    /// Called during application host initialization, usually to display progress.
    pub preloader: Box<dyn FnOnce() -> Box<dyn Renderable>>,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            title: String::from("UX Application"),
            width: 960,
            height: 540,
            fullscreen: Default::default(),
            before: box |w, h| {},
            preloader: box || box DefaultPreloader::default(),
        }
    }
}
