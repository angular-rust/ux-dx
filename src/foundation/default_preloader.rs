#![allow(unused_variables)]
use crate::prelude::Renderable;

use super::time;

/// Default dummy preloader implementation
pub struct DefaultPreloader;

impl Default for DefaultPreloader {
    fn default() -> Self {
        Self {}
    }
}

impl Renderable for DefaultPreloader {
    fn render(&self, t: time::Time) {}
}
