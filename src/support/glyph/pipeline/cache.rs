use crate::platform::gles::{core30::gl, enums::*};

pub struct Cache {
    pub(crate) texture: u32,
}

impl Cache {
    pub fn new(width: u32, height: u32) -> Cache {
        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1);

        let texture = {
            let handle = gl::gen_texture();

            gl::bind_texture(GL_TEXTURE_2D, handle);

            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);

            gl::empty_tex_image_2d(
                GL_TEXTURE_2D,
                0,
                GL_R8 as i32,
                width as i32,
                height as i32,
                0,
                GL_RED,
                GL_UNSIGNED_BYTE,
            );
            gl::bind_texture(GL_TEXTURE_2D, 0);

            handle
        };

        Cache { texture }
    }

    pub fn update(&self, offset: [u16; 2], size: [u16; 2], data: &[u8]) {
        let [offset_x, offset_y] = offset;
        let [width, height] = size;

        gl::bind_texture(GL_TEXTURE_2D, self.texture);

        gl::tex_sub_image_2d(
            GL_TEXTURE_2D,
            0,
            i32::from(offset_x),
            i32::from(offset_y),
            i32::from(width),
            i32::from(height),
            GL_RED,
            GL_UNSIGNED_BYTE,
            data,
        );

        gl::bind_texture(GL_TEXTURE_2D, 0);
    }

    pub fn destroy(&self) {
        gl::delete_textures(&[self.texture]);
    }
}
