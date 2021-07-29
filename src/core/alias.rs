use std::ffi::c_void;

pub type Angle = i32;
pub type Handle = c_void;
pub type MetaTexture = ();

// SECTION:primitive-texture
// @short_description: Interface for low-level textures like
//                     #Texture2D and #Texture3D.
//
// A #PrimitiveTexture is a texture that is directly represented
// by a single texture on the GPU. For example these could be a
// #Texture2D, #Texture3D or #TextureRectangle. This is
// opposed to high level meta textures which may be composed of
// multiple primitive textures or a sub-region of another texture such
// as #AtlasTexture and #Texture2DSliced.
//
// A texture that implements this interface can be directly used with
// the low level primitive_draw() API. Other types of textures
// need to be first resolved to primitive textures using the
// #MetaTexture interface.
//
// <note>Most developers won't need to use this interface directly but
// still it is worth understanding the distinction between high-level
// and primitive textures because you may find other references in the
// documentation that detail limitations of using
// primitive textures.</note>
pub type PrimitiveTexture = ();
pub type UserDataDestroyCallback = Box<dyn Fn() + 'static>;
