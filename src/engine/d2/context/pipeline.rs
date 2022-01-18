use bytes::Bytes;
use cgmath::prelude::*;
use intmap::IntMap;
use once_cell::sync::OnceCell;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    foundation::colorspace::Color,
    platform::{
        core::{
            BlendFactor, CompareFunction, Face, FrontFace, ShaderLocation, StencilOperation,
            VertexFormat,
        },
        gles::{core20::gl, enums::*},
    },
};

use super::{
    Canvas, CubeMap, Image, MipMapFilter, TextureAddressing, TextureFilter, TextureUnit,
    VertexBuffer, Video,
};

pub type CullMode = Option<Face>;

// #[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
// pub enum BlendFactor {
//     Undefined,
//     BlendOne,
//     BlendZero,
//     SourceAlpha,
//     DestinationAlpha,
//     InverseSourceAlpha,
//     InverseDestinationAlpha,
//     SourceColor,
//     DestinationColor,
//     InverseSourceColor,
//     InverseDestinationColor,
// }

pub enum VertexData {
    Float1,
    Float2,
    Float3,
    Float4,
    Float4x4,
    Short2Norm,
    Short4Norm,
}

// VertexStructure is a verdex data descriptor in terms of Vulkan or WebGPU
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexAttribute {
    pub format: VertexFormat,
    pub offset: u64,
    pub shader_location: u32,
}

impl VertexAttribute {}

// for OpenGL ES 2.0 is available GL_VERTEX_SHADER, GL_FRAGMENT_SHADER
// for OpenGL ES 3.0 is available GL_VERTEX_SHADER, GL_FRAGMENT_SHADER
// for OpenGL ES 3.1 is available GL_VERTEX_SHADER, GL_FRAGMENT_SHADER and GL_COMPUTE_SHADER
// for OpenGL ES 3.2 is available GL_COMPUTE_SHADER, GL_VERTEX_SHADER, GL_TESS_CONTROL_SHADER, GL_TESS_EVALUATION_SHADER, GL_GEOMETRY_SHADER, or GL_FRAGMENT_SHADER

// for OpenGL 2.0 (2.1) is available GL_VERTEX_SHADER, GL_FRAGMENT_SHADER
// for OpenGL 3.0 is available GL_VERTEX_SHADER, GL_GEOMETRY_SHADER or GL_FRAGMENT_SHADER
// for OpenGL 4.0 is available GL_COMPUTE_SHADER, GL_VERTEX_SHADER, GL_TESS_CONTROL_SHADER, GL_TESS_EVALUATION_SHADER, GL_GEOMETRY_SHADER or GL_FRAGMENT_SHADER
// for OpenGL 4.5 is available GL_COMPUTE_SHADER, GL_VERTEX_SHADER, GL_TESS_CONTROL_SHADER, GL_TESS_EVALUATION_SHADER, GL_GEOMETRY_SHADER or GL_FRAGMENT_SHADER

// GL_COMPUTE_SHADER is available only if the GL version is 4.3 or higher.

#[derive(Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
    // Geometry, // GL30, ES32
    // Compute, // GL43, ES31
    // TessControl, // GL4, ES32
    // TessEvaluation // GL4, ES32
}

pub struct ShaderSource {
    source: Bytes,
    kind: ShaderType,
}

impl ShaderSource {
    pub fn new(kind: ShaderType, data: &[u8]) -> Self {
        Self {
            kind,
            source: Bytes::copy_from_slice(data),
        }
    }
}

type Shader = u32;
type Program = u32;
type StencilValue = u32;

// TODO: rename to CompileShader
impl ShaderSource {
    pub fn compile(&self) -> Result<Shader, BuildError> {
        let kind = match self.kind {
            ShaderType::Vertex => GL_VERTEX_SHADER,
            ShaderType::Fragment => GL_FRAGMENT_SHADER,
        };

        let id = gl::create_shader(kind);

        gl::shader_source(id, &self.source);
        gl::compile_shader(id);

        let success = gl::get_shaderiv(id, GL_COMPILE_STATUS);

        if success == 0 {
            let len = gl::get_shaderiv(id, GL_INFO_LOG_LENGTH);

            return match gl::get_shader_info_log(id, len) {
                Some(message) => {
                    println!("COMPILE SHADER ERROR: {:?}\n{}", self.kind, message);
                    Err(BuildError::SHADER_COMPILE)
                }
                None => {
                    // compiled NOT successefull... and there no log info
                    println!("COMPILE SHADER ERROR: without info logs");
                    Err(BuildError::SHADER_COMPILE)
                }
            };
        }

        // compiled successefull
        Ok(id)
    }
}

pub struct PipelineBuilder {
    shaders: Vec<ShaderSource>,
    blend_source: Option<BlendFactor>,
    blend_destination: Option<BlendFactor>,
    alpha_blend_source: Option<BlendFactor>,
    alpha_blend_destination: Option<BlendFactor>,
    layout: Vec<VertexAttribute>,
}

impl PipelineBuilder {
    pub fn add_shader(&mut self, value: ShaderSource) -> &mut Self {
        self.shaders.push(value);
        self
    }

    pub fn set_blend_source(&mut self, value: BlendFactor) -> &mut Self {
        self.blend_source = Some(value);
        self
    }

    pub fn set_blend_destination(&mut self, value: BlendFactor) -> &mut Self {
        self.blend_destination = Some(value);
        self
    }

    pub fn set_alpha_blend_source(&mut self, value: BlendFactor) -> &mut Self {
        self.alpha_blend_source = Some(value);
        self
    }

    pub fn set_alpha_blend_destination(&mut self, value: BlendFactor) -> &mut Self {
        self.alpha_blend_destination = Some(value);
        self
    }

    // keep only shader pair - ie. Vertex/Fragment (no tess, compute or geometry)
    pub fn set_input_layout(&mut self, value: Vec<VertexAttribute>) -> &mut Self {
        self.layout = value;
        self
    }

    pub fn build(&self) -> Result<Pipeline, BuildError> {
        let shaders: Result<Vec<Shader>, BuildError> =
            self.shaders.iter().map(|src| src.compile()).collect();

        let program = match shaders {
            Ok(shaders) => {
                let program = gl::create_program();
                shaders
                    .iter()
                    .for_each(|shader| gl::attach_shader(program, *shader));

                gl::link_program(program);

                // error handling here
                let success = gl::get_programiv(program, GL_LINK_STATUS);

                if success == 0 {
                    let len = gl::get_programiv(program, GL_INFO_LOG_LENGTH);

                    return match gl::get_program_info_log(program, len) {
                        Some(message) => {
                            println!("PROGRAM LINK ERROR: \n{}", message);
                            Err(BuildError::PROGRAM_LINK)
                        }
                        None => {
                            println!("PROGRAM LINK ERROR: without info logs");
                            Err(BuildError::PROGRAM_LINK)
                        }
                    };
                }

                shaders
                    .into_iter()
                    .for_each(|shader| gl::detach_shader(program, shader));

                program
            }
            Err(err) => {
                return Err(err);
            }
        };

        let layout = self.layout.clone();

        let stride = layout
            .iter()
            .fold(0_i32, |stride, &attr| stride + attr.format.size() as i32);

        Ok(Pipeline {
            program,
            blend_source: self.blend_source.expect("Blend Source is not set"),
            blend_destination: self
                .blend_destination
                .expect("Blend Destination is not set"),
            alpha_blend_source: self
                .alpha_blend_source
                .expect("Alpha Blend Source is not set"),
            alpha_blend_destination: self
                .alpha_blend_destination
                .expect("Alpha Blend Destination is not set"),
            layout,
            stride,
            props: Default::default(),
        })
    }
}

// extendable error pattern ))
#[derive(Debug)]
pub struct BuildError(u32);

impl BuildError {
    pub const SHADER_COMPILE: Self = Self(1);
    pub const PROGRAM_LINK: Self = Self(2);
}

struct PipelineProps {
    // pub inputLayout: Vec<VertexStructure>,
    // pub vertexShader: VertexShader,
    // pub fragmentShader: FragmentShader,
    // pub geometryShader: GeometryShader,
    // pub tessellationControlShader: TessellationControlShader,
    // pub tessellationEvaluationShader: TessellationEvaluationShader,
    pub cull_mode: CullMode,

    pub depth_write: bool,
    pub depth_mode: CompareFunction,

    pub stencil_mode: CompareFunction,
    pub stencil_both_pass: StencilOperation,
    pub stencil_depth_fail: StencilOperation,
    pub stencil_fail: StencilOperation,
    pub stencil_reference_value: StencilValue,
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,

    // One, Zero deactivates blending
    pub blend_source: BlendFactor,
    pub blend_destination: BlendFactor,
    // pub blendOperation: BlendingOperation,
    pub alpha_blend_source: BlendFactor,
    pub alpha_blend_destination: BlendFactor,
    // pub alphaBlendOperation: BlendingOperation,

    // pub colorWriteMask/*(never, set)*/: bool,
    // pub colorWriteMaskRed/*(get, set)*/: bool,
    // pub colorWriteMaskGreen/*(get, set)*/: bool,
    // pub colorWriteMaskBlue/*(get, set)*/: bool,
    // pub colorWriteMaskAlpha/*(get, set)*/: bool,

    // pub colorWriteMasksRed: Vec<bool>,
    // pub colorWriteMasksGreen: Vec<bool>,
    // pub colorWriteMasksBlue: Vec<bool>,
    // pub colorWriteMasksAlpha: Vec<bool>,
    pub color_attachment_count: i32,
    // pub colorAttachments: Vec<TextureFormat>,

    // pub depthStencilAttachment: DepthStencilFormat,
}

impl Default for PipelineProps {
    fn default() -> Self {
        Self {
            // inputLayout: None,
            // vertexShader: None,
            // fragmentShader: None,
            // geometryShader: None,
            // tessellationControlShader: None,
            // tessellationEvaluationShader: None,
            cull_mode: None,

            depth_write: false,
            depth_mode: CompareFunction::Always,

            stencil_mode: CompareFunction::Always,
            stencil_both_pass: StencilOperation::Keep,
            stencil_depth_fail: StencilOperation::Keep,
            stencil_fail: StencilOperation::Keep,
            stencil_reference_value: 0, //Static(0),
            stencil_read_mask: 0xff,
            stencil_write_mask: 0xff,

            blend_source: BlendFactor::One,
            blend_destination: BlendFactor::Zero,
            // blendOperation: BlendingOperation::Add,
            alpha_blend_source: BlendFactor::One,
            alpha_blend_destination: BlendFactor::Zero,
            // alphaBlendOperation: BlendFactor::Add,

            // colorWriteMasksRed: Vec::new(),
            // colorWriteMasksGreen: Vec::new(),
            // colorWriteMasksBlue: Vec::new(),
            // colorWriteMasksAlpha: Vec::new(),
            // for (i in 0...8)
            //     colorWriteMasksRed.push(true);
            // for (i in 0...8)
            //     colorWriteMasksGreen.push(true);
            // for (i in 0...8)
            //     colorWriteMasksBlue.push(true);
            // for (i in 0...8)
            //     colorWriteMasksAlpha.push(true);
            color_attachment_count: 1,
            // colorAttachments: [],
            // for (i in 0...8)
            //     colorAttachments.push(TextureFormat.RGBA32),

            // depthStencilAttachment: DepthStencilFormat.NoDepthAndStencil,

            // conservativeRasterization: false,
        }
    }
}
pub struct Pipeline {
    program: Program,
    blend_source: BlendFactor,
    blend_destination: BlendFactor,
    alpha_blend_source: BlendFactor,
    alpha_blend_destination: BlendFactor,
    layout: Vec<VertexAttribute>,
    stride: i32,
    props: Arc<RwLock<PipelineProps>>,
}

// think i should build it
// cox i need to add vertex, tesslelation and other types up to fragment shader
impl Pipeline {
    pub fn construct() -> PipelineBuilder {
        PipelineBuilder {
            shaders: Vec::new(),
            blend_source: None,
            blend_destination: None,
            alpha_blend_source: None,
            alpha_blend_destination: None,
            layout: Vec::new(),
        }
    }

    fn set(&self) {
        gl::use_program(self.program);
        // println!("pipeline program id {}", self.program);

        // FIXME: complete logic
        // for (index in 0...textureValues.length) {
        // 	gl::uniform1i(textureValues[index], index);
        // }

        // FIXME: think it should be loacated in painter
        // gl::color_mask(true, true, true, true);
    }

    pub fn uniform_location(&self, name: &str) -> ShaderLocation {
        let location = gl::get_uniform_location(self.program, name);
        // let kind = gl::FLOAT;
        // let count = new NativeVec<i32>(1);
        // gl::glGetProgramiv(program, gl::ACTIVE_UNIFORMS, count, 0);
        // for (i in 0...count[0]) {
        // 	let nameArray = new NativeVec<Int8>(1024);
        // 	let length = new NativeVec<i32>(1);
        // 	let size = new NativeVec<i32>(1);
        // 	let typeArray = new NativeVec<i32>(1);
        // 	gl::glGetActiveUniform(program, i, 1024, length, 0, size, 0, typeArray, 0, nameArray, 0);
        // 	if compare(name, nameArray) || compare(name + "[0]", nameArray) {
        // 		kind = typeArray[0];
        // 		break;
        // 	}
        // }
        // return new android.graphics4.ConstantLocation(location, kind);

        // FIXME: should handle -1

        location as u32
    }
}

// should rename to Graphics trait
impl Pipeline {
    // pub fn new(target: Option<bool>) -> Self {
    //     gl::enable(gl::BLEND);
    //     gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    //     // TODO:
    //     // gl::viewport(0, 0, width, height);
    //     Self { target: None }
    // }

    pub fn begin(&self, additional_render_targets: Vec<Canvas> /* = None*/) {
        gl::enable(GL_BLEND);
        gl::blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        // match self.target {
        //     Some(ref render_target) => {
        //         // TODO:
        //         // gl::bind_framebuffer(gl::FRAMEBUFFER, 0);
        //         // gl::viewport(0, 0, width, height);
        //     }
        //     None => {
        //         // TODO:
        //         // gl::bind_framebuffer(gl::FRAMEBUFFER, self.render_traget.framebuffer);
        //         // gl::viewport(0, 0, self.render_traget.width, self.render_traget.height);
        //     }
        // }
    }

    pub fn begin_face(&self, face: i32) {}

    pub fn begin_eye(&self, eye: i32) {}

    pub fn end(&self) {}

    pub fn vsynced(&self) -> bool {
        true
    }

    pub fn refresh_rate(&self) -> i32 {
        60
    }

    pub fn clear(&self, color: Option<Color>, depth: Option<f32>, stencil: Option<i32>) {
        let mut clear_mask: u32 = 0;

        if let Some(color) = color {
            clear_mask |= GL_COLOR_BUFFER_BIT;
            gl::clear_color(color.red, color.green, color.blue, color.alpha);
        }

        if let Some(depth) = depth {
            clear_mask |= GL_DEPTH_BUFFER_BIT;
            gl::clear_depthf(depth);
        }

        if let Some(stencil) = stencil {
            clear_mask |= GL_STENCIL_BUFFER_BIT;
            gl::clear_stencil(stencil);
        }
        gl::clear(clear_mask);
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        gl::viewport(x, y, width, height);
    }

    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        gl::enable(GL_SCISSOR_TEST);
        gl::scissor(x, y, width, height);
        // Workaround to transform opengl y coordinate into kha's
        // gl::scissor(x, system.windowheight - y - height, width, height)
    }

    pub fn disable_scissor(&self) {
        gl::disable(GL_SCISSOR_TEST);
    }

    fn decode_format(val: VertexFormat) -> Option<(i32, u32)> {
        match val {
            VertexFormat::Float32 => Some((1, GL_FLOAT)),
            VertexFormat::Float32x2 => Some((2, GL_FLOAT)),
            VertexFormat::Float32x3 => Some((3, GL_FLOAT)),
            VertexFormat::Float32x4 => Some((4, GL_FLOAT)),

            VertexFormat::Uint32 => Some((1, GL_UNSIGNED_INT)),
            VertexFormat::Uint32x2 => Some((2, GL_UNSIGNED_INT)),
            VertexFormat::Uint32x3 => Some((3, GL_UNSIGNED_INT)),
            VertexFormat::Uint32x4 => Some((4, GL_UNSIGNED_INT)),

            VertexFormat::Sint32 => Some((1, GL_INT)),
            VertexFormat::Sint32x2 => Some((2, GL_INT)),
            VertexFormat::Sint32x3 => Some((3, GL_INT)),
            VertexFormat::Sint32x4 => Some((4, GL_INT)),

            VertexFormat::Sint16x2 => Some((2, GL_SHORT)),
            VertexFormat::Sint16x4 => Some((4, GL_SHORT)),

            VertexFormat::Uint16x2 => Some((2, GL_UNSIGNED_SHORT)),
            VertexFormat::Uint16x4 => Some((4, GL_UNSIGNED_SHORT)),

            VertexFormat::Uint8x2 => Some((2, GL_UNSIGNED_BYTE)),
            VertexFormat::Uint8x4 => Some((4, GL_UNSIGNED_BYTE)),

            VertexFormat::Sint8x2 => Some((2, GL_BYTE)),
            VertexFormat::Sint8x4 => Some((4, GL_BYTE)),

            // VertexFormat::Unorm8x2 => 2,
            // VertexFormat::Unorm8x4 => 4,

            // VertexFormat::Snorm8x2 => 2,
            // VertexFormat::Snorm8x4 => 4,

            // VertexFormat::Unorm16x2 => 2,
            // VertexFormat::Unorm16x4 => 4,

            // VertexFormat::Snorm16x2 => 2,
            // VertexFormat::Snorm16x4 => 4,

            // VertexFormat::Float16x2 => 2,
            // VertexFormat::Float16x4 => 4,

            // VertexFormat::Float64 => 1,
            // VertexFormat::Float64x2 => 2,
            // VertexFormat::Float64x3 => 3,
            // VertexFormat::Float64x4 => 4,
            _ => None,
        }
    }

    // pub fn createVertexBuffer(vertexCount: usize, structure: VertexStructure, usage: Usage, canRead: bool /*= false*/) -> u32 {
    // 	// return new VertexBuffer(vertexCount, structure, usage);
    //     unimplemented!()
    // }

    pub fn set_vertex_buffer(&self, buffer: u32) {
        gl::bind_buffer(GL_ARRAY_BUFFER, buffer);

        // for (i in 0...sizes.length) {
        // 	gl::glEnableVertexAttribArray(i);
        // 	gl::glVertexAttribPointer(i, sizes[i], GL_FLOAT, false, myStride, offsets[i]);
        // }

        // stride is
        // size, type, offset
        // seems should get data from vertex layout
        // println!("SET VERTEX BUFFER {}", self.stride);
        self.layout
            .iter()
            .enumerate()
            .for_each(|(index, attribute)| {
                match Self::decode_format(attribute.format) {
                    Some((size, vertex_type)) => {
                        gl::enable_vertex_attrib_array(attribute.shader_location);
                        gl::vertex_attrib_pointer_offset(
                            attribute.shader_location,
                            size,
                            vertex_type,
                            false, // should deal with normalized too when decode format
                            self.stride,
                            attribute.offset as u32,
                        );
                    }
                    None => {
                        println!("Cant expand vertex format");
                    }
                }
            });
    }

    pub fn set_vertex_buffers(&self, vertex_buffers: Vec<VertexBuffer>) {}

    // pub fn createIndexBuffer(indexCount: usize, usage: Usage, canRead: bool /* = false*/) -> u32 {
    // 	// return new IndexBuffer(indexCount, usage);
    //     unimplemented!()
    // }

    pub fn set_index_buffer(&self, buffer: u32) {
        gl::bind_buffer(GL_ELEMENT_ARRAY_BUFFER, buffer);
    }

    // TODO: need some logic here coz there some overlap with Image
    // Who should activate texture?
    // That problem is burn from Image purposes.
    // I mean the question - Is Image should implement Copy trait or available as reference only?

    pub fn set_texture(&self, location: ShaderLocation, texture_unit: u32, texture: Option<u32>) {
        // Set the sampler texture unit...
        gl::uniform1i(location as i32, texture_unit as i32);

        gl::active_texture(GL_TEXTURE0 + texture_unit);
        match texture {
            Some(texture) => {
                gl::bind_texture(GL_TEXTURE_2D, texture);
            }
            None => {
                // TODO:
                gl::bind_texture(GL_TEXTURE_2D, 0)
            }
        }
    }

    pub fn set_texture_depth(&self, unit: TextureUnit, texture: Image) {}

    pub fn set_texture_array(&self, unit: TextureUnit, texture: Image) {}

    pub fn set_video_texture(&self, unit: TextureUnit, texture: Video) {}

    pub fn set_image_texture(&self, unit: TextureUnit, texture: Image) {}

    pub fn set_texture_parameters(
        &self,
        texunit: TextureUnit,
        u_addressing: TextureAddressing,
        v_addressing: TextureAddressing,
        minification_filter: TextureFilter,
        magnification_filter: TextureFilter,
        mipmap_filter: MipMapFilter,
    ) {
        // TODO:
        gl::active_texture(GL_TEXTURE0);

        // switch (uAddressing) {
        // 	case Clamp:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
        // 	case Repeat:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
        // 	case Mirror:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT);
        // }

        // switch (vAddressing) {
        // 	case Clamp:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
        // 	case Repeat:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);
        // 	case Mirror:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT);
        // }

        // switch (minificationFilter) {
        // 	case PointFilter:
        // 		switch (mipmapFilter) {
        // 			case NoMipFilter:
        // 				gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
        // 			case PointMipFilter:
        // 				gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST);
        // 			case LinearMipFilter:
        // 				gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR);
        // 		}
        // 	case LinearFilter, AnisotropicFilter:
        // 		switch (mipmapFilter) {
        // 			case NoMipFilter:
        // 				gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        // 			case PointMipFilter:
        // 				gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST);
        // 			case LinearMipFilter:
        // 				gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR);
        // 		}
        // }

        // switch (magnificationFilter) {
        // 	case PointFilter:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
        // 	case LinearFilter, AnisotropicFilter:
        // 		gl::glTexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        // }
    }

    pub fn set_texture_3d_parameters(
        &self,
        texunit: TextureUnit,
        u_addressing: TextureAddressing,
        v_addressing: TextureAddressing,
        w_addressing: TextureAddressing,
        minification_filter: TextureFilter,
        magnification_filter: TextureFilter,
        mipmap_filter: MipMapFilter,
    ) {
    }

    pub fn set_texture_compare_mode(&self, texunit: TextureUnit, enabled: bool) {}

    pub fn set_cube_map_compare_mode(&self, texunit: TextureUnit, enabled: bool) {}

    pub fn set_cube_map(&self, unit: TextureUnit, cube_map: CubeMap) {}

    pub fn set_cube_map_depth(&self, unit: TextureUnit, cube_map: CubeMap) {}

    pub fn max_bound_textures() -> i32 {
        8
    }
    // pub fn maxTextureSize(&self) -> i32 {
    // unimplemented!()
    // }
    // pub fn supportsNonPow2Textures(&self) -> bool {
    // unimplemented!()
    // }
    pub fn set_stencil_reference_value(&self, value: i32) {}

    pub fn instanced_rendering_available(&self) -> bool {
        false
    }

    pub fn set_depth_mode(write: bool, mode: CompareFunction) {
        match mode {
            CompareFunction::Always => {
                if write {
                    gl::enable(GL_DEPTH_TEST);
                } else {
                    gl::disable(GL_DEPTH_TEST);
                }
                gl::depth_func(GL_ALWAYS);
            }
            CompareFunction::Never => {
                gl::enable(GL_DEPTH_TEST);
                gl::depth_func(GL_NEVER);
            }
            CompareFunction::Equal => {
                gl::enable(GL_DEPTH_TEST);
                gl::depth_func(GL_EQUAL);
            }
            CompareFunction::NotEqual => {
                gl::enable(GL_DEPTH_TEST);
                gl::depth_func(GL_NOTEQUAL);
            }
            CompareFunction::Less => {
                gl::enable(GL_DEPTH_TEST);
                gl::depth_func(GL_LESS);
            }
            CompareFunction::LessEqual => {
                gl::enable(GL_DEPTH_TEST);
                gl::depth_func(GL_LEQUAL);
            }
            CompareFunction::Greater => {
                gl::enable(GL_DEPTH_TEST);
                gl::depth_func(GL_GREATER);
            }
            CompareFunction::GreaterEqual => {
                gl::enable(GL_DEPTH_TEST);
                gl::depth_func(GL_GEQUAL);
            }
        }

        gl::depth_mask(write);
    }

    pub fn set_cull_mode(mode: CullMode) {
        match mode {
            Some(Face::Front) => {
                gl::enable(GL_CULL_FACE);
                gl::cull_face(GL_FRONT);
            }
            Some(Face::Back) => {
                gl::enable(GL_CULL_FACE);
                gl::cull_face(GL_BACK);
            }
            None => {
                gl::disable(GL_CULL_FACE);
            }
        }
    }

    pub fn set_stencil_parameters(
        compare_mode: CompareFunction,
        both_pass: StencilOperation,
        depth_fail: StencilOperation,
        stencil_fail: StencilOperation,
        reference_value: i32,
        read_mask: u32,  /* = 0xff*/
        write_mask: u32, /*= 0xff*/
    ) {
        if compare_mode == CompareFunction::Always
            && both_pass == StencilOperation::Keep
            && depth_fail == StencilOperation::Keep
            && stencil_fail == StencilOperation::Keep
        {
            gl::disable(GL_STENCIL_TEST);
        } else {
            gl::enable(GL_STENCIL_TEST);

            let stencil_func = match compare_mode {
                CompareFunction::Always => GL_ALWAYS,
                CompareFunction::Equal => GL_EQUAL,
                CompareFunction::Greater => GL_GREATER,
                CompareFunction::GreaterEqual => GL_GEQUAL,
                CompareFunction::Less => GL_LESS,
                CompareFunction::LessEqual => GL_LEQUAL,
                CompareFunction::Never => GL_NEVER,
                CompareFunction::NotEqual => GL_NOTEQUAL,
            };

            gl::stencil_mask(write_mask);
            gl::stencil_op(
                Self::convert_stencil_action(stencil_fail),
                Self::convert_stencil_action(depth_fail),
                Self::convert_stencil_action(both_pass),
            );
            gl::stencil_func(stencil_func, reference_value, read_mask);
        }
    }

    #[inline]
    fn convert_stencil_action(action: StencilOperation) -> u32 {
        match action {
            StencilOperation::DecrementClamp => GL_DECR,
            StencilOperation::DecrementWrap => GL_DECR_WRAP,
            StencilOperation::IncrementClamp => GL_INCR,
            StencilOperation::IncrementWrap => GL_INCR_WRAP,
            StencilOperation::Invert => GL_INVERT,
            StencilOperation::Keep => GL_KEEP,
            StencilOperation::Replace => GL_REPLACE,
            StencilOperation::Zero => GL_ZERO,
        }
    }

    fn set_blending_mode(source: BlendFactor, destination: BlendFactor) {
        if source == BlendFactor::One && destination == BlendFactor::Zero {
            gl::disable(GL_BLEND);
        } else {
            gl::enable(GL_BLEND);
            gl::blend_func(Self::blend_func(source), Self::blend_func(destination));
        }
    }

    fn blend_func(op: BlendFactor) -> u32 {
        match op {
            BlendFactor::Zero => GL_ZERO,
            BlendFactor::One => GL_ONE,
            BlendFactor::SrcAlpha => GL_SRC_ALPHA,
            BlendFactor::DstAlpha => GL_DST_ALPHA,
            BlendFactor::OneMinusSrcAlpha => GL_ONE_MINUS_SRC_ALPHA,
            BlendFactor::OneMinusDstAlpha => GL_ONE_MINUS_DST_ALPHA,
            _ => GL_ZERO,
        }
    }

    pub fn make_current(&self) {
        match self.props.read() {
            Ok(props) => {
                // Self::setCullMode(props.cullMode);
                // Self::setDepthMode(props.depthWrite, props.depthMode);
                // let stencilReferenceValue = 0;
                // // match props.stencilReferenceValue {
                // // 	Static(value) => {
                // // 		stencilReferenceValue = value;
                // //     }
                // // 	_ => {}
                // // }
                // Self::setStencilParameters(
                //     props.stencilMode,
                //     props.stencilBothPass,
                //     props.stencilDepthFail,
                //     props.stencilFail,
                //     stencilReferenceValue,
                //     props.stencilReadMask,
                //     props.stencilWriteMask,
                // );
                // Self::setBlendingMode(props.blendSource, props.blendDestination);
                self.set();
            }
            Err(err) => {
                println!("POISONED");
            }
        }
    }

    pub fn set_bool(&self, location: ShaderLocation, value: bool) {
        gl::uniform1i(location as i32, if value { 1 } else { 0 });
    }

    pub fn set_int(&self, location: ShaderLocation, value: i32) {
        gl::uniform1i(location as i32, value);
    }

    pub fn set_int2(&self, location: ShaderLocation, value1: i32, value2: i32) {
        gl::uniform2i(location as i32, value1, value2);
    }

    pub fn set_int3(&self, location: ShaderLocation, value1: i32, value2: i32, value3: i32) {
        gl::uniform3i(location as i32, value1, value2, value3);
    }

    pub fn set_int4(
        &self,
        location: ShaderLocation,
        value1: i32,
        value2: i32,
        value3: i32,
        value4: i32,
    ) {
        gl::uniform4i(location as i32, value1, value2, value3, value4);
    }

    pub fn set_ints(&self, location: ShaderLocation, ints: Vec<i32>) {
        // for (i in 0...values.length) {
        // 	intValuesCache[i] = values[i];
        // }
        // gl::glUniform1iv(cast(location, ShaderLocation).value, values.length, intValuesCache, 0);
    }

    pub fn set_float(&self, location: ShaderLocation, value: f32) {
        gl::uniform1f(location as i32, value);
    }

    pub fn set_float2(&self, location: ShaderLocation, value1: f32, value2: f32) {
        gl::uniform2f(location as i32, value1, value2);
    }

    pub fn set_float3(&self, location: ShaderLocation, value1: f32, value2: f32, value3: f32) {
        gl::uniform3f(location as i32, value1, value2, value3);
    }

    pub fn set_float4(
        &self,
        location: ShaderLocation,
        value1: f32,
        value2: f32,
        value3: f32,
        value4: f32,
    ) {
        gl::uniform4f(location as i32, value1, value2, value3, value4);
    }

    pub fn set_floats(&self, location: ShaderLocation, floats: Vec<f32>) {
        // for (i in 0...values.length) {
        // 	valuesCache[i] = values[i];
        // }
        // gl::glUniform1fv(cast(location, ShaderLocation).value, values.length, valuesCache, 0);
    }

    pub fn set_vector2(&self, location: ShaderLocation, value: cgmath::Vector2<f32>) {
        gl::uniform2f(location as i32, value.x, value.y);
    }

    pub fn set_vector3(&self, location: ShaderLocation, value: cgmath::Vector3<f32>) {
        gl::uniform3f(location as i32, value.x, value.y, value.z);
    }

    pub fn set_vector4(&self, location: ShaderLocation, value: cgmath::Vector4<f32>) {
        gl::uniform4f(location as i32, value.x, value.y, value.z, value.w);
    }

    pub fn set_matrix(&self, location: ShaderLocation, matrix: cgmath::Matrix4<f32>) {
        // matrixCache[0] = matrix._00;
        // matrixCache[1] = matrix._01;
        // matrixCache[2] = matrix._02;
        // matrixCache[3] = matrix._03;
        // matrixCache[4] = matrix._10;
        // matrixCache[5] = matrix._11;
        // matrixCache[6] = matrix._12;
        // matrixCache[7] = matrix._13;
        // matrixCache[8] = matrix._20;
        // matrixCache[9] = matrix._21;
        // matrixCache[10] = matrix._22;
        // matrixCache[11] = matrix._23;
        // matrixCache[12] = matrix._30;
        // matrixCache[13] = matrix._31;
        // matrixCache[14] = matrix._32;
        // matrixCache[15] = matrix._33;

        let value: &[f32; 16] = matrix.as_ref();
        // AsRef::<[f32; 16]>::as_ref(&matrix)
        gl::uniform_matrix4fv(location as i32, false, value);
    }
    pub fn set_matrix3(&self, location: ShaderLocation, value: cgmath::Matrix3<f32>) {
        // TODO:
        // matrix3Cache[0] = matrix._00;
        // matrix3Cache[1] = matrix._01;
        // matrix3Cache[2] = matrix._02;
        // matrix3Cache[3] = matrix._10;
        // matrix3Cache[4] = matrix._11;
        // matrix3Cache[5] = matrix._12;
        // matrix3Cache[6] = matrix._20;
        // matrix3Cache[7] = matrix._21;
        // matrix3Cache[8] = matrix._22;
        // gl::uniform_matrix3fv(cast(location, ShaderLocation).value, false, matrix3Cache);
    }

    pub fn draw_indexed_vertices(
        &self,
        start: usize,         /*  = 0*/
        count: Option<usize>, /* = -1*/
    ) {
        // // ES20 Android
        // let count = match count {
        //     Some(count) => count,
        //     None => indexBuffer.count(),
        // };

        // gl::draw_elements(
        //     gl::TRIANGLES,
        //     count,
        //     gl::UNSIGNED_SHORT,
        //     indexBuffer.data,
        // );

        // // ES30 WebGL2
        let count = match count {
            Some(count) => count,
            None => {
                // indicesCount
                0
            }
        };
        // let size = if type_ = gl::UNSIGNED_SHORT { 2 } else { 4 };
        let offset = start as u32 * 4;
        // println!("draw indeced {} {}", count, offset);

        gl::draw_elements_offset(GL_TRIANGLES, count as i32, GL_UNSIGNED_INT, offset);
    }

    pub fn draw_indexed_vertices_instanced(
        &self,
        instance_count: usize,
        start: usize, /* = 0*/
        count: usize, /* = -1*/
    ) {
        // V3
        // gl::draw_elements_instanced(mode, count, type_, indices, primcount)
    }

    pub fn flush(&self) {}
}

pub struct PipelineCache {
    pipelines: Mutex<IntMap<Arc<Pipeline>>>,
}

impl PipelineCache {
    fn new() -> Self {
        Self {
            pipelines: Mutex::new(IntMap::new()),
        }
    }

    /// Retrieves the singleton instance of `Settings`
    ///
    /// # Returns
    ///
    /// the instance of `Settings`. The
    ///  returned object is owned by internals and it should not be unreferenced
    ///  directly
    pub fn global() -> &'static Self {
        static CONTEXT_INSTANCE: OnceCell<PipelineCache> = OnceCell::new();
        CONTEXT_INSTANCE.get_or_init(Self::new)
    }

    pub fn get(&self, id: u64) -> Option<Arc<Pipeline>> {
        match self.pipelines.lock() {
            Ok(pipelines) => match pipelines.get(id) {
                Some(pipeline) => Some(pipeline.clone()),
                None => None,
            },
            Err(_) => panic!("PipelineCache poisoned"),
        }
    }

    pub fn set(&self, id: u64, pipeline: Pipeline) {
        match self.pipelines.lock() {
            Ok(mut pipelines) => {
                pipelines.insert(id, Arc::new(pipeline));
            }
            Err(_) => panic!("PipelineCache poisoned"),
        }
    }

    pub fn exists(&self, id: u64) -> bool {
        match self.pipelines.lock() {
            Ok(pipelines) => pipelines.contains_key(id),
            Err(_) => panic!("PipelineCache poisoned"),
        }
    }
}
