//! A [`ux-dx`] renderer for [`iced_native`].
//!
//! ![The native path of the Iced ecosystem](https://github.com/hecrj/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/native.png?raw=true)
//!
//! [`ux-dx`]: https://github.com/angular-rust/ux-dx
//! [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
// #![forbid(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod backend;
mod clipboard;
mod program;
mod mode;
mod quad;
mod text;
mod triangle;

pub mod conversion;
pub mod themes;

pub mod settings;
pub mod widget;
pub mod window;

pub use backend::Backend;
pub use clipboard::Clipboard;
pub use mode::Mode;
pub use settings::Settings;

pub(crate) use iced_graphics::Transformation;

#[doc(no_inline)]
pub use widget::*;

// promote some subject from iced_graphics
pub use iced_graphics::{Error, Viewport, triangle::{Mesh2D, Vertex2D}, Defaults, Primitive};
pub use iced_native::{Background, Color, Command, HorizontalAlignment, Length, Vector, VerticalAlignment};

/// A [`ux-dx`] graphics renderer for [`iced`].
///
/// [`ux-dx`]: https://github.com/angular-rust/ux-dx
/// [`iced`]: https://github.com/hecrj/iced
pub type Renderer = iced_graphics::Renderer<Backend>;
