//! The `dx` prelude.
//!
//! The purpose of this module is to alleviate imports of many common dx
//! traits by adding a glob import to the top of dx heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use dx::prelude::*;
//! ```

pub use crate::platform::core::traits::*;

pub use primitives::prelude::*;
pub use ruex::prelude::*;

// pub use crate::collision::bound::{PlaneBound, Relation};
// pub use crate::collision::traits::*;
// pub use crate::collision::volume::{Aabb, MinMax};

mod renderable;
pub use self::renderable::*;
