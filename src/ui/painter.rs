#![allow(unused_imports)]
use super::*;
use crate::gles::enums::*;
use crate::Color;
use cgmath::prelude::*;
use cgmath::{Matrix3, Matrix4, Vector2, Vector3};
use primitives::color;
use std::{cell::RefCell, rc::Rc};
use winit::dpi::PhysicalSize;

mod coloredpainter;
pub use self::coloredpainter::*;

mod gles_extensions;
pub use self::gles_extensions::*;

mod imagepainter;
pub use self::imagepainter::*;

mod image;
pub use self::image::*;

mod pipeline;
pub use self::pipeline::*;

mod textpainter;
pub use self::textpainter::*;

mod webgl_extensions;
pub use self::webgl_extensions::*;

#[derive(Default)]
pub struct Font;

impl Font {
    pub fn height(&self, val: i32) -> f32 {
        0.0
    }
}

pub struct ImageScaleQuality;
pub struct KeyCode;

pub struct Video;

pub struct Canvas;

pub struct IndexBuffer; // Vec<u32>

pub struct VertexBuffer;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TextureUnit;

pub struct TextureAddressing;

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
    my_font: Font,
    projectionMatrix: Matrix4<f32>,

    imagePainter: ImageShaderPainter,
    coloredPainter: ColoredShaderPainter,
    textPainter: TextShaderPainter,

    // static var videoPipeline: PipelineState,
    canvas: PhysicalSize<u32>,

    transformations: Vec<cgmath::Matrix3<f32>>,
    transformationIndex: usize,
    opacities: Vec<f32>,
    fontsize: f32,
}

impl PainterProps {
    pub fn new() -> Self {
        let fontsize = 18.0;

        let mut textPainter = TextShaderPainter::new();
        textPainter.fontsize = fontsize;

        let instance = Self {
            color: color::WHITE,
            my_font: Font {},
            projectionMatrix: Matrix4::identity(),
            transformations: vec![Matrix3::identity()],
            transformationIndex: 0,
            imagePainter: ImageShaderPainter::new(),
            coloredPainter: ColoredShaderPainter::new(),
            textPainter,
            opacities: vec![1.0],
            fontsize,
            canvas: PhysicalSize::new(900, 700),
        };

        // #if sys_g4
        // pipe = null;
        // #end

        // if (videoPipeline == null) {
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

// static current: Option<Graphics2> = null;

impl Painter {
    pub fn new() -> Self {
        let instance = Self {
            props: RefCell::new(PainterProps::new()),
        };

        instance.set_projection();
        instance
    }

    pub fn begin(&self, clear: bool /* = true*/, clear_color: Option<Color> /* = null*/) {
        // if current.is_none() {
        // 	current = self;
        // } else {
        // 	panic!("End before you begin");
        // }

        // self.g.begin();
        if clear {
            self.clear(clear_color);
        }
        self.set_projection();
    }

    pub fn end(&self) {
        self.flush();
        // self.g.end();

        // if current == this {
        // 	current = null;
        // } else {
        // 	panic!("Begin before you end");
        // }
    }

    pub fn flush(&self) {
        let mut props = self.props.borrow_mut();

        props.imagePainter.end();
        props.textPainter.end();
        props.coloredPainter.end();
    }

    // scale-filtering
    // draw/fillPolygon

    pub fn clear(&self, color: Option<Color> /* = null*/) {
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
        let mut props = self.props.borrow_mut();

        props.coloredPainter.end();
        props.textPainter.end();

        // TODO:
        // let mat: Matrix3<f32> = Matrix3::identity();
        // let vec: Vector3<f32> = Vector3::new(dx, dy, 0.0);
        // let a = mat * vec;

        // let p1 = transformation.multvec(Vector2<f32>::new((dx, dy + dh));
        // let p2 = transformation.multvec(Vector2<f32>::new((dx, dy));
        // let p3 = transformation.multvec(Vector2<f32>::new((dx + dw, dy));
        // let p4 = transformation.multvec(Vector2<f32>::new((dx + dw, dy + dh));
        // self.imagePainter.drawImage2(img, sx, sy, sw, sh, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y, opacity, this.color);
        unimplemented!()
    }

    pub fn draw_rect(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        strength: f32, /* = 1.0*/
    ) {
        let mut props = self.props.borrow_mut();

        props.imagePainter.end();
        props.textPainter.end();

        // let p1 = transformation.multvec(Vector2<f32>::new((x - strength / 2, y + strength / 2)); // bottom-left
        // let p2 = transformation.multvec(Vector2<f32>::new((x - strength / 2, y - strength / 2)); // top-left
        // let p3 = transformation.multvec(Vector2<f32>::new((x + width + strength / 2, y - strength / 2)); // top-right
        // let p4 = transformation.multvec(Vector2<f32>::new((x + width + strength / 2, y + strength / 2)); // bottom-right
        // self.coloredPainter.fillRect(opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y); // top

        // p1.setFrom(transformation.multvec(Vector2<f32>::new((x - strength / 2, y + height - strength / 2)));
        // p2.setFrom(transformation.multvec(Vector2<f32>::new((x - strength / 2, y + strength / 2)));
        // p3.setFrom(transformation.multvec(Vector2<f32>::new((x + strength / 2, y + strength / 2)));
        // p4.setFrom(transformation.multvec(Vector2<f32>::new((x + strength / 2, y + height - strength / 2)));
        // self.coloredPainter.fillRect(opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y); // left

        // p1.setFrom(transformation.multvec(Vector2<f32>::new((x - strength / 2, y + height + strength / 2)));
        // p2.setFrom(transformation.multvec(Vector2<f32>::new((x - strength / 2, y + height - strength / 2)));
        // p3.setFrom(transformation.multvec(Vector2<f32>::new((x + width + strength / 2, y + height - strength / 2)));
        // p4.setFrom(transformation.multvec(Vector2<f32>::new((x + width + strength / 2, y + height + strength / 2)));
        // self.coloredPainter.fillRect(opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y); // bottom

        // p1.setFrom(transformation.multvec(Vector2<f32>::new((x + width - strength / 2, y + height - strength / 2)));
        // p2.setFrom(transformation.multvec(Vector2<f32>::new((x + width - strength / 2, y + strength / 2)));
        // p3.setFrom(transformation.multvec(Vector2<f32>::new((x + width + strength / 2, y + strength / 2)));
        // p4.setFrom(transformation.multvec(Vector2<f32>::new((x + width + strength / 2, y + height - strength / 2)));
        // self.coloredPainter.fillRect(opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y); // right
        unimplemented!()
    }

    pub fn fill_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        let mut props = self.props.borrow_mut();

        props.imagePainter.end();
        props.textPainter.end();

        // let p1 = transformation.multvec(Vector2<f32>::new((x, y + height));
        // let p2 = transformation.multvec(Vector2<f32>::new((x, y));
        // let p3 = transformation.multvec(Vector2<f32>::new((x + width, y));
        // let p4 = transformation.multvec(Vector2<f32>::new((x + width, y + height));
        // self.coloredPainter.fillRect(opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y);
        unimplemented!()
    }

    // Draw a single line of text with the current `color`, `font` and `fontSize` properties.
    //
    // When drawing into rendertargets, you might have to use a different shader than the default one
    //  - use the default shader when drawing into a transparent section of your rendertarget
    //  - use a shader with `alphaBlendSource = BlendOne` when drawing into a non-transparent section of your rendertarget
    pub fn draw_string(&self, text: &str, x: f32, y: f32) {
        let mut props = self.props.borrow_mut();

        props.imagePainter.end();
        props.coloredPainter.end();

        let opacity = match props.opacities.last() {
            Some(opacity) => *opacity,
            None => 1.0,
        };

        let color = props.color;

        props.textPainter.drawString(
            text, x, y, opacity, color,
            /*, transformation*/
        );
    }

    // Draw a single line of characters with the current `color`, `font` and `fontSize` properties.
    //
    // When drawing into rendertargets, you might have to use a different shader than the default one
    //  - use the default shader when drawing into a transparent section of your rendertarget
    //  - use a shader with `alphaBlendSource = BlendOne` when drawing into a non-transparent section of your rendertarget
    pub fn draw_characters(&self, text: Vec<i32>, start: i32, length: i32, x: f32, y: f32) {
        let mut props = self.props.borrow_mut();

        props.imagePainter.end();
        props.coloredPainter.end();

        // self.textPainter.drawCharacters(text, start, length, opacity, color, x, y, transformation);
        unimplemented!()
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, strength: f32 /*= 1.0*/) {
        let mut props = self.props.borrow_mut();

        props.imagePainter.end();
        props.textPainter.end();

        // let vec = Vector2<f32>::new(();
        // if y2 == y1 {
        // 	vec.setFrom(Vector2<f32>::new((0, -1));
        // } else {
        // 	vec.setFrom(Vector2<f32>::new((1, -(x2 - x1) / (y2 - y1)));
        // }

        // vec.length = strength;
        // let p1 = Vector2<f32>::new((x1 + 0.5 * vec.x, y1 + 0.5 * vec.y);
        // let p2 = Vector2<f32>::new((x2 + 0.5 * vec.x, y2 + 0.5 * vec.y);
        // let p3 = p1.sub(vec);
        // let p4 = p2.sub(vec);

        // p1.setFrom(transformation.multvec(p1));
        // p2.setFrom(transformation.multvec(p2));
        // p3.setFrom(transformation.multvec(p3));
        // p4.setFrom(transformation.multvec(p4));

        // self.coloredPainter.fillTriangle(opacity, color, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        // self.coloredPainter.fillTriangle(opacity, color, p3.x, p3.y, p2.x, p2.y, p4.x, p4.y);
        unimplemented!()
    }

    pub fn draw_video(&self, video: Video, x: f32, y: f32, width: f32, height: f32) {
        // self.setPipeline(videoPipeline);
        // self.drawVideoInternal(video, x, y, width, height);
        // self.setPipeline(null);
        unimplemented!()
    }

    pub fn fill_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        // if bufferIndex > 0 {
        // 	self.drawBuffer(true); // Flush other buffer for right render order
        // }

        // if triangleBufferIndex + 1 >= triangleBufferSize {
        // 	self.drawTriBuffer(false);
        // }

        // self.setTriColors(opacity, color);
        // self.setTriVertices(x1, y1, x2, y2, x3, y3);
        // triangleBufferIndex += 1;
        unimplemented!()
    }

    // fn get_image_scale_quality(&self) -> ImageScaleQuality {
    // 	return myImageScaleQuality;
    // }

    // fn set_image_scale_quality(&self, value: ImageScaleQuality) -> ImageScaleQuality {
    // 	if value == myImageScaleQuality {
    // 		return value;
    // 	}
    // 	self.imagePainter.setBilinearFilter(value == ImageScaleQuality.High);
    // 	self.textPainter.setBilinearFilter(value == ImageScaleQuality.High);
    // 	return myImageScaleQuality = value;
    // }

    // fn get_mipmap_scale_quality(&self) -> ImageScaleQuality {
    // 	self.myMipmapScaleQuality
    // }

    // fn set_mipmap_scale_quality(&self, value: ImageScaleQuality) -> ImageScaleQuality {
    // 	self.imagePainter.setBilinearMipmapFilter(value == ImageScaleQuality.High);
    // 	// textPainter.setBilinearMipmapFilter(value == ImageScaleQuality.High); // TODO (DK) implement for fonts as well?
    // 	return myMipmapScaleQuality = value;
    // }

    // The color value is used for geometric primitives, images, and text. Remember to set it back to white to draw images unaltered.

    pub fn get_color(&self) -> Color {
        let props = self.props.borrow_mut();
        props.color
    }

    pub fn set_color(&self, color: Color) {
        let mut props = self.props.borrow_mut();
        props.color = color;
    }

    fn get_font(&self) -> Font {
        // 	self.myFont;
        unimplemented!()
    }

    fn set_font(&self, font: Font) {
        // 	self.textPainter.setFont(font);
        // 	self.myFont = font;
        unimplemented!()
    }

    fn get_font_size(&self) -> f32 {
        let props = self.props.borrow_mut();
        props.fontsize
    }

    fn set_font_size(&self, value: f32) {
        let mut props = self.props.borrow_mut();
        props.fontsize = value;
        props.textPainter.fontsize = value;
    }

    // public static var fontGlyphs: Vec<Int> = [for (i in 32...256) i];

    // public var transformation(get, set): cgmath::Matrix3<f32>; // works on the top of the transformation stack

    #[inline]
    fn get_transformation(&self) -> cgmath::Matrix3<f32> {
        // 	self.transformations[transformationIndex];
        unimplemented!()
    }

    #[inline]
    fn set_transformation(&self, transformation: cgmath::Matrix3<f32>) -> cgmath::Matrix3<f32> {
        // 	self.setTransformation(transformation);
        // 	self.transformations[transformationIndex].setFrom(transformation);
        // 	self.transformation
        unimplemented!()
    }

    #[inline]
    pub fn push_transformation(&self, trans: cgmath::Matrix3<f32>) {
        // 	self.transformationIndex++;
        // 	if (transformationIndex == transformations.length) {
        // 		self.transformations.push(Matrix3.identity());
        // 	}
        // 	self.transformations[transformationIndex].setFrom(trans);
        // 	self.setTransformation(get_transformation());
        unimplemented!()
    }

    pub fn pop_transformation(&self) -> cgmath::Matrix3<f32> {
        // 	self.transformationIndex--;
        // 	self.setTransformation(get_transformation());
        // 	self.transformations[transformationIndex + 1];
        unimplemented!()
    }

    pub fn scale(&self, x: f32, y: f32) {
        // 	self.transformation.setFrom(Matrix3.scale(x, y).multmat(transformation));
        unimplemented!()
    }

    #[inline]
    fn translation(&self, tx: f32, ty: f32) -> cgmath::Matrix3<f32> {
        // 	Matrix3.translation(tx, ty).multmat(transformation);
        unimplemented!()
    }

    pub fn translate(&self, tx: f32, ty: f32) {
        // 	self.transformation.setFrom(translation(tx, ty));
        unimplemented!()
    }

    pub fn push_translation(&self, tx: f32, ty: f32) {
        // 	self.pushTransformation(translation(tx, ty));
        unimplemented!()
    }

    #[inline]
    fn rotation(&self, angle: f32, centerx: f32, centery: f32) -> cgmath::Matrix3<f32> {
        // 	Matrix3.translation(centerx, centery)
        // 		.multmat(Matrix3.rotation(angle))
        // 		.multmat(Matrix3.translation(-centerx, -centery))
        // 		.multmat(transformation);
        unimplemented!()
    }

    pub fn rotate(&self, angle: f32, centerx: f32, centery: f32) {
        // 	self.transformation.setFrom(rotation(angle, centerx, centery));
        unimplemented!()
    }

    pub fn push_rotation(&self, angle: f32, centerx: f32, centery: f32) {
        // 	self.pushTransformation(rotation(angle, centerx, centery));
        unimplemented!()
    }

    // // public var opacity(get, set): f32; // works on the top of the opacity stack

    pub fn push_opacity(&self, opacity: f32) {
        let mut props = self.props.borrow_mut();
        // self.setOpacity(opacity);
        props.opacities.push(opacity);
    }

    pub fn pop_opacity(&self) -> f32 {
        let mut props = self.props.borrow_mut();
        let ret = props.opacities.pop().unwrap();
        // self.setOpacity(get_opacity());
        ret
    }

    pub fn get_opacity(&self) -> f32 {
        let props = self.props.borrow();
        *props.opacities.last().unwrap()
    }

    pub fn set_opacity(&self, opacity: f32) {
        let mut props = self.props.borrow_mut();
        // self.setOpacity(opacity); // super
        let index = props.opacities.len() - 1;
        props.opacities[index] = opacity;
    }

    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {}

    pub fn disable_scissor(&self) {}

    // #if sys_g4
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

    fn set_projection(&self) {
        let mut props = self.props.borrow_mut();
        let width = props.canvas.width as f32;
        let height = props.canvas.height as f32;

        // inverted y
        let projectionMatrix: Matrix4<f32> = cgmath::ortho(0.0, width, 0.0, height, 0.0, 1000.0);

        // if (Std.isOfType(canvas, Framebuffer)) {
        // 	projectionMatrix.setFrom(Matrix4.orthogonalProjection(0, width, height, 0, 0.1, 1000));
        // } else {
        // 	if (!Image.nonPow2Supported) {
        // 		width = upperPowerOfTwo(width);
        // 		height = upperPowerOfTwo(height);
        // 	}
        // 	if (Image.renderTargetsInvertedY()) {
        // 		props.projectionMatrix.setFrom(Matrix4.orthogonalProjection(0, width, 0, height, 0.1, 1000));
        // 	} else {
        // 		props.projectionMatrix.setFrom(Matrix4.orthogonalProjection(0, width, height, 0, 0.1, 1000));
        // 	}
        // }

        props.projectionMatrix = projectionMatrix;

        props.imagePainter.setProjection(projectionMatrix);
        props.coloredPainter.setProjection(projectionMatrix);
        props.textPainter.setProjection(projectionMatrix);
    }
}
