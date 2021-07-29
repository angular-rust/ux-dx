#![allow(unused_macros)]
use cgmath::prelude::*;
use std::rc::Rc;

use super::*;
use crate::{
    glchk,
    gles::{core20::gl, enums::*},
    BlendFactor, ShaderLocation, VertexFormat,
};

pub struct ImageShaderPainter {
    projectionMatrix: cgmath::Matrix4<f32>,
    projectionLocation: ShaderLocation,
    textureLocation: ShaderLocation,

    bufferSize: usize, // = 1500;
    quad_count: usize,
    rectVertexBuffer: Vec<f32>,
    indexBuffer: Vec<u32>,
    lastTexture: Option<u32>, // cg texture id

    bilinear: bool,        // = false;
    bilinearMipmaps: bool, // = false;

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
            bufferSize: 1500,
            quad_count: 0,
            rectVertexBuffer: Vec::new(),
            indexBuffer: Vec::new(),
            projectionMatrix: cgmath::Matrix4::identity(),
            projectionLocation: 0,
            lastTexture: None,
            textureLocation: 0,
            bilinear: false,
            bilinearMipmaps: false,
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

    pub fn setProjection(&mut self, projectionMatrix: cgmath::Matrix4<f32>) {
        self.projectionMatrix = projectionMatrix;
    }

    fn init_shaders(&mut self) {
        let cache = PipelineCache::global();
        match cache.get(Self::PIPELINE) {
            Some(pipeline) => {
                self.projectionLocation = pipeline.get_uniform_location("projectionMatrix");
                self.textureLocation = pipeline.get_uniform_location("tex");
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
                        self.projectionLocation = pipeline.get_uniform_location("projectionMatrix");
                        self.textureLocation = pipeline.get_uniform_location("tex");
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
    }

    const Z: f32 = 0.0; // -5.0;

    #[inline]
    fn setRectVertices(
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

        let new_len = baseIndex + 30;
        if self.rectVertexBuffer.len() < new_len {
            self.rectVertexBuffer.resize(new_len, 0.0);
        }
        self.rectVertexBuffer[baseIndex + 0] = bottomleftx;
        self.rectVertexBuffer[baseIndex + 1] = bottomlefty;
        self.rectVertexBuffer[baseIndex + 2] = Self::Z;

        self.rectVertexBuffer[baseIndex + 9] = topleftx;
        self.rectVertexBuffer[baseIndex + 10] = toplefty;
        self.rectVertexBuffer[baseIndex + 11] = Self::Z;

        self.rectVertexBuffer[baseIndex + 18] = toprightx;
        self.rectVertexBuffer[baseIndex + 19] = toprighty;
        self.rectVertexBuffer[baseIndex + 20] = Self::Z;

        self.rectVertexBuffer[baseIndex + 27] = bottomrightx;
        self.rectVertexBuffer[baseIndex + 28] = bottomrighty;
        self.rectVertexBuffer[baseIndex + 29] = Self::Z;
    }

    #[inline]
    fn setRectTexCoords(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        let baseIndex: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = baseIndex + 32;
        if self.rectVertexBuffer.len() < new_len {
            self.rectVertexBuffer.resize(new_len, 0.0);
        }

        self.rectVertexBuffer[baseIndex + 3] = left;
        self.rectVertexBuffer[baseIndex + 4] = bottom;

        self.rectVertexBuffer[baseIndex + 12] = left;
        self.rectVertexBuffer[baseIndex + 13] = top;

        self.rectVertexBuffer[baseIndex + 21] = right;
        self.rectVertexBuffer[baseIndex + 22] = top;

        self.rectVertexBuffer[baseIndex + 30] = right;
        self.rectVertexBuffer[baseIndex + 31] = bottom;
    }

    #[inline]
    fn setRectColor(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        let baseIndex: usize = self.quad_count * Self::VERTEX_SIZE * 4;

        let new_len = baseIndex + 36;
        if self.rectVertexBuffer.len() < new_len {
            self.rectVertexBuffer.resize(new_len, 0.0);
        }

        self.rectVertexBuffer[baseIndex + 5] = red;
        self.rectVertexBuffer[baseIndex + 6] = green;
        self.rectVertexBuffer[baseIndex + 7] = blue;
        self.rectVertexBuffer[baseIndex + 8] = alpha;

        self.rectVertexBuffer[baseIndex + 14] = red;
        self.rectVertexBuffer[baseIndex + 15] = green;
        self.rectVertexBuffer[baseIndex + 16] = blue;
        self.rectVertexBuffer[baseIndex + 17] = alpha;

        self.rectVertexBuffer[baseIndex + 23] = red;
        self.rectVertexBuffer[baseIndex + 24] = green;
        self.rectVertexBuffer[baseIndex + 25] = blue;
        self.rectVertexBuffer[baseIndex + 26] = alpha;

        self.rectVertexBuffer[baseIndex + 32] = red;
        self.rectVertexBuffer[baseIndex + 33] = green;
        self.rectVertexBuffer[baseIndex + 34] = blue;
        self.rectVertexBuffer[baseIndex + 35] = alpha;
    }

    fn drawBuffer(&mut self) {
        if self.quad_count == 0 {
            return;
        }

        // let pipeline = self.myPipeline.get(null, Depth24Stencil8);
        match PipelineCache::global().get(Self::PIPELINE) {
            Some(pipeline) => {
                pipeline.make_current();
                glchk!("set pipeline");

                // SOME LOW LEVEL LOGIC
                gl::bind_buffer(GL_ARRAY_BUFFER, self.vbo);
                glchk!("bind vertex buffer");

                gl::buffer_data(
                    GL_ARRAY_BUFFER,
                    self.rectVertexBuffer.as_slice(),
                    GL_DYNAMIC_DRAW,
                );
                glchk!("set vertex data");

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

                pipeline.setTexture(self.textureLocation, 0, self.lastTexture);

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

                pipeline.setMatrix(self.projectionLocation, self.projectionMatrix);
                glchk!("set projection matrix");

                pipeline.setIndexBuffer(self.ebo);
                glchk!("set ebo");

                pipeline.drawIndexedVertices(0, Some(self.quad_count * 2 * 3));

                // pipeline.setTexture(pipeline.textureLocation, None);

                self.quad_count = 0;
            }
            None => {
                println!("THERE NO PIPELINE IN CACHE FOR IMAGE PAINTER");
            }
        }
    }

    pub fn setBilinearFilter(&mut self, bilinear: bool) {
        self.drawBuffer();
        self.lastTexture = None;
        self.bilinear = bilinear;
    }

    pub fn setBilinearMipmapFilter(&mut self, bilinear: bool) {
        self.drawBuffer();
        self.lastTexture = None;
        self.bilinearMipmaps = bilinear;
    }

    // Opacity seems is some overcode
    // but i think it need to make smooth transition
    // coz it have more precission
    #[inline]
    pub fn drawImage(
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
        if self.quad_count + 1 >= self.bufferSize {
            // check the buffer overflow first
            self.drawBuffer();
        } else if let Some(last_texture) = self.lastTexture {
            // then check the texture changed
            if let Some(img_texure) = img.tex {
                if last_texture != img_texure {
                    self.drawBuffer();
                }
            } else {
                println!("IMAGE WITHOUT TEXTURE ID");
            }
        }

        self.setRectColor(tint.red, tint.green, tint.blue, tint.alpha * opacity);

        self.setRectTexCoords(
            0.0,
            0.0,
            img.width as f32 / img.real_width as f32,
            img.height as f32 / img.real_height as f32,
        );

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
        self.lastTexture = img.tex; // should detect the texture is correct
    }

    #[inline]
    pub fn drawImage2(
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

        if self.quad_count + 1 >= self.bufferSize {
            // check the buffer overflow first
            self.drawBuffer();
        } else if let Some(last_texture) = self.lastTexture {
            // then check the texture changed
            if let Some(img_texure) = img.tex {
                if last_texture != img_texure {
                    self.drawBuffer();
                }
            } else {
                println!("IMAGE WITHOUT TEXTURE ID");
            }
        }

        self.setRectTexCoords(
            sx / img.real_width as f32,
            sy / img.real_height as f32,
            (sx + sw) / img.real_width as f32,
            (sy + sh) / img.real_height as f32,
        );

        self.setRectColor(tint.red, tint.green, tint.blue, tint.alpha * opacity);

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
        self.lastTexture = img.tex;
    }

    #[inline]
    pub fn drawImageScale(
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

        if self.quad_count + 1 >= self.bufferSize {
            // check the buffer overflow first
            self.drawBuffer();
        } else if let Some(last_texture) = self.lastTexture {
            // then check the texture changed
            if let Some(img_texure) = img.tex {
                if last_texture != img_texure {
                    self.drawBuffer();
                }
            } else {
                println!("IMAGE WITHOUT TEXTURE ID");
            }
        }

        self.setRectTexCoords(
            sx / img.real_width as f32,
            sy / img.real_height as f32,
            (sx + sw) / img.real_width as f32,
            (sy + sh) / img.real_height as f32,
        );

        self.setRectColor(tint.red, tint.green, tint.blue, opacity);

        self.setRectVertices(left, bottom, left, top, right, top, right, bottom);

        self.quad_count += 1;
        self.lastTexture = img.tex;
    }

    pub fn end(&mut self) {
        if self.quad_count > 0 {
            self.drawBuffer();
        }
        self.lastTexture = None;
    }
}
