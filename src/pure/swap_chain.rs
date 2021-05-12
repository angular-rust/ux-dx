#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::Object;
use std::fmt;

pub struct SwapChain {
    // Object _parent;

    // Bool has_alpha;
  
    // int length;
}

impl SwapChain {
    pub fn new() -> SwapChain {
        // SwapChain *swap_chain = g_slice_new0 (SwapChain);

        // swap_chain->length = -1; /* no preference */
      
        // return _swap_chain_object_new (swap_chain);
        unimplemented!()
    }

    pub fn set_has_alpha(&self, has_alpha: bool) {
        // swap_chain->has_alpha = has_alpha;
        unimplemented!()
    }

    pub fn set_length(&self, length: i32) {
        // swap_chain->length = length;
        unimplemented!()
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
