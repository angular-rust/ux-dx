use super::{Filter, Wrap};
use crate::gles::{core30::gl, enums::*};
// use std::rc::Rc;

// pub type TextureId = <Context as HasContext>::Texture;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new() -> Result<Self, String> {
        let id = gl::gen_texture();
        Ok(Self { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        gl::bind_texture(GL_TEXTURE_2D, self.id);
    }

    pub fn unbind(&self) {
        gl::bind_texture(GL_TEXTURE_2D, 0);
    }

    pub fn init_image(&self, width: u32, height: u32, pixels: Option<&[u8]>) {
        match pixels {
            Some(pixels) => {
                gl::tex_image_2d(
                    GL_TEXTURE_2D,
                    0,
                    GL_RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    pixels,
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
        }
    }

    pub fn sub_image(
        &self,
        offset_x: u32,
        offset_y: u32,
        width: u32,
        height: u32,
        pixels: Option<&[u8]>,
    ) {
        match pixels {
            Some(pixels) => {
                gl::tex_sub_image_2d(
                    GL_TEXTURE_2D,
                    0,
                    offset_x as i32,
                    offset_y as i32,
                    width as i32,
                    height as i32,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    pixels,
                );
            }
            None => {
                gl::empty_tex_sub_image_2d(
                    GL_TEXTURE_2D,
                    0,
                    offset_x as i32,
                    offset_y as i32,
                    width as i32,
                    height as i32,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                );
            }
        }
    }

    pub fn set_filter(&self, filter: Filter) {
        gl::tex_parameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MIN_FILTER,
            filter.to_min_flag() as i32,
        );
        gl::tex_parameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MAG_FILTER,
            filter.to_mag_flag() as i32,
        );
    }

    pub fn generate_mipmap(&self) {
        gl::generate_mipmap(GL_TEXTURE_2D);
    }

    pub fn set_wrap(&self, wrap: Wrap) {
        gl::tex_parameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_WRAP_S,
            wrap.horizontal.to_flag() as i32,
        );
        gl::tex_parameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_WRAP_T,
            wrap.vertical.to_flag() as i32,
        );
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        gl::delete_textures(&[self.id]);
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
