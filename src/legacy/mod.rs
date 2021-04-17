#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use glib::translate::*;

mod atlas_texture;
pub use self::atlas_texture::AtlasTexture;

mod attribute;
pub use self::attribute::Attribute;

mod attribute_buffer;
pub use self::attribute_buffer::AttributeBuffer;

mod bitmap;
pub use self::bitmap::Bitmap;

mod color;
pub use self::color::Color;

mod context;
pub use self::context::Context;

mod debug_object_type_info;
pub use debug_object_type_info::DebugObjectTypeInfo;

mod depth_state;
pub use depth_state::DepthState;

mod display;
pub use self::display::Display;

mod euler;
pub use self::euler::Euler;

mod fence_closure;
pub use fence_closure::FenceClosure;

mod fence;
pub use fence::Fence;

mod fixed;
pub use self::fixed::Fixed;

mod frame_closure;
pub use self::frame_closure::FrameClosure;

mod frame_info;
pub use self::frame_info::FrameInfo;

mod framebuffer;
pub use self::framebuffer::{Framebuffer, FramebufferExt};

mod gles2_context;
pub use self::gles2_context::GLES2Context;

mod gles2_vtable;
pub use gles2_vtable::GLES2Vtable;

mod gtype_object;
pub use gtype_object::GtypeObject;

mod index_buffer;
pub use self::index_buffer::IndexBuffer;

mod handle;
pub use handle::Handle;

mod indices;
pub use self::indices::Indices;

mod kms_crtc;
pub use kms_crtc::KmsCrtc;

mod material;
pub use material::Material;

mod material_layer;
pub use material_layer::MaterialLayer;

mod matrix;
pub use self::matrix::Matrix;

mod matrix_entry;
pub use self::matrix_entry::MatrixEntry;

mod matrix_stack;
pub use self::matrix_stack::MatrixStack;

mod object;
pub use self::object::{Object, ObjectExt};

mod offscreen;
pub use self::offscreen::Offscreen;

mod onscreen;
pub use self::onscreen::Onscreen;

mod onscreen_dirty_info;
pub use onscreen_dirty_info::OnscreenDirtyInfo;

mod onscreen_dirty_closure;
pub use self::onscreen_dirty_closure::OnscreenDirtyClosure;

mod onscreen_resize_closure;
pub use self::onscreen_resize_closure::OnscreenResizeClosure;

mod onscreen_template;
pub use self::onscreen_template::OnscreenTemplate;

mod output;
pub use self::output::Output;

mod pipeline;
pub use self::pipeline::Pipeline;

mod pixel_buffer;
pub use self::pixel_buffer::PixelBuffer;

mod poll_fd;
pub use poll_fd::PollFD;

mod primitive;
pub use self::primitive::Primitive;

mod quaternion;
pub use self::quaternion::Quaternion;

mod renderer;
pub use self::renderer::Renderer;

mod snippet;
pub use self::snippet::Snippet;

mod sub_texture;
pub use self::sub_texture::SubTexture;

mod swap_chain;
pub use self::swap_chain::SwapChain;

mod texture;
pub use self::texture::{Texture, TextureExt};

mod texture2_d;
pub use self::texture2_d::Texture2D;

mod texture2_dsliced;
pub use self::texture2_dsliced::Texture2DSliced;

mod texture3_d;
pub use self::texture3_d::Texture3D;

mod texture_pixmap_x11;
pub use self::texture_pixmap_x11::TexturePixmapX11;

mod texture_rectangle;
pub use self::texture_rectangle::TextureRectangle;

mod texture_vertex;
pub use texture_vertex::TextureVertex;

mod user_data_key;
pub use user_data_key::UserDataKey;

mod vertex_p2;
pub use vertex_p2::VertexP2;

mod vertex_p2c4;
pub use vertex_p2c4::VertexP2C4;

mod vertex_p2t2;
pub use vertex_p2t2::VertexP2T2;

mod vertex_p2t2c4;
pub use vertex_p2t2c4::VertexP2T2C4;

mod vertex_p3;
pub use vertex_p3::VertexP3;

mod vertex_p3c4;
pub use vertex_p3c4::VertexP3C4;

mod vertex_p3t2;
pub use vertex_p3t2::VertexP3T2;

mod vertex_p3t2c4;
pub use vertex_p3t2c4::VertexP3T2C4;

mod enums;
pub use self::enums::AttributeType;
pub use self::enums::BitmapError;
pub use self::enums::BlendStringError;
pub use self::enums::BufferError;
pub use self::enums::BufferUpdateHint;
pub use self::enums::DepthTestFunction;
pub use self::enums::Driver;
pub use self::enums::FeatureID;
pub use self::enums::FilterReturn;
pub use self::enums::FogMode;
pub use self::enums::FrameEvent;
pub use self::enums::FramebufferError;
pub use self::enums::GLES2ContextError;
pub use self::enums::IndicesType;
pub use self::enums::MaterialAlphaFunc;
pub use self::enums::MaterialFilter;
pub use self::enums::MaterialLayerType;
pub use self::enums::MaterialWrapMode;
pub use self::enums::PipelineAlphaFunc;
pub use self::enums::PipelineCullFaceMode;
pub use self::enums::PipelineFilter;
pub use self::enums::PipelineWrapMode;
pub use self::enums::PixelFormat;
pub use self::enums::PollFDEvent;
pub use self::enums::RendererError;
pub use self::enums::ShaderType;
pub use self::enums::SnippetHook;
pub use self::enums::StereoMode;
pub use self::enums::SubpixelOrder;
pub use self::enums::SystemError;
pub use self::enums::TextureComponents;
pub use self::enums::TextureError;
pub use self::enums::TexturePixmapX11Error;
pub use self::enums::TexturePixmapX11ReportLevel;
pub use self::enums::TextureType;
pub use self::enums::VerticesMode;
pub use self::enums::Winding;
pub use self::enums::WinsysFeature;
pub use self::enums::WinsysID;

mod flags;
pub use self::flags::BufferAccess;
pub use self::flags::BufferBit;
pub use self::flags::BufferMapHint;
pub use self::flags::BufferTarget;
pub use self::flags::ColorMask;
pub use self::flags::FeatureFlags;
pub use self::flags::ReadPixelsFlags;
pub use self::flags::RendererConstraint;
pub use self::flags::TextureFlags;

mod alias;
pub use alias::Angle;
// pub use alias::Bool;
pub use alias::Buffer;
// pub use alias::Handle;
pub use alias::MetaTexture;
pub use alias::PrimitiveTexture;
pub use alias::UserDataDestroyCallback;

pub(crate) const TRUE: i32 = ::glib_sys::GTRUE;
// pub(crate) const FALSE:i32 = ::glib_sys::GFALSE;

pub fn source_new(context: &Context, priority: glib::Priority) -> glib::Source {
    unsafe {
        let source = ffi::cogl_glib_source_new(context.to_glib_none().0, priority.to_glib());
        from_glib_full(source)
    }
}

// let source = glib_sys::g_timeout_source_new(interval);
// glib_sys::g_source_set_callback(
//     source,
//     Some(trampoline::<F>),
//     into_raw(func),
//     Some(destroy_closure::<F>),
// );
// glib_sys::g_source_set_priority(source, priority.to_glib());

// if let Some(name) = name {
//     glib_sys::g_source_set_name(source, name.to_glib_none().0);
// }

#[doc(hidden)]
pub mod traits {
    pub use crate::FramebufferExt;
    pub use crate::ObjectExt;
    pub use crate::TextureExt;
}
