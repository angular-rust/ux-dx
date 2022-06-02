use std::fmt;

use crate::engine::d2::display::Sprite;

/// Defines an exported SWF symbol.

pub trait Symbol<T>: fmt::Debug
where
    T: AsRef<Sprite>,
{
    /// The name of this symbol.
    fn name(&self) -> Option<String>;

    /// Instantiate a sprite that displays this symbol.
    fn create_sprite(&self) -> T;
}
