use std::{fmt, rc::Rc};

use crate::engine::d2::display::{BlendMode, Graphics, Texture};

pub struct OverdrawGraphics {
    impls: Box<dyn Graphics>,
}

impl OverdrawGraphics {
    pub fn new(impls: Box<dyn Graphics>) -> Self {
        Self { impls }
    }

    /// Draws an overdraw region rectangle.
    fn draw_region(&self, x: f32, y: f32, width: f32, height: f32) {
        self.impls.fill_rect(0x101008, x, y, width, height);
    }
}

impl Graphics for OverdrawGraphics {
    fn will_render(&self) {
        self.impls.will_render();
        self.impls.save();
        self.impls.set_blend_mode(BlendMode::Add);
    }

    fn did_render(&self) {
        self.impls.restore();
        self.impls.did_render();
    }

    fn on_resize(&self, width: i32, height: i32) {
        self.impls.on_resize(width, height);
    }

    fn save(&self) {
        self.impls.save();
    }

    fn translate(&self, x: f32, y: f32) {
        self.impls.translate(x, y);
    }

    fn scale(&self, x: f32, y: f32) {
        self.impls.scale(x, y);
    }

    fn rotate(&self, rotation: f32) {
        self.impls.rotate(rotation);
    }

    fn transform(&self, m00: f32, m10: f32, m01: f32, m11: f32, m02: f32, m12: f32) {
        self.impls.transform(m00, m10, m01, m11, m02, m12);
    }

    fn multiply_alpha(&self, factor: f32) {
        // Ignore
    }

    fn set_alpha(&self, alpha: f32) {
        // Ignore
    }

    fn set_blend_mode(&self, blend_mode: BlendMode) {
        // Ignore
    }

    fn apply_scissor(&self, x: f32, y: f32, width: f32, height: f32) {
        self.impls.apply_scissor(x, y, width, height);
    }

    fn restore(&self) {
        self.impls.restore();
    }

    fn draw_texture(&self, texture: &Rc<dyn Texture>, dest_x: f32, dest_y: f32) {
        self.draw_region(
            dest_x,
            dest_y,
            texture.width() as f32,
            texture.height() as f32,
        );
    }

    fn draw_sub_texture(
        &self,
        texture: &Rc<dyn Texture>,
        dest_x: f32,
        dest_y: f32,
        source_x: f32,
        source_y: f32,
        source_w: f32,
        source_h: f32,
    ) {
        self.draw_region(dest_x, dest_y, source_w, source_h);
    }

    fn draw_pattern(
        &self,
        texture: &Rc<dyn Texture>,
        dest_x: f32,
        dest_y: f32,
        width: f32,
        height: f32,
    ) {
        self.draw_region(dest_x, dest_y, width, height);
    }

    fn fill_rect(&self, color: i32, x: f32, y: f32, width: f32, height: f32) {
        self.draw_region(x, y, width, height);
    }
}

impl fmt::Debug for OverdrawGraphics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OverdrawGraphics")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
