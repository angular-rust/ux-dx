#![allow(unused_variables)]
use crate::prelude::Renderable;

use super::time;

pub struct DefaultPreloader;

impl Default for DefaultPreloader {
    fn default() -> Self {
        Self {}
    }
}

impl Renderable for DefaultPreloader {
    fn render(&self, t: time::Time) {
        // todo!()
        
    }
}
