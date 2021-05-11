#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::Object;

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct SwapChain(Object<ffi::CoglSwapChain, SwapChainClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_swap_chain_get_gtype(),
    }
}

impl SwapChain {
    pub fn new() -> SwapChain {
        unsafe { from_glib_full(ffi::cogl_swap_chain_new()) }
    }

    pub fn set_has_alpha(&self, has_alpha: bool) {
        unsafe {
            ffi::cogl_swap_chain_set_has_alpha(self.to_glib_none().0, has_alpha as i32);
        }
    }

    pub fn set_length(&self, length: i32) {
        unsafe {
            ffi::cogl_swap_chain_set_length(self.to_glib_none().0, length);
        }
    }
}

impl Default for SwapChain {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SwapChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwapChain")
    }
}
