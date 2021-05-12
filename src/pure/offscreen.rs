use crate::Texture;
use std::fmt;

pub struct Offscreen {
}

impl Offscreen {
    // // * offscreen_new_with_texture:
    // // * @texture: A #Texture pointer
    // // *
    // // * This creates an offscreen framebuffer object using the given
    // // * @texture as the primary color buffer. It doesn't just initialize
    // // * the contents of the offscreen buffer with the @texture; they are
    // // * tightly bound so that drawing to the offscreen buffer effectively
    // // * updates the contents of the given texture. You don't need to
    // // * destroy the offscreen buffer before you can use the @texture again.
    // // *
    // // * <note>This api only works with low-level #Texture types such as
    // // * #Texture2D, #Texture3D and #TextureRectangle, and not
    // // * with meta-texture types such as #Texture2DSliced.</note>
    // // *
    // // * The storage for the framebuffer is actually allocated lazily
    // // * so this function will never return %NULL to indicate a runtime
    // // * error. This means it is still possible to configure the framebuffer
    // // * before it is really allocated.
    // // *
    // // * Simple applications without full error handling can simply rely on
    // // *  to lazily allocate the storage of framebuffers but you should
    // // * be aware that if  encounters an error (such as running out of
    // // * GPU memory) then your application will simply abort with an error
    // // * message. If you need to be able to catch such exceptions at runtime
    // // * then you can explicitly allocate your framebuffer when you have
    // // * finished configuring it by calling framebuffer_allocate() and
    // // * passing in a #Error argument to catch any exceptions.
    // // *
    // // * Return value: (transfer full): a newly instantiated #Offscreen
    // // *   framebuffer.
    // pub fn with_texture<P: IsA<Texture>>(texture: &P) -> Offscreen {
    //     unsafe {
    //         ffi::offscreen_new_with_texture
    //     }
    // }
}

impl fmt::Display for Offscreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offscreen")
    }
}
