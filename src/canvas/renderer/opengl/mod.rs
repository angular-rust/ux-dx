use crate::gles::{core30::gl, enums::*};
use fnv::FnvHashMap;
use imgref::ImgVec;
use rgb::{ComponentBytes, RGBA8};
use std::mem;

// #[cfg(not(target_arch = "wasm32"))]
// use std::ffi::c_void;

use crate::canvas::{
    renderer::{ImageId, Vertex},
    BlendFactor, Color, CompositeOperationState, ErrorKind, FillRule, ImageFilter, ImageInfo,
    ImageSource, ImageStore, Scissor,
};

use super::{Command, CommandType, Params, RenderTarget, Renderer, ShaderType};

mod program;
use program::MainProgram;

mod gl_texture;
use gl_texture::GlTexture;

mod framebuffer;
use framebuffer::Framebuffer;

mod uniform_array;
use uniform_array::UniformArray;

pub struct OpenGl {
    debug: bool,
    antialias: bool,
    is_opengles_2_0: bool,
    view: [f32; 2],
    screen_view: [f32; 2],
    main_program: MainProgram,
    vert_arr: Option<u32>,
    vert_buff: Option<u32>,
    framebuffers: FnvHashMap<ImageId, Result<Framebuffer, ErrorKind>>,
    screen_target: Option<Framebuffer>,
    current_render_target: RenderTarget,
}

impl OpenGl {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self, ErrorKind> {
        Self::new_from_context(false)
    }

    // pub fn new<F>(load_fn: F) -> Result<Self, ErrorKind>
    // where
    //     F: FnMut(&str) -> *const c_void,
    // {
    //     Self::new_from_context(false)
    // }

    #[cfg(target_arch = "wasm32")]
    pub fn new_from_html_canvas(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, ErrorKind> {
        let mut attrs = web_sys::WebGlContextAttributes::new();
        attrs.stencil(true);
        attrs.antialias(false);

        use wasm_bindgen::JsCast;
        let webgl1_context = canvas
            .get_context_with_context_options("webgl", attrs.as_ref())
            .map_err(|_| {
                ErrorKind::GeneralError(
                    "Canvas::getContext failed to retrieve WebGL 1 context".to_owned(),
                )
            })?
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();

        Self::new_from_context(true)
    }

    fn new_from_context(is_opengles_2_0: bool) -> Result<Self, ErrorKind> {
        let debug = cfg!(debug_assertions);
        let antialias = true;

        let main_program = MainProgram::new(antialias)?;

        let mut opengl = OpenGl {
            debug: debug,
            antialias: antialias,
            is_opengles_2_0: false,
            view: [0.0, 0.0],
            screen_view: [0.0, 0.0],
            main_program: main_program,
            vert_arr: Default::default(),
            vert_buff: Default::default(),
            framebuffers: Default::default(),
            screen_target: None,
            current_render_target: RenderTarget::Screen,
        };

        opengl.is_opengles_2_0 = is_opengles_2_0;

        opengl.vert_arr = Some(gl::gen_vertex_array());
        opengl.vert_buff = Some(gl::gen_buffer());

        Ok(opengl)
    }

    pub fn is_opengles(&self) -> bool {
        self.is_opengles_2_0
    }

    fn check_error(&self, label: &str) {
        if !self.debug {
            return;
        }

        let err = gl::get_error();

        if err == GL_NO_ERROR {
            return;
        }

        let message = match err {
            GL_INVALID_ENUM => "Invalid enum",
            GL_INVALID_VALUE => "Invalid value",
            GL_INVALID_OPERATION => "Invalid operation",
            GL_OUT_OF_MEMORY => "Out of memory",
            GL_INVALID_FRAMEBUFFER_OPERATION => "Invalid framebuffer operation",
            _ => "Unknown error",
        };

        eprintln!("({}) Error on {} - {}", err, label, message);
    }

    fn gl_factor(factor: BlendFactor) -> u32 {
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

    fn set_composite_operation(&self, blend_state: CompositeOperationState) {
        gl::blend_func_separate(
            Self::gl_factor(blend_state.src_rgb),
            Self::gl_factor(blend_state.dst_rgb),
            Self::gl_factor(blend_state.src_alpha),
            Self::gl_factor(blend_state.dst_alpha),
        );
    }

    fn convex_fill(&self, images: &ImageStore<GlTexture>, cmd: &Command, gpu_paint: &Params) {
        self.set_uniforms(images, gpu_paint, cmd.image, cmd.alpha_mask);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                gl::draw_arrays(GL_TRIANGLE_FAN, start as i32, count as i32);
            }

            if let Some((start, count)) = drawable.stroke_verts {
                gl::draw_arrays(GL_TRIANGLE_STRIP, start as i32, count as i32);
            }
        }

        self.check_error("convex_fill");
    }

    fn concave_fill(
        &self,
        images: &ImageStore<GlTexture>,
        cmd: &Command,
        stencil_paint: &Params,
        fill_paint: &Params,
    ) {
        gl::enable(GL_STENCIL_TEST);
        gl::stencil_mask(0xff);
        gl::stencil_func(GL_ALWAYS, 0, 0xff);
        gl::color_mask(false, false, false, false);
        //gl::depth_mask(GL_FALSE);

        self.set_uniforms(images, stencil_paint, None, None);

        gl::stencil_op_separate(GL_FRONT, GL_KEEP, GL_KEEP, GL_INCR_WRAP);
        gl::stencil_op_separate(GL_BACK, GL_KEEP, GL_KEEP, GL_DECR_WRAP);
        gl::disable(GL_CULL_FACE);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                gl::draw_arrays(GL_TRIANGLE_FAN, start as i32, count as i32);
            }
        }

        gl::enable(GL_CULL_FACE);
        // Draw anti-aliased pixels
        gl::color_mask(true, true, true, true);
        //gl::DepthMask(GL_TRUE);

        self.set_uniforms(images, fill_paint, cmd.image, cmd.alpha_mask);

        if self.antialias {
            match cmd.fill_rule {
                FillRule::NonZero => gl::stencil_func(GL_EQUAL, 0x0, 0xff),
                FillRule::EvenOdd => gl::stencil_func(GL_EQUAL, 0x0, 0x1),
            }

            gl::stencil_op(GL_KEEP, GL_KEEP, GL_KEEP);

            // draw fringes
            for drawable in &cmd.drawables {
                if let Some((start, count)) = drawable.stroke_verts {
                    gl::draw_arrays(GL_TRIANGLE_STRIP, start as i32, count as i32);
                }
            }
        }

        match cmd.fill_rule {
            FillRule::NonZero => gl::stencil_func(GL_NOTEQUAL, 0x0, 0xff),
            FillRule::EvenOdd => gl::stencil_func(GL_NOTEQUAL, 0x0, 0x1),
        }

        gl::stencil_op(GL_ZERO, GL_ZERO, GL_ZERO);

        if let Some((start, count)) = cmd.triangles_verts {
            gl::draw_arrays(GL_TRIANGLE_STRIP, start as i32, count as i32);
        }

        gl::disable(GL_STENCIL_TEST);

        self.check_error("concave_fill");
    }

    fn stroke(&self, images: &ImageStore<GlTexture>, cmd: &Command, paint: &Params) {
        self.set_uniforms(images, paint, cmd.image, cmd.alpha_mask);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                gl::draw_arrays(GL_TRIANGLE_STRIP, start as i32, count as i32);
            }
        }

        self.check_error("stroke");
    }

    fn stencil_stroke(
        &self,
        images: &ImageStore<GlTexture>,
        cmd: &Command,
        paint1: &Params,
        paint2: &Params,
    ) {
        gl::enable(GL_STENCIL_TEST);
        gl::stencil_mask(0xff);

        // Fill the stroke base without overlap
        gl::stencil_func(GL_EQUAL, 0x0, 0xff);
        gl::stencil_op(GL_KEEP, GL_KEEP, GL_INCR);

        self.set_uniforms(images, paint2, cmd.image, cmd.alpha_mask);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                gl::draw_arrays(GL_TRIANGLE_STRIP, start as i32, count as i32);
            }
        }

        // Draw anti-aliased pixels.
        self.set_uniforms(images, paint1, cmd.image, cmd.alpha_mask);

        gl::stencil_func(GL_EQUAL, 0x0, 0xff);
        gl::stencil_op(GL_KEEP, GL_KEEP, GL_KEEP);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                gl::draw_arrays(GL_TRIANGLE_STRIP, start as i32, count as i32);
            }
        }

        // Clear stencil buffer.
        gl::color_mask(false, false, false, false);
        gl::stencil_func(GL_ALWAYS, 0x0, 0xff);
        gl::stencil_op(GL_ZERO, GL_ZERO, GL_ZERO);

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.stroke_verts {
                gl::draw_arrays(GL_TRIANGLE_STRIP, start as i32, count as i32);
            }
        }

        gl::color_mask(true, true, true, true);
        gl::disable(GL_STENCIL_TEST);

        self.check_error("stencil_stroke");
    }

    fn triangles(&self, images: &ImageStore<GlTexture>, cmd: &Command, paint: &Params) {
        self.set_uniforms(images, paint, cmd.image, cmd.alpha_mask);

        if let Some((start, count)) = cmd.triangles_verts {
            gl::draw_arrays(GL_TRIANGLES, start as i32, count as i32);
        }

        self.check_error("triangles");
    }

    fn set_uniforms(
        &self,
        images: &ImageStore<GlTexture>,
        paint: &Params,
        image_tex: Option<ImageId>,
        alpha_tex: Option<ImageId>,
    ) {
        let arr = UniformArray::from(paint);
        self.main_program.set_config(arr.as_slice());
        self.check_error("set_uniforms uniforms");

        let tex = image_tex
            .and_then(|id| images.get(id))
            .map_or(0, |tex| tex.id());

        gl::active_texture(GL_TEXTURE0);
        gl::bind_texture(GL_TEXTURE_2D, tex);

        let masktex = alpha_tex
            .and_then(|id| images.get(id))
            .map_or(0, |tex| tex.id());

        gl::active_texture(GL_TEXTURE0 + 1);
        gl::bind_texture(GL_TEXTURE_2D, masktex);

        self.check_error("set_uniforms texture");
    }

    fn clear_rect(&self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        gl::enable(GL_SCISSOR_TEST);
        gl::scissor(
            x as i32,
            self.view[1] as i32 - (height as i32 + y as i32),
            width as i32,
            height as i32,
        );
        gl::clear_color(color.r, color.g, color.b, color.a);
        gl::clear(GL_COLOR_BUFFER_BIT | GL_STENCIL_BUFFER_BIT);
        gl::disable(GL_SCISSOR_TEST);
    }

    fn set_target(&mut self, images: &ImageStore<GlTexture>, target: RenderTarget) {
        self.current_render_target = target;
        match (target, &self.screen_target) {
            (RenderTarget::Screen, None) => {
                Framebuffer::unbind();
                self.view = self.screen_view;
                gl::viewport(0, 0, self.view[0] as i32, self.view[1] as i32);
            }
            (RenderTarget::Screen, Some(framebuffer)) => {
                framebuffer.bind();
                self.view = self.screen_view;
                gl::viewport(0, 0, self.view[0] as i32, self.view[1] as i32);
            }
            (RenderTarget::Image(id), _) => {
                if let Some(texture) = images.get(id) {
                    if let Ok(fb) = self
                        .framebuffers
                        .entry(id)
                        .or_insert_with(|| Framebuffer::new(texture))
                    {
                        fb.bind();

                        self.view[0] = texture.info().width() as f32;
                        self.view[1] = texture.info().height() as f32;

                        gl::viewport(
                            0,
                            0,
                            texture.info().width() as i32,
                            texture.info().height() as i32,
                        );
                    }
                }
            }
        }
    }

    /// Make the "Screen" RenderTarget actually render to a framebuffer object. This is useful when
    /// embedding femtovg into another program where final composition is handled by an external task.
    /// The given `framebuffer_object` must refer to a Framebuffer Object created on the current OpenGL
    /// Context, and must have a depth & stencil attachment.
    ///
    /// Pass `None` to clear any previous Framebuffer Object ID that was passed and target rendering to
    /// the default target (normally the window).
    pub fn set_screen_target(&mut self, framebuffer_object: Option<Framebuffer>) {
        match framebuffer_object {
            Some(fbo) => self.screen_target = Some(Framebuffer::from_external(fbo)),
            None => self.screen_target = None,
        }
    }

    fn render_filtered_image(
        &mut self,
        images: &mut ImageStore<GlTexture>,
        cmd: Command,
        target_image: ImageId,
        filter: ImageFilter,
    ) {
        match filter {
            ImageFilter::GaussianBlur { sigma } => {
                self.render_gaussian_blur(images, cmd, target_image, sigma)
            }
        }
    }

    fn render_gaussian_blur(
        &mut self,
        images: &mut ImageStore<GlTexture>,
        mut cmd: Command,
        target_image: ImageId,
        sigma: f32,
    ) {
        let original_render_target = self.current_render_target;

        // The filtering happens in two passes, first a horizontal blur and then the vertial blur. The
        // first pass therefore renders into an intermediate, temporarily allocated texture.

        let source_image_info = images.get(cmd.image.unwrap()).unwrap().info();

        let image_paint = crate::canvas::Paint::image(
            cmd.image.unwrap(),
            0.,
            0.,
            source_image_info.width() as _,
            source_image_info.height() as _,
            0.,
            1.,
        );
        let mut blur_params = Params::new(images, &image_paint, &Scissor::default(), 0., 0., 0.);
        blur_params.shader_type = ShaderType::FilterImage.to_f32();

        let gauss_coeff_x = 1. / ((2. * std::f32::consts::PI).sqrt() * sigma);
        let gauss_coeff_y = f32::exp(-0.5 / (sigma * sigma));
        let gauss_coeff_z = gauss_coeff_y * gauss_coeff_y;

        blur_params.image_blur_filter_coeff[0] = gauss_coeff_x;
        blur_params.image_blur_filter_coeff[1] = gauss_coeff_y;
        blur_params.image_blur_filter_coeff[2] = gauss_coeff_z;

        blur_params.image_blur_filter_direction = [1.0, 0.0];

        // GLES 2.0 does not allow non-constant loop indices, so limit the standard devitation to allow for a upper fixed limit
        // on the number of iterations in the fragment shader.
        blur_params.image_blur_filter_sigma = sigma.min(8.);

        let horizontal_blur_buffer = images.alloc(self, source_image_info).unwrap();
        self.set_target(images, RenderTarget::Image(horizontal_blur_buffer));
        self.main_program.set_view(self.view);

        self.clear_rect(
            0,
            0,
            source_image_info.width() as _,
            source_image_info.height() as _,
            Color::rgbaf(0., 0., 0., 0.),
        );

        self.triangles(images, &cmd, &blur_params);

        self.set_target(images, RenderTarget::Image(target_image));
        self.main_program.set_view(self.view);

        self.clear_rect(
            0,
            0,
            source_image_info.width() as _,
            source_image_info.height() as _,
            Color::rgbaf(0., 0., 0., 0.),
        );

        blur_params.image_blur_filter_direction = [0.0, 1.0];

        cmd.image = Some(horizontal_blur_buffer);

        self.triangles(images, &cmd, &blur_params);

        images.remove(self, horizontal_blur_buffer);

        // restore previous render target and view
        self.set_target(images, original_render_target);
        self.main_program.set_view(self.view);
    }
}

impl Renderer for OpenGl {
    type Image = GlTexture;

    fn set_size(&mut self, width: u32, height: u32, _dpi: f32) {
        self.view[0] = width as f32;
        self.view[1] = height as f32;

        self.screen_view = self.view;

        gl::viewport(0, 0, width as i32, height as i32);
    }

    fn render(
        &mut self,
        images: &mut ImageStore<Self::Image>,
        verts: &[Vertex],
        commands: Vec<Command>,
    ) {
        self.main_program.bind();

        gl::enable(GL_CULL_FACE);

        gl::cull_face(GL_BACK);
        gl::front_face(GL_CCW);
        gl::enable(GL_BLEND);
        gl::disable(GL_DEPTH_TEST);
        gl::disable(GL_SCISSOR_TEST);
        gl::color_mask(true, true, true, true);
        gl::stencil_mask(0xffff_ffff);
        gl::stencil_op(GL_KEEP, GL_KEEP, GL_KEEP);
        gl::stencil_func(GL_ALWAYS, 0, 0xffff_ffff);
        gl::active_texture(GL_TEXTURE0);
        // unbind texture
        gl::bind_texture(GL_TEXTURE_2D, 0);
        gl::active_texture(GL_TEXTURE0 + 1);
        // unbind texture
        gl::bind_texture(GL_TEXTURE_2D, 0);

        match self.vert_arr {
            Some(vert_arr) => gl::bind_vertex_array(vert_arr),
            None => gl::bind_vertex_array(0),
        }

        let vertex_size = mem::size_of::<Vertex>();

        match self.vert_buff {
            Some(vert_buff) => gl::bind_buffer(GL_ARRAY_BUFFER, vert_buff),
            None => gl::bind_buffer(GL_ARRAY_BUFFER, 0),
        }

        gl::buffer_data(GL_ARRAY_BUFFER, verts, GL_STREAM_DRAW);

        gl::enable_vertex_attrib_array(0);
        gl::enable_vertex_attrib_array(1);

        gl::vertex_attrib_pointer_offset(0, 2, GL_FLOAT, false, vertex_size as i32, 0);
        gl::vertex_attrib_pointer_offset(
            1,
            2,
            GL_FLOAT,
            false,
            vertex_size as i32,
            2 * mem::size_of::<f32>() as u32,
        );

        // Bind the two uniform samplers to texture units
        self.main_program.set_tex(0);
        self.main_program.set_masktex(1);

        self.check_error("render prepare");

        for cmd in commands.into_iter() {
            self.set_composite_operation(cmd.composite_operation);

            match cmd.cmd_type {
                CommandType::ConvexFill { ref params } => self.convex_fill(images, &cmd, params),
                CommandType::ConcaveFill {
                    ref stencil_params,
                    ref fill_params,
                } => self.concave_fill(images, &cmd, stencil_params, fill_params),
                CommandType::Stroke { ref params } => self.stroke(images, &cmd, params),
                CommandType::StencilStroke {
                    ref params1,
                    ref params2,
                } => self.stencil_stroke(images, &cmd, params1, params2),
                CommandType::Triangles { ref params } => self.triangles(images, &cmd, params),
                CommandType::ClearRect {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    self.clear_rect(x, y, width, height, color);
                }
                CommandType::SetRenderTarget(target) => {
                    self.set_target(images, target);
                    self.main_program.set_view(self.view);
                }
                CommandType::RenderFilteredImage {
                    target_image,
                    filter,
                } => self.render_filtered_image(images, cmd, target_image, filter),
            }
        }

        gl::disable_vertex_attrib_array(0);
        gl::disable_vertex_attrib_array(1);

        // unbind vao
        gl::bind_vertex_array(0);

        gl::disable(GL_CULL_FACE);

        // unbind vbo
        gl::bind_buffer(GL_ARRAY_BUFFER, 0);

        // unbind texture
        gl::bind_texture(GL_TEXTURE_2D, 0);

        self.main_program.unbind();

        self.check_error("render done");
    }

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind> {
        Self::Image::new(info, self.is_opengles_2_0)
    }

    fn update_image(
        &mut self,
        image: &mut Self::Image,
        data: ImageSource,
        x: usize,
        y: usize,
    ) -> Result<(), ErrorKind> {
        image.update(data, x, y, self.is_opengles_2_0)
    }

    fn delete_image(&mut self, image: Self::Image, image_id: ImageId) {
        self.framebuffers.remove(&image_id);
        image.delete();
    }

    fn screenshot(&mut self) -> Result<ImgVec<RGBA8>, ErrorKind> {
        //let mut image = image::RgbaImage::new(self.view[0] as u32, self.view[1] as u32);
        let w = self.view[0] as usize;
        let h = self.view[1] as usize;

        let mut image = ImgVec::new(
            vec![
                RGBA8 {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255
                };
                w * h
            ],
            w,
            h,
        );

        let buf = image.buf_mut().as_bytes_mut();
        gl::read_pixels(
            0,
            0,
            self.view[0] as i32,
            self.view[1] as i32,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            buf,
        );

        let mut flipped = Vec::with_capacity(w * h);

        for row in image.rows().rev() {
            flipped.extend_from_slice(row);
        }

        Ok(ImgVec::new(flipped, w, h))
    }
}

impl Drop for OpenGl {
    fn drop(&mut self) {
        if let Some(vert_arr) = self.vert_arr {
            gl::delete_vertex_arrays(&[vert_arr]);
        }

        if let Some(vert_buff) = self.vert_buff {
            gl::delete_buffers(&[vert_buff]);
        }
    }
}
