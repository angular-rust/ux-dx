#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

//! Some partially implemented abstractions

mod atlas;
pub use self::atlas::Atlas;

mod atlas_texture;
pub use self::atlas_texture::AtlasTexture;

mod attribute;
pub use self::attribute::Attribute;

mod attribute_buffer;
pub use self::attribute_buffer::AttributeBuffer;

mod bitmap;
pub use self::bitmap::Bitmap;

mod buffer;
pub use self::buffer::{Buffer, BufferBindTarget, BufferFlags, BufferUsageHint};

mod camera;
pub use self::camera::*;

// mod color;
// pub use self::color::Color;

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
pub use self::framebuffer::{Framebuffer, FramebufferBits, FramebufferConfig, FramebufferExt};

mod gles2_context;
pub use self::gles2_context::GLES2Context;

mod gles2_vtable;
pub use gles2_vtable::GLES2Vtable;

mod gpuinfo;
pub use gpuinfo::GpuInfo;

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

mod path;
pub use self::path::*;

mod pipeline;
pub use self::pipeline::Pipeline;

mod pixel_buffer;
pub use self::pixel_buffer::PixelBuffer;

mod platform;
pub(crate) use self::platform::*;

mod poll_fd;
pub use poll_fd::PollFD;

mod primitive;
pub use self::primitive::Primitive;

mod primitives;
pub use self::primitives::*;

mod quaternion;
pub use self::quaternion::Quaternion;

mod rectangle_map;
pub use self::rectangle_map::*;

mod renderer;
pub use self::renderer::Renderer;

mod snippet;
pub use self::snippet::Snippet;

mod sub_texture;
pub use self::sub_texture::SubTexture;

mod swapchain;
pub use self::swapchain::LegacySwapChain;

mod texture;
pub use self::texture::{Texture, TextureExt};

mod texture2d;
pub use self::texture2d::Texture2D;

mod texture2d_sliced;
pub use self::texture2d_sliced::Texture2DSliced;

mod texture3d;
pub use self::texture3d::Texture3D;

mod texture_pixmap_x11;
pub use self::texture_pixmap_x11::TexturePixmapX11;

mod texture_rectangle;
pub use self::texture_rectangle::TextureRectangle;

mod texture_vertex;
pub use texture_vertex::TextureVertex;

#[doc(hidden)]
pub mod traits;

mod user_data_key;
pub use user_data_key::UserDataKey;

mod vertex2p;
pub use vertex2p::Vertex2p;

mod vertex2p4c;
pub use vertex2p4c::Vertex2p4c;

mod vertex2p2t;
pub use vertex2p2t::Vertex2p2t;

mod vertex2p2t4c;
pub use vertex2p2t4c::Vertex2p2t4c;

mod vertex3p;
pub use vertex3p::Vertex3p;

mod vertex3p4c;
pub use vertex3p4c::Vertex3p4c;

mod vertex3p2t;
pub use vertex3p2t::Vertex3p2t;

mod vertex3p2t4c;
pub use vertex3p2t4c::Vertex3p2t4c;

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
pub use self::enums::FramebufferState;
pub use self::enums::FramebufferStateIndex;
pub use self::enums::FramebufferType;
pub use self::enums::GLES2ContextError;
pub use self::enums::IndicesType;
pub use self::enums::MaterialAlphaFunc;
pub use self::enums::MaterialFilter;
pub use self::enums::MaterialLayerType;
pub use self::enums::MaterialWrapMode;
pub use self::enums::OffscreenAllocateFlags;
pub use self::enums::OffscreenFlags;
pub use self::enums::PipelineAlphaFunc;
pub use self::enums::PipelineCullFaceMode;
pub use self::enums::PipelineFilter;
pub use self::enums::PipelineWrapMode;
pub use self::enums::PixelFormat;
pub use self::enums::PollFDEvent;
pub use self::enums::PrivateReadPixelsFlags;
pub use self::enums::RendererError;
pub use self::enums::ShaderType;
pub use self::enums::SnippetHook;
pub use self::enums::StereoMode;
pub use self::enums::SubpixelOrder;
pub use self::enums::SystemError;
pub use self::enums::TextureComponents;
pub use self::enums::TextureError;
pub use self::enums::TextureLoader;
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
pub use alias::MetaTexture;
pub use alias::PrimitiveTexture;
pub use alias::UserDataDestroyCallback;

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
