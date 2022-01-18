#![allow(unused_macros)]
use cgmath::prelude::*;
use std::rc::Rc;

use super::*;
use crate::{
    glchk,
    platform::{
        core::{BlendFactor, ShaderLocation, VertexFormat},
        gles::{core20::gl, enums::*},
    },
};

pub(super) struct ImageShaderPainter {
    projection_matrix: cgmath::Matrix4<f32>,
    projection_location: ShaderLocation,
    texture_location: ShaderLocation,

    buffer_size: usize, // = 1500;
    quad_count: usize,
    rect_vertex_buffer: Vec<f32>,
    index_buffer: Vec<u32>,
    last_texture: Option<u32>, // cg texture id

    bilinear: bool,         // = false;
    bilinear_mipmaps: bool, // = false;

    vbo: u32,
    ebo: u32,
}

impl ImageShaderPainter {
    const PIPELINE: u64 = 0x1;
    const VERTEX_SIZE: usize = 9;
    const VERTEX_SHADER: &'static str = r"
	#version 100

    attribute vec3 vertexPosition;
    attribute vec2 texPosition;
    attribute vec4 vertexColor;

    uniform mat4 projectionMatrix;

    varying vec2 texCoord;
    varying vec4 color;

    void main() {
        gl_Position = projectionMatrix * vec4(vertexPosition, 1.0);
        texCoord = texPosition;
        color = vertexColor;
    }
    ";

    const FRAGMENT_SHADER: &'static str = r"
	#version 100
    precision mediump float;

    uniform sampler2D tex;

    in mediump vec2 texCoord;
    in mediump vec4 color;

    void main() {
        vec4 texcolor = texture2D(tex, texCoord) * color;
        texcolor.rgb *= color.a;
        gl_FragColor = texcolor;
    }";

    pub fn new() -> Self {
        let mut instance = Self {
            buffer_size: 1500,
            quad_count: 0,
            rect_vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            projection_matrix: cgmath::Matrix4::identity(),
            projection_location: 0,
            last_texture: None,
            texture_location: 0,
            bilinear: false,
            bilinear_mipmaps: false,
            vbo: 0,
            ebo: 0,
        };

        instance.init_shaders();
        instance.init_buffers();
        instance
    }

    // fn get_pipeline(&self) -> PipelineCache {
    //     self.myPipeline
    // }

    // fn set_pipeline(&self, pipe: Option<PipelineCache>) {
    //     let pipe = match pipe {
    //         Some(pipe) => pipe,
    //         None => self.standardImagePipeline,
    //     };
    //     self.myPipeline = pipe
    // }

    pub fn set_projection(&mut self, projection_matrix: cgmath::Matrix4<f32>) {
        self.projection_matrix = projection_matrix;
    }

    fn init_shaders(&mut self) {
        let cache = PipelineCache::global();
        match cache.get(Self::PIPELINE) {
            Some(pipeline) => {
                self.projection_location = pipeline.uniform_location("projectionMatrix");
                self.texture_location = pipeline.uniform_location("tex");
            }
            None => {
                // Describe pipeline layout
                let layout = vec![
                    VertexAttribute {
                        // vertexPosition
                        format: VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    },
                    VertexAttribute {
                        // texPosition
                        format: VertexFormat::Float32x2,
                        offset: 4 * 3, // summ of previous formats
                        shader_location: 1,
                    },
                    VertexAttribute {
                        // vertexColor
                        format: VertexFormat::Float32x4,
                        offset: 4 * (3 + 2), // summ of previous formats
                        shader_location: 2,
                    },
                ];

                let pipeline = Pipeline::construct()
                    .add_shader(ShaderSource::new(
                        ShaderType::Fragment,
                        Self::FRAGMENT_SHADER.as_bytes(),
                    ))
                    .add_shader(ShaderSource::new(
                        ShaderType::Vertex,
                        Self::VERTEX_SHADER.as_bytes(),
                    ))
                    .set_blend_source(BlendFactor::One)
                    .set_blend_destination(BlendFactor::OneMinusSrcAlpha)
                    .set_alpha_blend_source(BlendFactor::One)
                    .set_alpha_blend_destination(BlendFactor::OneMinusSrcAlpha)
                    .set_input_layout(layout)
                    .build();
                match pipeline {
                    Ok(pipeline) => {
                        self.projection_location = pipeline.uniform_location("projectionMatrix");
                        self.texture_location = pipeline.uniform_location("tex");
                        cache.set(Self::PIPELINE, pipeline);
                    }
                    Err(err) => {
                        panic!("Build pipeline error");
                    }
                }
            }
        }
    }

    fn init_buffers(&mut self) {
        let vbos = gl::gen_buffers(2).as_slice();
        let (vbo, ebo) = {
            let v = gl::gen_buffers(2);
            (v[0], v[1])
        };

        self.vbo = vbo;
        self.ebo = ebo;

        self.rect_vertex_buffer = Vec::with_capacity(self.buffer_size * 4);

        let index_buffer_len = self.buffer_size * 3 * 2;
        self.index_buffer = Vec::with_capacity(index_buffer_len);
        self.index_buffer.resize(index_buffer_len, 0);

        for idx in 0..self.buffer_size {
            self.index_buffer[idx * 3 * 2 + 0] = idx as u32 * 4 + 0;
            self.index_buffer[idx * 3 * 2 + 1] = idx as u32 * 4 + 1;
            self.index_buffer[idx * 3 * 2 + 2] = idx as u32 * 4 + 2;
            self.index_buffer[idx * 3 * 2 + 3] = idx as u32 * 4 + 0;
            self.index_buffer[idx * 3 * 2 + 4] = idx as u32 * 4 + 2;
            self.index_buffer[idx * 3 * 2 + 5] = idx as u32 * 4 + 3;
        }
    }

    const Z: f32 = 0.0; // -5.0;

    #[inline]
    fn set_rect_vertices(
        &mut self,
        bottomleftx: f32,
        bottomlefty: f32,
        topleftx: f32,
        toplefty: f32,
        toprightx: f32,
        toprighty: f32,
        bottomrightx: f32,
        bottomrighty: f32,
    ) {
        let base_index: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = base_index + 30;
        if self.rect_vertex_buffer.len() < new_len {
            self.rect_vertex_buffer.resize(new_len, 0.0);
        }
        self.rect_vertex_buffer[base_index + 0] = bottomleftx;
        self.rect_vertex_buffer[base_index + 1] = bottomlefty;
        self.rect_vertex_buffer[base_index + 2] = Self::Z;

        self.rect_vertex_buffer[base_index + 9] = topleftx;
        self.rect_vertex_buffer[base_index + 10] = toplefty;
        self.rect_vertex_buffer[base_index + 11] = Self::Z;

        self.rect_vertex_buffer[base_index + 18] = toprightx;
        self.rect_vertex_buffer[base_index + 19] = toprighty;
        self.rect_vertex_buffer[base_index + 20] = Self::Z;

        self.rect_vertex_buffer[base_index + 27] = bottomrightx;
        self.rect_vertex_buffer[base_index + 28] = bottomrighty;
        self.rect_vertex_buffer[base_index + 29] = Self::Z;
    }

    #[inline]
    fn set_rect_tex_coords(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        let base_index: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = base_index + 32;
        if self.rect_vertex_buffer.len() < new_len {
            self.rect_vertex_buffer.resize(new_len, 0.0);
        }

        self.rect_vertex_buffer[base_index + 3] = left;
        self.rect_vertex_buffer[base_index + 4] = bottom;

        self.rect_vertex_buffer[base_index + 12] = left;
        self.rect_vertex_buffer[base_index + 13] = top;

        self.rect_vertex_buffer[base_index + 21] = right;
        self.rect_vertex_buffer[base_index + 22] = top;

        self.rect_vertex_buffer[base_index + 30] = right;
        self.rect_vertex_buffer[base_index + 31] = bottom;
    }

    #[inline]
    fn set_rect_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        let base_index: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = base_index + 36;
        if self.rect_vertex_buffer.len() < new_len {
            self.rect_vertex_buffer.resize(new_len, 0.0);
        }

        self.rect_vertex_buffer[base_index + 5] = red;
        self.rect_vertex_buffer[base_index + 6] = green;
        self.rect_vertex_buffer[base_index + 7] = blue;
        self.rect_vertex_buffer[base_index + 8] = alpha;

        self.rect_vertex_buffer[base_index + 14] = red;
        self.rect_vertex_buffer[base_index + 15] = green;
        self.rect_vertex_buffer[base_index + 16] = blue;
        self.rect_vertex_buffer[base_index + 17] = alpha;

        self.rect_vertex_buffer[base_index + 23] = red;
        self.rect_vertex_buffer[base_index + 24] = green;
        self.rect_vertex_buffer[base_index + 25] = blue;
        self.rect_vertex_buffer[base_index + 26] = alpha;

        self.rect_vertex_buffer[base_index + 32] = red;
        self.rect_vertex_buffer[base_index + 33] = green;
        self.rect_vertex_buffer[base_index + 34] = blue;
        self.rect_vertex_buffer[base_index + 35] = alpha;
    }

    fn draw_buffer(&mut self) {
        if self.quad_count == 0 {
            return;
        }

        // let pipeline = self.myPipeline.get(None, Depth24Stencil8);
        match PipelineCache::global().get(Self::PIPELINE) {
            Some(pipeline) => {
                pipeline.make_current();
                glchk!("set pipeline");

                // SOME LOW LEVEL LOGIC
                gl::bind_buffer(GL_ARRAY_BUFFER, self.vbo);
                glchk!("bind vertex buffer");

                gl::buffer_data(
                    GL_ARRAY_BUFFER,
                    self.rect_vertex_buffer.as_slice(),
                    GL_DYNAMIC_DRAW,
                );
                glchk!("set vertex data");

                gl::bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.ebo);
                glchk!("bind indices buffer");

                gl::buffer_data(
                    GL_ELEMENT_ARRAY_BUFFER,
                    self.index_buffer.as_slice(),
                    GL_STATIC_DRAW,
                );
                glchk!("set indices data");

                // then reenable again VBO
                pipeline.set_vertex_buffer(self.vbo);
                glchk!("set vbo");

                pipeline.set_texture(self.texture_location, 0, self.last_texture);

                // pipeline.setTextureParameters(
                //     pipeline.textureLocation,
                //     TextureAddressing.Clamp,
                //     TextureAddressing.Clamp,
                //     if self.bilinear {
                //         TextureFilter.LinearFilter
                //     } else {
                //         TextureFilter.PointFilter
                //     },
                //     if self.bilinear {
                //         TextureFilter.LinearFilter
                //     } else {
                //         TextureFilter.PointFilter
                //     },
                //     if self.bilinearMipmaps {
                //         MipMapFilter.LinearMipFilter
                //     } else {
                //         MipMapFilter.NoMipFilter
                //     },
                // );

                pipeline.set_matrix(self.projection_location, self.projection_matrix);
                glchk!("set projection matrix");

                pipeline.set_index_buffer(self.ebo);
                glchk!("set ebo");

                pipeline.draw_indexed_vertices(0, Some(self.quad_count * 2 * 3));

                // pipeline.setTexture(pipeline.textureLocation, None);

                self.quad_count = 0;
            }
            None => {
                println!("THERE NO PIPELINE IN CACHE FOR IMAGE PAINTER");
            }
        }
    }

    pub fn set_bilinear_filter(&mut self, bilinear: bool) {
        self.draw_buffer();
        self.last_texture = None;
        self.bilinear = bilinear;
    }

    pub fn set_bilinear_mipmap_filter(&mut self, bilinear: bool) {
        self.draw_buffer();
        self.last_texture = None;
        self.bilinear_mipmaps = bilinear;
    }

    // Opacity seems is some overcode
    // but i think it need to make smooth transition
    // coz it have more precission
    #[inline]
    pub fn draw_image(
        &mut self,
        img: &Image,
        bottomleftx: f32,
        bottomlefty: f32,
        topleftx: f32,
        toplefty: f32,
        toprightx: f32,
        toprighty: f32,
        bottomrightx: f32,
        bottomrighty: f32,
        opacity: f32,
        tint: Color,
    ) {
        if self.quad_count + 1 >= self.buffer_size {
            // check the buffer overflow first
            self.draw_buffer();
        } else if let Some(last_texture) = self.last_texture {
            // then check the texture changed
            if let Some(img_texure) = img.tex {
                if last_texture != img_texure {
                    self.draw_buffer();
                }
            } else {
                println!("IMAGE WITHOUT TEXTURE ID");
            }
        }

        self.set_rect_color(tint.red, tint.green, tint.blue, tint.alpha * opacity);

        self.set_rect_tex_coords(
            0.0,
            0.0,
            img.width as f32 / img.real_width as f32,
            img.height as f32 / img.real_height as f32,
        );

        self.set_rect_vertices(
            bottomleftx,
            bottomlefty,
            topleftx,
            toplefty,
            toprightx,
            toprighty,
            bottomrightx,
            bottomrighty,
        );

        self.quad_count += 1;
        self.last_texture = img.tex; // should detect the texture is correct
    }

    #[inline]
    pub fn draw_image2(
        &mut self,
        img: &Image,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
        bottomleftx: f32,
        bottomlefty: f32,
        topleftx: f32,
        toplefty: f32,
        toprightx: f32,
        toprighty: f32,
        bottomrightx: f32,
        bottomrighty: f32,
        opacity: f32,
        tint: Color,
    ) {
        // println!("DRAW IMAGE 2");

        if self.quad_count + 1 >= self.buffer_size {
            // check the buffer overflow first
            self.draw_buffer();
        } else if let Some(last_texture) = self.last_texture {
            // then check the texture changed
            if let Some(img_texure) = img.tex {
                if last_texture != img_texure {
                    self.draw_buffer();
                }
            } else {
                println!("IMAGE WITHOUT TEXTURE ID");
            }
        }

        self.set_rect_tex_coords(
            sx / img.real_width as f32,
            sy / img.real_height as f32,
            (sx + sw) / img.real_width as f32,
            (sy + sh) / img.real_height as f32,
        );

        self.set_rect_color(tint.red, tint.green, tint.blue, tint.alpha * opacity);

        self.set_rect_vertices(
            bottomleftx,
            bottomlefty,
            topleftx,
            toplefty,
            toprightx,
            toprighty,
            bottomrightx,
            bottomrighty,
        );

        self.quad_count += 1;
        self.last_texture = img.tex;
    }

    #[inline]
    pub fn draw_image_scale(
        &mut self,
        img: &Image,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        opacity: f32,
        tint: Color,
    ) {
        // println!("DRAW IMAGE SCALED");

        if self.quad_count + 1 >= self.buffer_size {
            // check the buffer overflow first
            self.draw_buffer();
        } else if let Some(last_texture) = self.last_texture {
            // then check the texture changed
            if let Some(img_texure) = img.tex {
                if last_texture != img_texure {
                    self.draw_buffer();
                }
            } else {
                println!("IMAGE WITHOUT TEXTURE ID");
            }
        }

        self.set_rect_tex_coords(
            sx / img.real_width as f32,
            sy / img.real_height as f32,
            (sx + sw) / img.real_width as f32,
            (sy + sh) / img.real_height as f32,
        );

        self.set_rect_color(tint.red, tint.green, tint.blue, opacity);

        self.set_rect_vertices(left, bottom, left, top, right, top, right, bottom);

        self.quad_count += 1;
        self.last_texture = img.tex;
    }

    pub fn end(&mut self) {
        if self.quad_count > 0 {
            self.draw_buffer();
        }
        self.last_texture = None;
    }
}
