use crate::{
    canvas::ErrorKind,
    gles::{core30::gl, enums::*},
};

use super::GlTexture;

pub struct Framebuffer {
    id: u32,
    depth_stencil_rbo: Option<u32>,
}

impl Framebuffer {
    pub fn from_external(framebuffer: Framebuffer) -> Self {
        Framebuffer {
            id: framebuffer.id,
            depth_stencil_rbo: None,
        }
    }
    pub fn new(texture: &GlTexture) -> Result<Self, ErrorKind> {
        let fbo = gl::gen_framebuffer();
        gl::bind_framebuffer(GL_FRAMEBUFFER, fbo);

        let width = texture.info().width() as u32;
        let height = texture.info().height() as u32;

        let attach_texture = {
            || {
                gl::framebuffer_texture_2d(
                    GL_FRAMEBUFFER,
                    GL_COLOR_ATTACHMENT0,
                    GL_TEXTURE_2D,
                    texture.id(),
                    0,
                );
            }
        };

        attach_texture();

        let depth_stencil_rbo = gl::gen_renderbuffer();
        gl::bind_renderbuffer(GL_RENDERBUFFER, depth_stencil_rbo);
        gl::renderbuffer_storage(
            GL_RENDERBUFFER,
            GL_DEPTH24_STENCIL8,
            width as i32,
            height as i32,
        );

        // unbind renderbuffer
        gl::bind_renderbuffer(GL_RENDERBUFFER, 0);

        let attach_stencil_rbo = {
            || {
                gl::framebuffer_renderbuffer(
                    GL_FRAMEBUFFER,
                    GL_DEPTH_STENCIL_ATTACHMENT,
                    GL_RENDERBUFFER,
                    depth_stencil_rbo,
                );
            }
        };

        attach_stencil_rbo();

        let status = gl::check_framebuffer_status(GL_FRAMEBUFFER);

        if status != GL_FRAMEBUFFER_COMPLETE {
            // DEPTH24_STENCIL8 is not supported in WebGL 1, so fall back to the unsized DEPTH_STENCIL, a workaround
            // that's in the WebGL 1.0 spec.
            gl::bind_renderbuffer(GL_RENDERBUFFER, depth_stencil_rbo);
            gl::renderbuffer_storage(
                GL_RENDERBUFFER,
                GL_DEPTH_STENCIL,
                width as i32,
                height as i32,
            );

            // unbind renderbuffer
            gl::bind_renderbuffer(GL_RENDERBUFFER, 0);
            attach_texture();
            attach_stencil_rbo();

            let status = gl::check_framebuffer_status(GL_FRAMEBUFFER);

            if status != GL_FRAMEBUFFER_COMPLETE {
                let reason = match status {
                    GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                        format!("({}) Framebuffer incomplete attachment", status)
                    }
                    //GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => format!("({}) Framebuffer incomplete draw buffer", status),
                    //GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => format!("({}) Framebuffer incomplete layer targets", status),
                    //FIXME: will be in next release: GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS => format!("({}) Framebuffer incomplete dimensions", status),
                    GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                        format!("({}) Framebuffer incomplete missing attachment", status)
                    }
                    GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                        format!("({}) Framebuffer incomplete multisample", status)
                    }
                    //GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER => format!("({}) Framebuffer incomplete read buffer", status),
                    GL_FRAMEBUFFER_UNSUPPORTED => format!("({}) Framebuffer unsupported", status),
                    _ => format!("({}) Framebuffer not complete!", status),
                };

                return Err(ErrorKind::RenderTargetError(reason));
            }
        }

        // unbind framebuffer
        gl::bind_framebuffer(GL_FRAMEBUFFER, 0);

        Ok(Framebuffer {
            id: fbo,
            depth_stencil_rbo: Some(depth_stencil_rbo),
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

    //     {
    //         gl::bind_framebuffer(GL_READ_FRAMEBUFFER, self.fbo);
    //         gl::bind_framebuffer(GL_DRAW_FRAMEBUFFER, dest_fbo.fbo);

    //         gl::blit_framebuffer(
    //             0,
    //             0,
    //             self.width as i32,
    //             self.height as i32,
    //             0,
    //             0,
    //             dest_fbo.width as i32,
    //             dest_fbo.height as i32,
    //             GL_COLOR_BUFFER_BIT,
    //             GL_NEAREST
    //         );

    //         gl::bind_framebuffer(GL_READ_FRAMEBUFFER, 0);
    //         gl::bind_framebuffer(GL_DRAW_FRAMEBUFFER, 0);
    //     }
    // }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        gl::delete_framebuffers(&[self.id]);
        if let Some(depth_stencil_rbo) = self.depth_stencil_rbo {
            gl::delete_renderbuffers(&[depth_stencil_rbo]);
        }
    }
}
