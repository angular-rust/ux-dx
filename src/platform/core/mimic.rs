#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{
    borrow::Cow,
    ffi::CString,
    mem,
    num::NonZeroU8,
    ops::Range,
    os::raw::c_void,
    ptr, str,
    sync::{mpsc::Receiver, Arc},
};

pub use wgpu_types::{
    AddressMode, BindGroupLayoutEntry, BlendState, BufferAddress, BufferSize, BufferUsages,
    ColorTargetState, ColorWrites, CompareFunction, DepthStencilState, Extent3d, Face, FilterMode,
    FrontFace, ImageDataLayout, ImageSubresourceRange, MultisampleState, Origin3d, PolygonMode,
    PresentMode, PrimitiveState, PrimitiveTopology, PushConstantRange, QueryType,
    SamplerBorderColor, SurfaceConfiguration, TextureFormat, TextureUsages, VertexAttribute,
    VertexStepMode,
};

use winit::{
    dpi::{LogicalSize, PhysicalSize, Size},
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    foundation::colorspace::Color,
    platform::gles::{core20::gl, enums::*},
};

use super::{Buffer, Texture};

pub struct ShaderFlags(u32);

impl ShaderFlags {
    pub const VALIDATION: ShaderFlags = ShaderFlags(0);
    pub const EXPERIMENTAL_TRANSLATION: ShaderFlags = ShaderFlags(1);
}

impl Default for ShaderFlags {
    fn default() -> Self {
        ShaderFlags::VALIDATION
    }
}

pub type Label<'a> = Option<&'a str>;

pub type CommandEncoderDescriptor<'a> = wgpu_types::CommandEncoderDescriptor<Label<'a>>;

pub type ImageCopyBuffer<'a> = wgpu_types::ImageCopyBuffer<&'a Buffer>;

pub type ImageCopyTexture<'a> = wgpu_types::ImageCopyTexture<&'a Texture>;

pub enum ShaderSource<'a> {
    Vertex(Cow<'a, [u8]>),
    Fragment(Cow<'a, [u8]>),
    Geometry(Cow<'a, [u8]>),       // GL30, ES32
    Compute(Cow<'a, [u8]>),        // GL43, ES31
    TessControl(Cow<'a, [u8]>),    // GL4, ES32
    TessEvaluation(Cow<'a, [u8]>), // GL4, ES32
}

pub struct ShaderModule(u32);

pub struct ShaderModuleDescriptor<'a> {
    pub label: Label<'a>,
    pub source: ShaderSource<'a>,
    pub flags: ShaderFlags,
}

pub enum LoadOp<V> {
    Clear(V),
    Load,
}

pub struct Operations<V> {
    pub load: LoadOp<V>,
    pub store: bool,
}

pub struct RenderPassColorAttachment<'a> {
    pub view: &'a TextureView,
    pub resolve_target: Option<&'a TextureView>,
    pub ops: Operations<Color>,
}

pub struct RenderPassDepthStencilAttachment<'a> {
    pub view: &'a TextureView,
    pub depth_ops: Option<Operations<f32>>,
    pub stencil_ops: Option<Operations<u32>>,
}

pub struct RenderPassDescriptor<'a, 'b> {
    pub label: Label<'a>,
    pub color_attachments: &'b [RenderPassColorAttachment<'a>],
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
}

pub struct CommandBuffer {/* fields omitted */}

pub struct RenderPass<'a> {
    label: Label<'a>,
}

pub struct ComputePass<'a> {
    label: Label<'a>,
}

pub struct ComputePassDescriptor<'a> {
    pub label: Label<'a>,
}

pub struct CommandEncoder;

impl CommandEncoder {
    pub fn finish(&self) -> CommandBuffer {
        unimplemented!()
    }

    pub fn begin_render_pass<'a>(
        &'a mut self,
        desc: &RenderPassDescriptor<'a, '_>,
    ) -> RenderPass<'a> {
        RenderPass {
            label: desc.label.clone(),
        }
    }

    pub fn begin_compute_pass<'a>(&mut self, desc: &ComputePassDescriptor<'a>) -> ComputePass<'a> {
        ComputePass {
            label: desc.label.clone(),
        }
    }

    pub fn copy_buffer_to_buffer(
        &mut self,
        source: &Buffer,
        source_offset: BufferAddress,
        destination: &Buffer,
        destination_offset: BufferAddress,
        copy_size: BufferAddress,
    ) {
    }

    pub fn copy_buffer_to_texture(
        &mut self,
        source: ImageCopyBuffer<'_>,
        destination: ImageCopyTexture<'_>,
        copy_size: Extent3d,
    ) {
    }

    pub fn copy_texture_to_buffer(
        &mut self,
        source: ImageCopyTexture<'_>,
        destination: ImageCopyBuffer<'_>,
        copy_size: Extent3d,
    ) {
    }

    pub fn copy_texture_to_texture(
        &mut self,
        source: ImageCopyTexture<'_>,
        destination: ImageCopyTexture<'_>,
        copy_size: Extent3d,
    ) {
    }

    pub fn clear_texture(&mut self, texture: &Texture, subresource_range: &ImageSubresourceRange) {}

    pub fn clear_buffer(
        &mut self,
        buffer: &Buffer,
        offset: BufferAddress,
        size: Option<BufferSize>,
    ) {
    }

    pub fn insert_debug_marker(&mut self, label: &str) {}

    pub fn push_debug_group(&mut self, label: &str) {}

    pub fn pop_debug_group(&mut self) {}

    pub fn write_timestamp(&mut self, query_set: &QuerySet, query_index: u32) {}

    pub fn resolve_query_set(
        &mut self,
        query_set: &QuerySet,
        query_range: Range<u32>,
        destination: &Buffer,
        destination_offset: BufferAddress,
    ) {
    }
}

pub struct RenderBundleEncoder;

pub struct RenderBundleEncoderDescriptor<'a> {
    pub label: Label<'a>,
    pub color_formats: &'a [TextureFormat],
    pub depth_stencil_format: Option<TextureFormat>,
    pub sample_count: u32,
}

pub struct BindGroup;

pub struct TextureView {/* fields omitted */}

pub struct BufferBinding<'a> {
    pub buffer: &'a Buffer,
    pub offset: BufferAddress,
    pub size: Option<BufferSize>,
}

#[non_exhaustive]
pub enum BindingResource<'a> {
    Buffer(BufferBinding<'a>),
    BufferArray(&'a [BufferBinding<'a>]),
    Sampler(&'a Sampler),
    TextureView(&'a TextureView),
    TextureViewArray(&'a [&'a TextureView]),
}

pub struct BindGroupEntry<'a> {
    pub binding: u32,
    pub resource: BindingResource<'a>,
}

pub struct BindGroupDescriptor<'a> {
    pub label: Label<'a>,
    pub layout: &'a BindGroupLayout,
    pub entries: &'a [BindGroupEntry<'a>],
}

pub struct BindGroupLayout;

pub struct BindGroupLayoutDescriptor<'a> {
    pub label: Label<'a>,
    pub entries: &'a [BindGroupLayoutEntry],
}

pub struct PipelineLayout;

pub struct PipelineLayoutDescriptor<'a> {
    pub label: Label<'a>,
    pub bind_group_layouts: &'a [&'a BindGroupLayout],
    pub push_constant_ranges: &'a [PushConstantRange],
}

pub struct RenderPipeline(u32);

pub struct VertexBufferLayout<'a> {
    pub array_stride: BufferAddress,
    pub step_mode: VertexStepMode,
    pub attributes: &'a [VertexAttribute],
}

pub struct VertexState<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: &'a str,
    pub buffers: &'a [VertexBufferLayout<'a>],
}

pub struct FragmentState<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: &'a str,
    pub targets: &'a [ColorTargetState],
}

pub struct RenderPipelineDescriptor<'a> {
    pub label: Label<'a>,
    pub layout: Option<&'a PipelineLayout>,
    pub vertex: VertexState<'a>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState<'a>>,
}

pub struct ComputePipeline(u32);

pub struct ComputePipelineDescriptor<'a> {
    pub label: Label<'a>,
    pub layout: Option<&'a PipelineLayout>,
    pub module: &'a ShaderModule,
    pub entry_point: &'a str,
}

// pub struct Buffer;

pub type BufferDescriptor<'a> = wgpu_types::BufferDescriptor<Label<'a>>;

// pub struct Texture;

pub type TextureDescriptor<'a> = wgpu_types::TextureDescriptor<Label<'a>>;

pub struct Sampler;

pub struct SamplerDescriptor<'a> {
    pub label: Label<'a>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: Option<NonZeroU8>,
    pub border_color: Option<SamplerBorderColor>,
}

pub struct QuerySet;

pub struct QuerySetDescriptor {
    pub ty: QueryType,
    pub count: u32,
}

pub struct BufferInitDescriptor<'a> {
    pub label: Label<'a>,
    pub contents: &'a [u8],
    pub usage: BufferUsages,
}

#[derive(Debug)]
pub enum SwapChainError {
    Timeout,
    Outdated,
    Lost,
    OutOfMemory,
}

pub struct SwapChainTexture {
    pub view: TextureView,
    // some fields omitted
}

pub struct SwapChainFrame {
    pub output: SwapChainTexture,
    pub suboptimal: bool,
}

pub struct SwapChain {
    egl: Arc<egl::Instance<egl::Dynamic<libloading::Library, egl::EGL1_4>>>,
    display: egl::Display,
}

impl SwapChain {
    pub fn present(&self, surface: &Surface) {
        self.egl
            .swap_buffers(self.display, surface.raw)
            .expect("unable to post EGL context");
    }

    pub fn get_current_frame(&self) -> Result<SwapChainFrame, SwapChainError> {
        Ok(SwapChainFrame {
            output: SwapChainTexture {
                view: TextureView {},
            },
            suboptimal: true,
        })
    }
}

// #[repr(C)]
// pub struct SurfaceConfiguration {
//     pub usage: TextureUsage,
//     pub format: TextureFormat,
//     pub width: u32,
//     pub height: u32,
//     pub present_mode: PresentMode,
// }

// impl SurfaceConfiguration {
//     fn new() -> Self {
//         Self {
//             usage: TextureUsage::SAMPLED,
//             format: TextureFormat::Rgba8Uint,
//             width: 0,
//             height: 0,
//             present_mode: PresentMode::Immediate,
//         }
//     }
// }

// impl Default for SurfaceConfiguration {
//     fn default() -> Self {
//         Self::new()
//     }
// }

pub struct Surface {
    pub(crate) raw: egl::Surface,
}

impl Surface {}

pub struct Queue;

impl Queue {
    fn new() -> Self {
        Self {}
    }

    pub fn write_buffer(&self, buffer: &Buffer, offset: BufferAddress, data: &[u8]) {
        unimplemented!()
    }

    pub fn write_texture(
        &self,
        texture: ImageCopyTexture<'_>,
        data: &[u8],
        data_layout: ImageDataLayout,
        size: Extent3d,
    ) {
        unimplemented!()
    }

    pub fn submit<I: IntoIterator<Item = CommandBuffer>>(&self, command_buffers: I) {
        unimplemented!()
    }

    pub fn get_timestamp_period(&self) -> f32 {
        unimplemented!()
    }
}

pub struct Device {
    egl: Arc<egl::Instance<egl::Dynamic<libloading::Library, egl::EGL1_4>>>,
    display: egl::Display,
}

impl Device {
    pub fn create_shader_module(&self, desc: &ShaderModuleDescriptor) -> ShaderModule {
        let (source, kind) = match &desc.source {
            ShaderSource::Vertex(source) => (source, GL_VERTEX_SHADER),
            ShaderSource::Fragment(source) => (source, GL_FRAGMENT_SHADER),
            ShaderSource::Compute(source) => (source, GL_COMPUTE_SHADER),
            ShaderSource::Geometry(source) => (source, GL_GEOMETRY_SHADER),
            ShaderSource::TessControl(source) => (source, GL_TESS_CONTROL_SHADER),
            ShaderSource::TessEvaluation(source) => (source, GL_TESS_EVALUATION_SHADER),
        };

        let id = gl::create_shader(kind);

        gl::shader_source(id, source.as_ref());
        gl::compile_shader(id);

        let success = gl::get_shaderiv(id, GL_COMPILE_STATUS);

        if success == 0 {
            let len = gl::get_shaderiv(id, GL_INFO_LOG_LENGTH);

            match gl::get_shader_info_log(id, len) {
                Some(message) => panic!("{}: {}", desc.label.unwrap_or("Shader"), message),
                None => panic!("{}: Compile error", desc.label.unwrap_or("Shader")),
            };
        }

        ShaderModule(id)
    }

    pub fn create_command_encoder(&self, desc: &CommandEncoderDescriptor) -> CommandEncoder {
        CommandEncoder {}
    }

    pub fn create_render_bundle_encoder(
        &self,
        desc: &RenderBundleEncoderDescriptor,
    ) -> RenderBundleEncoder {
        unimplemented!()
    }

    pub fn create_bind_group(&self, desc: &BindGroupDescriptor) -> BindGroup {
        unimplemented!()
    }

    pub fn create_bind_group_layout(&self, desc: &BindGroupLayoutDescriptor) -> BindGroupLayout {
        unimplemented!()
    }

    pub fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor) -> PipelineLayout {
        PipelineLayout {}
    }

    pub fn create_render_pipeline(&self, desc: &RenderPipelineDescriptor) -> RenderPipeline {
        let program_id = gl::create_program();

        gl::attach_shader(program_id, desc.vertex.module.0);
        if let Some(fragment) = &desc.fragment {
            gl::attach_shader(program_id, fragment.module.0);
        }

        gl::link_program(program_id);

        // error handling here
        let success = gl::get_programiv(program_id, GL_LINK_STATUS);

        if success == 0 {
            let len = gl::get_programiv(program_id, GL_INFO_LOG_LENGTH);

            match gl::get_program_info_log(program_id, len) {
                Some(message) => panic!("{}: {}", desc.label.unwrap_or("Render Pipeline"), message),
                None => panic!("{}: Link error", desc.label.unwrap_or("Render Pipeline")),
            };
        }

        gl::detach_shader(program_id, desc.vertex.module.0);
        if let Some(fragment) = &desc.fragment {
            gl::detach_shader(program_id, fragment.module.0);
        }

        RenderPipeline(program_id)
    }

    pub fn create_compute_pipeline(&self, desc: &ComputePipelineDescriptor) -> ComputePipeline {
        unimplemented!()
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> Buffer {
        unimplemented!()
    }

    pub fn create_texture(&self, desc: &TextureDescriptor) -> Texture {
        unimplemented!()
    }

    pub fn create_sampler(&self, desc: &SamplerDescriptor) -> Sampler {
        unimplemented!()
    }

    pub fn create_query_set(&self, desc: &QuerySetDescriptor) -> QuerySet {
        unimplemented!()
    }

    pub fn create_swap_chain(&self, surface: &Surface, desc: &SurfaceConfiguration) -> SwapChain {
        SwapChain {
            egl: self.egl.clone(),
            display: self.display,
        }
    }

    pub fn on_uncaptured_error(&self) {
        unimplemented!()
    }

    pub fn start_capture(&self) {
        unimplemented!()
    }

    pub fn stop_capture(&self) {
        unimplemented!()
    }

    // DeviceExt

    pub fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> Buffer {
        unimplemented!()
    }

    pub fn create_texture_with_data(
        &self,
        queue: &Queue,
        desc: &TextureDescriptor,
        data: &[u8],
    ) -> Texture {
        unimplemented!()
    }
}

pub struct Adapter {
    egl: Arc<egl::Instance<egl::Dynamic<libloading::Library, egl::EGL1_4>>>,
    display: egl::Display,
}

impl Adapter {
    pub fn request_device(&self) -> (Device, Queue) {
        let device = Device {
            egl: self.egl.clone(),
            display: self.display,
        };

        let queue = Queue {};

        (device, queue)
    }
}

pub struct Instance {
    egl: Arc<egl::Instance<egl::Dynamic<libloading::Library, egl::EGL1_4>>>,
    display: egl::Display,
}

impl Instance {
    pub fn new() -> Self {
        // let lib = libloading::Library::new("libEGL.so.1").expect("unable to find libEGL.so.1");
        // let egl = unsafe { egl::DynamicInstance::<egl::EGL1_4>::load_required_from(lib).expect("unable to load libEGL.so.1") };

        // EGL setup here
        let egl = unsafe {
            Arc::new(
                egl::DynamicInstance::<egl::EGL1_4>::load_required()
                    .expect("unable to load libEGL.so.1"),
            )
        };

        // Setup OpenGL ES API
        egl.bind_api(egl::OPENGL_ES_API)
            .expect("unable to select OpenGL ES API"); // for OpenGL ES

        // Setup Display
        let display = egl
            .get_display(egl::DEFAULT_DISPLAY)
            .expect("unable to get display");

        egl.initialize(display).expect("unable to init EGL");

        use std::ffi::CStr;

        unsafe {
            libloading::Library::new("libGLESv2.so.2").expect("unable to find libGLESv2.so.2");
            crate::platform::gles::ffi::load_global_gl_with(|c_str| {
                let procname = CStr::from_ptr(c_str).to_str().unwrap();
                egl.get_proc_address(procname).unwrap() as *mut std::ffi::c_void
            });
            // unsafe { mem::transmute(egli::egl::get_proc_address(s)) }
        };

        Self { egl, display }
    }

    pub fn request_adapter(&self) -> Adapter {
        Adapter {
            egl: self.egl.clone(),
            display: self.display,
        }
    }

    pub fn create_surface(&self, window: &Window) -> Surface {
        // Create context
        let attrib_list = [
            egl::BUFFER_SIZE,
            16,
            egl::DEPTH_SIZE,
            16,
            egl::STENCIL_SIZE,
            0,
            egl::SURFACE_TYPE,
            egl::WINDOW_BIT,
            egl::NONE,
        ];

        // // Get the number of matching configurations.
        // let count = egl
        //     .matching_config_count(display, &attrib_list)
        //     .expect("no available configurations");

        // Get the matching configuration.
        let config = self
            .egl
            .choose_first_config(self.display, &attrib_list)
            .expect("unable to choose EGL configuration")
            .expect("no EGL configuration found");

        // if ou not set CONTEXT_CLIENT_VERSION if will set to GLES v.1, otherwise 2 or more;
        let ctx_attribs = [egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE]; // GLESv2/3+
                                                                       // let ctx_attribs = [egl::NONE]; // GLESv1+
        let ctx = self
            .egl
            .create_context(self.display, config, None, &ctx_attribs)
            .expect("unable to create EGL context");

        // Create a EGL surface
        let surface = unsafe {
            let window_handle = match window.raw_window_handle() {
                RawWindowHandle::Xlib(handle) => handle.window as egl::NativeWindowType,
                RawWindowHandle::Xcb(handle) => handle.window as egl::NativeWindowType,
                RawWindowHandle::Wayland(handle) => handle.surface as egl::NativeWindowType,
                _ => {
                    panic!("Other handle type");
                }
            };

            self.egl
                .create_window_surface(self.display, config, window_handle, None)
                .expect("unable to create an EGL surface")
        };

        self.egl
            .make_current(self.display, Some(surface), Some(surface), Some(ctx))
            .expect("unable to bind the context");

        Surface { raw: surface }
    }
}
