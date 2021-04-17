use crate::{
    Color, ColorMask, Context, DepthState, Matrix, Object, PipelineAlphaFunc, PipelineCullFaceMode,
    PipelineFilter, PipelineWrapMode, Snippet, Texture, TextureType, Winding,
};

use glib::object::IsA;
use glib::translate::*;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Pipeline(Object<ffi::CoglPipeline, PipelineClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_pipeline_get_gtype(),
    }
}

impl Pipeline {
    /// Allocates and initializes a default simple pipeline that will color
    /// a primitive white.
    ///
    /// ## `context`
    /// a `Context`
    ///
    /// # Returns
    ///
    /// a pointer to a new `Pipeline`
    pub fn new(context: &Context) -> Pipeline {
        unsafe { from_glib_full(ffi::cogl_pipeline_new(context.to_glib_none().0)) }
    }

    /// Adds a shader snippet that will hook on to the given layer of the
    /// pipeline. The exact part of the pipeline that the snippet wraps
    /// around depends on the hook that is given to
    /// `Snippet::new`. Note that some hooks can't be used with a layer
    /// and need to be added with `Pipeline::add_snippet` instead.
    /// ## `layer`
    /// The layer to hook the snippet to
    /// ## `snippet`
    /// A `Snippet`
    pub fn add_layer_snippet(&self, layer: i32, snippet: &Snippet) {
        unsafe {
            ffi::cogl_pipeline_add_layer_snippet(
                self.to_glib_none().0,
                layer,
                snippet.to_glib_none().0,
            );
        }
    }

    /// Adds a shader snippet to `self`. The snippet will wrap around or
    /// replace some part of the pipeline as defined by the hook point in
    /// `snippet`. Note that some hook points are specific to a layer and
    /// must be added with `Pipeline::add_layer_snippet` instead.
    /// ## `snippet`
    /// The `Snippet` to add to the vertex processing hook
    pub fn add_snippet(&self, snippet: &Snippet) {
        unsafe {
            ffi::cogl_pipeline_add_snippet(self.to_glib_none().0, snippet.to_glib_none().0);
        }
    }

    /// Creates a new pipeline with the configuration copied from the
    /// source pipeline.
    ///
    /// We would strongly advise developers to always aim to use
    /// `Pipeline::copy` instead of `Pipeline::new` whenever there will
    /// be any similarity between two pipelines. Copying a pipeline helps Cogl
    /// keep track of a pipelines ancestry which we may use to help minimize GPU
    /// state changes.
    ///
    ///
    /// # Returns
    ///
    /// a pointer to the newly allocated `Pipeline`
    pub fn copy(&self) -> Option<Pipeline> {
        unsafe { from_glib_full(ffi::cogl_pipeline_copy(self.to_glib_none().0)) }
    }

    /// Iterates all the layer indices of the given `self`.
    ///
    /// ## `callback`
    /// A `CoglPipelineLayerCallback` to be
    ///  called for each layer index
    /// ## `user_data`
    /// Private data that will be passed to the
    ///  callback
    pub fn foreach_layer<P: FnMut(&Pipeline, i32) -> i32>(&self, callback: P) {
        //TODO: should replace i32 to bool in callback
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Pipeline, i32) -> i32>(
            pipeline: *mut ffi::CoglPipeline,
            layer_index: libc::c_int,
            user_data: glib_sys::gpointer,
        ) -> ffi::CoglBool {
            let pipeline = from_glib_borrow(pipeline);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&pipeline, layer_index)
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::cogl_pipeline_foreach_layer(
                self.to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    ///
    ///
    /// # Returns
    ///
    /// The alpha test function of `self`.
    pub fn get_alpha_test_function(&self) -> PipelineAlphaFunc {
        unsafe {
            from_glib(ffi::cogl_pipeline_get_alpha_test_function(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    ///
    /// # Returns
    ///
    /// The alpha test reference value of `self`.
    pub fn get_alpha_test_reference(&self) -> f32 {
        unsafe { ffi::cogl_pipeline_get_alpha_test_reference(self.to_glib_none().0) }
    }

    /// Retrieves the current ambient color for `self`
    ///
    /// ## `ambient`
    /// The location to store the ambient color
    pub fn get_ambient(&self, ambient: &mut Color) {
        unsafe {
            ffi::cogl_pipeline_get_ambient(self.to_glib_none().0, ambient.to_glib_none_mut().0);
        }
    }

    /// Retrieves the current pipeline color.
    ///
    /// ## `color`
    /// The location to store the color
    pub fn get_color(&self) -> Color {
        unsafe {
            let mut color = Color::uninitialized();
            ffi::cogl_pipeline_get_color(self.to_glib_none().0, color.to_glib_none_mut().0);
            color
        }
    }

    /// Gets the current `ColorMask` of which channels would be written to the
    /// current framebuffer. Each bit set in the mask means that the
    /// corresponding color would be written.
    ///
    /// # Returns
    ///
    /// A `ColorMask`
    pub fn get_color_mask(&self) -> ColorMask {
        unsafe { from_glib(ffi::cogl_pipeline_get_color_mask(self.to_glib_none().0)) }
    }

    ///
    ///
    /// # Returns
    ///
    /// the cull face mode that was previously set with
    /// `Pipeline::set_cull_face_mode`.
    ///
    /// Status: Unstable
    pub fn get_cull_face_mode(&self) -> PipelineCullFaceMode {
        unsafe { from_glib(ffi::cogl_pipeline_get_cull_face_mode(self.to_glib_none().0)) }
    }

    /// Retrieves the current depth state configuration for the given
    /// `self` as previously set using `Pipeline::set_depth_state`.
    ///
    /// ## `state_out`
    /// A destination `DepthState` struct
    pub fn get_depth_state(&self) -> DepthState {
        unsafe {
            let mut state_out = DepthState::uninitialized();
            ffi::cogl_pipeline_get_depth_state(
                self.to_glib_none().0,
                state_out.to_glib_none_mut().0,
            );
            state_out
        }
    }

    /// Retrieves the current diffuse color for `self`
    ///
    /// ## `diffuse`
    /// The location to store the diffuse color
    pub fn get_diffuse(&self, diffuse: &mut Color) {
        unsafe {
            ffi::cogl_pipeline_get_diffuse(self.to_glib_none().0, diffuse.to_glib_none_mut().0);
        }
    }

    /// Retrieves the pipelines current emission color.
    ///
    /// ## `emission`
    /// The location to store the emission color
    pub fn get_emission(&self, emission: &mut Color) {
        unsafe {
            ffi::cogl_pipeline_get_emission(self.to_glib_none().0, emission.to_glib_none_mut().0);
        }
    }

    /// The order of the vertices within a primitive specifies whether it
    /// is considered to be front or back facing. This function specifies
    /// which order is considered to be the front
    /// faces. `Winding::CounterClockwise` sets the front faces to
    /// primitives with vertices in a counter-clockwise order and
    /// `Winding::Clockwise` sets them to be clockwise. The default is
    /// `Winding::CounterClockwise`.
    ///
    ///
    /// # Returns
    ///
    /// The `self` front face winding
    ///
    /// Status: Unstable
    pub fn get_front_face_winding(&self) -> Winding {
        unsafe {
            from_glib(ffi::cogl_pipeline_get_front_face_winding(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the currently set magnification `PipelineFilter` set on
    /// the specified layer. The magnification filter determines how the
    /// layer should be sampled when up-scaled.
    ///
    /// The default filter is `PipelineFilter::Linear` but this can be
    /// changed using `Pipeline::set_layer_filters`.
    /// ## `layer_index`
    /// the layer number to change.
    ///
    /// # Returns
    ///
    /// The magnification `PipelineFilter` for the
    ///  specified layer.
    pub fn get_layer_mag_filter(&self, layer_index: i32) -> PipelineFilter {
        unsafe {
            from_glib(ffi::cogl_pipeline_get_layer_mag_filter(
                self.to_glib_none().0,
                layer_index,
            ))
        }
    }

    /// Retrieves the currently set minification `PipelineFilter` set on
    /// the specified layer. The miniifcation filter determines how the
    /// layer should be sampled when down-scaled.
    ///
    /// The default filter is `PipelineFilter::Linear` but this can be
    /// changed using `Pipeline::set_layer_filters`.
    /// ## `layer_index`
    /// the layer number to change.
    ///
    /// # Returns
    ///
    /// The minification `PipelineFilter` for the
    ///  specified layer.
    pub fn get_layer_min_filter(&self, layer_index: i32) -> PipelineFilter {
        unsafe {
            from_glib(ffi::cogl_pipeline_get_layer_min_filter(
                self.to_glib_none().0,
                layer_index,
            ))
        }
    }

    /// Gets whether point sprite coordinate generation is enabled for this
    /// texture layer.
    ///
    /// ## `layer_index`
    /// the layer number to check.
    ///
    /// # Returns
    ///
    /// whether the texture coordinates will be replaced with
    /// point sprite coordinates.
    pub fn get_layer_point_sprite_coords_enabled(&self, layer_index: i32) -> bool {
        unsafe {
            ffi::cogl_pipeline_get_layer_point_sprite_coords_enabled(
                self.to_glib_none().0,
                layer_index,
            ) == crate::TRUE
        }
    }

    /// ## `layer_index`
    /// the index of the layer
    ///
    /// # Returns
    ///
    /// the texture that was set for the
    ///  given layer of the pipeline or `None` if no texture was set.
    pub fn get_layer_texture(&self, layer_index: i32) -> Option<Texture> {
        unsafe {
            from_glib_none(ffi::cogl_pipeline_get_layer_texture(
                self.to_glib_none().0,
                layer_index,
            ))
        }
    }

    /// Returns the wrap mode for the 'p' coordinate of texture lookups on this
    /// layer.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    ///
    /// # Returns
    ///
    /// the wrap mode for the 'p' coordinate of texture lookups on
    /// this layer.
    pub fn get_layer_wrap_mode_p(&self, layer_index: i32) -> PipelineWrapMode {
        unsafe {
            from_glib(ffi::cogl_pipeline_get_layer_wrap_mode_p(
                self.to_glib_none().0,
                layer_index,
            ))
        }
    }

    /// Returns the wrap mode for the 's' coordinate of texture lookups on this
    /// layer.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    ///
    /// # Returns
    ///
    /// the wrap mode for the 's' coordinate of texture lookups on
    /// this layer.
    pub fn get_layer_wrap_mode_s(&self, layer_index: i32) -> PipelineWrapMode {
        unsafe {
            from_glib(ffi::cogl_pipeline_get_layer_wrap_mode_s(
                self.to_glib_none().0,
                layer_index,
            ))
        }
    }

    /// Returns the wrap mode for the 't' coordinate of texture lookups on this
    /// layer.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    ///
    /// # Returns
    ///
    /// the wrap mode for the 't' coordinate of texture lookups on
    /// this layer.
    pub fn get_layer_wrap_mode_t(&self, layer_index: i32) -> PipelineWrapMode {
        unsafe {
            from_glib(ffi::cogl_pipeline_get_layer_wrap_mode_t(
                self.to_glib_none().0,
                layer_index,
            ))
        }
    }

    /// Retrieves the number of layers defined for the given `self`
    ///
    ///
    /// # Returns
    ///
    /// the number of layers
    pub fn get_n_layers(&self) -> i32 {
        unsafe { ffi::cogl_pipeline_get_n_layers(self.to_glib_none().0) }
    }

    ///
    ///
    /// # Returns
    ///
    /// `true` if the pipeline has per-vertex point size
    ///  enabled or `false` otherwise. The per-vertex point size can be
    ///  enabled with `Pipeline::set_per_vertex_point_size`.
    pub fn get_per_vertex_point_size(&self) -> bool {
        unsafe {
            ffi::cogl_pipeline_get_per_vertex_point_size(self.to_glib_none().0) == crate::TRUE
        }
    }

    /// Get the size of points drawn when `VerticesMode::Points` is
    /// used with the vertex buffer API.
    ///
    ///
    /// # Returns
    ///
    /// the point size of the `self`.
    pub fn get_point_size(&self) -> f32 {
        unsafe { ffi::cogl_pipeline_get_point_size(self.to_glib_none().0) }
    }

    /// Retrieves the pipelines current emission color.
    ///
    ///
    /// # Returns
    ///
    /// The pipelines current shininess value
    pub fn get_shininess(&self) -> f32 {
        unsafe { ffi::cogl_pipeline_get_shininess(self.to_glib_none().0) }
    }

    /// Retrieves the pipelines current specular color.
    ///
    /// ## `specular`
    /// The location to store the specular color
    pub fn get_specular(&self, specular: &mut Color) {
        unsafe {
            ffi::cogl_pipeline_get_specular(self.to_glib_none().0, specular.to_glib_none_mut().0);
        }
    }

    /// This is used to get an integer representing the uniform with the
    /// name `uniform_name`. The integer can be passed to functions such as
    /// `Pipeline::set_uniform_1f` to set the value of a uniform.
    ///
    /// This function will always return a valid integer. Ie, unlike
    /// OpenGL, it does not return -1 if the uniform is not available in
    /// this pipeline so it can not be used to test whether uniforms are
    /// present. It is not necessary to set the program on the pipeline
    /// before calling this function.
    ///
    /// ## `uniform_name`
    /// The name of a uniform
    ///
    /// # Returns
    ///
    /// A integer representing the location of the given uniform.
    pub fn get_uniform_location(&self, uniform_name: &str) -> i32 {
        unsafe {
            ffi::cogl_pipeline_get_uniform_location(
                self.to_glib_none().0,
                uniform_name.to_glib_none().0,
            )
        }
    }

    //pub fn get_user_program(&self) -> /*Unimplemented*/Option<Handle> {
    //    unsafe { TODO: call cogl_sys:cogl_pipeline_get_user_program() }
    //}

    /// This function removes a layer from your pipeline
    /// ## `layer_index`
    /// Specifies the layer you want to remove
    pub fn remove_layer(&self, layer_index: i32) {
        unsafe {
            ffi::cogl_pipeline_remove_layer(self.to_glib_none().0, layer_index);
        }
    }

    /// Before a primitive is blended with the framebuffer, it goes through an
    /// alpha test stage which lets you discard fragments based on the current
    /// alpha value. This function lets you change the function used to evaluate
    /// the alpha channel, and thus determine which fragments are discarded
    /// and which continue on to the blending stage.
    ///
    /// The default is `PipelineAlphaFunc::Always`
    ///
    /// ## `alpha_func`
    /// A `PipelineAlphaFunc` constant
    /// ## `alpha_reference`
    /// A reference point that the chosen alpha function uses
    ///  to compare incoming fragments to.
    pub fn set_alpha_test_function(&self, alpha_func: PipelineAlphaFunc, alpha_reference: f32) {
        unsafe {
            ffi::cogl_pipeline_set_alpha_test_function(
                self.to_glib_none().0,
                alpha_func.to_glib(),
                alpha_reference,
            );
        }
    }

    /// Sets the pipeline's ambient color, in the standard OpenGL lighting
    /// model. The ambient color affects the overall color of the object.
    ///
    /// Since the diffuse color will be intense when the light hits the surface
    /// directly, the ambient will be most apparent where the light hits at a
    /// slant.
    ///
    /// The default value is (0.2, 0.2, 0.2, 1.0)
    ///
    /// ## `ambient`
    /// The components of the desired ambient color
    pub fn set_ambient(&self, ambient: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_ambient(self.to_glib_none().0, ambient.to_glib_none().0);
        }
    }

    /// Conveniently sets the diffuse and ambient color of `self` at the same
    /// time. See `Pipeline::set_ambient` and `Pipeline::set_diffuse`.
    ///
    /// The default ambient color is (0.2, 0.2, 0.2, 1.0)
    ///
    /// The default diffuse color is (0.8, 0.8, 0.8, 1.0)
    ///
    /// ## `color`
    /// The components of the desired ambient and diffuse colors
    pub fn set_ambient_and_diffuse(&self, color: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_ambient_and_diffuse(
                self.to_glib_none().0,
                color.to_glib_none().0,
            );
        }
    }

    /// If not already familiar; please refer <link linkend="cogl-Blend-Strings">here`</link>`
    /// for an overview of what blend strings are, and their syntax.
    ///
    /// Blending occurs after the alpha test function, and combines fragments with
    /// the framebuffer.
    ///
    /// Currently the only blend function Cogl exposes is ADD(). So any valid
    /// blend statements will be of the form:
    ///
    ///
    /// ```text
    ///   &lt;channel-mask&gt;=ADD(SRC_COLOR*(&lt;factor&gt;), DST_COLOR*(&lt;factor&gt;))
    /// ```
    ///
    /// This is the list of source-names usable as blend factors:
    /// `<itemizedlist>`
    ///  `<listitem>``<para>`SRC_COLOR: The color of the in comming fragment`</para>``</listitem>`
    ///  `<listitem>``<para>`DST_COLOR: The color of the framebuffer`</para>``</listitem>`
    ///  `<listitem>``<para>`CONSTANT: The constant set via `Pipeline::set_blend_constant``</para>``</listitem>`
    /// `</itemizedlist>`
    ///
    /// The source names can be used according to the
    /// <link linkend="cogl-Blend-String-syntax">color-source and factor syntax`</link>`,
    /// so for example "(1-SRC_COLOR[A])" would be a valid factor, as would
    /// "(CONSTANT[RGB])"
    ///
    /// These can also be used as factors:
    /// `<itemizedlist>`
    ///  `<listitem>`0: (0, 0, 0, 0)`</listitem>`
    ///  `<listitem>`1: (1, 1, 1, 1)`</listitem>`
    ///  `<listitem>`SRC_ALPHA_SATURATE_FACTOR: (f,f,f,1) where f = MIN(SRC_COLOR[A],1-DST_COLOR[A])`</listitem>`
    /// `</itemizedlist>`
    ///
    /// `<note>`Remember; all color components are normalized to the range [0, 1]
    /// before computing the result of blending.`</note>`
    ///
    /// <example id="cogl-Blend-Strings-blend-unpremul">
    ///  `<title>`Blend Strings/1`</title>`
    ///  `<para>`Blend a non-premultiplied source over a destination with
    ///  premultiplied alpha:`</para>`
    ///  `<programlisting>`
    /// "RGB = ADD(SRC_COLOR*(SRC_COLOR[A]), DST_COLOR*(1-SRC_COLOR[A]))"
    /// "A = ADD(SRC_COLOR, DST_COLOR*(1-SRC_COLOR[A]))"
    ///  `</programlisting>`
    /// `</example>`
    ///
    /// <example id="cogl-Blend-Strings-blend-premul">
    ///  `<title>`Blend Strings/2`</title>`
    ///  `<para>`Blend a premultiplied source over a destination with
    ///  premultiplied alpha`</para>`
    ///  `<programlisting>`
    /// "RGBA = ADD(SRC_COLOR, DST_COLOR*(1-SRC_COLOR[A]))"
    ///  `</programlisting>`
    /// `</example>`
    ///
    /// The default blend string is:
    ///
    /// ```text
    ///    RGBA = ADD (SRC_COLOR, DST_COLOR*(1-SRC_COLOR[A]))
    /// ```
    ///
    /// That gives normal alpha-blending when the calculated color for the pipeline
    /// is in premultiplied form.
    ///
    /// ## `blend_string`
    /// A <link linkend="cogl-Blend-Strings">Cogl blend string`</link>`
    ///  describing the desired blend function.
    ///
    /// # Returns
    ///
    /// `true` if the blend string was successfully parsed, and the
    ///  described blending is supported by the underlying driver/hardware. If
    ///  there was an error, `false` is returned and `error` is set accordingly (if
    ///  present).
    pub fn set_blend(&self, blend_string: &str) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_pipeline_set_blend(
                self.to_glib_none().0,
                blend_string.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// When blending is setup to reference a CONSTANT blend factor then
    /// blending will depend on the constant set with this function.
    ///
    /// ## `constant_color`
    /// The constant color you want
    pub fn set_blend_constant(&self, constant_color: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_blend_constant(
                self.to_glib_none().0,
                constant_color.to_glib_none().0,
            );
        }
    }

    /// Sets the basic color of the pipeline, used when no lighting is enabled.
    ///
    /// Note that if you don't add any layers to the pipeline then the color
    /// will be blended unmodified with the destination; the default blend
    /// expects premultiplied colors: for example, use (0.5, 0.0, 0.0, 0.5) for
    /// semi-transparent red. See `Color::premultiply`.
    ///
    /// The default value is (1.0, 1.0, 1.0, 1.0)
    ///
    /// ## `color`
    /// The components of the color
    pub fn set_color(&self, color: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_color(self.to_glib_none().0, color.to_glib_none().0);
        }
    }

    /// Sets the basic color of the pipeline, used when no lighting is enabled.
    ///
    /// The default value is (1.0, 1.0, 1.0, 1.0)
    ///
    /// ## `red`
    /// The red component
    /// ## `green`
    /// The green component
    /// ## `blue`
    /// The blue component
    /// ## `alpha`
    /// The alpha component
    pub fn set_color4f(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            ffi::cogl_pipeline_set_color4f(self.to_glib_none().0, red, green, blue, alpha);
        }
    }

    /// Sets the basic color of the pipeline, used when no lighting is enabled.
    ///
    /// The default value is (0xff, 0xff, 0xff, 0xff)
    ///
    /// ## `red`
    /// The red component
    /// ## `green`
    /// The green component
    /// ## `blue`
    /// The blue component
    /// ## `alpha`
    /// The alpha component
    pub fn set_color4ub(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        unsafe {
            ffi::cogl_pipeline_set_color4ub(self.to_glib_none().0, red, green, blue, alpha);
        }
    }

    /// Defines a bit mask of which color channels should be written to the
    /// current framebuffer. If a bit is set in `color_mask` that means that
    /// color will be written.
    /// ## `color_mask`
    /// A `ColorMask` of which color channels to write to
    ///  the current framebuffer.
    pub fn set_color_mask(&self, color_mask: ColorMask) {
        unsafe {
            ffi::cogl_pipeline_set_color_mask(self.to_glib_none().0, color_mask.to_glib());
        }
    }

    /// Sets which faces will be culled when drawing. Face culling can be
    /// used to increase efficiency by avoiding drawing faces that would
    /// get overridden. For example, if a model has gaps so that it is
    /// impossible to see the inside then faces which are facing away from
    /// the screen will never be seen so there is no point in drawing
    /// them. This can be acheived by setting the cull face mode to
    /// `PipelineCullFaceMode::Back`.
    ///
    /// Face culling relies on the primitives being drawn with a specific
    /// order to represent which faces are facing inside and outside the
    /// model. This order can be specified by calling
    /// `Pipeline::set_front_face_winding`.
    ///
    /// Status: Unstable
    ///
    /// ## `cull_face_mode`
    /// The new mode to set
    pub fn set_cull_face_mode(&self, cull_face_mode: PipelineCullFaceMode) {
        unsafe {
            ffi::cogl_pipeline_set_cull_face_mode(self.to_glib_none().0, cull_face_mode.to_glib());
        }
    }

    /// This commits all the depth state configured in `state` struct to the
    /// given `self`. The configuration values are copied into the
    /// pipeline so there is no requirement to keep the `DepthState`
    /// struct around if you don't need it any more.
    ///
    /// Note: Since some platforms do not support the depth range feature
    /// it is possible for this function to fail and report an `error`.
    ///
    /// ## `state`
    /// A `DepthState` struct
    ///
    /// # Returns
    ///
    /// TRUE if the GPU supports all the given `state` else `false`
    ///  and returns an `error`.
    pub fn set_depth_state(&self, state: &DepthState) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_pipeline_set_depth_state(
                self.to_glib_none().0,
                state.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Sets the pipeline's diffuse color, in the standard OpenGL lighting
    /// model. The diffuse color is most intense where the light hits the
    /// surface directly - perpendicular to the surface.
    ///
    /// The default value is (0.8, 0.8, 0.8, 1.0)
    ///
    /// ## `diffuse`
    /// The components of the desired diffuse color
    pub fn set_diffuse(&self, diffuse: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_diffuse(self.to_glib_none().0, diffuse.to_glib_none().0);
        }
    }

    /// Sets the pipeline's emissive color, in the standard OpenGL lighting
    /// model. It will look like the surface is a light source emitting this
    /// color.
    ///
    /// The default value is (0.0, 0.0, 0.0, 1.0)
    ///
    /// ## `emission`
    /// The components of the desired emissive color
    pub fn set_emission(&self, emission: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_emission(self.to_glib_none().0, emission.to_glib_none().0);
        }
    }

    /// The order of the vertices within a primitive specifies whether it
    /// is considered to be front or back facing. This function specifies
    /// which order is considered to be the front
    /// faces. `Winding::CounterClockwise` sets the front faces to
    /// primitives with vertices in a counter-clockwise order and
    /// `Winding::Clockwise` sets them to be clockwise. The default is
    /// `Winding::CounterClockwise`.
    ///
    /// Status: Unstable
    ///
    /// ## `front_winding`
    /// the winding order
    pub fn set_front_face_winding(&self, front_winding: Winding) {
        unsafe {
            ffi::cogl_pipeline_set_front_face_winding(
                self.to_glib_none().0,
                front_winding.to_glib(),
            );
        }
    }

    /// If not already familiar; you can refer
    /// <link linkend="cogl-Blend-Strings">here`</link>` for an overview of what blend
    /// strings are and there syntax.
    ///
    /// These are all the functions available for texture combining:
    /// `<itemizedlist>`
    ///  `<listitem>`REPLACE(arg0) = arg0`</listitem>`
    ///  `<listitem>`MODULATE(arg0, arg1) = arg0 x arg1`</listitem>`
    ///  `<listitem>`ADD(arg0, arg1) = arg0 + arg1`</listitem>`
    ///  `<listitem>`ADD_SIGNED(arg0, arg1) = arg0 + arg1 - 0.5`</listitem>`
    ///  `<listitem>`INTERPOLATE(arg0, arg1, arg2) = arg0 x arg2 + arg1 x (1 - arg2)`</listitem>`
    ///  `<listitem>`SUBTRACT(arg0, arg1) = arg0 - arg1`</listitem>`
    ///  `<listitem>`
    ///  `<programlisting>`
    ///  DOT3_RGB(arg0, arg1) = 4 x ((arg0[R] - 0.5)) * (arg1[R] - 0.5) +
    ///  (arg0[G] - 0.5)) * (arg1[G] - 0.5) +
    ///  (arg0[B] - 0.5)) * (arg1[B] - 0.5))
    ///  `</programlisting>`
    ///  `</listitem>`
    ///  `<listitem>`
    ///  `<programlisting>`
    ///  DOT3_RGBA(arg0, arg1) = 4 x ((arg0[R] - 0.5)) * (arg1[R] - 0.5) +
    ///  (arg0[G] - 0.5)) * (arg1[G] - 0.5) +
    ///  (arg0[B] - 0.5)) * (arg1[B] - 0.5))
    ///  `</programlisting>`
    ///  `</listitem>`
    /// `</itemizedlist>`
    ///
    /// Refer to the
    /// <link linkend="cogl-Blend-String-syntax">color-source syntax`</link>` for
    /// describing the arguments. The valid source names for texture combining
    /// are:
    /// `<variablelist>`
    ///  `<varlistentry>`
    ///  `<term>`TEXTURE`</term>`
    ///  `<listitem>`Use the color from the current texture layer`</listitem>`
    ///  `</varlistentry>`
    ///  `<varlistentry>`
    ///  `<term>`TEXTURE_0, TEXTURE_1, etc`</term>`
    ///  `<listitem>`Use the color from the specified texture layer`</listitem>`
    ///  `</varlistentry>`
    ///  `<varlistentry>`
    ///  `<term>`CONSTANT`</term>`
    ///  `<listitem>`Use the color from the constant given with
    ///  `Pipeline::set_layer_combine_constant``</listitem>`
    ///  `</varlistentry>`
    ///  `<varlistentry>`
    ///  `<term>`PRIMARY`</term>`
    ///  `<listitem>`Use the color of the pipeline as set with
    ///  `Pipeline::set_color``</listitem>`
    ///  `</varlistentry>`
    ///  `<varlistentry>`
    ///  `<term>`PREVIOUS`</term>`
    ///  `<listitem>`Either use the texture color from the previous layer, or
    ///  if this is layer 0, use the color of the pipeline as set with
    ///  `Pipeline::set_color``</listitem>`
    ///  `</varlistentry>`
    /// `</variablelist>`
    ///
    /// <refsect2 id="cogl-Layer-Combine-Examples">
    ///  `<title>`Layer Combine Examples`</title>`
    ///  `<para>`This is effectively what the default blending is:`</para>`
    ///  `<informalexample>``<programlisting>`
    ///  RGBA = MODULATE (PREVIOUS, TEXTURE)
    ///  `</programlisting>``</informalexample>`
    ///  `<para>`This could be used to cross-fade between two images, using
    ///  the alpha component of a constant as the interpolator. The constant
    ///  color is given by calling
    ///  `Pipeline::set_layer_combine_constant`.`</para>`
    ///  `<informalexample>``<programlisting>`
    ///  RGBA = INTERPOLATE (PREVIOUS, TEXTURE, CONSTANT[A])
    ///  `</programlisting>``</informalexample>`
    /// `</refsect2>`
    ///
    /// `<note>`You can't give a multiplication factor for arguments as you can
    /// with blending.`</note>`
    ///
    /// ## `layer_index`
    /// Specifies the layer you want define a combine function for
    /// ## `blend_string`
    /// A <link linkend="cogl-Blend-Strings">Cogl blend string`</link>`
    ///  describing the desired texture combine function.
    ///
    /// # Returns
    ///
    /// `true` if the blend string was successfully parsed, and the
    ///  described texture combining is supported by the underlying driver and
    ///  or hardware. On failure, `false` is returned and `error` is set
    pub fn set_layer_combine(
        &self,
        layer_index: i32,
        blend_string: &str,
    ) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_pipeline_set_layer_combine(
                self.to_glib_none().0,
                layer_index,
                blend_string.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// When you are using the 'CONSTANT' color source in a layer combine
    /// description then you can use this function to define its value.
    ///
    /// ## `layer_index`
    /// Specifies the layer you want to specify a constant used
    ///  for texture combining
    /// ## `constant`
    /// The constant color you want
    pub fn set_layer_combine_constant(&self, layer_index: i32, constant: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_layer_combine_constant(
                self.to_glib_none().0,
                layer_index,
                constant.to_glib_none().0,
            );
        }
    }

    /// Changes the decimation and interpolation filters used when a texture is
    /// drawn at other scales than 100%.
    ///
    /// `<note>`It is an error to pass anything other than
    /// `PipelineFilter::Nearest` or `PipelineFilter::Linear` as
    /// magnification filters since magnification doesn't ever need to
    /// reference values stored in the mipmap chain.`</note>`
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `min_filter`
    /// the filter used when scaling a texture down.
    /// ## `mag_filter`
    /// the filter used when magnifying a texture.
    pub fn set_layer_filters(
        &self,
        layer_index: i32,
        min_filter: PipelineFilter,
        mag_filter: PipelineFilter,
    ) {
        unsafe {
            ffi::cogl_pipeline_set_layer_filters(
                self.to_glib_none().0,
                layer_index,
                min_filter.to_glib(),
                mag_filter.to_glib(),
            );
        }
    }

    /// This function lets you set a matrix that can be used to e.g. translate
    /// and rotate a single layer of a pipeline used to fill your geometry.
    /// ## `layer_index`
    /// the index for the layer inside `self`
    /// ## `matrix`
    /// the transformation matrix for the layer
    pub fn set_layer_matrix(&self, layer_index: i32, matrix: &Matrix) {
        unsafe {
            ffi::cogl_pipeline_set_layer_matrix(
                self.to_glib_none().0,
                layer_index,
                matrix.to_glib_none().0,
            );
        }
    }

    /// Sets the texture for this layer to be the default texture for the
    /// given type. This is equivalent to calling
    /// `Pipeline::set_layer_texture` with `None` for the texture
    /// argument except that you can also specify the type of default
    /// texture to use. The default texture is a 1x1 pixel white texture.
    ///
    /// This function is mostly useful if you want to create a base
    /// pipeline that you want to create multiple copies from using
    /// `Pipeline::copy`. In that case this function can be used to
    /// specify the texture type so that any pipeline copies can share the
    /// internal texture type state for efficiency.
    /// ## `layer_index`
    /// The layer number to modify
    /// ## `texture_type`
    /// The type of the default texture to use
    pub fn set_layer_null_texture(&self, layer_index: i32, texture_type: TextureType) {
        unsafe {
            ffi::cogl_pipeline_set_layer_null_texture(
                self.to_glib_none().0,
                layer_index,
                texture_type.to_glib(),
            );
        }
    }

    /// When rendering points, if `enable` is `true` then the texture
    /// coordinates for this layer will be replaced with coordinates that
    /// vary from 0.0 to 1.0 across the primitive. The top left of the
    /// point will have the coordinates 0.0,0.0 and the bottom right will
    /// have 1.0,1.0. If `enable` is `false` then the coordinates will be
    /// fixed for the entire point.
    ///
    /// This function will only work if `FeatureID::OglFeatureIdPointSprite` is
    /// available. If the feature is not available then the function will
    /// return `false` and set `error`.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `enable`
    /// whether to enable point sprite coord generation.
    ///
    /// # Returns
    ///
    /// `true` if the function succeeds, `false` otherwise.
    pub fn set_layer_point_sprite_coords_enabled(
        &self,
        layer_index: i32,
        enable: bool,
    ) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_pipeline_set_layer_point_sprite_coords_enabled(
                self.to_glib_none().0,
                layer_index,
                enable as i32,
                &mut error,
            );
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn set_layer_texture<P: IsA<Texture>>(&self, layer_index: i32, texture: &P) {
        unsafe {
            ffi::cogl_pipeline_set_layer_texture(
                self.to_glib_none().0,
                layer_index,
                texture.as_ref().to_glib_none().0,
            );
        }
    }

    /// Sets the wrap mode for all three coordinates of texture lookups on
    /// this layer. This is equivalent to calling
    /// `Pipeline::set_layer_wrap_mode_s`,
    /// `Pipeline::set_layer_wrap_mode_t` and
    /// `Pipeline::set_layer_wrap_mode_p` separately.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `mode`
    /// the new wrap mode
    pub fn set_layer_wrap_mode(&self, layer_index: i32, mode: PipelineWrapMode) {
        unsafe {
            ffi::cogl_pipeline_set_layer_wrap_mode(
                self.to_glib_none().0,
                layer_index,
                mode.to_glib(),
            );
        }
    }

    /// Sets the wrap mode for the 'p' coordinate of texture lookups on
    /// this layer. 'p' is the third coordinate.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `mode`
    /// the new wrap mode
    pub fn set_layer_wrap_mode_p(&self, layer_index: i32, mode: PipelineWrapMode) {
        unsafe {
            ffi::cogl_pipeline_set_layer_wrap_mode_p(
                self.to_glib_none().0,
                layer_index,
                mode.to_glib(),
            );
        }
    }

    /// Sets the wrap mode for the 's' coordinate of texture lookups on this layer.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `mode`
    /// the new wrap mode
    pub fn set_layer_wrap_mode_s(&self, layer_index: i32, mode: PipelineWrapMode) {
        unsafe {
            ffi::cogl_pipeline_set_layer_wrap_mode_s(
                self.to_glib_none().0,
                layer_index,
                mode.to_glib(),
            );
        }
    }

    /// Sets the wrap mode for the 't' coordinate of texture lookups on this layer.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `mode`
    /// the new wrap mode
    pub fn set_layer_wrap_mode_t(&self, layer_index: i32, mode: PipelineWrapMode) {
        unsafe {
            ffi::cogl_pipeline_set_layer_wrap_mode_t(
                self.to_glib_none().0,
                layer_index,
                mode.to_glib(),
            );
        }
    }

    /// Sets whether to use a per-vertex point size or to use the value set
    /// by `Pipeline::set_point_size`. If per-vertex point size is
    /// enabled then the point size can be set for an individual point
    /// either by drawing with a `Attribute` with the name
    /// ‘point_size_in’ or by writing to the GLSL builtin
    /// ‘point_size_out’ from a vertex shader snippet.
    ///
    /// If per-vertex point size is enabled and this attribute is not used
    /// and point_size_out is not written to then the results are
    /// undefined.
    ///
    /// Note that enabling this will only work if the
    /// `FeatureID::OglFeatureIdPerVertexPointSize` feature is available. If
    /// this is not available then the function will return `false` and set
    /// a `CoglError`.
    ///
    /// ## `enable`
    /// whether to enable per-vertex point size
    ///
    /// # Returns
    ///
    /// `true` if the change suceeded or `false` otherwise
    pub fn set_per_vertex_point_size(&self, enable: bool) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_pipeline_set_per_vertex_point_size(
                self.to_glib_none().0,
                enable as i32,
                &mut error,
            );
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Changes the size of points drawn when `VerticesMode::Points` is
    /// used with the attribute buffer API. Note that typically the GPU
    /// will only support a limited minimum and maximum range of point
    /// sizes. If the chosen point size is outside that range then the
    /// nearest value within that range will be used instead. The size of a
    /// point is in screen space so it will be the same regardless of any
    /// transformations.
    ///
    /// If the point size is set to 0.0 then drawing points with the
    /// pipeline will have undefined results. This is the default value so
    /// if an application wants to draw points it must make sure to use a
    /// pipeline that has an explicit point size set on it.
    ///
    /// ## `point_size`
    /// the new point size.
    pub fn set_point_size(&self, point_size: f32) {
        unsafe {
            ffi::cogl_pipeline_set_point_size(self.to_glib_none().0, point_size);
        }
    }

    /// Sets the shininess of the pipeline, in the standard OpenGL lighting
    /// model, which determines the size of the specular highlights. A
    /// higher `shininess` will produce smaller highlights which makes the
    /// object appear more shiny.
    ///
    /// The default value is 0.0
    ///
    /// ## `shininess`
    /// The desired shininess; must be >= 0.0
    pub fn set_shininess(&self, shininess: f32) {
        unsafe {
            ffi::cogl_pipeline_set_shininess(self.to_glib_none().0, shininess);
        }
    }

    /// Sets the pipeline's specular color, in the standard OpenGL lighting
    /// model. The intensity of the specular color depends on the viewport
    /// position, and is brightest along the lines of reflection.
    ///
    /// The default value is (0.0, 0.0, 0.0, 1.0)
    ///
    /// ## `specular`
    /// The components of the desired specular color
    pub fn set_specular(&self, specular: &Color) {
        unsafe {
            ffi::cogl_pipeline_set_specular(self.to_glib_none().0, specular.to_glib_none().0);
        }
    }

    /// Sets a new value for the uniform at `uniform_location`. If this
    /// pipeline has a user program attached and is later used as a source
    /// for drawing, the given value will be assigned to the uniform which
    /// can be accessed from the shader's source. The value for
    /// `uniform_location` should be retrieved from the string name of the
    /// uniform by calling `Pipeline::get_uniform_location`.
    ///
    /// This function should be used to set uniforms that are of type
    /// float. It can also be used to set a single member of a float array
    /// uniform.
    ///
    /// ## `uniform_location`
    /// The uniform's location identifier
    /// ## `value`
    /// The new value for the uniform
    pub fn set_uniform_1f(&self, uniform_location: i32, value: f32) {
        unsafe {
            ffi::cogl_pipeline_set_uniform_1f(self.to_glib_none().0, uniform_location, value);
        }
    }

    /// Sets a new value for the uniform at `uniform_location`. If this
    /// pipeline has a user program attached and is later used as a source
    /// for drawing, the given value will be assigned to the uniform which
    /// can be accessed from the shader's source. The value for
    /// `uniform_location` should be retrieved from the string name of the
    /// uniform by calling `Pipeline::get_uniform_location`.
    ///
    /// This function should be used to set uniforms that are of type
    /// int. It can also be used to set a single member of a int array
    /// uniform or a sampler uniform.
    ///
    /// ## `uniform_location`
    /// The uniform's location identifier
    /// ## `value`
    /// The new value for the uniform
    pub fn set_uniform_1i(&self, uniform_location: i32, value: i32) {
        unsafe {
            ffi::cogl_pipeline_set_uniform_1i(self.to_glib_none().0, uniform_location, value);
        }
    }

    /// Sets new values for the uniform at `uniform_location`. If this
    /// pipeline has a user program attached and is later used as a source
    /// for drawing, the given values will be assigned to the uniform which
    /// can be accessed from the shader's source. The value for
    /// `uniform_location` should be retrieved from the string name of the
    /// uniform by calling `Pipeline::get_uniform_location`.
    ///
    /// This function can be used to set any floating point type uniform,
    /// including float arrays and float vectors. For example, to set a
    /// single vec4 uniform you would use 4 for `n_components` and 1 for
    /// `count`. To set an array of 8 float values, you could use 1 for
    /// `n_components` and 8 for `count`.
    ///
    /// ## `uniform_location`
    /// The uniform's location identifier
    /// ## `n_components`
    /// The number of components in the corresponding uniform's type
    /// ## `count`
    /// The number of values to set
    /// ## `value`
    /// Pointer to the new values to set
    pub fn set_uniform_float(
        &self,
        uniform_location: i32,
        n_components: i32,
        count: i32,
        value: &[f32],
    ) {
        unsafe {
            ffi::cogl_pipeline_set_uniform_float(
                self.to_glib_none().0,
                uniform_location,
                n_components,
                count,
                value.as_ptr(),
            );
        }
    }

    /// Sets new values for the uniform at `uniform_location`. If this
    /// pipeline has a user program attached and is later used as a source
    /// for drawing, the given values will be assigned to the uniform which
    /// can be accessed from the shader's source. The value for
    /// `uniform_location` should be retrieved from the string name of the
    /// uniform by calling `Pipeline::get_uniform_location`.
    ///
    /// This function can be used to set any integer type uniform,
    /// including int arrays and int vectors. For example, to set a single
    /// ivec4 uniform you would use 4 for `n_components` and 1 for
    /// `count`. To set an array of 8 int values, you could use 1 for
    /// `n_components` and 8 for `count`.
    ///
    /// ## `uniform_location`
    /// The uniform's location identifier
    /// ## `n_components`
    /// The number of components in the corresponding uniform's type
    /// ## `count`
    /// The number of values to set
    /// ## `value`
    /// Pointer to the new values to set
    pub fn set_uniform_int(
        &self,
        uniform_location: i32,
        n_components: i32,
        count: i32,
        value: &[i32],
    ) {
        unsafe {
            ffi::cogl_pipeline_set_uniform_int(
                self.to_glib_none().0,
                uniform_location,
                n_components,
                count,
                value.as_ptr(),
            );
        }
    }

    /// Sets new values for the uniform at `uniform_location`. If this
    /// pipeline has a user program attached and is later used as a source
    /// for drawing, the given values will be assigned to the uniform which
    /// can be accessed from the shader's source. The value for
    /// `uniform_location` should be retrieved from the string name of the
    /// uniform by calling `Pipeline::get_uniform_location`.
    ///
    /// This function can be used to set any matrix type uniform, including
    /// matrix arrays. For example, to set a single mat4 uniform you would
    /// use 4 for `dimensions` and 1 for `count`. To set an array of 8
    /// mat3 values, you could use 3 for `dimensions` and 8 for `count`.
    ///
    /// If `transpose` is `false` then the matrix is expected to be in
    /// column-major order or if it is `true` then the matrix is in
    /// row-major order. You can pass a `Matrix` by calling by passing
    /// the result of `Matrix::get_array` in `value` and setting
    /// `transpose` to `false`.
    ///
    /// ## `uniform_location`
    /// The uniform's location identifier
    /// ## `dimensions`
    /// The size of the matrix
    /// ## `count`
    /// The number of values to set
    /// ## `transpose`
    /// Whether to transpose the matrix
    /// ## `value`
    /// Pointer to the new values to set
    pub fn set_uniform_matrix(
        &self,
        uniform_location: i32,
        dimensions: i32,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            ffi::cogl_pipeline_set_uniform_matrix(
                self.to_glib_none().0,
                uniform_location,
                dimensions,
                count,
                transpose as i32,
                value.as_ptr(),
            );
        }
    }

    //pub fn set_user_program(&self, program: /*Unimplemented*/Handle) {
    //    unsafe { TODO: call cogl_sys:cogl_pipeline_set_user_program() }
    //}
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pipeline")
    }
}
