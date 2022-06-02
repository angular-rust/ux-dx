use std::{fmt, rc::Rc};

use crate::prelude::*;

use super::{OffscreenAllocateFlags, OffscreenFlags, Texture};

#[derive(Debug)]
pub struct Offscreen {
    // Framebuffer  _parent;
    // GLFramebuffer gl_framebuffer;
    pub texture: Rc<Texture>,
    pub texture_level: u32,

    pub depth_texture: Option<Texture>,

    pub allocation_flags: OffscreenAllocateFlags,

    // FIXME: _offscreen_new_with_texture_full should be made to use
    // fb->config to configure if we want a depth or stencil buffer so
    // we can get rid of these flags
    pub create_flags: OffscreenFlags,
}

impl Offscreen {
    // offscreen_new_with_texture:
    // @texture: A #Texture pointer
    //
    // This creates an offscreen framebuffer object using the given
    // @texture as the primary color buffer. It doesn't just initialize
    // the contents of the offscreen buffer with the @texture; they are
    // tightly bound so that drawing to the offscreen buffer effectively
    // updates the contents of the given texture. You don't need to
    // destroy the offscreen buffer before you can use the @texture again.
    //
    // This api only works with low-level #Texture types such as
    // #Texture2D, #Texture3D and #TextureRectangle, and not
    // with meta-texture types such as #Texture2DSliced.
    //
    // The storage for the framebuffer is actually allocated lazily
    // so this fn will never return %NULL to indicate a runtime
    // error. This means it is still possible to configure the framebuffer
    // before it is really allocated.
    //
    // Simple applications without full error handling can simply rely on
    //  to lazily allocate the storage of framebuffers but you should
    // be aware that if  encounters an error (such as running out of
    // GPU memory) then your application will simply abort with an error
    // message. If you need to be able to catch such exceptions at runtime
    // then you can explicitly allocate your framebuffer when you have
    // finished configuring it by calling framebuffer_allocate() and
    // passing in a #Error argument to catch any exceptions.
    //
    // Return value: (transfer full): a newly instantiated #Offscreen
    // framebuffer.
    pub fn with_texture(texture: Rc<Texture>) -> Offscreen {
        // NB: we can't assume we can query the texture's width yet, since
        // it might not have been allocated yet and for example if the
        // texture is being loaded from a file then the file might not
        // have been read yet.

        // _framebuffer_init(fb,
        //                         ctx,
        //                         FRAMEBUFFER_TYPE_OFFSCREEN,
        //                         -1, // unknown width, until allocation
        //                         -1); // unknown height until allocation

        // _texture_associate_framebuffer(texture, fb);

        Self {
            texture,
            create_flags: OffscreenFlags::None,
            texture_level: 0,
            depth_texture: None,
            allocation_flags: OffscreenAllocateFlags::None,
        }
    }
}

impl fmt::Display for Offscreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offscreen")
    }
}
