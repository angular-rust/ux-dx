use core::ffi::c_void;
use iced_graphics::{Antialiasing, Size};
use iced_native::mouse;

use crate::{
    platform::gles::{core30::gl, enums::*},
    support::iced::{Backend, Color, Error, Renderer, Settings, Viewport},
};

/// A window graphics backend for iced powered by `glow`.
#[allow(missing_debug_implementations)]
pub struct Compositor {}

impl iced_graphics::window::GLCompositor for Compositor {
    type Settings = Settings;
    type Renderer = Renderer;

    unsafe fn new(
        settings: Self::Settings,
        loader_function: impl FnMut(&str) -> *const c_void,
    ) -> Result<(Self, Self::Renderer), Error> {
        // Enable auto-conversion from/to sRGB
        // gl::enable(GL_FRAMEBUFFER_SRGB); // DV

        // Enable alpha blending
        gl::enable(GL_BLEND);
        gl::blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        // Disable multisampling by default
        // gl::disable(GL_MULTISAMPLE); // DV

        let renderer = Renderer::new(Backend::new(settings));

        Ok((Self {}, renderer))
    }

    fn sample_count(settings: &Settings) -> u32 {
        settings
            .antialiasing
            .map(Antialiasing::sample_count)
            .unwrap_or(0)
    }

    fn resize_viewport(&mut self, physical_size: Size<u32>) {
        gl::viewport(
            0,
            0,
            physical_size.width as i32,
            physical_size.height as i32,
        );
    }

    fn draw<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        color: Color,
        output: &<Self::Renderer as iced_native::Renderer>::Output,
        overlay: &[T],
    ) -> mouse::Interaction {
        let [r, g, b, a] = color.into_linear();

        gl::clear_color(r, g, b, a);
        gl::clear(GL_COLOR_BUFFER_BIT);

        renderer.backend_mut().draw(viewport, output, overlay)
    }
}
