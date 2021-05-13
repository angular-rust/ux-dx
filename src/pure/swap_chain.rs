use std::{fmt, cell::RefCell};

struct SwapChainProps {
    has_alpha: bool,
    length: i32,
}

impl Default for SwapChainProps {
    fn default() -> Self {
        Self {
            has_alpha: false,
            length: -1,
        }
    }
}

pub struct SwapChain {
    props: RefCell<SwapChainProps>,    
}

impl SwapChain {
    pub fn new() -> SwapChain {
        Self {
            props: Default::default()
        }
    }

    pub fn set_has_alpha(&self, has_alpha: bool) {
        let mut props = self.props.borrow_mut();
        props.has_alpha = has_alpha;
    }

    pub fn set_length(&self, length: i32) {
        let mut props = self.props.borrow_mut();
        props.length = length;
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
