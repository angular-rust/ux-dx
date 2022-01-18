use cgmath::prelude::*;
use gb::GlyphCruncher;
use glyph_brush as gb;
use std::rc::Rc;
use twox_hash::RandomXxHashBuilder64;
// use glyph_brush::{self as gb, ab_glyph::{Rect}, *};

use super::*;
use crate::{
    glchk,
    platform::{
        core::{BlendFactor, ShaderLocation, VertexFormat},
        gles::{core20::gl, enums::*},
    },
};

// pub type Font = gb::GlyphBrush<[f32; 13]>;

#[derive(Copy, Clone, Default)]
pub struct Font;

impl Font {
    pub fn height(&self, val: f32) -> f32 {
        0.0
    }
}

pub(super) struct TextShaderPainter {
    projection_matrix: cgmath::Matrix4<f32>,
    projection_location: ShaderLocation,
    texture_location: ShaderLocation,

    glyph_brush: gb::GlyphBrush<[f32; 13]>,
    // static {
    // standardTextPipeline: PipelineCache, // = None;
    buffer_size: usize, // = 1000;
    quad_count: usize,
    rect_vertex_buffer: Vec<f32>,
    index_buffer: Vec<u32>,
    fonttexture: Option<u32>,
    // static }
    // font: Kravur,
    // g: &'a Graphics,
    // myPipeline: PipelineCache, // = None;

    // pub pipeline(get, set): PipelineCache;
    vbo: u32,
    ebo: u32,
    pub fontsize: f32,

    bilinear: bool, // = false;
}

impl TextShaderPainter {
    const PIPELINE: u64 = 0x2;
    const VERTEX_SIZE: usize = 9;
    // painter_text_vert
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

    // painter_text_frag
    const FRAGMENT_SHADER: &'static str = r"
	#version 100
    precision mediump float;

    uniform sampler2D tex;

    in mediump vec2 texCoord;
    in mediump vec4 color;

    void main() {
        // get channel value like alpha
        float alpha = texture2D(tex, texCoord).r;
        
        // skip if lower 1 percent (2.55)
        if (alpha <= 0.0 ) {
           discard;
        } else {
           gl_FragColor = vec4(color.rgb, alpha * color.a);
        }
    }
    ";

    pub fn new() -> Self {
        let dejavu = gb::ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../../../../assets/fonts/OpenSans-Light.ttf"
        ))
        .unwrap();

        let glyph_brush = gb::GlyphBrushBuilder::using_font(dejavu).build();

        let mut instance = Self {
            buffer_size: 1000,
            quad_count: 0,
            rect_vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            projection_matrix: cgmath::Matrix4::identity(),
            projection_location: 0,
            fonttexture: None,
            texture_location: 0,
            bilinear: false,

            glyph_brush,
            vbo: 0,
            ebo: 0,
            fontsize: 72.0,
        };

        instance.init_shaders();
        instance.init_buffers();
        // init texture for glyph map
        instance.init_texture();
        instance
    }

    // fn get_pipeline(&self) -> PipelineCache {
    //     self.myPipeline
    // }

    // fn set_pipeline(&self, pipe: Option<PipelineCache>) {
    //     let pipe = match pipe {
    //         Some(pipe) => pipe,
    //         None => self.standardTextPipeline,
    //     };
    //     self.myPipeline = pipe;
    // }

    pub fn set_projection(&mut self, projection_matrix: cgmath::Matrix4<f32>) {
        self.projection_matrix = projection_matrix;
    }

    fn init_texture(&mut self) {
        // Use tightly packed data
        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1);
        // Generate a Texture Object Handle
        let texture_id = {
            let textures = gl::gen_textures(1);
            textures[0]
        };

        let texture_unit = 0;
        // Activate a texture
        gl::active_texture(GL_TEXTURE0 + texture_unit);
        // Bind the texture object
        gl::bind_texture(GL_TEXTURE_2D, texture_id);

        // Set the filtering mode
        gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as i32);
        gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as i32);

        self.fonttexture = Some(texture_id);
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

        // self.rectVertexBuffer.unlock(self.bufferIndex * 4);

        // let pipeline = self.myPipeline.get(None, Depth24Stencil8);
        match PipelineCache::global().get(Self::PIPELINE) {
            Some(pipeline) => {
                // println!("OOPS {} {}", self.quad_count, self.rectVertexBuffer.get(0).unwrap());

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

                // pipeline.setTexture(self.textureLocation, 0, self.lastTexture);

                pipeline.set_texture(self.texture_location, 0, self.fonttexture);
                // pipeline.setTexture(self.textureLocation, 0, Some(1));

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

                // self.quad_count = 0;
            }
            None => {
                println!("THERE NO PIPELINE IN CACHE FOR TEXT PAINTER");
            }
        }
    }

    pub fn set_bilinear_filter(&mut self, bilinear: bool) {
        self.end();
        // self.bilinear = bilinear;
    }

    pub fn set_font(&mut self, font: Font) {
        // self.font = cast(font, Kravur);
    }

    fn find_index(&self, char_code: i32) -> i32 {
        // // let glyphs = graphics2.Graphics.fontGlyphs;
        // let blocks = Kravur.KravurImage.charBlocks;
        // let offset = 0;
        // for i in 0..blocks.length / 2 {
        //     // usize
        //     let start = blocks[i * 2];
        //     let end = blocks[i * 2 + 1];
        //     if charCode >= start && charCode <= end {
        //         return offset + charCode - start;
        //     }
        //     offset += end - start + 1;
        // }
        return 0;
    }

    // let bakedQuadCache = new Kravur.AlignedQuad();

    pub fn measure(&mut self, text: &str) -> Option<(f32, f32)> {
        let scale = (self.fontsize * 1.0).round();
        let base_text = gb::Text::new(&text).with_scale(scale);
        let res = self
            .glyph_brush
            .glyph_bounds(gb::Section::default().add_text(base_text));

        res.map(|rect| {
            let width = rect.max.x - rect.min.x;
            let height = rect.max.y - rect.min.y;
            (width, height)
        })
    }

    pub fn draw_string(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        opacity: f32,
        color: Color,
        // transformation: cgmath::Matrix3<f32>,
    ) {
        let scale = (self.fontsize * 1.0).round();

        let base_text = gb::Text::new(&text).with_scale(scale);

        // let text_len = text.len();
        // // if the next text will overflow buffer size so flush the painter
        // if self.quad_count + text_len >= self.bufferSize {
        //     self.draw_buffer();
        // }

        self.glyph_brush.queue(
            gb::Section::default()
                .add_text(base_text.with_color([
                    color.red,   /* opacity*/
                    color.green, /* opacity*/
                    color.blue,  /* opacity*/
                    color.alpha, /* opacity*/
                ]))
                // .with_bounds((width / 3.15, height))
                .with_screen_position((x, y)),
        );

        // let max_image_dimension = 32768;
        // // real texture dimension
        // let mut real_dim = (0_f32, 0_f32);

        // let texture_id = self.lastTexture.unwrap();

        // let mut brush_action;
        // loop {
        //     brush_action = self.glyph_brush.process_queued(
        //         |rect, tex_data| {
        //             real_dim = (rect.width() as f32, rect.height() as f32);

        //             // Update part of gpu texture with new glyph alpha values
        //             gl::bind_texture(gl::TEXTURE_2D, texture_id);
        //             // println!("TEXT {} {} {}", texture_id, rect.width(), rect.height());

        //             gl::tex_image_2d(
        //                 gl::TEXTURE_2D,
        //                 0,
        //                 gl::LUMINANCE as i32,
        //                 rect.width() as i32,
        //                 rect.height() as i32,
        //                 0,
        //                 gl::LUMINANCE,
        //                 gl::UNSIGNED_BYTE,
        //                 tex_data,
        //             );
        //             glchk!("");
        //         },
        //         to_vertex,
        //     );

        //     match brush_action {
        //         Ok(_) => break,
        //         Err(gb::BrushError::TextureTooSmall { suggested, .. }) => {
        //             let (new_width, new_height) = if (suggested.0 > max_image_dimension
        //                 || suggested.1 > max_image_dimension)
        //                 && (self.glyph_brush.texture_dimensions().0 < max_image_dimension
        //                     || self.glyph_brush.texture_dimensions().1 < max_image_dimension)
        //             {
        //                 (max_image_dimension, max_image_dimension)
        //             } else {
        //                 suggested
        //             };
        //             print!("\r                            \r");
        //             println!("Resizing glyph texture -> {}x{}", new_width, new_height);

        //             // Recreate texture as a larger size to fit more
        //             // texture = GlyphTexture::new((new_width, new_height));

        //             self.glyph_brush.resize_texture(new_width, new_height);
        //         }
        //     }
        // }

        // match brush_action.unwrap() {
        //     gb::BrushAction::Draw(vertices) => {
        //         // logical texture dimension
        //         let logical_dim = self.glyph_brush.texture_dimensions();
        //         // X scale factor
        //         let fx = logical_dim.0 as f32 / real_dim.0;
        //         // Y scale factor
        //         let fy = logical_dim.1 as f32 / real_dim.1;

        //         // text_pipe.upload_vertices(&vertices)
        //         println!("======== DRAW ======== {} {}", vertices.len(), self.quad_count);

        //         for glyph in vertices.iter() {
        //             let lx = glyph[0];
        //             let rx = glyph[3];
        //             let ty = glyph[4];
        //             let by = glyph[1];

        //             // left top
        //             let tlx = glyph[5];
        //             let tty = glyph[8];

        //             // right bottom
        //             let trx = glyph[7];
        //             let tby = glyph[6];

        //             let red = glyph[9];
        //             let green = glyph[10];
        //             let blue = glyph[11];
        //             let alpha = glyph[12];

        //             // color
        //             self.setRectColor(red, green, blue, alpha);

        //             // texture
        //             self.setRectTexCoords(tlx * fx, tty * fy, trx * fx, tby * fy);
        //             // self.setRectTexCoords(0.0, 0.0, 1.0, 1.0);

        //             #[rustfmt::skip]
        //             self.setRectVertices(
        //                 lx, by,
        //                 lx, ty,
        //                 rx, ty,
        //                 rx, by);
        //             self.quad_count += 1;
        //         }
        //     }
        //     gb::BrushAction::ReDraw => {}
        // }

        // let font = self.font._get(self.fontSize);
        // let tex = font.getTexture();
        // if self.lastTexture.is_some() && tex != self.lastTexture {
        //     self.draw_buffer();
        // }

        // self.lastTexture = tex;

        // let xpos = x;
        // let ypos = y;
        // for i in 0..text.length() {
        //     let charCode = StringTools.fastCodeAt(text, i);
        //     let q = font.getBakedQuad(bakedQuadCache, findIndex(charCode), xpos, ypos);
        //     if q != None {
        //         if self.bufferIndex + 1 >= self.bufferSize {
        //             self.draw_buffer();
        //         }
        //         self.setRectColors(opacity, color);
        //         self.setRectTexCoords(
        //             q.s0 * tex.width / tex.realWidth,
        //             q.t0 * tex.height / tex.realHeight,
        //             q.s1 * tex.width / tex.realWidth,
        //             q.t1 * tex.height / tex.realHeight,
        //         );
        //         let p0 = transformation.multvec(Vector2(q.x0, q.y1)); // bottom-left
        //         let p1 = transformation.multvec(Vector2(q.x0, q.y0)); // top-left
        //         let p2 = transformation.multvec(Vector2(q.x1, q.y0)); // top-right
        //         let p3 = transformation.multvec(Vector2(q.x1, q.y1)); // bottom-right
        //         self.setRectVertices(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        //         xpos += q.xadvance;
        //         self.bufferIndex += 1;
        //     }
        // }
    }

    pub fn draw_characters(
        &self,
        text: Vec<i32>,
        start: i32,
        length: i32,
        opacity: f32,
        color: Color,
        x: f32,
        y: f32,
        transformation: cgmath::Matrix3<f32>,
    ) {
        // let font = self.font._get(self.fontSize);
        // let tex = font.getTexture();
        // if self.lastTexture.is_some() && tex != self.lastTexture {
        //     self.draw_buffer();
        // }

        // self.lastTexture = tex;

        // let xpos = x;
        // let ypos = y;
        // for i in start..start + length {
        //     let q = font.getBakedQuad(self.bakedQuadCache, self.findIndex(text[i]), xpos, ypos);
        //     if q != None {
        //         if self.bufferIndex + 1 >= self.bufferSize {
        //             self.draw_buffer();
        //         }
        //         self.setRectColors(opacity, color);
        //         self.setRectTexCoords(
        //             q.s0 * tex.width / tex.realWidth,
        //             q.t0 * tex.height / tex.realHeight,
        //             q.s1 * tex.width / tex.realWidth,
        //             q.t1 * tex.height / tex.realHeight,
        //         );
        //         let p0 = transformation.multvec(Vector2(q.x0, q.y1)); // bottom-left
        //         let p1 = transformation.multvec(Vector2(q.x0, q.y0)); // top-left
        //         let p2 = transformation.multvec(Vector2(q.x1, q.y0)); // top-right
        //         let p3 = transformation.multvec(Vector2(q.x1, q.y1)); // bottom-right
        //         self.setRectVertices(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        //         xpos += q.xadvance;
        //         self.bufferIndex += 1;
        //     }
        // }
    }

    pub fn end(&mut self) {
        let max_image_dimension = 32768;

        let texture_id = self.fonttexture.unwrap();

        let mut brush_action;

        gl::active_texture(GL_TEXTURE0);
        gl::bind_texture(GL_TEXTURE_2D, texture_id);
        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1); // needed for NVG

        // update texture parts
        let (tex_width, tex_height) = self.glyph_brush.texture_dimensions();

        loop {
            brush_action = self.glyph_brush.process_queued(
                |rect, tex_data| {
                    // gl::active_texture(GL_TEXTURE0);
                    // gl::bind_texture(GL_TEXTURE_2D, texture_id);

                    // Update part of gpu texture with new glyph alpha values
                    // TODO: need to track the maximum's ot tex
                    if rect.min[0] == 0 && rect.min[1] == 0 {
                        gl::empty_tex_image_2d(
                            GL_TEXTURE_2D,
                            0,
                            GL_LUMINANCE as i32,
                            tex_width as i32,
                            tex_height as i32,
                            0,
                            GL_LUMINANCE,
                            GL_UNSIGNED_BYTE,
                        );
                        glchk!("");
                    }

                    gl::tex_sub_image_2d(
                        GL_TEXTURE_2D,
                        0,
                        rect.min[0] as i32,
                        rect.min[1] as i32,
                        rect.width() as i32,
                        rect.height() as i32,
                        GL_LUMINANCE,
                        GL_UNSIGNED_BYTE,
                        tex_data,
                    );
                    glchk!("update sub umage");
                },
                to_vertex,
            );

            match brush_action {
                Ok(_) => break,
                Err(gb::BrushError::TextureTooSmall { suggested, .. }) => {
                    let (new_width, new_height) = if (suggested.0 > max_image_dimension
                        || suggested.1 > max_image_dimension)
                        && (self.glyph_brush.texture_dimensions().0 < max_image_dimension
                            || self.glyph_brush.texture_dimensions().1 < max_image_dimension)
                    {
                        (max_image_dimension, max_image_dimension)
                    } else {
                        suggested
                    };
                    print!("\r                            \r");
                    println!("Resizing glyph texture -> {}x{}", new_width, new_height);

                    // Recreate texture as a larger size to fit more
                    // texture = GlyphTexture::new((new_width, new_height));

                    self.glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }

        // a bit strange to handle it here
        match brush_action.unwrap() {
            gb::BrushAction::Draw(vertices) => {
                // logical texture dimension
                let logical_dim = self.glyph_brush.texture_dimensions();

                // text_pipe.upload_vertices(&vertices)
                // println!("======== DRAW ======== {} {}", vertices.len(), self.quad_count);

                // just reset quad count
                self.quad_count = 0;
                for glyph in vertices.iter() {
                    let lx = glyph[0];
                    let rx = glyph[3];
                    let ty = glyph[4];
                    let by = glyph[1];

                    // left top
                    let tlx = glyph[5];
                    let tty = glyph[8];

                    // right bottom
                    let trx = glyph[7];
                    let tby = glyph[6];

                    let red = glyph[9];
                    let green = glyph[10];
                    let blue = glyph[11];
                    let alpha = glyph[12];

                    // color
                    self.set_rect_color(red, green, blue, alpha);

                    // texture
                    self.set_rect_tex_coords(tlx, tty, trx, tby);
                    // self.setRectTexCoords(0.0, 0.0, 1.0, 1.0);

                    self.set_rect_vertices(lx, by, lx, ty, rx, ty, rx, by);
                    self.quad_count += 1;
                }
            }
            gb::BrushAction::ReDraw => {
                // TODO: should be handled coz it called frequently at window resize
                // println!("WANT REDRAW");
            }
        }

        if self.quad_count > 0 {
            self.draw_buffer();
        }

        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 4); // needed for NVG
        gl::bind_texture(GL_TEXTURE_2D, 0);
    }
}

// NPOT things
// // <https://software.intel.com/content/www/us/en/develop/articles/opengl-performance-tips-power-of-two-textures-have-better-performance.html>
// // create a pow2 scaled version
// // auto pow2 = [](unsigned v) { int p = 2; while (v >>= 1) p <<= 1; return p; };
// // w2 = h2 = max(pow2(w), pow2(h)); std::vector<GLubyte> img2(w2 * h2 * 4);
// let height = {
//     let mut v = rect.height();
//     let mut p = 2;
//     let a = while {
//         v >>= 1;
//         v > 0
//     } {
//         p <<= 1;
//     };
//     p
// };

// // let try deal with max by width or height
// let dim = rect.width().max(height) as i32;

type Vertex = [f32; 13];

#[inline]
fn to_vertex(
    glyph_brush::GlyphVertex {
        mut tex_coords,
        pixel_coords,
        bounds,
        extra,
    }: glyph_brush::GlyphVertex,
) -> Vertex {
    let gl_bounds = bounds;

    let mut gl_rect = gb::ab_glyph::Rect {
        min: gb::ab_glyph::point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
        max: gb::ab_glyph::point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
    };

    // println!("TO VERT TY {} {}", tex_coords.width(), tex_coords.height());

    // handle overlapping bounds, modify uv_rect to preserve texture aspect
    if gl_rect.max.x > gl_bounds.max.x {
        let old_width = gl_rect.width();
        gl_rect.max.x = gl_bounds.max.x;
        tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
    }

    if gl_rect.min.x < gl_bounds.min.x {
        let old_width = gl_rect.width();
        gl_rect.min.x = gl_bounds.min.x;
        tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
    }

    if gl_rect.max.y > gl_bounds.max.y {
        let old_height = gl_rect.height();
        gl_rect.max.y = gl_bounds.max.y;
        tex_coords.max.y = tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
    }

    if gl_rect.min.y < gl_bounds.min.y {
        let old_height = gl_rect.height();
        gl_rect.min.y = gl_bounds.min.y;
        tex_coords.min.y = tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
    }

    // println!("TO VERTEX {}x{} {}x{}", gl_rect.min.x, gl_rect.max.y, gl_rect.max.x, gl_rect.min.y);
    // println!("TO VERTEX {}x{}", gl_rect.max.x - gl_rect.min.x, gl_rect.max.y - gl_rect.min.y);
    // println!("TX {}x{} {}x{}", tex_coords.min.x, tex_coords.max.y, tex_coords.max.x, tex_coords.min.y);

    [
        gl_rect.min.x,    // 0
        gl_rect.max.y,    // 1
        extra.z,          // 2
        gl_rect.max.x,    // 3
        gl_rect.min.y,    // 4
        tex_coords.min.x, // 5 slx
        tex_coords.max.y, // 6 sby
        tex_coords.max.x, // 7 srx
        tex_coords.min.y, // 8 sty
        extra.color[0],
        extra.color[1],
        extra.color[2],
        extra.color[3],
    ]
}
