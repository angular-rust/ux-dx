use crate::Texture;
use std::fmt;

pub struct Offscreen {
}

impl Offscreen {
    // // * cogl_offscreen_new_with_texture:
    // // * @texture: A #CoglTexture pointer
    // // *
    // // * This creates an offscreen framebuffer object using the given
    // // * @texture as the primary color buffer. It doesn't just initialize
    // // * the contents of the offscreen buffer with the @texture; they are
    // // * tightly bound so that drawing to the offscreen buffer effectively
    // // * updates the contents of the given texture. You don't need to
    // // * destroy the offscreen buffer before you can use the @texture again.
    // // *
    // // * <note>This api only works with low-level #CoglTexture types such as
    // // * #CoglTexture2D, #CoglTexture3D and #CoglTextureRectangle, and not
    // // * with meta-texture types such as #CoglTexture2DSliced.</note>
    // // *
    // // * The storage for the framebuffer is actually allocated lazily
    // // * so this function will never return %NULL to indicate a runtime
    // // * error. This means it is still possible to configure the framebuffer
    // // * before it is really allocated.
    // // *
    // // * Simple applications without full error handling can simply rely on
    // // * Cogl to lazily allocate the storage of framebuffers but you should
    // // * be aware that if Cogl encounters an error (such as running out of
    // // * GPU memory) then your application will simply abort with an error
    // // * message. If you need to be able to catch such exceptions at runtime
    // // * then you can explicitly allocate your framebuffer when you have
    // // * finished configuring it by calling cogl_framebuffer_allocate() and
    // // * passing in a #CoglError argument to catch any exceptions.
    // // *
    // // * Return value: (transfer full): a newly instantiated #CoglOffscreen
    // // *   framebuffer.
    // pub fn with_texture<P: IsA<Texture>>(texture: &P) -> Offscreen {
    //     unsafe {
    //         ffi::cogl_offscreen_new_with_texture
    //     }
    // }
}

impl fmt::Display for Offscreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offscreen")
    }
}
