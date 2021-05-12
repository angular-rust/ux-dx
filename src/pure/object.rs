use std::fmt;

pub struct Object {
    // ObjectClass  *klass; /* equivalent to GTypeInstance */

    // UserDataEntry user_data_entry[
    //   COGL_OBJECT_N_PRE_ALLOCATED_USER_DATA_ENTRIES];
    // GArray           *user_data_array;
    // int               n_user_data_entries;
  
    // unsigned int      ref_count;
}

impl Object {
    //pub fn ref_(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unimplemented!()
    //}

    //pub fn unref(object: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unimplemented!()
    //}

    //pub fn value_get_object(value: &Value) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unimplemented!()
    //}

    //pub fn value_set_object(value: &mut Value, object: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unimplemented!()
    //}
}

/// Trait containing all `Object` methods.
///
/// # Implementors
///
/// [`AtlasTexture`](struct.AtlasTexture.html), [`AttributeBuffer`](struct.AttributeBuffer.html), [`Attribute`](struct.Attribute.html), [`Bitmap`](struct.Bitmap.html), [`Context`](struct.Context.html), [`Display`](struct.Display.html), [`FrameInfo`](struct.FrameInfo.html), [`Framebuffer`](struct.Framebuffer.html), [`GLES2Context`](struct.GLES2Context.html), [`IndexBuffer`](struct.IndexBuffer.html), [`Indices`](struct.Indices.html), [`MatrixStack`](struct.MatrixStack.html), [`Object`](struct.Object.html), [`OnscreenTemplate`](struct.OnscreenTemplate.html), [`Onscreen`](struct.Onscreen.html), [`Output`](struct.Output.html), [`Pipeline`](struct.Pipeline.html), [`PixelBuffer`](struct.PixelBuffer.html), [`Primitive`](struct.Primitive.html), [`Renderer`](struct.Renderer.html), [`Snippet`](struct.Snippet.html), [`SubTexture`](struct.SubTexture.html), [`SwapChain`](struct.SwapChain.html), [`Texture2DSliced`](struct.Texture2DSliced.html), [`Texture2D`](struct.Texture2D.html), [`Texture3D`](struct.Texture3D.html), [`TexturePixmapX11`](struct.TexturePixmapX11.html), [`TextureRectangle`](struct.TextureRectangle.html), [`Texture`](struct.Texture.html)
pub trait ObjectExt: 'static {
    //fn get_user_data(&self, key: &mut UserDataKey) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    //fn set_user_data(&self, key: &mut UserDataKey, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: UserDataDestroyCallback);
}

// impl<O: Is<Object>> ObjectExt for O {
//     //fn get_user_data(&self, key: &mut UserDataKey) -> /*Unimplemented*/Option<Fundamental: Pointer> {
//     //    unsafe { TODO: call sys:object_get_user_data() }
//     //}

//     //fn set_user_data(&self, key: &mut UserDataKey, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: UserDataDestroyCallback) {
//     //    unsafe { TODO: call sys:object_set_user_data() }
//     //}
// }

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object")
    }
}
