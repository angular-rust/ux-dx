use cgmath::prelude::*;
use std::rc::Rc;

use super::*;
use crate::{
    glchk,
    gles::{core20::gl, enums::*},
    BlendFactor, ShaderLocation, VertexFormat,
};

pub struct ColoredShaderPainter {
    projectionMatrix: cgmath::Matrix4<f32>,
    projectionLocation: ShaderLocation,

    bufferSize: usize, // = 1000;
    quad_count: usize,
    rectVertexBuffer: Vec<f32>,
    indexBuffer: Vec<u32>,

    triangleBufferSize: usize, // = 1000;
    triangle_count: usize,
    triangleVertexBuffer: Vec<f32>,
    triangleIndexBuffer: Vec<u32>,

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
            bufferSize: 1000,
            quad_count: 0,
            rectVertexBuffer: Vec::new(),
            indexBuffer: Vec::new(),

            triangleBufferSize: 1000,
            triangle_count: 0,
            triangleVertexBuffer: Vec::new(),
            triangleIndexBuffer: Vec::new(),
            projectionMatrix: cgmath::Matrix4::identity(),
            projectionLocation: 0,
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

    pub fn setProjection(&mut self, projectionMatrix: cgmath::Matrix4<f32>) {
        self.projectionMatrix = projectionMatrix;
    }

    // called only from constructor
    fn init_shaders(&mut self) {
        let cache = PipelineCache::global();
        match cache.get(Self::PIPELINE) {
            Some(pipeline) => {
                self.projectionLocation = pipeline.get_uniform_location("projectionMatrix");
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
                        self.projectionLocation = pipeline.get_uniform_location("projectionMatrix");
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

        self.rectVertexBuffer = Vec::with_capacity(self.bufferSize * 4);

        let index_buffer_len = self.bufferSize * 3 * 2;
        self.indexBuffer = Vec::with_capacity(index_buffer_len);
        self.indexBuffer.resize(index_buffer_len, 0);

        for idx in 0..self.bufferSize {
            self.indexBuffer[idx * 3 * 2 + 0] = idx as u32 * 4 + 0;
            self.indexBuffer[idx * 3 * 2 + 1] = idx as u32 * 4 + 1;
            self.indexBuffer[idx * 3 * 2 + 2] = idx as u32 * 4 + 2;
            self.indexBuffer[idx * 3 * 2 + 3] = idx as u32 * 4 + 0;
            self.indexBuffer[idx * 3 * 2 + 4] = idx as u32 * 4 + 2;
            self.indexBuffer[idx * 3 * 2 + 5] = idx as u32 * 4 + 3;
        }

        let triangle_index_buffer_len = self.triangleBufferSize * 3;
        self.triangleVertexBuffer = Vec::with_capacity(triangle_index_buffer_len);

        self.triangleIndexBuffer = Vec::with_capacity(triangle_index_buffer_len);
        self.triangleIndexBuffer
            .resize(triangle_index_buffer_len, 0);
        for idx in 0..self.bufferSize {
            self.triangleIndexBuffer[idx * 3 + 0] = idx as u32 * 3 + 0;
            self.triangleIndexBuffer[idx * 3 + 1] = idx as u32 * 3 + 1;
            self.triangleIndexBuffer[idx * 3 + 2] = idx as u32 * 3 + 2;
        }
    }

    const Z: f32 = 0.0; // -5.0;

    pub fn setRectVertices(
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
        let baseIndex: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = baseIndex + 24;
        if self.rectVertexBuffer.len() < new_len {
            self.rectVertexBuffer.resize(new_len, 0.0);
        }

        self.rectVertexBuffer[baseIndex + 0] = bottomleftx;
        self.rectVertexBuffer[baseIndex + 1] = bottomlefty;
        self.rectVertexBuffer[baseIndex + 2] = Self::Z;

        self.rectVertexBuffer[baseIndex + 7] = topleftx;
        self.rectVertexBuffer[baseIndex + 8] = toplefty;
        self.rectVertexBuffer[baseIndex + 9] = Self::Z;

        self.rectVertexBuffer[baseIndex + 14] = toprightx;
        self.rectVertexBuffer[baseIndex + 15] = toprighty;
        self.rectVertexBuffer[baseIndex + 16] = Self::Z;

        self.rectVertexBuffer[baseIndex + 21] = bottomrightx;
        self.rectVertexBuffer[baseIndex + 22] = bottomrighty;
        self.rectVertexBuffer[baseIndex + 23] = Self::Z;
    }

    pub fn setRectColors(&mut self, opacity: f32, color: Color) {
        // println!("SET RECT COLORS");
        let baseIndex: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = baseIndex + 28;
        if self.rectVertexBuffer.len() < new_len {
            self.rectVertexBuffer.resize(new_len, 0.0);
        }

        let a: f32 = opacity * color.alpha;
        let r: f32 = a * color.red;
        let g: f32 = a * color.green;
        let b: f32 = a * color.blue;

        self.rectVertexBuffer[baseIndex + 3] = r;
        self.rectVertexBuffer[baseIndex + 4] = g;
        self.rectVertexBuffer[baseIndex + 5] = b;
        self.rectVertexBuffer[baseIndex + 6] = a;

        self.rectVertexBuffer[baseIndex + 10] = r;
        self.rectVertexBuffer[baseIndex + 11] = g;
        self.rectVertexBuffer[baseIndex + 12] = b;
        self.rectVertexBuffer[baseIndex + 13] = a;

        self.rectVertexBuffer[baseIndex + 17] = r;
        self.rectVertexBuffer[baseIndex + 18] = g;
        self.rectVertexBuffer[baseIndex + 19] = b;
        self.rectVertexBuffer[baseIndex + 20] = a;

        self.rectVertexBuffer[baseIndex + 24] = r;
        self.rectVertexBuffer[baseIndex + 25] = g;
        self.rectVertexBuffer[baseIndex + 26] = b;
        self.rectVertexBuffer[baseIndex + 27] = a;
    }

    fn setTriVertices(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let baseIndex: usize = self.triangle_count * Self::VERTEX_SIZE * 3;

        let new_len = baseIndex + 17;
        if self.triangleVertexBuffer.len() < new_len {
            self.triangleVertexBuffer.resize(new_len, 0.0);
        }

        self.triangleVertexBuffer[baseIndex + 0] = x1;
        self.triangleVertexBuffer[baseIndex + 1] = y1;
        self.triangleVertexBuffer[baseIndex + 2] = Self::Z;

        self.triangleVertexBuffer[baseIndex + 7] = x2;
        self.triangleVertexBuffer[baseIndex + 8] = y2;
        self.triangleVertexBuffer[baseIndex + 9] = Self::Z;

        self.triangleVertexBuffer[baseIndex + 14] = x3;
        self.triangleVertexBuffer[baseIndex + 15] = y3;
        self.triangleVertexBuffer[baseIndex + 16] = Self::Z;
    }

    fn setTriColors(&mut self, opacity: f32, color: Color) {
        let baseIndex: usize = self.triangle_count * Self::VERTEX_SIZE * 3;

        let new_len = baseIndex + 21;
        if self.triangleVertexBuffer.len() < new_len {
            self.triangleVertexBuffer.resize(new_len, 0.0);
        }

        let alpha: f32 = opacity * color.alpha;
        let red: f32 = alpha * color.red;
        let green: f32 = alpha * color.green;
        let blue: f32 = alpha * color.blue;

        self.triangleVertexBuffer[baseIndex + 3] = red;
        self.triangleVertexBuffer[baseIndex + 4] = green;
        self.triangleVertexBuffer[baseIndex + 5] = blue;
        self.triangleVertexBuffer[baseIndex + 6] = alpha;

        self.triangleVertexBuffer[baseIndex + 10] = red;
        self.triangleVertexBuffer[baseIndex + 11] = green;
        self.triangleVertexBuffer[baseIndex + 12] = blue;
        self.triangleVertexBuffer[baseIndex + 13] = alpha;

        self.triangleVertexBuffer[baseIndex + 17] = red;
        self.triangleVertexBuffer[baseIndex + 18] = green;
        self.triangleVertexBuffer[baseIndex + 19] = blue;
        self.triangleVertexBuffer[baseIndex + 20] = alpha;
    }

    fn drawBuffer(&mut self, trisDone: bool) {
        if self.quad_count == 0 {
            return;
        }

        if !trisDone {
            self.endTris(true);
        }

        // let pipeline = self.myPipeline.get(null, Depth24Stencil8);
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
                    self.rectVertexBuffer.as_slice(),
                    GL_DYNAMIC_DRAW,
                );
                glchk!("set vertex data");

                // set index buffer
                gl::bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.ebo);
                glchk!("bind indices buffer");

                gl::buffer_data(
                    GL_ELEMENT_ARRAY_BUFFER,
                    self.indexBuffer.as_slice(),
                    GL_STATIC_DRAW,
                );
                glchk!("set indices data");

                // then reenable again VBO
                pipeline.setVertexBuffer(self.vbo);
                glchk!("set vbo");

                pipeline.setMatrix(self.projectionLocation, self.projectionMatrix);
                glchk!("set projection matrix");

                pipeline.setIndexBuffer(self.ebo);
                glchk!("set ebo");

                pipeline.drawIndexedVertices(0, Some(self.quad_count * 2 * 3));
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

    fn drawTriBuffer(&mut self, rectsDone: bool) {
        if !rectsDone {
            self.endRects(true);
        }

        // let pipeline = self.myPipeline.get(null, Depth24Stencil8);
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
                    self.triangleVertexBuffer.as_slice(),
                    GL_DYNAMIC_DRAW,
                );

                glchk!("set vertex data");

                // here correct

                gl::bind_buffer(GL_ELEMENT_ARRAY_BUFFER, self.triebo);
                glchk!("bind indices buffer");

                gl::buffer_data(
                    GL_ELEMENT_ARRAY_BUFFER,
                    self.triangleIndexBuffer.as_slice(),
                    GL_STATIC_DRAW,
                );
                glchk!("set indices data");

                // then reenable again VBO
                pipeline.setVertexBuffer(self.trivbo);
                glchk!("set vbo");

                pipeline.setMatrix(self.projectionLocation, self.projectionMatrix);
                glchk!("set projection matrix");

                pipeline.setIndexBuffer(self.triebo);
                glchk!("set ebo");

                pipeline.drawIndexedVertices(0, Some(self.triangle_count * 3));
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

    pub fn fillRect(
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
            self.drawTriBuffer(true); // Flush other buffer for right render order
        }

        if self.quad_count + 1 >= self.bufferSize {
            self.drawBuffer(false);
        }

        self.setRectColors(opacity, color);
        self.setRectVertices(
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

    pub fn fillTriangle(
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
            self.drawBuffer(true); // Flush other buffer for right render order
        }

        if self.triangle_count + 1 >= self.triangleBufferSize {
            self.drawTriBuffer(false);
        }

        self.setTriColors(opacity, color);
        self.setTriVertices(x1, y1, x2, y2, x3, y3);
        self.triangle_count += 1;
    }

    #[inline]
    pub fn endTris(&mut self, rectsDone: bool) {
        if self.triangle_count > 0 {
            self.drawTriBuffer(rectsDone);
        }
    }

    #[inline]
    pub fn endRects(&mut self, trisDone: bool) {
        if self.quad_count > 0 {
            self.drawBuffer(trisDone);
        }
    }

    #[inline]
    pub fn end(&mut self) {
        self.endTris(false);
        self.endRects(false);
    }
}
