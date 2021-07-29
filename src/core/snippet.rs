#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::SnippetHook;
use std::fmt;

// SECTION:snippet
// @short_description: Functions for creating and manipulating shader snippets
//
// #Snippet<!-- -->s are used to modify or replace parts of a
// #Pipeline using GLSL. GLSL is a programming language supported
// by OpenGL on programmable hardware to provide a more flexible
// description of what should be rendered. A description of GLSL
// itself is outside the scope of this documentation but any good
// OpenGL book should help to describe it.
//
// Unlike in OpenGL, when using GLSL with  it is possible to write
// short snippets to replace small sections of the pipeline instead of
// having to replace the whole of either the vertex or fragment
// pipelines. Of course it is also possible to replace the whole of
// the pipeline if needed.
//
// Each snippet is a standalone chunk of code which would attach to
// the pipeline at a particular point. The code is split into four
// separate strings (all of which are optional):
//
// <glosslist>
//  <glossentry>
//   <glossterm>declarations</glossterm>
//   <glossdef><para>
// The code in this string will be inserted outside of any function in
// the global scope of the shader. This can be used to declare
// uniforms, attributes, varyings and functions to be used by the
// snippet.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>pre</glossterm>
//   <glossdef><para>
// The code in this string will be inserted before the hook point.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>post</glossterm>
//   <glossdef><para>
// The code in this string will be inserted after the hook point. This
// can be used to modify the results of the builtin generated code for
// that hook point.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>replace</glossterm>
//   <glossdef><para>
// If present the code in this string will replace the generated code
// for the hook point.
//   </para></glossdef>
//  </glossentry>
// </glosslist>
//
// All of the strings apart from the declarations string of a pipeline
// are generated in a single function so they can share variables
// declared from one string in another. The scope of the code is
// limited to each snippet so local variables declared in the snippet
// will not collide with variables declared in another
// snippet. However, code in the 'declarations' string is global to
// the shader so it is the application's responsibility to ensure that
// variables declared here will not collide with those from other
// snippets.
//
// The snippets can be added to a pipeline with
// pipeline_add_snippet() or
// pipeline_add_layer_snippet(). Which function to use depends on
// which hook the snippet is targetting. The snippets are all
// generated in the order they are added to the pipeline. That is, the
// post strings are executed in the order they are added to the
// pipeline and the pre strings are executed in reverse order. If any
// replace strings are given for a snippet then any other snippets
// with the same hook added before that snippet will be ignored. The
// different hooks are documented under #SnippetHook.
//
// For portability with GLES2, it is recommended not to use the GLSL
// builtin names such as gl_FragColor. Instead there are replacement
// names under the * namespace which can be used instead. These
// are:
//
// <glosslist>
//  <glossentry>
//   <glossterm>uniform mat4
//         <emphasis>modelview_matrix</emphasis></glossterm>
//   <glossdef><para>
//    The current modelview matrix. This is equivalent to
//    #gl_ModelViewMatrix.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>uniform mat4
//         <emphasis>projection_matrix</emphasis></glossterm>
//   <glossdef><para>
//    The current projection matrix. This is equivalent to
//    #gl_ProjectionMatrix.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>uniform mat4
//         <emphasis>modelview_projection_matrix</emphasis></glossterm>
//   <glossdef><para>
//    The combined modelview and projection matrix. A vertex shader
//    would typically use this to transform the incoming vertex
//    position. The separate modelview and projection matrices are
//    usually only needed for lighting calculations. This is
//    equivalent to #gl_ModelViewProjectionMatrix.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>uniform mat4
//         <emphasis>texture_matrix</emphasis>[]</glossterm>
//   <glossdef><para>
//    An array of matrices for transforming the texture
//    coordinates. This is equivalent to #gl_TextureMatrix.
//   </para></glossdef>
//  </glossentry>
// </glosslist>
//
// In a vertex shader, the following are also available:
//
// <glosslist>
//  <glossentry>
//   <glossterm>attribute vec4
//         <emphasis>position_in</emphasis></glossterm>
//   <glossdef><para>
//    The incoming vertex position. This is equivalent to #gl_Vertex.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>attribute vec4
//         <emphasis>color_in</emphasis></glossterm>
//   <glossdef><para>
//    The incoming vertex color. This is equivalent to #gl_Color.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>attribute vec4
//         <emphasis>tex_coord_in</emphasis></glossterm>
//   <glossdef><para>
//    The texture coordinate for layer 0. This is an alternative name
//    for #tex_coord0_in.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>attribute vec4
//         <emphasis>tex_coord0_in</emphasis></glossterm>
//   <glossdef><para>
//    The texture coordinate for the layer 0. This is equivalent to
//    #gl_MultiTexCoord0. There will also be #tex_coord1_in and
//    so on if more layers are added to the pipeline.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>attribute vec3
//         <emphasis>normal_in</emphasis></glossterm>
//   <glossdef><para>
//    The normal of the vertex. This is equivalent to #gl_Normal.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>vec4
//         <emphasis>position_out</emphasis></glossterm>
//   <glossdef><para>
//    The calculated position of the vertex. This must be written to
//    in all vertex shaders. This is equivalent to #gl_Position.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>float
//         <emphasis>point_size_in</emphasis></glossterm>
//   <glossdef><para>
//    The incoming point size from the point_size_in attribute.
//    This is only available if
//    pipeline_set_per_vertex_point_size() is set on the
//    pipeline.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>float
//         <emphasis>point_size_out</emphasis></glossterm>
//   <glossdef><para>
//    The calculated size of a point. This is equivalent to #gl_PointSize.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>varying vec4
//         <emphasis>color_out</emphasis></glossterm>
//   <glossdef><para>
//    The calculated color of a vertex. This is equivalent to #gl_FrontColor.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>varying vec4
//         <emphasis>tex_coord0_out</emphasis></glossterm>
//   <glossdef><para>
//    The calculated texture coordinate for layer 0 of the pipeline.
//    This is equivalent to #gl_TexCoord[0]. There will also be
//    #tex_coord1_out and so on if more layers are added to the
//    pipeline. In the fragment shader, this varying is called
//    #tex_coord0_in.
//   </para></glossdef>
//  </glossentry>
// </glosslist>
//
// In a fragment shader, the following are also available:
//
// <glosslist>
//  <glossentry>
//   <glossterm>varying vec4 <emphasis>color_in</emphasis></glossterm>
//   <glossdef><para>
//    The calculated color of a vertex. This is equivalent to #gl_FrontColor.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>varying vec4
//              <emphasis>tex_coord0_in</emphasis></glossterm>
//   <glossdef><para>
//    The texture coordinate for layer 0. This is equivalent to
//    #gl_TexCoord[0]. There will also be #tex_coord1_in and so
//    on if more layers are added to the pipeline.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>vec4 <emphasis>color_out</emphasis></glossterm>
//   <glossdef><para>
//    The final calculated color of the fragment. All fragment shaders
//    must write to this variable. This is equivalent to
//    #gl_FrontColor.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>float <emphasis>depth_out</emphasis></glossterm>
//   <glossdef><para>
//    An optional output variable specifying the depth value to use
//    for this fragment. This is equivalent to #gl_FragDepth.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>bool <emphasis>front_facing</emphasis></glossterm>
//   <glossdef><para>
//    A readonly variable that will be true if the current primitive
//    is front facing. This can be used to implement two-sided
//    coloring algorithms. This is equivalent to #gl_FrontFacing.
//   </para></glossdef>
//  </glossentry>
//  <glossentry>
//   <glossterm>vec2 <emphasis>point_coord</emphasis></glossterm>
//   <glossdef><para>
//    When rendering points, this will contain a vec2 which represents
//    the position within the point of the current fragment.
//    vec2(0.0,0.0) will be the topleft of the point and vec2(1.0,1.0)
//    will be the bottom right. Note that there is currently a bug in
//     where when rendering to an offscreen buffer these
//    coordinates will be upside-down. The value is undefined when not
//    rendering points. This builtin can only be used if the
//    %FEATURE_ID_POINT_SPRITE feature is available.
//   </para></glossdef>
//  </glossentry>
// </glosslist>
//
// Here is an example of using a snippet to add a desaturate effect to the
// generated color on a pipeline.
//
// <programlisting>
//   Pipeline *pipeline = pipeline_new ();
//
//   /<!-- -->* Set up the pipeline here, ie by adding a texture or other
//      layers *<!-- -->/
//
//   /<!-- -->* Create the snippet. The first string is the declarations which
//      we will use to add a uniform. The second is the 'post' string which
//      will contain the code to perform the desaturation. *<!-- -->/
//   Snippet *snippet =
//     snippet_new (SNIPPET_HOOK_FRAGMENT,
//                       "uniform float factor;",
//                       "float gray = dot (vec3 (0.299, 0.587, 0.114), "
//                       "                  color_out.rgb);"
//                       "color_out.rgb = mix (vec3 (gray),"
//                       "                          color_out.rgb,"
//                       "                          factor);");
//
//   /<!-- -->* Add it to the pipeline *<!-- -->/
//   pipeline_add_snippet (pipeline, snippet);
//   /<!-- -->* The pipeline keeps a reference to the snippet
//      so we don't need to *<!-- -->/
//   object_unref (snippet);
//
//   /<!-- -->* Update the custom uniform on the pipeline *<!-- -->/
//   int location = pipeline_get_uniform_location (pipeline, "factor");
//   pipeline_set_uniform_1f (pipeline, location, 0.5f);
//
//   /<!-- -->* Now we can render with the snippet as usual *<!-- -->/
//   push_source (pipeline);
//   rectangle (0, 0, 10, 10);
//   pop_source ();
// </programlisting>
pub struct Snippet {
    hook: SnippetHook,

    // This is set to true the first time the snippet is attached to the
    // pipeline. After that any attempts to modify the snippet will be ignored.
    immutable: bool,

    declarations: String,
    pre: String,
    replace: String,
    post: String,
}

impl Snippet {
    /// Allocates and initializes a new snippet with the given source strings.
    /// ## `hook`
    /// The point in the pipeline that this snippet will wrap around
    ///  or replace.
    /// ## `declarations`
    /// The source code for the declarations for this
    ///  snippet or `None`. See `Snippet::set_declarations`.
    /// ## `post`
    /// The source code to run after the hook point where this
    ///  shader snippet is attached or `None`. See `Snippet::set_post`.
    ///
    /// # Returns
    ///
    /// a pointer to a new `Snippet`
    pub fn new(hook: SnippetHook, declarations: &str, post: &str) -> Snippet {
        // Snippet *snippet = g_slice_new0 (Snippet);

        // _snippet_object_new (snippet);

        // snippet->hook = hook;

        // snippet_set_declarations (snippet, declarations);
        // snippet_set_post (snippet, post);

        // return snippet;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_declarations` or `None` if none was set.
    pub fn get_declarations(&self) -> Option<String> {
        // _RETURN_VAL_IF_FAIL (is_snippet (snippet), NULL);

        // return snippet->declarations;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the hook that was set when `Snippet::new` was
    ///  called.
    pub fn get_hook(&self) -> SnippetHook {
        // _RETURN_VAL_IF_FAIL (is_snippet (snippet), 0);

        // return snippet->hook;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_post` or `None` if none was set.
    pub fn get_post(&self) -> Option<String> {
        // _RETURN_VAL_IF_FAIL (is_snippet (snippet), NULL);

        // return snippet->post;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_pre` or `None` if none was set.
    pub fn get_pre(&self) -> Option<String> {
        // _RETURN_VAL_IF_FAIL (is_snippet (snippet), NULL);

        // return snippet->pre;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_replace` or `None` if none was set.
    pub fn get_replace(&self) -> Option<String> {
        // _RETURN_VAL_IF_FAIL (is_snippet (snippet), NULL);

        // return snippet->replace;
        unimplemented!()
    }

    /// Sets a source string that will be inserted in the global scope of
    /// the generated shader when this snippet is used on a pipeline. This
    /// string is typically used to declare uniforms, attributes or
    /// functions that will be used by the other parts of the snippets.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `declarations`
    /// The new source string for the declarations section
    ///  of this snippet.
    pub fn set_declarations(&self, declarations: &str) {
        // _RETURN_IF_FAIL (is_snippet (snippet));

        // if (!_snippet_modify (snippet))
        //     return;

        // g_free (snippet->declarations);
        // snippet->declarations = declarations ? g_strdup (declarations) : NULL;
        unimplemented!()
    }

    /// Sets a source string that will be inserted after the hook point in
    /// the generated shader for the pipeline that this snippet is attached
    /// to. Please see the documentation of each hook point in
    /// `Pipeline` for a description of how this string should be used.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `post`
    /// The new source string for the post section of this snippet.
    pub fn set_post(&self, post: &str) {
        // _RETURN_IF_FAIL (is_snippet (snippet));

        // if (!_snippet_modify (snippet))
        //   return;

        // g_free (snippet->post);
        // snippet->post = post ? g_strdup (post) : NULL;
        unimplemented!()
    }

    /// Sets a source string that will be inserted before the hook point in
    /// the generated shader for the pipeline that this snippet is attached
    /// to. Please see the documentation of each hook point in
    /// `Pipeline` for a description of how this string should be used.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `pre`
    /// The new source string for the pre section of this snippet.
    pub fn set_pre(&self, pre: &str) {
        // _RETURN_IF_FAIL (is_snippet (snippet));

        // if (!_snippet_modify (snippet))
        //   return;

        // g_free (snippet->pre);
        // snippet->pre = pre ? g_strdup (pre) : NULL;
        unimplemented!()
    }

    /// Sets a source string that will be used instead of any generated
    /// source code or any previous snippets for this hook point. Please
    /// see the documentation of each hook point in `Pipeline` for a
    /// description of how this string should be used.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `replace`
    /// The new source string for the replace section of this snippet.
    pub fn set_replace(&self, replace: &str) {
        // _RETURN_IF_FAIL (is_snippet (snippet));

        // if (!_snippet_modify (snippet))
        //     return;

        // g_free (snippet->replace);
        // snippet->replace = replace ? g_strdup (replace) : NULL;
        unimplemented!()
    }
}

impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Snippet")
    }
}
