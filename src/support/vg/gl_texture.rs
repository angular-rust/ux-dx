use vg::{ErrorKind, ImageFlags, ImageInfo, ImageSource, PixelFormat};

use crate::platform::gles::{core30::gl, enums::*};

pub struct GlTexture {
    id: u32,
    info: ImageInfo,
}

impl GlTexture {
    pub fn new(info: ImageInfo, opengles_2_0: bool) -> Result<Self, ErrorKind> {
        let id = {
            let id = gl::gen_texture();
            gl::bind_texture(GL_TEXTURE_2D, id);
            gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1);
            if !opengles_2_0 {
                gl::pixel_storei(GL_UNPACK_ROW_LENGTH, info.width() as i32);
                gl::pixel_storei(GL_UNPACK_SKIP_PIXELS, 0);
                gl::pixel_storei(GL_UNPACK_SKIP_ROWS, 0);
            }
            id
        };

        let texture = Self { id, info };

        match info.format() {
            PixelFormat::Gray8 => {
                let internal_format = if opengles_2_0 { GL_LUMINANCE } else { GL_R8 };
                let format = if opengles_2_0 {
                    internal_format
                } else {
                    GL_RED
                };

                gl::empty_tex_image_2d(
                    GL_TEXTURE_2D,
                    0,
                    internal_format as i32,
                    texture.info.width() as i32,
                    texture.info.height() as i32,
                    0,
                    format,
                    GL_UNSIGNED_BYTE,
                );
            }
            PixelFormat::Rgb8 => gl::empty_tex_image_2d(
                GL_TEXTURE_2D,
                0,
                GL_RGB as i32,
                texture.info.width() as i32,
                texture.info.height() as i32,
                0,
                GL_RGB,
                GL_UNSIGNED_BYTE,
            ),
            PixelFormat::Rgba8 => gl::empty_tex_image_2d(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                texture.info.width() as i32,
                texture.info.height() as i32,
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
            ),
        }

        let flags = texture.info.flags();

        if flags.contains(ImageFlags::GENERATE_MIPMAPS) {
            if flags.contains(ImageFlags::NEAREST) {
                gl::tex_parameteri(
                    GL_TEXTURE_2D,
                    GL_TEXTURE_MIN_FILTER,
                    GL_NEAREST_MIPMAP_NEAREST as i32,
                );
            } else {
                gl::tex_parameteri(
                    GL_TEXTURE_2D,
                    GL_TEXTURE_MIN_FILTER,
                    GL_LINEAR_MIPMAP_LINEAR as i32,
                );
            }
        } else {
            if flags.contains(ImageFlags::NEAREST) {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as i32);
            } else {
                gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
            }
        }

        if flags.contains(ImageFlags::NEAREST) {
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as i32);
        } else {
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
        }

        if flags.contains(ImageFlags::REPEAT_X) {
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as i32);
        } else {
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
        }

        if flags.contains(ImageFlags::REPEAT_Y) {
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as i32);
        } else {
            gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
        }

        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 4);
        if !opengles_2_0 {
            gl::pixel_storei(GL_UNPACK_ROW_LENGTH, 0);
            gl::pixel_storei(GL_UNPACK_SKIP_PIXELS, 0);
            gl::pixel_storei(GL_UNPACK_SKIP_ROWS, 0);
        }

        if flags.contains(ImageFlags::GENERATE_MIPMAPS) {
            gl::generate_mipmap(GL_TEXTURE_2D);
            //gl::tex_parameteri(GL_TEXTURE_2D, GL_GENERATE_MIPMAP, GL_TRUE);
        }

        gl::bind_texture(GL_TEXTURE_2D, 0);

        Ok(texture)
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn update(
        &mut self,
        src: ImageSource,
        x: usize,
        y: usize,
        opengles_2_0: bool,
    ) -> Result<(), ErrorKind> {
        let size = src.dimensions();

        if x + size.0 > self.info.width() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if y + size.1 > self.info.height() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if self.info.format() != src.format() {
            return Err(ErrorKind::ImageUpdateWithDifferentFormat);
        }

        gl::bind_texture(GL_TEXTURE_2D, self.id);
        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1);
        if !opengles_2_0 {
            gl::pixel_storei(GL_UNPACK_ROW_LENGTH, size.0 as i32);
        }

        match src {
            ImageSource::Gray(data) => {
                let format = if opengles_2_0 { GL_LUMINANCE } else { GL_R8 };

                gl::tex_sub_image_2d(
                    GL_TEXTURE_2D,
                    0,
                    x as i32,
                    y as i32,
                    size.0 as i32,
                    size.1 as i32,
                    format,
                    GL_UNSIGNED_BYTE,
                    data.buf(),
                );
            }
            ImageSource::Rgb(data) => gl::tex_sub_image_2d(
                GL_TEXTURE_2D,
                0,
                x as i32,
                y as i32,
                size.0 as i32,
                size.1 as i32,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                data.buf(),
            ),
            ImageSource::Rgba(data) => gl::tex_sub_image_2d(
                GL_TEXTURE_2D,
                0,
                x as i32,
                y as i32,
                size.0 as i32,
                size.1 as i32,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                data.buf(),
            ),
            #[cfg(target_arch = "wasm32")]
            ImageSource::HtmlImageElement(image_element) => {
                gl::tex_sub_image_2d_with_html_image(
                    GL_TEXTURE_2D,
                    0,
                    x as i32,
                    y as i32,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    image_element,
                )
            },
            _=> {}
        }

        if self.info.flags().contains(ImageFlags::GENERATE_MIPMAPS) {
            gl::generate_mipmap(GL_TEXTURE_2D);
            //gl::tex_parameteri(GL_TEXTURE_2D, GL_GENERATE_MIPMAP, GL_TRUE);
        }

        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 4);
        if !opengles_2_0 {
            gl::pixel_storei(GL_UNPACK_ROW_LENGTH, 0);
        }
        //gl::pixel_storei(GL_UNPACK_SKIP_PIXELS, 0);
        //gl::pixel_storei(GL_UNPACK_SKIP_ROWS, 0);
        gl::bind_texture(GL_TEXTURE_2D, 0);

        Ok(())
    }

    pub fn delete(self) {
        gl::delete_textures(&[self.id]);
    }

    pub fn info(&self) -> ImageInfo {
        self.info
    }
}
