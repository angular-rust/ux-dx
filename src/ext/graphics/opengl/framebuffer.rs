use super::Attachment;
use crate::gles::{core30::gl, enums::*};

pub struct Framebuffer {
    id: u32,
}

impl Framebuffer {
    pub fn new() -> Result<Self, String> {
        let id = gl::gen_framebuffer();
        Ok(Self { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        gl::bind_framebuffer(GL_FRAMEBUFFER, self.id);
    }

    pub fn unbind(&self) {
        gl::bind_framebuffer(GL_FRAMEBUFFER, 0);
    }

    pub fn attach_texture(&self, attachment: Attachment, texture_id: Option<u32>) {
        match texture_id {
            Some(texture_id) => {
                gl::framebuffer_texture_2d(
                    GL_FRAMEBUFFER,
                    attachment.to_flag(),
                    GL_TEXTURE_2D,
                    texture_id,
                    0,
                );
            }
            None => {
                gl::framebuffer_texture_2d(
                    GL_FRAMEBUFFER,
                    attachment.to_flag(),
                    GL_TEXTURE_2D,
                    0,
                    0,
                );
            }
        }
    }

    pub fn check_status(&self) -> Result<(), String> {
        let status = gl::check_framebuffer_status(GL_FRAMEBUFFER);
        match status {
            GL_FRAMEBUFFER_COMPLETE => Ok(()),
            GL_FRAMEBUFFER_UNDEFINED => Err("framebuffer undefined".to_owned()),
            GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                Err("framebuffer incomplete attachment".to_owned())
            }
            GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS => {
                Err("framebuffer incomplete missing attachment".to_owned())
            }
            // GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
            //     Err("framebuffer incomplete draw buffer".to_owned())
            // }
            // GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
            //     Err("framebuffer incomplete read buffer".to_owned())
            // }
            GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                Err("framebuffer incomplete layer targets".to_owned())
            }
            GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                Err("framebuffer incomplete missing attachment".to_owned())
            }
            GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                Err("framebuffer incomplete multisample".to_owned())
            }
            GL_FRAMEBUFFER_UNSUPPORTED => Err("framebuffer unsupported".to_owned()),
            _ => Err(format!("framebuffer error with status: {}", status)),
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        gl::delete_framebuffers(&[self.id]);
    }
}

impl PartialEq for Framebuffer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
