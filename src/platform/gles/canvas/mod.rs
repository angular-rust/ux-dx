//! Canvas support on top of nvg library
use nvg::renderer::*;
use slab::Slab;

use super::{core30::gl, enums::*, GLenum, GLuint};

pub use nvg::*;

struct Shader {
    prog: u32,
    frag: u32,
    vert: u32,
    loc_viewsize: i32,
    loc_tex: i32,
    loc_frag: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::delete_program(self.prog);
        gl::delete_shader(self.vert);
        gl::delete_shader(self.frag);
    }
}

impl Shader {
    fn load() -> anyhow::Result<Shader> {
        let prog = gl::create_program();
        let vert = gl::create_shader(GL_VERTEX_SHADER);
        let frag = gl::create_shader(GL_FRAGMENT_SHADER);

        let vert_source = include_bytes!("shader.vert");
        let frag_source = include_bytes!("shader.frag");

        gl::shader_source(vert, vert_source);
        gl::shader_source(frag, frag_source);

        gl::compile_shader(vert);

        if gl::get_shaderiv(vert, GL_COMPILE_STATUS) == 0 {
            return Err(shader_error(vert, "shader.vert"));
        }

        gl::compile_shader(frag);

        if gl::get_shaderiv(frag, GL_COMPILE_STATUS) == 0 {
            return Err(shader_error(vert, "shader.frag"));
        }

        gl::attach_shader(prog, vert);
        gl::attach_shader(prog, frag);

        gl::bind_attrib_location(prog, 0, "vertex");
        gl::bind_attrib_location(prog, 1, "tcoord");

        gl::link_program(prog);

        if gl::get_programiv(prog, GL_LINK_STATUS) == 0 {
            return Err(program_error(prog));
        }

        Ok(Shader {
            prog,
            frag,
            vert,
            loc_viewsize: gl::get_uniform_location(prog, "viewSize"),
            loc_tex: gl::get_uniform_location(prog, "tex"),
            loc_frag: gl::get_uniform_block_index(prog, "frag"),
        })
    }
}

enum ShaderType {
    FillGradient,
    FillImage,
    Simple,
    Image,
}

#[derive(PartialEq, Eq)]
enum CallType {
    Fill,
    ConvexFill,
    Stroke,
    Triangles,
}

struct Blend {
    src_rgb: GLenum,
    dst_rgb: GLenum,
    src_alpha: GLenum,
    dst_alpha: GLenum,
}

impl From<CompositeOperationState> for Blend {
    fn from(state: CompositeOperationState) -> Self {
        Blend {
            src_rgb: convert_blend_factor(state.src_rgb),
            dst_rgb: convert_blend_factor(state.dst_rgb),
            src_alpha: convert_blend_factor(state.src_alpha),
            dst_alpha: convert_blend_factor(state.dst_alpha),
        }
    }
}

struct Call {
    call_type: CallType,
    image: Option<usize>,
    path_offset: usize,
    path_count: usize,
    triangle_offset: usize,
    triangle_count: usize,
    uniform_offset: usize,
    blend_func: Blend,
}

struct Texture {
    tex: GLuint,
    width: usize,
    height: usize,
    texture_type: TextureType,
    flags: ImageFlags,
}

impl Drop for Texture {
    fn drop(&mut self) {
        gl::delete_textures(&[self.tex])
    }
}

struct GLPath {
    fill_offset: usize,
    fill_count: usize,
    stroke_offset: usize,
    stroke_count: usize,
}

#[derive(Default)]
#[allow(dead_code)]
struct FragUniforms {
    scissor_mat: [f32; 12],
    paint_mat: [f32; 12],
    inner_color: Color,
    outer_color: Color,
    scissor_ext: [f32; 2],
    scissor_scale: [f32; 2],
    extent: [f32; 2],
    radius: f32,
    feather: f32,
    stroke_mult: f32,
    stroke_thr: f32,
    tex_type: i32,
    type_: i32,
}

pub struct Renderer {
    shader: Shader,
    textures: Slab<Texture>,
    view: Extent,
    vert_buf: GLuint,
    vert_arr: GLuint,
    frag_buf: GLuint,
    frag_size: usize,
    calls: Vec<Call>,
    paths: Vec<GLPath>,
    vertexes: Vec<Vertex>,
    uniforms: Vec<u8>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        gl::delete_buffers(&[self.frag_buf]);
        gl::delete_buffers(&[self.vert_buf]);
        gl::delete_vertex_arrays(&[self.vert_arr]);
    }
}

impl Renderer {
    pub fn create() -> anyhow::Result<Renderer> {
        let shader = Shader::load()?;

        let vert_arr = gl::gen_vertex_array();

        let vert_buf = gl::gen_buffer();

        gl::uniform_block_binding(shader.prog, shader.loc_frag, 0);

        let frag_buf = gl::gen_buffer();

        let align = gl::get_integerv(GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT);

        let frag_size = std::mem::size_of::<FragUniforms>() + (align as usize)
            - std::mem::size_of::<FragUniforms>() % (align as usize);

        gl::finish();

        Ok(Renderer {
            shader,
            textures: Default::default(),
            view: Default::default(),
            vert_buf,
            vert_arr,
            frag_buf,
            frag_size,
            calls: Default::default(),
            paths: Default::default(),
            vertexes: Default::default(),
            uniforms: Default::default(),
        })
    }

    fn set_uniforms(&self, offset: usize, img: Option<usize>) {
        gl::bind_buffer_range(
            GL_UNIFORM_BUFFER,
            0,
            self.frag_buf,
            (offset * self.frag_size) as isize,
            std::mem::size_of::<FragUniforms>() as isize,
        );

        if let Some(img) = img {
            if let Some(texture) = self.textures.get(img) {
                gl::bind_texture(GL_TEXTURE_2D, texture.tex);
            }
        } else {
            gl::bind_texture(GL_TEXTURE_2D, 0);
        }
    }

    fn do_fill(&self, call: &Call) {
        let paths = &self.paths[call.path_offset..call.path_offset + call.path_count];

        gl::enable(GL_STENCIL_TEST);
        gl::stencil_mask(0xff);
        gl::stencil_func(GL_ALWAYS, 0, 0xff);
        gl::color_mask(false, false, false, false);

        self.set_uniforms(call.uniform_offset, call.image);

        gl::stencil_op_separate(GL_FRONT, GL_KEEP, GL_KEEP, GL_INCR_WRAP);
        gl::stencil_op_separate(GL_BACK, GL_KEEP, GL_KEEP, GL_DECR_WRAP);
        gl::disable(GL_CULL_FACE);
        for path in paths {
            gl::draw_arrays(
                GL_TRIANGLE_FAN,
                path.fill_offset as i32,
                path.fill_count as i32,
            );
        }
        gl::enable(GL_CULL_FACE);

        gl::color_mask(true, true, true, true);

        self.set_uniforms(call.uniform_offset + 1, call.image);

        gl::stencil_func(GL_EQUAL, 0x00, 0xff);
        gl::stencil_op(GL_KEEP, GL_KEEP, GL_KEEP);
        for path in paths {
            gl::draw_arrays(
                GL_TRIANGLE_STRIP,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }

        gl::stencil_func(GL_NOTEQUAL, 0x00, 0xff);
        gl::stencil_op(GL_ZERO, GL_ZERO, GL_ZERO);
        gl::draw_arrays(
            GL_TRIANGLE_STRIP,
            call.triangle_offset as i32,
            call.triangle_count as i32,
        );

        gl::disable(GL_STENCIL_TEST);
    }

    fn do_convex_fill(&self, call: &Call) {
        let paths = &self.paths[call.path_offset..call.path_offset + call.path_count];
        self.set_uniforms(call.uniform_offset, call.image);
        for path in paths {
            gl::draw_arrays(
                GL_TRIANGLE_FAN,
                path.fill_offset as i32,
                path.fill_count as i32,
            );
            if path.stroke_count > 0 {
                gl::draw_arrays(
                    GL_TRIANGLE_STRIP,
                    path.stroke_offset as i32,
                    path.stroke_count as i32,
                );
            }
        }
    }

    fn do_stroke(&self, call: &Call) {
        let paths = &self.paths[call.path_offset..call.path_offset + call.path_count];

        gl::enable(GL_STENCIL_TEST);
        gl::stencil_mask(0xff);
        gl::stencil_func(GL_EQUAL, 0x0, 0xff);
        gl::stencil_op(GL_KEEP, GL_KEEP, GL_INCR);
        self.set_uniforms(call.uniform_offset + 1, call.image);
        for path in paths {
            gl::draw_arrays(
                GL_TRIANGLE_STRIP,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }

        self.set_uniforms(call.uniform_offset, call.image);
        gl::stencil_func(GL_EQUAL, 0x0, 0xff);
        gl::stencil_op(GL_KEEP, GL_KEEP, GL_KEEP);
        for path in paths {
            gl::draw_arrays(
                GL_TRIANGLE_STRIP,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }

        gl::color_mask(false, false, false, false);
        gl::stencil_func(GL_ALWAYS, 0x0, 0xff);
        gl::stencil_op(GL_ZERO, GL_ZERO, GL_ZERO);
        for path in paths {
            gl::draw_arrays(
                GL_TRIANGLE_STRIP,
                path.stroke_offset as i32,
                path.stroke_count as i32,
            );
        }
        gl::color_mask(true, true, true, true);

        gl::disable(GL_STENCIL_TEST);
    }

    fn do_triangles(&self, call: &Call) {
        self.set_uniforms(call.uniform_offset, call.image);
        gl::draw_arrays(
            GL_TRIANGLES,
            call.triangle_offset as i32,
            call.triangle_count as i32,
        );
    }

    fn convert_paint(
        &self,
        paint: &Paint,
        scissor: &Scissor,
        width: f32,
        fringe: f32,
        stroke_thr: f32,
    ) -> FragUniforms {
        let mut frag = FragUniforms {
            scissor_mat: Default::default(),
            paint_mat: Default::default(),
            inner_color: premul_color(paint.inner_color),
            outer_color: premul_color(paint.outer_color),
            scissor_ext: Default::default(),
            scissor_scale: Default::default(),
            extent: Default::default(),
            radius: 0.0,
            feather: 0.0,
            stroke_mult: 0.0,
            stroke_thr,
            tex_type: 0,
            type_: 0,
        };

        if scissor.extent.width < -0.5 || scissor.extent.height < -0.5 {
            frag.scissor_ext[0] = 1.0;
            frag.scissor_ext[1] = 1.0;
            frag.scissor_scale[0] = 1.0;
            frag.scissor_scale[1] = 1.0;
        } else {
            frag.scissor_mat = xform_to_3x4(scissor.xform.inverse());
            frag.scissor_ext[0] = scissor.extent.width;
            frag.scissor_ext[1] = scissor.extent.height;
            frag.scissor_scale[0] = (scissor.xform.0[0] * scissor.xform.0[0]
                + scissor.xform.0[2] * scissor.xform.0[2])
                .sqrt()
                / fringe;
            frag.scissor_scale[1] = (scissor.xform.0[1] * scissor.xform.0[1]
                + scissor.xform.0[3] * scissor.xform.0[3])
                .sqrt()
                / fringe;
        }

        frag.extent = [paint.extent.width, paint.extent.height];
        frag.stroke_mult = (width * 0.5 + fringe * 0.5) / fringe;

        let mut invxform = Transform::default();

        if let Some(img) = paint.image {
            if let Some(texture) = self.textures.get(img) {
                if texture.flags.contains(ImageFlags::FLIPY) {
                    let m1 = Transform::translate(0.0, frag.extent[1] * 0.5) * paint.xform;
                    let m2 = Transform::scale(1.0, -1.0) * m1;
                    let m1 = Transform::translate(0.0, -frag.extent[1] * 0.5) * m2;
                    invxform = m1.inverse();
                } else {
                    invxform = paint.xform.inverse();
                };

                frag.type_ = ShaderType::FillImage as i32;
                match texture.texture_type {
                    TextureType::RGBA => {
                        frag.tex_type = if texture.flags.contains(ImageFlags::PREMULTIPLIED) {
                            0
                        } else {
                            1
                        }
                    }
                    TextureType::Alpha => frag.tex_type = 2,
                }
            }
        } else {
            frag.type_ = ShaderType::FillGradient as i32;
            frag.radius = paint.radius;
            frag.feather = paint.feather;
            invxform = paint.xform.inverse();
        }

        frag.paint_mat = xform_to_3x4(invxform);

        frag
    }

    fn append_uniforms(&mut self, uniforms: FragUniforms) {
        self.uniforms
            .resize(self.uniforms.len() + self.frag_size, 0);

        let idx = self.uniforms.len() - self.frag_size;
        unsafe {
            let p = self.uniforms.as_mut_ptr().add(idx) as *mut FragUniforms;
            *p = uniforms;
        }
    }
}

impl renderer::Renderer for Renderer {
    fn edge_antialias(&self) -> bool {
        true
    }

    fn create_texture(
        &mut self,
        texture_type: TextureType,
        width: usize,
        height: usize,
        flags: ImageFlags,
        data: Option<&[u8]>,
    ) -> anyhow::Result<ImageId> {
        let tex = {
            let tex = gl::gen_texture();
            gl::bind_texture(GL_TEXTURE_2D, tex);
            gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1);

            match texture_type {
                TextureType::RGBA => match data {
                    Some(data) => {
                        gl::tex_image_2d(
                            GL_TEXTURE_2D,
                            0,
                            GL_RGBA as i32,
                            width as i32,
                            height as i32,
                            0,
                            GL_RGBA,
                            GL_UNSIGNED_BYTE,
                            &data,
                        );
                    }
                    None => {
                        gl::empty_tex_image_2d(
                            GL_TEXTURE_2D,
                            0,
                            GL_RGBA as i32,
                            width as i32,
                            height as i32,
                            0,
                            GL_RGBA,
                            GL_UNSIGNED_BYTE,
                        );
                    }
                },
                TextureType::Alpha => match data {
                    Some(data) => {
                        gl::tex_image_2d(
                            GL_TEXTURE_2D,
                            0,
                            GL_R8 as i32,
                            width as i32,
                            height as i32,
                            0,
                            GL_RED,
                            GL_UNSIGNED_BYTE,
                            &data,
                        );
                    }
                    None => {
                        gl::empty_tex_image_2d(
                            GL_TEXTURE_2D,
                            0,
                            GL_R8 as i32,
                            width as i32,
                            height as i32,
                            0,
                            GL_RED,
                            GL_UNSIGNED_BYTE,
                        );
                    }
                },
            }

            if flags.contains(ImageFlags::GENERATE_MIPMAPS) {
                if flags.contains(ImageFlags::NEAREST) {
                    gl::tex_parameteri(
                        GL_TEXTURE_2D,
                        GL_TEXTURE_MIN_FILTER,
                        GL_NEAREST_MIPMAP_NEAREST as i32,
                    );
                } else {
                    gl::tex_parameteri(
                        GL_TEXTURE_2D,
                        GL_TEXTURE_MIN_FILTER,
                        GL_LINEAR_MIPMAP_LINEAR as i32,
                    );
                }
            } else {
                if flags.contains(ImageFlags::NEAREST) {
                    gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as i32);
                } else {
                    gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
                }
            }

            if flags.contains(ImageFlags::NEAREST) {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as i32);
            } else {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
            }

            if flags.contains(ImageFlags::REPEATX) {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as i32);
            } else {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
            }

            if flags.contains(ImageFlags::REPEATY) {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as i32);
            } else {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
            }

            gl::pixel_storei(GL_UNPACK_ALIGNMENT, 4);

            if flags.contains(ImageFlags::GENERATE_MIPMAPS) {
                gl::generate_mipmap(GL_TEXTURE_2D);
            }

            gl::bind_texture(GL_TEXTURE_2D, 0);
            tex
        };

        let id = self.textures.insert(Texture {
            tex,
            width,
            height,
            texture_type,
            flags,
        });
        Ok(id)
    }

    fn delete_texture(&mut self, img: ImageId) -> anyhow::Result<()> {
        if let Some(texture) = self.textures.get(img) {
            gl::delete_textures(&[texture.tex]);
            self.textures.remove(img);
            Ok(())
        } else {
            bail!("texture '{}' not found", img);
        }
    }

    fn update_texture(
        &mut self,
        img: ImageId,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> anyhow::Result<()> {
        if let Some(texture) = self.textures.get(img) {
            gl::bind_texture(GL_TEXTURE_2D, texture.tex);
            gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1);

            match texture.texture_type {
                TextureType::RGBA => gl::tex_sub_image_2d(
                    GL_TEXTURE_2D,
                    0,
                    x as i32,
                    y as i32,
                    width as i32,
                    height as i32,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    data,
                ),
                TextureType::Alpha => gl::tex_sub_image_2d(
                    GL_TEXTURE_2D,
                    0,
                    x as i32,
                    y as i32,
                    width as i32,
                    height as i32,
                    GL_RED,
                    GL_UNSIGNED_BYTE,
                    data,
                ),
            }

            gl::pixel_storei(GL_UNPACK_ALIGNMENT, 4);
            gl::bind_texture(GL_TEXTURE_2D, 0);

            Ok(())
        } else {
            bail!("texture '{}' not found", img);
        }
    }

    fn texture_size(&self, img: ImageId) -> anyhow::Result<(usize, usize)> {
        if let Some(texture) = self.textures.get(img) {
            Ok((texture.width, texture.height))
        } else {
            bail!("texture '{}' not found", img);
        }
    }

    fn viewport(&mut self, extent: Extent, _device_pixel_ratio: f32) -> anyhow::Result<()> {
        self.view = extent;
        Ok(())
    }

    fn cancel(&mut self) -> anyhow::Result<()> {
        self.vertexes.clear();
        self.paths.clear();
        self.calls.clear();
        self.uniforms.clear();
        Ok(())
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        if !self.calls.is_empty() {
            gl::use_program(self.shader.prog);

            gl::enable(GL_CULL_FACE);
            gl::cull_face(GL_BACK);
            gl::front_face(GL_CCW);
            gl::enable(GL_BLEND);
            gl::disable(GL_DEPTH_TEST);
            gl::disable(GL_SCISSOR_TEST);
            gl::color_mask(true, true, true, true);
            gl::stencil_mask(0xffffffff);
            gl::stencil_op(GL_KEEP, GL_KEEP, GL_KEEP);
            gl::stencil_func(GL_ALWAYS, 0, 0xffffffff);
            gl::active_texture(GL_TEXTURE0);
            gl::bind_texture(GL_TEXTURE_2D, 0);

            gl::bind_buffer(GL_UNIFORM_BUFFER, self.frag_buf);
            gl::buffer_data(GL_UNIFORM_BUFFER, self.uniforms.as_slice(), GL_STREAM_DRAW);

            gl::bind_vertex_array(self.vert_arr);
            gl::bind_buffer(GL_ARRAY_BUFFER, self.vert_buf);
            gl::buffer_data(GL_ARRAY_BUFFER, self.vertexes.as_slice(), GL_STREAM_DRAW);
            gl::enable_vertex_attrib_array(0);
            gl::enable_vertex_attrib_array(1);
            gl::vertex_attrib_pointer_offset(
                0,
                2,
                GL_FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                0,
            );
            gl::vertex_attrib_pointer_offset(
                1,
                2,
                GL_FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                2 * std::mem::size_of::<f32>() as u32,
            );

            gl::uniform1i(self.shader.loc_tex, 0);
            let view = [self.view.width, self.view.height];
            gl::uniform2fv(self.shader.loc_viewsize, &view);

            gl::bind_buffer(GL_UNIFORM_BUFFER, self.frag_buf);

            for call in &self.calls {
                let blend = &call.blend_func;

                gl::blend_func_separate(
                    blend.src_rgb,
                    blend.dst_rgb,
                    blend.src_alpha,
                    blend.dst_alpha,
                );

                match call.call_type {
                    CallType::Fill => self.do_fill(&call),
                    CallType::ConvexFill => self.do_convex_fill(&call),
                    CallType::Stroke => self.do_stroke(&call),
                    CallType::Triangles => self.do_triangles(&call),
                }
            }

            gl::disable_vertex_attrib_array(0);
            gl::disable_vertex_attrib_array(1);
            gl::bind_vertex_array(0);
            gl::disable(GL_CULL_FACE);
            gl::bind_buffer(GL_ARRAY_BUFFER, 0);
            gl::use_program(0);
            gl::bind_texture(GL_TEXTURE_2D, 0);
        }

        self.vertexes.clear();
        self.paths.clear();
        self.calls.clear();
        self.uniforms.clear();
        Ok(())
    }

    fn fill(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        bounds: Bounds,
        paths: &[Path],
    ) -> anyhow::Result<()> {
        let mut call = Call {
            call_type: CallType::Fill,
            image: paint.image,
            path_offset: self.paths.len(),
            path_count: paths.len(),
            triangle_offset: 0,
            triangle_count: 4,
            uniform_offset: 0,
            blend_func: composite_operation.into(),
        };

        if paths.len() == 1 && paths[0].convex {
            call.call_type = CallType::ConvexFill;
        }

        let mut offset = self.vertexes.len();
        for path in paths {
            let fill = path.get_fill();
            let mut gl_path = GLPath {
                fill_offset: 0,
                fill_count: 0,
                stroke_offset: 0,
                stroke_count: 0,
            };

            if !fill.is_empty() {
                gl_path.fill_offset = offset;
                gl_path.fill_count = fill.len();
                self.vertexes.extend(fill);
                offset += fill.len();
            }

            let stroke = path.get_stroke();
            if !stroke.is_empty() {
                gl_path.stroke_offset = offset;
                gl_path.stroke_count = stroke.len();
                self.vertexes.extend(stroke);
                offset += stroke.len();
            }

            self.paths.push(gl_path);
        }

        if call.call_type == CallType::Fill {
            call.triangle_offset = offset;
            self.vertexes
                .push(Vertex::new(bounds.max.x, bounds.max.y, 0.5, 1.0));
            self.vertexes
                .push(Vertex::new(bounds.max.x, bounds.min.y, 0.5, 1.0));
            self.vertexes
                .push(Vertex::new(bounds.min.x, bounds.max.y, 0.5, 1.0));
            self.vertexes
                .push(Vertex::new(bounds.min.x, bounds.min.y, 0.5, 1.0));

            call.uniform_offset = self.uniforms.len() / self.frag_size;
            self.append_uniforms(FragUniforms {
                stroke_thr: -1.0,
                type_: ShaderType::Simple as i32,
                ..FragUniforms::default()
            });
            self.append_uniforms(self.convert_paint(paint, scissor, fringe, fringe, -1.0));
        } else {
            call.uniform_offset = self.uniforms.len() / self.frag_size;
            self.append_uniforms(self.convert_paint(paint, scissor, fringe, fringe, -1.0));
        }

        self.calls.push(call);
        Ok(())
    }

    fn stroke(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        stroke_width: f32,
        paths: &[Path],
    ) -> anyhow::Result<()> {
        let mut call = Call {
            call_type: CallType::Stroke,
            image: paint.image,
            path_offset: self.paths.len(),
            path_count: paths.len(),
            triangle_offset: 0,
            triangle_count: 0,
            uniform_offset: 0,
            blend_func: composite_operation.into(),
        };

        let mut offset = self.vertexes.len();
        for path in paths {
            let mut gl_path = GLPath {
                fill_offset: 0,
                fill_count: 0,
                stroke_offset: 0,
                stroke_count: 0,
            };

            let stroke = path.get_stroke();
            if !stroke.is_empty() {
                gl_path.stroke_offset = offset;
                gl_path.stroke_count = stroke.len();
                self.vertexes.extend(stroke);
                offset += stroke.len();
                self.paths.push(gl_path);
            }
        }

        call.uniform_offset = self.uniforms.len() / self.frag_size;
        self.append_uniforms(self.convert_paint(paint, scissor, stroke_width, fringe, -1.0));
        self.append_uniforms(self.convert_paint(
            paint,
            scissor,
            stroke_width,
            fringe,
            1.0 - 0.5 / 255.0,
        ));

        self.calls.push(call);
        Ok(())
    }

    fn triangles(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        vertexes: &[Vertex],
    ) -> anyhow::Result<()> {
        let call = Call {
            call_type: CallType::Triangles,
            image: paint.image,
            path_offset: 0,
            path_count: 0,
            triangle_offset: self.vertexes.len(),
            triangle_count: vertexes.len(),
            uniform_offset: self.uniforms.len() / self.frag_size,
            blend_func: composite_operation.into(),
        };

        self.calls.push(call);
        self.vertexes.extend(vertexes);

        let mut uniforms = self.convert_paint(paint, scissor, 1.0, 1.0, -1.0);
        uniforms.type_ = ShaderType::Image as i32;
        self.append_uniforms(uniforms);
        Ok(())
    }
}

fn shader_error(shader: GLuint, filename: &str) -> anyhow::Error {
    let len = gl::get_shaderiv(shader, GL_INFO_LOG_LENGTH);

    match gl::get_shader_info_log(shader, len) {
        Some(message) => anyhow!("failed to compile shader: {}: {}", filename, message),
        None => anyhow!("failed to compile shader: {}", filename),
    }
}

fn program_error(prog: GLuint) -> anyhow::Error {
    let len = gl::get_programiv(prog, GL_INFO_LOG_LENGTH);

    match gl::get_program_info_log(prog, len) {
        Some(message) => anyhow!("failed to link program: {}", message),
        None => anyhow!("failed to link program"),
    }
}

fn convert_blend_factor(factor: BlendFactor) -> GLenum {
    match factor {
        BlendFactor::Zero => GL_ZERO,
        BlendFactor::One => GL_ONE,
        BlendFactor::SrcColor => GL_SRC_COLOR,
        BlendFactor::OneMinusSrcColor => GL_ONE_MINUS_SRC_COLOR,
        BlendFactor::DstColor => GL_DST_COLOR,
        BlendFactor::OneMinusDstColor => GL_ONE_MINUS_DST_COLOR,
        BlendFactor::SrcAlpha => GL_SRC_ALPHA,
        BlendFactor::OneMinusSrcAlpha => GL_ONE_MINUS_SRC_ALPHA,
        BlendFactor::DstAlpha => GL_DST_ALPHA,
        BlendFactor::OneMinusDstAlpha => GL_ONE_MINUS_DST_ALPHA,
        BlendFactor::SrcAlphaSaturate => GL_SRC_ALPHA_SATURATE,
    }
}

#[inline]
fn premul_color(color: Color) -> Color {
    Color {
        r: color.r * color.a,
        g: color.g * color.a,
        b: color.b * color.a,
        a: color.a,
    }
}

#[inline]
fn xform_to_3x4(xform: Transform) -> [f32; 12] {
    let mut m = [0f32; 12];
    let t = &xform.0;
    m[0] = t[0];
    m[1] = t[1];
    m[2] = 0.0;
    m[3] = 0.0;
    m[4] = t[2];
    m[5] = t[3];
    m[6] = 0.0;
    m[7] = 0.0;
    m[8] = t[4];
    m[9] = t[5];
    m[10] = 1.0;
    m[11] = 0.0;
    m
}
