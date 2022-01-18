use std::{fmt, rc::Rc};

use super::{BlendMode, Texture};

/// Draws to a surface.
pub trait Graphics: fmt::Debug {
    /// Called at the beginning of a frame.
    fn will_render(&self);

    /// Called at the end of a frame.
    fn did_render(&self);

    /// Called when the buffer being drawn to was resized.
    fn on_resize(&self, width: i32, height: i32);

    /// Saves the graphics state until the next restore(). The state contains the transformation
    /// matrix, alpha, blend mode, and scissor rectangle.
    fn save(&self);

    /// Translates the transformation matrix.
    fn translate(&self, x: f32, y: f32);

    /// Scales the transformation matrix.
    fn scale(&self, x: f32, y: f32);

    /// Rotates the transformation matrix by the given angle, in degrees.
    fn rotate(&self, rotation: f32);

    /// Multiplies the transformation matrix by the given matrix.
    fn transform(&self, m00: f32, m10: f32, m01: f32, m11: f32, m02: f32, m12: f32);

    /// Multiplies the alpha by the given factor.
    fn multiply_alpha(&self, factor: f32);

    /// Sets the alpha to use for drawing.
    fn set_alpha(&self, alpha: f32);

    /// Sets the blend mode to use for drawing.
    fn set_blend_mode(&self, blend_mode: BlendMode);

    /// Sets the scissor rectangle to the intersection of the current scissor rectangle and the given
    /// rectangle, in local coordinates.
    fn apply_scissor(&self, x: f32, y: f32, width: f32, height: f32);

    /// Restores the graphics state back to the previous save().
    fn restore(&self);

    /// Draws a texture at the given point.
    fn draw_texture(&self, texture: &Rc<dyn Texture>, dest_x: f32, dest_y: f32);

    /// Draws a texture sub-region at the given point.
    fn draw_sub_texture(
        &self,
        texture: &Rc<dyn Texture>,
        dest_x: f32,
        dest_y: f32,
        source_x: f32,
        source_y: f32,
        source_w: f32,
        source_h: f32,
    );

    /// Draws a repeating texture to the given region.
    fn draw_pattern(&self, texture: &Rc<dyn Texture>, dest_x: f32, dest_y: f32, width: f32, height: f32);

    /// Draws a colored rectangle at the given region.
    fn fill_rect(&self, color: i32, x: f32, y: f32, width: f32, height: f32);
}
