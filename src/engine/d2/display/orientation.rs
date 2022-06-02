use std::fmt;

/// A display orientation for devices, used by `Stage.lockOrientation()`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::Landscape => write!(f, "Orientation::Landscape"),
            Orientation::Portrait => write!(f, "Orientation::Portrait"),
        }
    }
}
