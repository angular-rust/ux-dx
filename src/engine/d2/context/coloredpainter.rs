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

pub(super) struct ColoredShaderPainter {
    projection_matrix: cgmath::Matrix4<f32>,
    projection_location: ShaderLocation,

    pub buffer_size: usize, // = 1000;
    pub quad_count: usize,
    pub rect_vertex_buffer: Vec<f32>,
    index_buffer: Vec<u32>,

    pub triangle_buffer_size: usize, // = 1000;
    pub triangle_count: usize,
    pub triangle_vertex_buffer: Vec<f32>,
    triangle_index_buffer: Vec<u32>,

    vbo: u32,
    ebo: u32,
    trivbo: u32,
    triebo: u32,
}

impl ColoredShaderPainter {
    const PIPELINE: u64 = 0x0;
    const VERTEX_SIZE: usize = 7;
    const VERTEX_SHADER: &'static str = r"
	#version 100

	attribute vec3 vertexPosition;
	attribute vec4 vertexColor;
	
	uniform mat4 projectionMatrix;

	varying vec4 fragmentColor;
	
	void main() {
		gl_Position = projectionMatrix * vec4(vertexPosition, 1.0);
		fragmentColor = vertexColor;
	}";

    const FRAGMENT_SHADER: &'static str = r"
	#version 100
    precision mediump float;

	in mediump vec4 fragmentColor;
	
	void main() {
		gl_FragColor = fragmentColor;
	}";

    pub fn new() -> Self {
        let mut instance = Self {
            buffer_size: 1000,
            quad_count: 0,
            rect_vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),

            triangle_buffer_size: 1000,
            triangle_count: 0,
            triangle_vertex_buffer: Vec::new(),
            triangle_index_buffer: Vec::new(),
            projection_matrix: cgmath::Matrix4::identity(),
            projection_location: 0,
            vbo: 0,
            ebo: 0,
            trivbo: 0,
            triebo: 0,
        };

        instance.init_shaders();
        instance.init_buffers();
        instance
    }

    // fn get_pipeline(&self) -> PipelineCache {
    // 	self.myPipeline
    // }

    // fn set_pipeline(&self, pipe: Option<PipelineCache>) {
    // 	self.myPipeline = match pipe {
    // 		Some(pipe) => pipe,
    // 		None => self.standardColorPipeline.clone(),
    // 	}
    // }

    pub fn set_projection(&mut self, projection_matrix: cgmath::Matrix4<f32>) {
        self.projection_matrix = projection_matrix;
    }

    // called only from constructor
    fn init_shaders(&mut self) {
        let cache = PipelineCache::global();
        match cache.get(Self::PIPELINE) {
            Some(pipeline) => {
                self.projection_location = pipeline.uniform_location("projectionMatrix");
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
                        // vertexColor
                        format: VertexFormat::Float32x4,
                        offset: 4 * 3, // VertexFormap::size() from prev's format, aka summ
                        shader_location: 1,
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
                        cache.set(Self::PIPELINE, pipeline);
                    }
                    Err(err) => {
                        panic!("Build pipeline error");
                    }
                }
            }
        }
    }

    // called only from constructor
    fn init_buffers(&mut self) {
        let vbos = gl::gen_buffers(2).as_slice();
        let (vbo, ebo, trivbo, triebo) = {
            let v = gl::gen_buffers(4);
            (v[0], v[1], v[2], v[3])
        };

        self.vbo = vbo;
        self.ebo = ebo;

        self.trivbo = trivbo;
        self.triebo = triebo;

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

        let triangle_index_buffer_len = self.triangle_buffer_size * 3;
        self.triangle_vertex_buffer = Vec::with_capacity(triangle_index_buffer_len);

        self.triangle_index_buffer = Vec::with_capacity(triangle_index_buffer_len);
        self.triangle_index_buffer
            .resize(triangle_index_buffer_len, 0);
        for idx in 0..self.buffer_size {
            self.triangle_index_buffer[idx * 3 + 0] = idx as u32 * 3 + 0;
            self.triangle_index_buffer[idx * 3 + 1] = idx as u32 * 3 + 1;
            self.triangle_index_buffer[idx * 3 + 2] = idx as u32 * 3 + 2;
        }
    }

    const Z: f32 = 0.0; // -5.0;

    pub fn set_rect_vertices(
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

        let new_len = base_index + 24;
        if self.rect_vertex_buffer.len() < new_len {
            self.rect_vertex_buffer.resize(new_len, 0.0);
        }

        self.rect_vertex_buffer[base_index + 0] = bottomleftx;
        self.rect_vertex_buffer[base_index + 1] = bottomlefty;
        self.rect_vertex_buffer[base_index + 2] = Self::Z;

        self.rect_vertex_buffer[base_index + 7] = topleftx;
        self.rect_vertex_buffer[base_index + 8] = toplefty;
        self.rect_vertex_buffer[base_index + 9] = Self::Z;

        self.rect_vertex_buffer[base_index + 14] = toprightx;
        self.rect_vertex_buffer[base_index + 15] = toprighty;
        self.rect_vertex_buffer[base_index + 16] = Self::Z;

        self.rect_vertex_buffer[base_index + 21] = bottomrightx;
        self.rect_vertex_buffer[base_index + 22] = bottomrighty;
        self.rect_vertex_buffer[base_index + 23] = Self::Z;
    }

    pub fn set_rect_colors(&mut self, opacity: f32, color: Color) {
        // println!("SET RECT COLORS");
        let base_index: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = base_index + 28;
        if self.rect_vertex_buffer.len() < new_len {
            self.rect_vertex_buffer.resize(new_len, 0.0);
        }

        let a: f32 = opacity * color.alpha;
        let r: f32 = a * color.red;
        let g: f32 = a * color.green;
        let b: f32 = a * color.blue;

        self.rect_vertex_buffer[base_index + 3] = r;
        self.rect_vertex_buffer[base_index + 4] = g;
        self.rect_vertex_buffer[base_index + 5] = b;
        self.rect_vertex_buffer[base_index + 6] = a;

        self.rect_vertex_buffer[base_index + 10] = r;
        self.rect_vertex_buffer[base_index + 11] = g;
        self.rect_vertex_buffer[base_index + 12] = b;
        self.rect_vertex_buffer[base_index + 13] = a;

        self.rect_vertex_buffer[base_index + 17] = r;
        self.rect_vertex_buffer[base_index + 18] = g;
        self.rect_vertex_buffer[base_index + 19] = b;
        self.rect_vertex_buffer[base_index + 20] = a;

        self.rect_vertex_buffer[base_index + 24] = r;
        self.rect_vertex_buffer[base_index + 25] = g;
        self.rect_vertex_buffer[base_index + 26] = b;
        self.rect_vertex_buffer[base_index + 27] = a;
    }

    pub(super) fn set_tri_vertices(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) {
        let base_index: usize = self.triangle_count * Self::VERTEX_SIZE * 3;

        let new_len = base_index + 17;
        if self.triangle_vertex_buffer.len() < new_len {
            self.triangle_vertex_buffer.resize(new_len, 0.0);
        }

        self.triangle_vertex_buffer[base_index + 0] = x1;
        self.triangle_vertex_buffer[base_index + 1] = y1;
        self.triangle_vertex_buffer[base_index + 2] = Self::Z;

        self.triangle_vertex_buffer[base_index + 7] = x2;
        self.triangle_vertex_buffer[base_index + 8] = y2;
        self.triangle_vertex_buffer[base_index + 9] = Self::Z;

        self.triangle_vertex_buffer[base_index + 14] = x3;
        self.triangle_vertex_buffer[base_index + 15] = y3;
        self.triangle_vertex_buffer[base_index + 16] = Self::Z;
    }

    pub(super) fn set_tri_colors(&mut self, opacity: f32, color: Color) {
        let base_index: usize = self.triangle_count * Self::VERTEX_SIZE * 3;

        let new_len = base_index + 21;
        if self.triangle_vertex_buffer.len() < new_len {
            self.triangle_vertex_buffer.resize(new_len, 0.0);
        }

        let alpha: f32 = opacity * color.alpha;
        let red: f32 = alpha * color.red;
        let green: f32 = alpha * color.green;
        let blue: f32 = alpha * color.blue;

        self.triangle_vertex_buffer[base_index + 3] = red;
        self.triangle_vertex_buffer[base_index + 4] = green;
        self.triangle_vertex_buffer[base_index + 5] = blue;
        self.triangle_vertex_buffer[base_index + 6] = alpha;

        self.triangle_vertex_buffer[base_index + 10] = red;
        self.triangle_vertex_buffer[base_index + 11] = green;
        self.triangle_vertex_buffer[base_index + 12] = blue;
        self.triangle_vertex_buffer[base_index + 13] = alpha;

        self.triangle_vertex_buffer[base_index + 17] = red;
        self.triangle_vertex_buffer[base_index + 18] = green;
        self.triangle_vertex_buffer[base_index + 19] = blue;
        self.triangle_vertex_buffer[base_index + 20] = alpha;
    }

    pub(super) fn draw_buffer(&mut self, tris_done: bool) {
        if self.quad_count == 0 {
            return;
        }

        if !tris_done {
            self.end_tris(true);
        }

        // let pipeline = self.myPipeline.get(None, Depth24Stencil8);
        match PipelineCache::global().get(Self::PIPELINE) {
            Some(ref pipeline) => {
                // println!("== GOT PIPELINE WITH: {} {} {}", self.bufferIndex, self.rectVertexBuffer.len(), self.indexBuffer.len());
                pipeline.make_current();
                glchk!("set pipeline");

                // set vertex buffer
                gl::bind_buffer(GL_ARRAY_BUFFER, self.vbo);
                glchk!("bind vertex buffer");

                gl::buffer_data(
                    GL_ARRAY_BUFFER,
                    self.rect_vertex_buffer.as_slice(),
                    GL_DYNAMIC_DRAW,
                );
                glchk!("set vertex data");

                // set index buffer
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

                pipeline.set_matrix(self.projection_location, self.projection_matrix);
                glchk!("set projection matrix");

                pipeline.set_index_buffer(self.ebo);
                glchk!("set ebo");

                pipeline.draw_indexed_vertices(0, Some(self.quad_count * 2 * 3));
                glchk!("draw indexed");

                self.quad_count = 0;

                // gl::bind_buffer(gl::ARRAY_BUFFER, 0);
                // glchk!("back to zero VBO");
                // gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, 0);
                // glchk!("back to zero EBO");
            }
            None => {
                println!("THERE NO PIPELINE IN CACHE FOR COLOR PAINTER");
            }
        }
    }

    pub(super) fn draw_tri_buffer(&mut self, rects_done: bool) {
        if !rects_done {
            self.end_rects(true);
        }

        // let pipeline = self.myPipeline.get(None, Depth24Stencil8);
        match PipelineCache::global().get(Self::PIPELINE) {
            Some(ref pipeline) => {
                glchk!("before set pipeline");
                pipeline.make_current();
                glchk!("set pipeline");

                // SOME LOW LEVEL LOGIC
                gl::bind_buffer(GL_ARRAY_BUFFER, self.trivbo);
                glchk!("bind vertex buffer");

                gl::buffer_data(
                    GL_ARRAY_BUFFER,
                    self.triangle_vertex_buffer.as_slice(),
                    GL_DYNAMIC_DRAW,
                );

                glchk!("set vertex data");

                // here correct

                gl::bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.triebo);
                glchk!("bind indices buffer");

                gl::buffer_data(
                    GL_ELEMENT_ARRAY_BUFFER,
                    self.triangle_index_buffer.as_slice(),
                    GL_STATIC_DRAW,
                );
                glchk!("set indices data");

                // then reenable again VBO
                pipeline.set_vertex_buffer(self.trivbo);
                glchk!("set vbo");

                pipeline.set_matrix(self.projection_location, self.projection_matrix);
                glchk!("set projection matrix");

                pipeline.set_index_buffer(self.triebo);
                glchk!("set ebo");

                pipeline.draw_indexed_vertices(0, Some(self.triangle_count * 3));
                glchk!("draw indexed");

                self.triangle_count = 0;

                gl::bind_buffer(GL_ARRAY_BUFFER, 0);
                glchk!("back to zero VBO");
                gl::bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0);
                glchk!("back to zero EBO");
            }
            None => {
                println!("THERE NO PIPELINE IN CACHE FOR COLOR PAINTER");
            }
        }
    }

    pub fn fill_rect(
        &mut self,
        opacity: f32,
        color: Color,
        bottomleftx: f32,
        bottomlefty: f32,
        topleftx: f32,
        toplefty: f32,
        toprightx: f32,
        toprighty: f32,
        bottomrightx: f32,
        bottomrighty: f32,
    ) {
        // println!("FILL RECT");
        if self.triangle_count > 0 {
            self.draw_tri_buffer(true); // Flush other buffer for right render order
        }

        if self.quad_count + 1 >= self.buffer_size {
            self.draw_buffer(false);
        }

        self.set_rect_colors(opacity, color);
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
    }

    pub fn fill_triangle(
        &mut self,
        opacity: f32,
        color: Color,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) {
        if self.quad_count > 0 {
            self.draw_buffer(true); // Flush other buffer for right render order
        }

        if self.triangle_count + 1 >= self.triangle_buffer_size {
            self.draw_tri_buffer(false);
        }

        self.set_tri_colors(opacity, color);
        self.set_tri_vertices(x1, y1, x2, y2, x3, y3);
        self.triangle_count += 1;
    }

    #[inline]
    pub fn end_tris(&mut self, rects_done: bool) {
        if self.triangle_count > 0 {
            self.draw_tri_buffer(rects_done);
        }
    }

    #[inline]
    pub fn end_rects(&mut self, tris_done: bool) {
        if self.quad_count > 0 {
            self.draw_buffer(tris_done);
        }
    }

    #[inline]
    pub fn end(&mut self) {
        self.end_tris(false);
        self.end_rects(false);
    }
}
