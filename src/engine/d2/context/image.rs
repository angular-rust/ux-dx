use bytes::Bytes;
use std::{fs::File, io::Read};
use warmy::{Load, Loaded, SimpleKey, Storage};

use crate::{
    foundation::resource::{error::Error, Store, StoreKey},
    platform::{
        core::{TextureFormat, TextureSampleType},
        gles::{core20::gl, enums::*},
    },
};

use super::*;

/// By default should be TextureFormat::Rgba32Uint
/// For Depth And Stencil use TextureFormat::Depth32Float,
/// TextureFormat::Depth24Plus, TextureFormat::Depth24PlusStencil8
///
/// It not yet handled Compressed textures like a:
/// DXT1, DXT3, DXT5, RGTC1, RGTC2, BPTC (Bc1, Bc2, Bc3, Bc4, Bc5 Bc6, Bc7),
/// Etc2, Eac, Etc, Astc
///
// implements Canvas implements Resource
#[derive(Debug, Clone, Copy)]
pub struct Image {
    // static
    // pub assets: AssetManager,
    pub width: u32,
    pub height: u32,
    pub real_width: u32,
    pub real_height: u32,
    pub format: TextureFormat,

    pub tex: Option<u32>,         // = -1;
    pub framebuffer: Option<u32>, // = -1;

    pub depth_buffer: Option<u32>,
    pub stencil_buffer: Option<u32>,
    // let bytes: Bytes = None;
}

impl Default for Image {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            real_width: 0,
            real_height: 0,
            format: TextureFormat::Rgba32Float,

            tex: None,         // = -1;
            framebuffer: None, // = -1;

            depth_buffer: None,
            stencil_buffer: None,
            // let bytes: Bytes = None;
        }
    }
}
impl Image {
    // return image and still binded to texture so
    // you can load data or switch to default texture
    pub fn new(width: u32, height: u32, format: TextureFormat, render_target: bool) -> Self {
        let tex = Self::create_texture();
        let mut instance = Self {
            width,
            height,
            real_width: Image::upper_power_of_two(width),
            real_height: Image::upper_power_of_two(height),
            format,
            tex: Some(tex),
            framebuffer: None,
            depth_buffer: None,
            stencil_buffer: None,
        };

        // Bind the texture object
        gl::bind_texture(GL_TEXTURE_2D, tex);
        // Sys.gl.pixelStorei(Sys.gl.UNPACK_FLIP_Y_WEBGL, true);

        // Set the filtering mode
        gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
        gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);

        // do not repeat texture horizontal
        gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
        // do not repeat texture vertical
        gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);

        // let format_info = format.describe();

        if render_target {
            let framebuffer = Self::create_framebuffer();
            instance.framebuffer = Some(framebuffer);
            gl::bind_framebuffer(GL_FRAMEBUFFER, framebuffer);
            gl::tex_image_2d(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                instance.real_width as i32,
                instance.real_height as i32,
                0,
                GL_RGBA,
                if format == TextureFormat::Rgba32Float {
                    GL_FLOAT
                } else {
                    GL_UNSIGNED_BYTE
                },
                &[] as &[u8],
            );
            gl::framebuffer_texture_2d(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, tex, 0);

            match format {
                TextureFormat::Depth24Plus | TextureFormat::Depth32Float => {
                    instance.setup_depth_buffer_only();
                }
                TextureFormat::Depth24PlusStencil8 => {
                    // #if debug
                    // log::info!("DepthAndStencilFormat 'Depth24Stencil8' not (yet?) supported on android, using target defaults");
                    // #end
                    instance.run_depth_and_stencil_setup_chain();
                }
                _ => {
                    panic!("should use renderTarget with Depth and Stencil formats only");
                }
            }

            gl::bind_framebuffer(GL_FRAMEBUFFER, 0);
        } else {
            // // why we need to init it here?
            // match format {
            //     TextureFormat::R8Uint => {
            //         gl::tex_image_2d(
            //             GL_TEXTURE_2D,
            //             0,
            //             GL_LUMINANCE as i32,
            //             instance.real_width as i32,
            //             instance.real_height as i32,
            //             0,
            //             GL_LUMINANCE,
            //             GL_UNSIGNED_BYTE,
            //             &[] as &[u8],
            //         );
            //     }
            //     TextureFormat::Rgba32Float => {
            //         gl::tex_image_2d(
            //             GL_TEXTURE_2D,
            //             0,
            //             GL_RGBA as i32,
            //             instance.real_width as i32,
            //             instance.real_height as i32,
            //             0,
            //             GL_RGBA,
            //             GL_FLOAT,
            //             &[] as &[u8],
            //         );
            //     }
            //     TextureFormat::Rgba8Uint => {
            //         gl::tex_image_2d(
            //             GL_TEXTURE_2D,
            //             0,
            //             GL_RGBA as i32,
            //             instance.real_width as i32,
            //             instance.real_height as i32,
            //             0,
            //             GL_RGBA,
            //             GL_UNSIGNED_BYTE,
            //             &[] as &[u8],
            //         );
            //     }
            //     _ => {
            //         gl::tex_image_2d(
            //             GL_TEXTURE_2D,
            //             0,
            //             GL_RGBA as i32,
            //             instance.real_width as i32,
            //             instance.real_height as i32,
            //             0,
            //             gl::RGBA,
            //             gl::UNSIGNED_BYTE,
            //             &[] as &[u8],
            //         );
            //     }
            // }
        }
        // Sys.gl.generateMipmap(Sys.gl.TEXTURE_2D);

        // // return to default
        // gl::bind_texture(gl::TEXTURE_2D, 0);

        instance
    }

    pub fn create(width: u32, height: u32, format: TextureFormat) -> Image {
        Image::new(width, height, format, false)
    }

    // // android only
    // // @:allow(LoaderImpl)
    // // static createFromFile
    // pub fn from_file(filename: String) -> Image {
    //     let b = BitmapFactory.decodeStream(assets.open(filename)); // IO Exception here
    //     let image = Image::new(b.getWidth(), b.getHeight(), TextureFormat.RGBA32, false, DepthStencilFormat.NoDepthAndStencil);
    //     gl::bind_texture(gl::TEXTURE_2D, image.tex);

    //     gl::tex_image_2d(gl::TEXTURE_2D, 0, gl::RGBA, image.realWidth, image.realHeight, 0, gl::RGBA, gl::UNSIGNED_BYTE, None);

    //     GLUtils.texSubImage2D(gl::TEXTURE_2D, 0, 0, 0, b, gl::RGBA, gl::UNSIGNED_BYTE);

    //     // let buffer = ByteBuffer.allocateDirect(b.getWidth() * b.getHeight() * 4);
    //     // b.copyPixelsToBuffer(buffer);
    //     // gl::glTexImage2D(gl::TEXTURE_2D, 0, gl::RGBA, image.realWidth, image.realHeight, 0, gl::RGBA, gl::UNSIGNED_BYTE, buffer);
    //     // gl::tex_sub_image_2d(gl::TEXTURE_2D, 0, 0, 0, b.getWidth(), b.getHeight(), gl::RGBA, gl::UNSIGNED_BYTE, buffer);

    //     // GLUtils.texImage2D(gl::TEXTURE_2D, 0, b, 0);

    //     image
    // }

    // native only
    // TODO: we should also handle with compressed types
    pub fn from_file(filename: &str) -> Image {
        use ::image::ColorType;

        // Use tightly packed data
        gl::pixel_storei(GL_UNPACK_ALIGNMENT, 1); // ?

        let im = ::image::open(filename).unwrap();

        // // image color type
        // match im.color() {
        //     // Pixel is 8-bit luminance
        //     ColorType::L8 => {
        //         // mean grayscale
        //     }
        //     // Pixel contains 8-bit R, G and B channels
        //     ColorType::Rgb8 => {}
        //     // Pixel is 8-bit RGB with an alpha channel
        //     ColorType::Rgba8 => {}
        //     _ => {
        //         panic!("Unsupported color format for image {}", filename)
        //     }
        // }

        // seems we handle only rgba8 images

        let im = im.to_rgba8();

        Self::from_bytes(
            im.as_raw().as_slice(),
            im.width(),
            im.height(),
            TextureFormat::Rgba8Uint,
        )
    }

    pub fn from_bytes(bytes: &[u8], width: u32, height: u32, format: TextureFormat) -> Image {
        let image = Image::new(width, height, format, false);

        gl::bind_texture(GL_TEXTURE_2D, image.tex.unwrap());

        // Load the texture
        gl::tex_image_2d(
            GL_TEXTURE_2D,
            0,
            GL_RGBA as i32,
            width as i32,
            height as i32,
            0,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            bytes,
        );
        image
    }

    // static
    pub fn create_3d(width: i32, height: i32, depth: i32, format: TextureFormat) -> Image {
        unimplemented!()
    }

    // // static
    pub fn create_render_target(
        width: u32,
        height: u32,
        format: TextureFormat,
        anti_aliasing_samples: u32, /*= 1*/
        context_id: u32,            /*= 0*/
    ) -> Image {
        Image::new(width, height, format, true)
    }

    // static
    pub fn from_bytes_3d(
        bytes: Bytes,
        width: u32,
        height: u32,
        depth: u32,
        format: TextureFormat,
    ) -> Image {
        unimplemented!()
    }

    // TODO: should deal later
    fn run_depth_and_stencil_setup_chain(&self) {
        println!("runDepthAndStencilSetupChain");
        // let setupModes = [self.setup_oesExtension(), self.setup_separate_buffers()];
        // let succeeded = false;

        // for setup in setupModes {
        //     let result = setup();
        //     Self::logFramebufferStatus(result);

        //     if result == GL_FRAMEBUFFER_COMPLETE {
        //         succeeded = true;
        //         log::info!("working depth/stencil combination found");
        //         break;
        //     }
        //     log::info!("trying next setup");
        // }

        // if !succeeded {
        //     log::info!("no valid depth/stencil combination found");
        // }
    }

    fn setup_depth_buffer_only(&mut self) -> u32 {
        log::info!("GL_DEPTH_COMPONENT16 setup");

        let depth_buffer = gl::gen_renderbuffer();

        gl::bind_renderbuffer(GL_RENDERBUFFER, depth_buffer);
        gl::renderbuffer_storage(
            GL_RENDERBUFFER,
            GL_DEPTH_COMPONENT16,
            self.real_width as i32,
            self.real_height as i32,
        );
        gl::framebuffer_renderbuffer(
            GL_FRAMEBUFFER,
            GL_DEPTH_ATTACHMENT,
            GL_RENDERBUFFER,
            depth_buffer,
        );

        gl::bind_renderbuffer(GL_RENDERBUFFER, 0);

        let result = gl::check_framebuffer_status(GL_FRAMEBUFFER);

        // TODO: WTF?
        if result != GL_FRAMEBUFFER_COMPLETE {
            gl::delete_renderbuffers(&[depth_buffer]);
        }

        self.depth_buffer = Some(depth_buffer);

        result
    }

    fn setup_oes_extension(&mut self) -> u32 {
        log::info!("GL_OES_packed_depth_stencil");

        let depth_buffer = gl::gen_renderbuffer(); // TODO: WTF?

        gl::bind_texture(GL_TEXTURE_2D, depth_buffer);

        gl::tex_parameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);

        // TODO: // need to deal urgent // DV
        // gl::tex_image_2d(GL_TEXTURE_2D, 0, GLES11Ext::GL_DEPTH_STENCIL_OES, self.myRealWidth, self.myRealWidth, 0, GLES11Ext.GL_DEPTH_STENCIL_OES,
        // 	GLES11Ext::GL_UNSIGNED_INT_24_8_OES, None);

        gl::framebuffer_texture_2d(
            GL_FRAMEBUFFER,
            GL_DEPTH_ATTACHMENT,
            GL_TEXTURE_2D,
            depth_buffer,
            0,
        );
        gl::framebuffer_texture_2d(
            GL_FRAMEBUFFER,
            GL_STENCIL_ATTACHMENT,
            GL_TEXTURE_2D,
            depth_buffer,
            0,
        );
        gl::bind_texture(GL_TEXTURE_2D, 0);

        let result = gl::check_framebuffer_status(GL_FRAMEBUFFER);

        // TODO: WTF?
        if result != GL_FRAMEBUFFER_COMPLETE {
            gl::delete_renderbuffers(&[depth_buffer]);
        }

        self.depth_buffer = Some(depth_buffer);

        result
    }

    // TODO (DK)
    //	-doesn't fail, but doesn't work on my htc desire x -.-
    //	-fails on galaxy s4 at work
    fn setup_separate_buffers(&mut self) -> u32 {
        log::info!("GL_DEPTH_COMPONENT16 / GL_STENCIL_INDEX8 setup");

        // self.depthStencilBuffers = gl::gen_renderbuffers(2);

        // let depthBuffer = self.depthStencilBuffers[0];
        // let stencilBuffer = self.depthStencilBuffers[1];

        // gl::bind_renderbuffer(GL_RENDERBUFFER, depthBuffer);
        // gl::renderbuffer_storage(
        //     GL_RENDERBUFFER,
        //     GL_DEPTH_COMPONENT16,
        //     self.real_width as i32,
        //     self.real_height as i32,
        // );

        // gl::bind_renderbuffer(GL_RENDERBUFFER, stencilBuffer);
        // gl::renderbuffer_storage(
        //     GL_RENDERBUFFER,
        //     GL_STENCIL_INDEX8,
        //     self.real_width as i32,
        //     self.real_height as i32,
        // );

        // gl::framebuffer_renderbuffer(
        //     GL_FRAMEBUFFER,
        //     GL_DEPTH_ATTACHMENT,
        //     GL_RENDERBUFFER,
        //     depthBuffer,
        // );
        // gl::framebuffer_renderbuffer(
        //     GL_FRAMEBUFFER,
        //     GL_STENCIL_ATTACHMENT,
        //     GL_RENDERBUFFER,
        //     stencilBuffer,
        // );

        // gl::bind_renderbuffer(GL_RENDERBUFFER, 0);

        // let result = gl::check_framebuffer_status(GL_FRAMEBUFFER);

        // if result != GL_FRAMEBUFFER_COMPLETE {
        //     gl::delete_renderbuffers(self.depthStencilBuffers.as_slice());
        // }

        // result
        unimplemented!()
    }

    // static
    fn convert_framebuffer_status(status: u32) -> String {
        let message = if status == GL_FRAMEBUFFER_COMPLETE {
            "complete"
        } else if status == GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT {
            "incomplete attachments"
        } else if status == GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT {
            "incomplete missing attachments"
        } else if status == GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS {
            "incomplete dimensions"
        } else if status == GL_FRAMEBUFFER_UNSUPPORTED {
            "invalid combination of attachments"
        } else {
            "unknown"
        };

        message.into()
    }

    //static
    fn log_framebuffer_status(status: u32) {
        let message = Self::convert_framebuffer_status(status);
        log::info!("framebuffer status '{}'", message);
    }

    // pub let g1(get, never): graphics1.Graphics;

    // fn get_g1(): graphics1.Graphics {
    // 	if graphics1 == None {
    // 		graphics1 = new graphics2.Graphics1(this);
    // 	}
    // 	return graphics1;
    // }

    // pub let g2(get, never): graphics2.Graphics;

    // fn get_g2(): graphics2.Graphics {
    // 	if graphics2 == None {
    // 		graphics2 = new graphics4.Graphics2(this);
    // 	}
    // 	return graphics2;
    // }

    // pub let g4(get, never): graphics4.Graphics;

    // fn get_g4(): graphics4.Graphics {
    // 	if graphics4 == None {
    // 		graphics4 = new android.Graphics(this);
    // 	}
    // 	return graphics4;
    // }

    pub fn unload(&mut self) {
        if self.tex.is_some() {
            gl::delete_textures(&[self.tex.unwrap()]);
            self.tex = None;
        }

        if self.framebuffer.is_some() {
            gl::delete_buffers(&[self.framebuffer.unwrap()]);
            self.framebuffer = None;
        }

        if let Some(buffer) = self.depth_buffer {
            gl::delete_renderbuffers(&[buffer]);
        }

        if let Some(buffer) = self.stencil_buffer {
            gl::delete_renderbuffers(&[buffer]);
        }
    }

    // static
    fn create_framebuffer() -> u32 {
        let framebuffers = gl::gen_framebuffers(1);
        framebuffers[0]
    }

    // static
    fn create_texture() -> u32 {
        let textures = gl::gen_textures(1);
        textures[0]
    }

    pub fn make_active(&self, texture_unit: u32) {
        gl::active_texture(GL_TEXTURE0 + texture_unit);
        gl::bind_texture(GL_TEXTURE_2D, self.tex.unwrap());
    }

    // pub let width(get, never) -> i32;

    fn width(&self) -> u32 {
        self.width
    }

    // pub let height(get, never) -> i32;

    fn height(&self) -> u32 {
        self.height
    }

    // pub let depth(get, never) -> i32;

    fn depth(&self) -> u32 {
        1
    }

    // pub let format(get, never): TextureFormat;

    fn format(&self) -> TextureFormat {
        self.format
    }

    // pub let realWidth(get, never) -> i32;

    fn real_width(&self) -> u32 {
        self.real_width
    }

    // pub let realHeight(get, never) -> i32;

    fn real_height(&self) -> u32 {
        self.real_height
    }

    // pub let stride(get, never) -> i32;

    fn stride(&self) -> u32 {
        // if self.myFormat == TextureFormat::RGBA32 {
        //     4 * width
        // } else {
        //     if self.myFormat == TextureFormat::RGBA128 {
        //         16 * width
        //     } else {
        //         width
        //     }
        // }
        unimplemented!()
    }

    pub fn at(x: i32, y: i32) -> i32 {
        0
    }

    pub fn is_opaque(x: i32, y: i32) -> bool {
        // return (b.getPixel(x, y) >> 24) != 0;
        true
    }

    pub fn lock(&self, level: i32 /* = 0*/) -> Bytes {
        // let len = if self.myFormat == TextureFormat::RGBA32 {
        //     4 * width * height
        // } else {
        //     if myFormat == TextureFormat::RGBA128 {
        //         16 * width * height
        //     } else {
        //         width * height
        //     }
        // };

        // self.bytes = Bytes.alloc(len);
        // self.bytes
        unimplemented!()
    }

    pub fn unlock(&self) {
        // gl::bind_texture(gl::TEXTURE_2D, self.tex);
        // // Sys.gl.pixelStorei(Sys.gl.UNPACK_FLIP_Y_WEBGL, true);

        // match self.myFormat {
        // 	TextureFormat::L8 => {
        //         gl::tex_sub_image_2d(gl::TEXTURE_2D, 0, 0, 0, width, height, gl::LUMINANCE, gl::UNSIGNED_BYTE,
        // 			ByteBuffer.wrap(bytes.getData()));
        //     }
        // 	TextureFormat::RGBA128 => {
        // 		gl::tex_sub_image_2d(gl::TEXTURE_2D, 0, 0, 0, width, height, gl::RGBA, gl::FLOAT, ByteBuffer.wrap(bytes.getData()));
        //     }
        // 	TextureFormat::RGBA32 => {
        // 		gl::tex_sub_image_2d(gl::TEXTURE_2D, 0, 0, 0, width, height, gl::RGBA, gl::UNSIGNED_BYTE,
        // 			ByteBuffer.wrap(bytes.getData()));
        //     }
        // 	_ => {
        // 		gl::tex_sub_image_2d(gl::TEXTURE_2D, 0, 0, 0, width, height, gl::RGBA, gl::UNSIGNED_BYTE,
        // 			ByteBuffer.wrap(bytes.getData()));
        //     }
        // }

        // // Sys.gl.generateMipmap(Sys.gl.TEXTURE_2D);
        // gl::bind_texture(gl::TEXTURE_2D, 0);
        // bytes = None;
        unimplemented!()
    }

    pub fn pixels(&self) -> Bytes {
        // return None;
        unimplemented!()
    }

    pub fn generate_mipmaps(&self, levels: i32) {}

    pub fn set_mipmaps(&self, mipmaps: Vec<Image>) {}

    pub fn set_depth_stencil_from(&self, image: Image) {}

    pub fn clear(x: i32, y: i32, z: i32, width: i32, height: i32, depth: i32, color: Color) {}

    // static
    // pub let maxSize(get, never) -> i32;

    // static
    fn max_size(&self) -> i32 {
        2048
    }

    // static
    // pub let nonPow2Supported(get, never) -> bool {};

    // static
    fn non_pow2_supported(&self) -> bool {
        false
    }

    // pub static
    fn render_targets_inverted_y(&self) -> bool {
        true
    }

    // static
    fn upper_power_of_two(v: u32) -> u32 {
        // let mut res = v - 1;
        // res |= res >> 1;
        // res |= res >> 2;
        // res |= res >> 4;
        // res |= res >> 8;
        // res |= res >> 16;
        // res += 1;
        // res
        v
    }
}

impl<C> Load<C, StoreKey> for Image {
    type Error = Error;

    fn load(
        key: StoreKey,
        storage: &mut Storage<C, StoreKey>,
        _: &mut C,
    ) -> Result<Loaded<Self, StoreKey>, Self::Error> {
        // as we only accept filesystem here, we’ll ensure the key is a filesystem one
        match key.0 {
            SimpleKey::Path(path) => {
                let im = ::image::open(&path)
                    .map_err(|e| Error::IOError(format!("{}: {:?}", e, &path)))?;

                let data = im.to_rgba8();

                let result = Self::from_bytes(
                    data.as_raw().as_slice(),
                    data.width(),
                    data.height(),
                    TextureFormat::Rgba8Uint,
                );

                Ok(result.into())
            }

            SimpleKey::Logical(_) => Err(Error::CannotLoadFromLogical(
                String::from("Load from logical not implemented"),
                String::from("No reason"),
            )),
        }
    }
}
