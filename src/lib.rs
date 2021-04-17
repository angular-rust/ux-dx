#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]

#[macro_use]
extern crate glib;

#[macro_use]
extern crate bitflags;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
mod legacy;
pub use legacy::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
