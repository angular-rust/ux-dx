//! Core abstractions

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

mod atlas;
pub use self::atlas::*;

mod atlas_texture;
pub use self::atlas_texture::*;

mod attribute;
pub use self::attribute::*;

mod attribute_buffer;
pub use self::attribute_buffer::*;

mod bitmap;
pub use self::bitmap::*;

mod buffer;
pub use self::buffer::*;

// mod color;
// pub use self::color::Color;

mod context;
pub use self::context::*;

mod debug_object_type_info;
pub use debug_object_type_info::*;

mod depth_state;
pub use depth_state::*;

mod display;
pub use self::display::*;

mod euler;
pub use self::euler::*;

mod fence_closure;
pub use fence_closure::*;

mod fence;
pub use fence::*;

mod fixed;
pub use self::fixed::*;

mod frame_closure;
pub use self::frame_closure::*;

mod frame_info;
pub use self::frame_info::*;

mod framebuffer;
pub use self::framebuffer::*;

mod gles2_context;
pub use self::gles2_context::*;

mod gles2_vtable;
pub use gles2_vtable::*;

mod gpuinfo;
pub use gpuinfo::*;

mod gtype_object;
pub use gtype_object::*;

mod index_buffer;
pub use self::index_buffer::*;

mod handle;
pub use handle::Handle;

mod indices;
pub use self::indices::*;

mod kms_crtc;
pub use kms_crtc::*;

mod material;
pub use material::*;

mod material_layer;
pub use material_layer::*;

mod matrix;
pub use self::matrix::*;

mod matrix_entry;
pub use self::matrix_entry::*;

mod matrix_stack;
pub use self::matrix_stack::*;

mod mimic;
pub use self::mimic::*;

mod offscreen;
pub use self::offscreen::*;

mod onscreen;
pub use self::onscreen::*;

mod onscreen_dirty_info;
pub use onscreen_dirty_info::*;

mod onscreen_dirty_closure;
pub use self::onscreen_dirty_closure::*;

mod onscreen_resize_closure;
pub use self::onscreen_resize_closure::*;

mod onscreen_template;
pub use self::onscreen_template::*;

mod output;
pub use self::output::*;

mod path;
pub use self::path::*;

mod pipeline;
pub use self::pipeline::*;

mod pixel_buffer;
pub use self::pixel_buffer::*;

mod platform;
pub(crate) use self::platform::*;

mod poll_fd;
pub use poll_fd::*;

mod primitive;
pub use self::primitive::*;

mod primitives;
pub use self::primitives::*;

mod quaternion;
pub use self::quaternion::*;

mod rectangle_map;
pub use self::rectangle_map::*;

mod renderer;
pub use self::renderer::*;

mod snippet;
pub use self::snippet::*;

mod sub_texture;
pub use self::sub_texture::*;

mod swapchain;
pub use self::swapchain::*;

mod texture;
pub use self::texture::*;

mod texture2d;
pub use self::texture2d::*;

mod texture2d_sliced;
pub use self::texture2d_sliced::*;

mod texture3d;
pub use self::texture3d::*;

mod texture_pixmap_x11;
pub use self::texture_pixmap_x11::*;

mod texture_rectangle;
pub use self::texture_rectangle::*;

mod texture_vertex;
pub use texture_vertex::*;

#[doc(hidden)]
pub mod traits;

mod user_data_key;
pub use user_data_key::*;

mod enums;
pub use self::enums::*;

mod flags;
pub use self::flags::*;

mod alias;
pub use alias::*;

pub use wgpu_types::{
    AddressMode,
    BlendFactor, // BlendingFactor
    BlendOperation,
    BufferAddress,
    CompareFunction, // CompareMode
    Face,
    FilterMode,
    FrontFace,
    ShaderLocation,
    StencilOperation, // StencilAction
    TextureFormat,
    TextureSampleType,
    VertexAttribute,
    VertexFormat,
    VertexStepMode,
};

pub(crate) fn init() {
    static mut INITIALIZED: u32 = 0;

    unsafe {
        if INITIALIZED == 0 {
            println!("DX Init");
            #[cfg(feature = "nls")]
            {
                bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR);
                bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8");
            }

            // _config_read ();
            // _debug_check_environment ();
        }

        INITIALIZED += 1;

        if INITIALIZED > 10 {
            panic!("Trying to initialize framework lots of times")
        }
    }
}
