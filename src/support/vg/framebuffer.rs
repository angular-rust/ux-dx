use vg::ErrorKind;

use crate::platform::gles::{core30::gl, enums::*};

use super::GlTexture;

pub struct Framebuffer {
    id: u32,
    stencil_rbo: Option<u32>,
}

impl Framebuffer {
    pub fn from_external(framebuffer: Framebuffer) -> Self {
        Self {
            id: framebuffer.id,
            stencil_rbo: None,
        }
    }
    pub fn new(texture: &GlTexture) -> Result<Self, ErrorKind> {
        let fbo = gl::gen_framebuffer();
        gl::bind_framebuffer(GL_FRAMEBUFFER, fbo);

        let width = texture.info().width() as u32;
        let height = texture.info().height() as u32;

        gl::framebuffer_texture_2d(
            GL_FRAMEBUFFER,
            GL_COLOR_ATTACHMENT0,
            GL_TEXTURE_2D,
            texture.id(),
            0,
        );

        let stencil_rbo = gl::gen_renderbuffer();

        gl::bind_renderbuffer(GL_RENDERBUFFER, stencil_rbo);
        gl::renderbuffer_storage(
            GL_RENDERBUFFER,
            GL_STENCIL_INDEX8,
            width as i32,
            height as i32,
        );
        gl::bind_renderbuffer(GL_RENDERBUFFER, 0);

        gl::framebuffer_renderbuffer(
            GL_FRAMEBUFFER,
            GL_STENCIL_ATTACHMENT,
            GL_RENDERBUFFER,
            stencil_rbo,
        );

        let status = gl::check_framebuffer_status(GL_FRAMEBUFFER);

        if status != GL_FRAMEBUFFER_COMPLETE {
            let reason = match status {
                GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    format!("({}) Framebuffer incomplete attachment", status)
                }
                //GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => format!("({}) Framebuffer incomplete draw buffer", status),
                //GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => format!("({}) Framebuffer incomplete layer targets", status),
                //FIXME: will be in next glow release: GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS => format!("({}) Framebuffer incomplete dimensions", status),
                GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    format!("({}) Framebuffer incomplete missing attachment", status)
                }
                GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                    format!("({}) Framebuffer incomplete multisample", status)
                }
                //GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER => format!("({}) Framebuffer incomplete read buffer", status),
                GL_FRAMEBUFFER_UNSUPPORTED => {
                    format!("({}) Framebuffer unsupported", status)
                }
                _ => format!("({}) Framebuffer not complete!", status),
            };

            return Err(ErrorKind::RenderTargetError(reason));
        }

        gl::bind_framebuffer(GL_FRAMEBUFFER, 0);

        Ok(Framebuffer {
            id: fbo,
            stencil_rbo: Some(stencil_rbo),
        })
    }

    pub fn bind(&self) {
        gl::bind_framebuffer(GL_FRAMEBUFFER, self.id);
    }

    pub fn unbind() {
        gl::bind_framebuffer(GL_FRAMEBUFFER, 0);
    }

    // pub fn blit_to_texture(&self, texture: &GlTexture) {
    //     let dest_fbo = Self::new(texture);

    //     gl::bind_framebuffer(GL_READ_FRAMEBUFFER, self.id);
    //     gl::bind_framebuffer(GL_DRAW_FRAMEBUFFER, dest_fbo.id);

    //     gl::blit_framebuffer(
    //         0,
    //         0,
    //         self.width as i32,
    //         self.height as i32,
    //         0,
    //         0,
    //         dest_fbo.width as i32,
    //         dest_fbo.height as i32,
    //         GL_COLOR_BUFFER_BIT,
    //         GL_NEAREST
    //     );

    //     gl::bind_framebuffer(GL_READ_FRAMEBUFFER, 0);
    //     gl::bind_framebuffer(GL_DRAW_FRAMEBUFFER, 0);

    // }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        gl::delete_framebuffers(&[self.id]);
        if let Some(stencil_rbo) = self.stencil_rbo {
            gl::delete_renderbuffers(&[stencil_rbo]);
        }
    }
}
