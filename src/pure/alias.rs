pub type Angle = i32;
// pub type Bool = i32;
pub type Buffer = ();
// pub type Handle = /*Unimplemented*/Fundamental: Pointer;
pub type MetaTexture = ();

// * SECTION:cogl-primitive-texture
// * @short_description: Interface for low-level textures like
// *                     #CoglTexture2D and #CoglTexture3D.
// *
// * A #CoglPrimitiveTexture is a texture that is directly represented
// * by a single texture on the GPU. For example these could be a
// * #CoglTexture2D, #CoglTexture3D or #CoglTextureRectangle. This is
// * opposed to high level meta textures which may be composed of
// * multiple primitive textures or a sub-region of another texture such
// * as #CoglAtlasTexture and #CoglTexture2DSliced.
// *
// * A texture that implements this interface can be directly used with
// * the low level cogl_primitive_draw() API. Other types of textures
// * need to be first resolved to primitive textures using the
// * #CoglMetaTexture interface.
// *
// * <note>Most developers won't need to use this interface directly but
// * still it is worth understanding the distinction between high-level
// * and primitive textures because you may find other references in the
// * documentation that detail limitations of using
// * primitive textures.</note>
pub type PrimitiveTexture = ();
pub type UserDataDestroyCallback = Box<dyn Fn() + 'static>;
