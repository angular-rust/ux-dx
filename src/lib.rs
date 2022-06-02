//! Game Engines and 3D Graphics support library

#![doc(html_logo_url = "https://dudochkin-victor.github.io/assets/ux-dx/logo.svg")]

// #![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
// #![cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
// #[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
// #[cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

#![allow(
    clippy::enum_variant_names,
    clippy::comparison_chain,
    clippy::option_map_unit_fn,
    clippy::redundant_closure,
    clippy::if_same_then_else,
    clippy::new_without_default,
    clippy::needless_bool,
    clippy::comparison_to_empty,
    clippy::manual_swap,
    clippy::new_ret_no_self,
    clippy::unnecessary_operation,
    clippy::single_match,
    clippy::clone_on_copy,
    clippy::excessive_precision,
    clippy::ptr_arg,
    clippy::search_is_some,
    clippy::let_and_return,
    clippy::missing_safety_doc,
    clippy::let_unit_value,
    clippy::needless_range_loop,
    clippy::iter_nth_zero,
    clippy::len_zero,
    clippy::assign_op_pattern,
    clippy::needless_borrow,
    clippy::too_many_arguments,
    clippy::single_char_pattern,
    clippy::map_clone,
    clippy::needless_return,
    clippy::useless_format,
    clippy::match_like_matches_macro,
    clippy::needless_update,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::slow_vector_initialization,
    clippy::derivable_impls,
    clippy::redundant_clone,
    clippy::drop_ref,
    clippy::should_implement_trait,
    clippy::wrong_self_convention,
    clippy::manual_map,
    clippy::identity_op,
    clippy::module_inception,
    clippy::or_fun_call,
    clippy::map_entry,
    clippy::while_let_loop,
    clippy::redundant_clone,
    clippy::vec_init_then_push,
    clippy::borrowed_box,
    clippy::manual_unwrap_or,
    clippy::collapsible_else_if,
    clippy::redundant_static_lifetimes,
    clippy::collapsible_if,
    clippy::redundant_field_names,
    clippy::deprecated_cfg_attr,
    clippy::eq_op,
    clippy::approx_constant,
    clippy::len_without_is_empty,
)]

#![feature(box_syntax)]
#![feature(async_closure)]

// #![warn(missing_docs)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate anyhow;

pub use winit;

pub mod engine;
pub mod foundation;
pub mod platform;
pub mod prelude;
pub mod support;
pub mod ui;
pub mod utils;

/// Detect `assets` folder for Debug/Release modes
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

/// Load asset from `assets` folder as Cow<'a, &[u8]>
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

// AddressMode, BlendOperation, BufferAddress, VertexFormat,
// InputStepMode, ShaderLocation, StencilOperation,
// Face, FilterMode, TextureSampleType, VertexAttribute,

mod deprecated {
    use crate::platform::core::{
        BlendFactor, CompareFunction, FrontFace, TextureFormat, VertexStructure,
    };

    #[derive(Copy, Clone, Debug)]
    pub enum BlendingOperation {
        Add,
        Subtract,
        ReverseSubtract,
        Min,
        Max,
    }

    impl Default for BlendingOperation {
        fn default() -> Self {
            Self::Add
        }
    }

    #[derive(Default, Clone, Debug)]
    pub struct ConstantLocation;

    #[derive(Copy, Clone, Debug)]
    pub enum DepthStencilFormat {
        DepthOnly,
        NoDepthAndStencil,
    }

    impl Default for DepthStencilFormat {
        fn default() -> Self {
            Self::NoDepthAndStencil
        }
    }

    #[derive(Default)]
    pub struct FragmentShader;

    #[derive(Copy, Clone, Debug)]
    pub enum MipMapFilter {
        NoMipFilter,
        PointMipFilter,
        LinearMipFilter,
    }

    impl Default for MipMapFilter {
        fn default() -> Self {
            Self::NoMipFilter
        }
    }

    #[derive(Clone, Debug)]
    pub struct PipelineState {
        pub input_layout: Vec<VertexStructure>,
        pub depth_write: bool,
        pub depth_mode: CompareFunction,
        pub cull_mode: FrontFace,
        pub blend_source: BlendFactor,
        pub blend_destination: BlendFactor,
        pub blend_operation: BlendingOperation,
        pub alpha_blend_source: BlendFactor,
        pub alpha_blend_destination: BlendFactor,
        pub alpha_blend_operation: BlendingOperation,
        pub color_write_masks_red: Vec<bool>,
        pub color_write_masks_green: Vec<bool>,
        pub color_write_masks_blue: Vec<bool>,
        pub color_write_masks_alpha: Vec<bool>,
        pub color_attachment_count: usize,
        pub color_attachments: Vec<TextureFormat>,
        pub depth_stencil_attachment: Option<DepthStencilFormat>,
        pub conservative_rasterization: Option<bool>,
        pub vertex_shader: Option<bool>,
        pub fragment_shader: Option<bool>,
        pub geometry_shader: Option<bool>,
        pub tessellation_control_shader: Option<bool>,
        pub tessellation_evaluation_shader: Option<bool>,
    }

    impl PipelineState {
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl Default for PipelineState {
        fn default() -> Self {
            Self {
                input_layout: Vec::new(),
                depth_write: false,
                depth_mode: CompareFunction::Equal,
                cull_mode: FrontFace::Ccw,
                blend_source: BlendFactor::OneMinusSrc,
                blend_destination: BlendFactor::OneMinusSrc,
                blend_operation: BlendingOperation::Add,
                alpha_blend_source: BlendFactor::OneMinusSrc,
                alpha_blend_destination: BlendFactor::OneMinusSrc,
                alpha_blend_operation: BlendingOperation::Add,
                color_write_masks_red: Vec::new(),
                color_write_masks_green: Vec::new(),
                color_write_masks_blue: Vec::new(),
                color_write_masks_alpha: Vec::new(),
                color_attachment_count: 0,
                color_attachments: Vec::new(),
                depth_stencil_attachment: None,
                conservative_rasterization: None,
                vertex_shader: None,
                fragment_shader: None,
                geometry_shader: None,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum TextureAddressing {
        Repeat,
    }

    impl Default for TextureAddressing {
        fn default() -> Self {
            Self::Repeat
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum TextureFilter {
        LinearFilter,
    }

    impl Default for TextureFilter {
        fn default() -> Self {
            Self::LinearFilter
        }
    }

    #[derive(Default, Clone, Debug)]
    pub struct TextureUnit;

    #[derive(Default)]
    pub struct VertexShader;

    // #[deprecated]
    #[derive(Default)]
    pub struct Graphics;

    #[deprecated]
    #[derive(Default)]
    pub struct Value;

    #[derive(Default)]
    pub struct Sound;

    #[derive(Default)]
    pub struct AudioChannel;
}

// pub use self::deprecated::*;
