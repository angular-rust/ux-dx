//! Context

#![allow(unused_imports)]

use cgmath::prelude::*;
use cgmath::{Deg, Matrix3, Matrix4, Vector2, Vector3};
use std::{cell::RefCell, rc::Rc};
use winit::dpi::PhysicalSize;

use crate::prelude::color;

use crate::{foundation::colorspace::Color, platform::gles::enums::*};

use super::*;

mod coloredpainter;
use self::coloredpainter::*;

mod gles_extensions;
pub use self::gles_extensions::*;

mod imagepainter;
use self::imagepainter::*;

mod image;
pub use self::image::*;

mod pipeline;
pub use self::pipeline::*;

mod textpainter;
pub use self::textpainter::*;

mod webgl_extensions;
pub use self::webgl_extensions::*;

pub enum ImageScaleQuality {
    HIGH,
    LOW,
}

pub struct Video;

pub struct Canvas;

pub struct IndexBuffer; // Vec<u32>

pub struct VertexBuffer;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TextureUnit;

pub struct TextureAddressing;

#[derive(Default)]
pub struct CubeMap;

pub struct MipMapFilter;

pub struct TextureFilter;

pub struct DepthStencilFormat;

// An EBO (Element Buffer Object) is a buffer, just like a vertex buffer object (VBO),
// that stores indices that OpenGL uses to decide what vertices to draw.
// This so called indexed drawing is exactly the solution to our problem.
// We first have to specify the (unique) vertices and the indices to draw
// them as a rectangle.
// A Vertex Array Object (VAO) is an OpenGL Object that stores all of
// the state needed to supply vertex data (with one minor exception noted below).
// It stores the format of the vertex data as well as the Buffer Objects
// providing the vertex data arrays.

struct PainterProps {
    color: Color,
    font: Font,
    projection_matrix: Matrix4<f32>,

    image_painter: ImageShaderPainter,
    colored_painter: ColoredShaderPainter,
    text_painter: TextShaderPainter,

    // static var videoPipeline: PipelineState,
    canvas: PhysicalSize<u32>,

    transformations: Vec<cgmath::Matrix3<f32>>,
    transformation_index: usize,
    opacities: Vec<f32>,
    fontsize: f32,
}

impl PainterProps {
    pub fn new() -> Self {
        let fontsize = 18.0;

        let mut text_painter = TextShaderPainter::new();
        text_painter.fontsize = fontsize;

        let instance = Self {
            color: color::WHITE,
            font: Font {},
            projection_matrix: Matrix4::identity(),
            transformations: vec![Matrix3::identity()],
            transformation_index: 0,
            image_painter: ImageShaderPainter::new(),
            colored_painter: ColoredShaderPainter::new(),
            text_painter,
            opacities: vec![1.0],
            fontsize,
            canvas: PhysicalSize::new(900, 700),
        };

        // #if stage4
        // pipe = None;
        // #end

        // if videoPipeline == None {
        // 	videoPipeline = createImagePipeline(createImageVertexStructure());
        // 	videoPipeline.fragmentShader = Shaders.painter_video_frag;
        // 	videoPipeline.vertexShader = Shaders.painter_video_vert;
        // 	videoPipeline.compile();
        // }
        instance
    }
}

pub struct Painter {
    props: RefCell<PainterProps>,
}

impl Default for Painter {
    fn default() -> Self {
        Painter::new()
    }
}

// static current: Option<Graphics2> = None;

impl Painter {
    pub fn new() -> Self {
        let instance = Self {
            props: RefCell::new(PainterProps::new()),
        };

        // instance.set_projection();
        instance
    }

    pub fn begin(&self, clear: bool /* = true*/, clear_color: Option<Color> /* = None*/) {
        // if current.is_none() {
        // 	current = self;
        // } else {
        // 	panic!("End before you begin");
        // }

        // self.g.begin();
        if clear {
            self.clear(clear_color);
        }
        // self.set_projection();
    }

    pub fn end(&self) {
        self.flush();
        // self.g.end();

        // if current == this {
        // 	current = None;
        // } else {
        // 	panic!("Begin before you end");
        // }
    }

    pub fn flush(&self) {
        let mut props = self.props.borrow_mut();

        props.image_painter.end();
        props.text_painter.end();
        props.colored_painter.end();
    }

    // scale-filtering
    // draw/fillPolygon

    pub fn clear(&self, color: Option<Color> /* = None*/) {
        self.flush();
        // let color = match color {
        // 	Some(color) => color,
        // 	None => Color::Black,
        // };

        // self.g.clear(color);
    }

    pub fn draw_image(&self, img: &Image, x: f32, y: f32) {
        self.draw_subimage(img, x, y, 0.0, 0.0, img.width as f32, img.height as f32);
    }

    pub fn draw_subimage(&self, img: &Image, x: f32, y: f32, sx: f32, sy: f32, sw: f32, sh: f32) {
        self.draw_scaled_subimage(img, sx, sy, sw, sh, x, y, sw, sh);
    }

    pub fn draw_scaled_image(&self, img: &Image, dx: f32, dy: f32, dw: f32, dh: f32) {
        self.draw_scaled_subimage(
            img,
            0.0,
            0.0,
            img.width as f32,
            img.height as f32,
            dx,
            dy,
            dw,
            dh,
        );
    }

    pub fn draw_scaled_subimage(
        &self,
        image: &Image,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
        dx: f32,
        dy: f32,
        dw: f32,
        dh: f32,
    ) {
        let trans_mat = self.transformation();
        let opacity = self.opacity();
        let tint = self.color();

        let mut props = self.props.borrow_mut();

        props.colored_painter.end();
        props.text_painter.end();

        // Z act as scale factor
        let p1 = trans_mat * Vector3::new(dx, dy + dh, 1_f32);
        let p2 = trans_mat * Vector3::new(dx, dy, 1_f32);
        let p3 = trans_mat * Vector3::new(dx + dw, dy, 1_f32);
        let p4 = trans_mat * Vector3::new(dx + dw, dy + dh, 1_f32);

        props.image_painter.draw_image2(
            image, sx, sy, sw, sh, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y, opacity, tint,
        );
    }

    pub fn draw_rect(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        strength: f32, /* = 1.0*/
    ) {
        let trans_mat = self.transformation();
        let opacity = self.opacity();
        let color = self.color();

        let mut props = self.props.borrow_mut();

        props.image_painter.end();
        props.text_painter.end();

        let mut p1 = trans_mat * Vector3::new(x - strength / 2.0, y + strength / 2.0, 1_f32); // bottom-left
        let mut p2 = trans_mat * Vector3::new(x - strength / 2.0, y - strength / 2.0, 1_f32); // top-left
        let mut p3 =
            trans_mat * Vector3::new(x + width + strength / 2.0, y - strength / 2.0, 1_f32); // top-right
        let mut p4 =
            trans_mat * Vector3::new(x + width + strength / 2.0, y + strength / 2.0, 1_f32); // bottom-right

        props.colored_painter.fill_rect(
            opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y,
        ); // top

        p1 = trans_mat * Vector3::new(x - strength / 2.0, y + height - strength / 2.0, 1_f32);
        p2 = trans_mat * Vector3::new(x - strength / 2.0, y + strength / 2.0, 1_f32);
        p3 = trans_mat * Vector3::new(x + strength / 2.0, y + strength / 2.0, 1_f32);
        p4 = trans_mat * Vector3::new(x + strength / 2.0, y + height - strength / 2.0, 1_f32);

        props.colored_painter.fill_rect(
            opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y,
        ); // left

        p1 = trans_mat * Vector3::new(x - strength / 2.0, y + height + strength / 2.0, 1_f32);
        p2 = trans_mat * Vector3::new(x - strength / 2.0, y + height - strength / 2.0, 1_f32);
        p3 = trans_mat
            * Vector3::new(
                x + width + strength / 2.0,
                y + height - strength / 2.0,
                1_f32,
            );
        p4 = trans_mat
            * Vector3::new(
                x + width + strength / 2.0,
                y + height + strength / 2.0,
                1_f32,
            );

        props.colored_painter.fill_rect(
            opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y,
        ); // bottom

        p1 = trans_mat
            * Vector3::new(
                x + width - strength / 2.0,
                y + height - strength / 2.0,
                1_f32,
            );
        p2 = trans_mat * Vector3::new(x + width - strength / 2.0, y + strength / 2.0, 1_f32);
        p3 = trans_mat * Vector3::new(x + width + strength / 2.0, y + strength / 2.0, 1_f32);
        p4 = trans_mat
            * Vector3::new(
                x + width + strength / 2.0,
                y + height - strength / 2.0,
                1_f32,
            );

        props.colored_painter.fill_rect(
            opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y,
        ); // right
    }

    pub fn fill_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        let trans_mat = self.transformation();
        let color = self.color();
        let opacity = self.opacity();

        let mut props = self.props.borrow_mut();

        props.image_painter.end();
        props.text_painter.end();

        let p1 = trans_mat * Vector3::new(x, y + height, 1_f32);
        let p2 = trans_mat * Vector3::new(x, y, 1_f32);
        let p3 = trans_mat * Vector3::new(x + width, y, 1_f32);
        let p4 = trans_mat * Vector3::new(x + width, y + height, 1_f32);

        props.colored_painter.fill_rect(
            opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y,
        );
    }

    pub fn measure(&self, text: &str) -> Option<(f32, f32)> {
        let mut props = self.props.borrow_mut();
        props.text_painter.measure(text)
    }
    // Draw a single line of text with the current `color`, `font` and `fontSize` properties.
    //
    // When drawing into rendertargets, you might have to use a different shader than the default one
    //  - use the default shader when drawing into a transparent section of your rendertarget
    //  - use a shader with `alphaBlendSource = BlendOne` when drawing into a non-transparent section of your rendertarget
    pub fn draw_string(&self, text: &str, x: f32, y: f32) {
        let trans_mat = self.transformation();

        let mut props = self.props.borrow_mut();

        props.image_painter.end();
        props.colored_painter.end();

        let opacity = match props.opacities.last() {
            Some(opacity) => *opacity,
            None => 1.0,
        };

        let color = props.color;

        let p1 = trans_mat * Vector3::new(x, y, 1.0_f32);

        props
            .text_painter
            .draw_string(text, p1.x, p1.y, opacity, color);
    }

    // Draw a single line of characters with the current `color`, `font` and `fontSize` properties.
    //
    // When drawing into rendertargets, you might have to use a different shader than the default one
    //  - use the default shader when drawing into a transparent section of your rendertarget
    //  - use a shader with `alphaBlendSource = BlendOne` when drawing into a non-transparent section of your rendertarget
    pub fn draw_characters(&self, text: Vec<i32>, start: i32, length: i32, x: f32, y: f32) {
        let mut props = self.props.borrow_mut();

        props.image_painter.end();
        props.colored_painter.end();

        let color = self.color();
        let opacity = self.opacity();
        let transformation = self.transformation();

        props.text_painter.draw_characters(
            text,
            start,
            length,
            opacity,
            color,
            x,
            y,
            transformation,
        );
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, strength: f32 /*= 1.0*/) {
        let trans_mat = self.transformation();
        let opacity = self.opacity();
        let color = self.color();

        let mut props = self.props.borrow_mut();

        props.image_painter.end();
        props.text_painter.end();

        let mut vec = if y2 == y1 {
            Vector3::new(0.0, -1.0, 1_f32)
        } else {
            Vector3::new(1.0, -(x2 - x1) / (y2 - y1), 1_f32)
        };

        vec = vec * strength;

        let mut p1 = Vector3::new(x1 + 0.5 * vec.x, y1 + 0.5 * vec.y, 1_f32);
        let mut p2 = Vector3::new(x2 + 0.5 * vec.x, y2 + 0.5 * vec.y, 1_f32);
        let mut p3 = Vector3::new(p1.x - vec.x, p1.y - vec.y, 1_f32);
        let mut p4 = Vector3::new(p2.x - vec.x, p2.y - vec.y, 1_f32);

        p1 = trans_mat * p1;
        p2 = trans_mat * p2;
        p3 = trans_mat * p3;
        p4 = trans_mat * p4;

        props
            .colored_painter
            .fill_triangle(opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);

        props
            .colored_painter
            .fill_triangle(opacity, color, p3.x, p3.y, p2.x, p2.y, p4.x, p4.y);
    }

    pub fn draw_video(&self, video: Video, x: f32, y: f32, width: f32, height: f32) {
        // self.setPipeline(videoPipeline);
        // self.drawVideoInternal(video, x, y, width, height);
        // self.setPipeline(None);
        unimplemented!()
    }

    pub fn fill_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        // prepare data to pretend `already mutable borrowed`
        let opacity = self.opacity();
        let color = self.color();

        let mut props = self.props.borrow_mut();

        if props.colored_painter.quad_count > 0 {
            props.colored_painter.draw_buffer(true); // Flush other buffer for right render order
        }

        if props.colored_painter.triangle_count + 1 >= props.colored_painter.triangle_count {
            props.colored_painter.draw_tri_buffer(false);
        }

        props.colored_painter.set_tri_colors(opacity, color);
        props
            .colored_painter
            .set_tri_vertices(x1, y1, x2, y2, x3, y3);
        props.colored_painter.triangle_count += 1;
    }

    // fn get_image_scale_quality(&self) -> ImageScaleQuality {
    // 	return myImageScaleQuality;
    // }

    // fn set_image_scale_quality(&self, value: ImageScaleQuality) -> ImageScaleQuality {
    // 	if value == myImageScaleQuality {
    // 		return value;
    // 	}
    // 	self.image_painter.set_bilinear_filter(value == ImageScaleQuality::HIGH);
    // 	self.text_painter.set_bilinear_filter(value == ImageScaleQuality::HIGH);
    // 	return myImageScaleQuality = value;
    // }

    // fn get_mipmap_scale_quality(&self) -> ImageScaleQuality {
    // 	self.myMipmapScaleQuality
    // }

    // fn set_mipmap_scale_quality(&self, value: ImageScaleQuality) -> ImageScaleQuality {
    // 	self.image_painter.set_bilinear_mipmap_filter(value == ImageScaleQuality::HIGH);
    // 	// text_painter.set_bilinear_mipmap_filter(value == ImageScaleQuality::HIGH); // TODO (DK) implement for fonts as well?
    // 	return myMipmapScaleQuality = value;
    // }

    // The color value is used for geometric primitives, images, and text. Remember to set it back to white to draw images unaltered.

    pub fn color(&self) -> Color {
        let props = self.props.borrow();
        props.color
    }

    pub fn set_color(&self, color: Color) {
        let mut props = self.props.borrow_mut();
        props.color = color;
    }

    pub fn font(&self) -> Font {
        let props = self.props.borrow();
        props.font
    }

    pub fn set_font(&self, font: Font) {
        let mut props = self.props.borrow_mut();
        props.text_painter.set_font(font);
        props.font = font;
    }

    pub fn font_size(&self) -> f32 {
        let props = self.props.borrow();
        props.fontsize
    }

    pub fn set_font_size(&self, value: f32) {
        let mut props = self.props.borrow_mut();
        props.fontsize = value;
        props.text_painter.fontsize = value;
    }

    // public static var fontGlyphs: Vec<i32> = [for (i in 32...256) i];

    // public var transformation(get, set): cgmath::Matrix3<f32>; // works on the top of the transformation stack

    #[inline]
    pub fn transformation(&self) -> cgmath::Matrix3<f32> {
        let props = self.props.borrow();
        let index = props.transformation_index;
        props.transformations[index]
    }

    #[inline]
    fn set_transformation(&self, transformation: cgmath::Matrix3<f32>) -> cgmath::Matrix3<f32> {
        let mut props = self.props.borrow_mut();
        let index = props.transformation_index;
        props.transformations[index] = transformation;
        transformation
    }

    #[inline]
    pub fn push_transformation(&self, trans: cgmath::Matrix3<f32>) {
        let mut props = self.props.borrow_mut();

        let index = props.transformation_index + 1;
        if index == props.transformations.len() {
            props.transformations.push(trans);
        } else {
            props.transformations[index] = trans;
        }
        props.transformation_index = index;
    }

    pub fn pop_transformation(&self) -> cgmath::Matrix3<f32> {
        let mut props = self.props.borrow_mut();
        let index = props.transformation_index;
        props.transformation_index -= 1;
        props.transformations[index]
    }

    pub fn scale(&self, x: f32, y: f32) -> cgmath::Matrix3<f32> {
        self.set_transformation(Matrix3::from_nonuniform_scale(x, y) * self.transformation())
    }

    #[inline]
    fn translation(&self, tx: f32, ty: f32) -> cgmath::Matrix3<f32> {
        Matrix3::from_translation(Vector2::new(tx, ty)) * self.transformation()
    }

    pub fn translate(&self, tx: f32, ty: f32) -> cgmath::Matrix3<f32> {
        self.set_transformation(self.translation(tx, ty))
    }

    pub fn push_translation(&self, tx: f32, ty: f32) {
        self.push_transformation(self.translation(tx, ty));
    }

    #[inline]
    fn rotation(&self, angle: f32, centerx: f32, centery: f32) -> cgmath::Matrix3<f32> {
        Matrix3::from_translation(Vector2::new(centerx, centery))
            * Matrix3::from_angle_z(Deg(angle))
            * Matrix3::from_translation(Vector2::new(-centerx, -centery))
            * self.transformation()
    }

    pub fn rotate(&self, angle: f32, centerx: f32, centery: f32) -> cgmath::Matrix3<f32> {
        self.set_transformation(self.rotation(angle, centerx, centery))
    }

    pub fn push_rotation(&self, angle: f32, centerx: f32, centery: f32) {
        self.set_transformation(self.rotation(angle, centerx, centery));
    }

    // // public var opacity(get, set): f32; // works on the top of the opacity stack

    pub fn push_opacity(&self, opacity: f32) {
        self.set_opacity(opacity);

        let mut props = self.props.borrow_mut();
        props.opacities.push(opacity);
    }

    pub fn pop_opacity(&self) -> f32 {
        let mut props = self.props.borrow_mut();
        props.opacities.pop().unwrap()
    }

    pub fn opacity(&self) -> f32 {
        let props = self.props.borrow();
        let index = props.opacities.len() - 1;
        props.opacities[index]
    }

    pub fn set_opacity(&self, opacity: f32) {
        let mut props = self.props.borrow_mut();
        let index = props.opacities.len() - 1;
        props.opacities[index] = opacity;
    }

    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {}

    pub fn disable_scissor(&self) {}

    // #if stage4
    // var pipe: PipelineState;

    // public var pipeline(get, set): PipelineState;

    // fn get_pipeline(): PipelineState {
    // 	return pipe;
    // }

    // fn set_pipeline(pipeline: PipelineState): PipelineState {
    // 	setPipeline(pipeline);
    // 	return pipe = pipeline;
    // }
    // #end

    // fn set_transformation(transformation: cgmath::Matrix3<f32>) {}

    // fn set_opacity(opacity: f32) {}

    fn set_pipeline(&self, pipeline: Pipeline) {}

    pub fn set_projection(&self, projection_matrix: cgmath::Matrix4<f32>) {
        let mut props = self.props.borrow_mut();

        // let width = props.canvas.width as f32;
        // let height = props.canvas.height as f32;

        // // inverted y
        // let projection_matrix: Matrix4<f32> = cgmath::ortho(0.0, width, 0.0, height, 0.0, 1000.0);

        // if Std.isOfType(canvas, Framebuffer) {
        // 	projectionMatrix.setFrom(Matrix4.orthogonalProjection(0, width, height, 0, 0.1, 1000));
        // } else {
        // 	if !Image.nonPow2Supported {
        // 		width = upperPowerOfTwo(width);
        // 		height = upperPowerOfTwo(height);
        // 	}
        // 	if Image.renderTargetsInvertedY() {
        // 		props.projectionMatrix.setFrom(Matrix4.orthogonalProjection(0, width, 0, height, 0.1, 1000));
        // 	} else {
        // 		props.projectionMatrix.setFrom(Matrix4.orthogonalProjection(0, width, height, 0, 0.1, 1000));
        // 	}
        // }

        props.projection_matrix = projection_matrix;

        props.image_painter.set_projection(projection_matrix);
        props.colored_painter.set_projection(projection_matrix);
        props.text_painter.set_projection(projection_matrix);
    }

    pub fn projection(&self) -> cgmath::Matrix4<f32> {
        let props = self.props.borrow();
        props.projection_matrix
    }
}
