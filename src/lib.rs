#![doc(html_logo_url = "https://dudochkin-victor.github.io/assets/ux-dx/logo.svg")]

// #![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
// #![cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
// #[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
// #[cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

#[macro_use]
extern crate bitflags;

pub mod ext;

pub mod app;

pub mod canvas;

pub mod core;

pub mod gles;

mod mimic;
pub use self::mimic::*;

pub mod resource;

pub mod scene;

pub mod time;

pub mod ui;

pub mod utils;

pub use wgpu_types::{
    BlendFactor,     // BlendingFactor
    CompareFunction, // CompareMode
    Face,
    FrontFace,
    ShaderLocation,
    StencilOperation, // StencilAction
    TextureFormat,
    TextureSampleType,
    VertexFormat,
};

pub use primitives::{color, colorspace::Color};

// Prelude provides all the traits of the library in a convenient form
pub mod prelude {
    pub use crate::core::traits::*;
    pub use primitives::prelude::*;
    // pub use crate::collision::bound::{PlaneBound, Relation};
    // pub use crate::collision::traits::*;
    // pub use crate::collision::volume::{Aabb, MinMax};
}

// Detect `assets` folder for Debug/Release modes
#[macro_export]
macro_rules! assets {
    ($path:tt) => {{
        use std::{
            env,
            path::{Path, PathBuf},
        };
        // std::println!($($arg)*)
        let path = PathBuf::from($path);
        if path.is_relative() {
            // deal with relative paths
            if cfg!(debug_assertions) {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets")
                    .join(path)
            } else {
                match env::current_exe().unwrap().parent() {
                    Some(dir) => dir.join("assets").join(path),
                    None => {
                        // exe located in root. aka "/"
                        path
                    }
                }
            }
        } else {
            path
        }
    }};
}

// Load asset from `assets` folder as Cow<'a, &[u8]>
#[macro_export]
macro_rules! assets_source {
    ($path:tt) => {{
        use std::{
            borrow::Cow,
            env,
            fs::File,
            io::prelude::*,
            path::{Path, PathBuf},
        };
        // std::println!($($arg)*)
        let path = PathBuf::from($path);
        let path = if path.is_relative() {
            // deal with relative paths
            if cfg!(debug_assertions) {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets")
                    .join(path)
            } else {
                match env::current_exe().unwrap().parent() {
                    Some(dir) => dir.join("assets").join(path),
                    None => {
                        // exe located in root. aka "/"
                        path
                    }
                }
            }
        } else {
            path
        };

        let mut buffer = Vec::new();
        let mut file = File::open(path).expect("File open error");
        file.read_to_end(&mut buffer).expect("File read error");

        Cow::from(buffer)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
