use std::{cell::RefCell, fmt};

#[derive(Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
pub struct LegacySwapChain {
    props: RefCell<SwapChainProps>,
}

impl LegacySwapChain {
    pub fn new() -> LegacySwapChain {
        Self {
            props: Default::default(),
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

impl fmt::Display for LegacySwapChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwapChain")
    }
}
