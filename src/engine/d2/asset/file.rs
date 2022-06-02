use std::fmt;

use super::Asset;

/// A loaded file containing raw data.
pub trait File: Asset + fmt::Display + fmt::Debug {
    // To return the contents of this file as a string. use fmt::Display
}
