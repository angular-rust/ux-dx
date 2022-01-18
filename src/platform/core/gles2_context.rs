use super::Context;
use std::{fmt, ptr};

// typedef struct _GLES2Offscreen
// {
//   List link;
//   Offscreen *original_offscreen;
//   GLFramebuffer gl_framebuffer;
// } GLES2Offscreen;

// typedef struct
// {
//   /* GL's ID for the shader */
//   GLuint object_id;
//   /* Shader type */
//   GLenum type;

//   /* Number of references to this shader. The shader will have one
//    * reference when it is created. This reference will be removed when
//    * glDeleteShader is called. An additional reference will be taken
//    * whenever the shader is attached to a program. This is necessary
//    * to correctly detect when a shader is destroyed because
//    * glDeleteShader doesn't actually delete the object if it is
//    * attached to a program */
//   int ref_count;

//   /* Set once this object has had glDeleteShader called on it. We need
//    * to keep track of this so we don't deref the data twice if the
//    * application calls glDeleteShader multiple times */
//   Bool deleted;
// } GLES2ShaderData;

// typedef enum
// {
//   GLES2_FLIP_STATE_UNKNOWN,
//   GLES2_FLIP_STATE_NORMAL,
//   GLES2_FLIP_STATE_FLIPPED
// } GLES2FlipState;

// typedef struct
// {
//   /* GL's ID for the program */
//   GLuint object_id;

//   /* List of shaders attached to this program */
//   GList *attached_shaders;

//   /* Reference count. There can be up to two references. One of these
//    * will exist between glCreateProgram and glDeleteShader, the other
//    * will exist while the program is made current. This is necessary
//    * to correctly detect when the program is deleted because
//    * glDeleteShader will delay the deletion if the program is
//    * current */
//   int ref_count;

//   /* Set once this object has had glDeleteProgram called on it. We need
//    * to keep track of this so we don't deref the data twice if the
//    * application calls glDeleteProgram multiple times */
//   Bool deleted;

//   GLuint flip_vector_location;

//   /* A cache of what value we've put in the flip vector uniform so
//    * that we don't flush unless it's changed */
//   GLES2FlipState flip_vector_state;

//   GLES2Context *context;
// } GLES2ProgramData;

// State tracked for each texture unit */
// typedef struct
// {
//   /* The currently bound texture for the GL_TEXTURE_2D */
//   GLuint current_texture_2d;
// } GLES2TextureUnitData;

// State tracked for each texture object */
// typedef struct
// {
//   /* GL's ID for this object */
//   GLuint object_id;

//   GLenum target;

//   /* The details for texture when it has a 2D target */
//   int width, height;
//   GLenum format;
// } GLES2TextureObjectData;

pub struct GLES2Context {
    // This is set to false until the first time the GLES2 context is
//  * bound to something. We need to keep track of this so we can set
//  * the viewport and scissor the first time it is bound. */
// Bool has_been_bound;

// Framebuffer *read_buffer;
// GLES2Offscreen *gles2_read_buffer;
// Framebuffer *write_buffer;
// GLES2Offscreen *gles2_write_buffer;

// GLuint current_fbo_handle;

// List foreign_offscreens;

// GLES2Vtable *vtable;

// Hash table mapping GL's IDs for shaders and objects to ShaderData
//  * and ProgramData so that we can maintain extra data for these
//  * objects. Although technically the IDs will end up global across
//  * all GLES2 contexts because they will all be in the same share
//  * list, we don't really want to expose this outside of the  API
//  * so we will assume it is undefined behaviour if an application
//  * relies on this. */
// GHashTable *shader_map;
// GHashTable *program_map;

// Currently in use program. We need to keep track of this so that
//  * we can keep a reference to the data for the program while it is
//  * current */
// GLES2ProgramData *current_program;

// Whether the currently bound framebuffer needs flipping. This is
//  * used to check for changes so that we can dirty the following
//  * state flags */
// GLES2FlipState current_flip_state;

// The following state is tracked separately from the GL context
//  * because we need to modify it depending on whether we are flipping
//  * the geometry. */
// Bool viewport_dirty;
// int viewport[4];
// Bool scissor_dirty;
// int scissor[4];
// Bool front_face_dirty;
// GLenum front_face;

// We need to keep track of the pack alignment so we can flip the
//  * results of glReadPixels read from a Offscreen */
// int pack_alignment;

// A hash table of GLES2TextureObjects indexed by the texture
//  * object ID so that we can track some state */
// GHashTable *texture_object_map;

// Array of GLES2TextureUnits to keep track of state for each
//  * texture unit */
// GArray *texture_units;

// The currently active texture unit indexed from 0 (not from
//  * GL_TEXTURE0) */
// int current_texture_unit;

// void *winsys;
}

impl GLES2Context {
    /// Allocates a new OpenGLES 2.0 context that can be used to render to
    /// `Offscreen` framebuffers (Rendering to `Onscreen`
    /// framebuffers is not currently supported).
    ///
    /// To actually access the OpenGLES 2.0 api itself you need to use
    /// `GLES2Context::get_vtable`. You should not try to directly link
    /// to and use the symbols provided by the a system OpenGLES 2.0
    /// driver.
    ///
    /// Once you have allocated an OpenGLES 2.0 context you can make it
    /// current using `push_gles2_context`. For those familiar with
    /// using the EGL api, this serves a similar purpose to eglMakeCurrent.
    ///
    /// `<note>`Before using this api applications can check for OpenGLES 2.0
    /// api support by checking for `FeatureID::OglFeatureIdGles2Context` support
    /// with `has_feature`. This fn will return `false` and
    /// return an `GLES2ContextError::Unsupported` error if the
    /// feature isn't available.`</note>`
    ///
    /// ## `ctx`
    /// A `Context`
    ///
    /// # Returns
    ///
    /// A newly allocated `GLES2Context` or `None` if there
    ///  was an error and `error` will be updated in that case.
    pub fn new(ctx: &Context) -> GLES2Context {
        // GLES2Context *gles2_ctx;
        // const WinsysVtable *winsys;

        // if (!has_feature (ctx, FEATURE_ID_GLES2_CONTEXT))
        //     {
        //     _set_error (error, GLES2_CONTEXT_ERROR,
        //                 GLES2_CONTEXT_ERROR_UNSUPPORTED,
        //                 "Backend doesn't support creating GLES2 contexts");

        //     return NULL;
        //     }

        // gles2_ctx = g_malloc0 (sizeof (GLES2Context));

        // gles2_ctx->context = ctx;

        // _list_init (&gles2_ctx->foreign_offscreens);

        // winsys = ctx->display->renderer->winsys_vtable;
        // gles2_ctx->winsys = winsys->context_create_gles2_context (ctx, error);
        // if (gles2_ctx->winsys == NULL)
        //     {
        //     g_free (gles2_ctx);
        //     return NULL;
        //     }

        // gles2_ctx->current_flip_state = GLES2_FLIP_STATE_UNKNOWN;
        // gles2_ctx->viewport_dirty = true;
        // gles2_ctx->scissor_dirty = true;
        // gles2_ctx->front_face_dirty = true;
        // gles2_ctx->front_face = GL_CCW;
        // gles2_ctx->pack_alignment = 4;

        // gles2_ctx->vtable = g_malloc0 (sizeof (GLES2Vtable));
        // #define EXT_BEGIN(name, \
        //                     min_gl_major, min_gl_minor, \
        //                     gles_availability, \
        //                     extension_suffixes, extension_names)

        // #define EXT_FUNCTION(ret, name, args) \
        // gles2_ctx->vtable->name = (void *) ctx->name;

        // #define EXT_END()

        // #include "gl-prototypes/cogl-gles2-functions.h"

        // #undef EXT_BEGIN
        // #undef EXT_FUNCTION
        // #undef EXT_END

        // gles2_ctx->vtable->glBindFramebuffer =
        //     (void *) gl_bind_framebuffer_wrapper;
        // gles2_ctx->vtable->glReadPixels =
        //     (void *) gl_read_pixels_wrapper;
        // gles2_ctx->vtable->glCopyTexImage2D =
        //     (void *) gl_copy_tex_image_2d_wrapper;
        // gles2_ctx->vtable->glCopyTexSubImage2D =
        //     (void *) gl_copy_tex_sub_image_2d_wrapper;

        // gles2_ctx->vtable->glCreateShader = gl_create_shader_wrapper;
        // gles2_ctx->vtable->glDeleteShader = gl_delete_shader_wrapper;
        // gles2_ctx->vtable->glCreateProgram = gl_create_program_wrapper;
        // gles2_ctx->vtable->glDeleteProgram = gl_delete_program_wrapper;
        // gles2_ctx->vtable->glUseProgram = gl_use_program_wrapper;
        // gles2_ctx->vtable->glAttachShader = gl_attach_shader_wrapper;
        // gles2_ctx->vtable->glDetachShader = gl_detach_shader_wrapper;
        // gles2_ctx->vtable->glShaderSource = gl_shader_source_wrapper;
        // gles2_ctx->vtable->glGetShaderSource = gl_get_shader_source_wrapper;
        // gles2_ctx->vtable->glLinkProgram = gl_link_program_wrapper;
        // gles2_ctx->vtable->glGetProgramiv = gl_get_program_iv_wrapper;
        // gles2_ctx->vtable->glGetProgramInfoLog = gl_get_program_info_log_wrapper;
        // gles2_ctx->vtable->glGetShaderInfoLog = gl_get_shader_info_log_wrapper;
        // gles2_ctx->vtable->glClear = gl_clear_wrapper;
        // gles2_ctx->vtable->glDrawElements = gl_draw_elements_wrapper;
        // gles2_ctx->vtable->glDrawArrays = gl_draw_arrays_wrapper;
        // gles2_ctx->vtable->glFrontFace = gl_front_face_wrapper;
        // gles2_ctx->vtable->glViewport = gl_viewport_wrapper;
        // gles2_ctx->vtable->glScissor = gl_scissor_wrapper;
        // gles2_ctx->vtable->glGetBooleanv = gl_get_boolean_v_wrapper;
        // gles2_ctx->vtable->glGetIntegerv = gl_get_integer_v_wrapper;
        // gles2_ctx->vtable->glGetFloatv = gl_get_float_v_wrapper;
        // gles2_ctx->vtable->glPixelStorei = gl_pixel_store_i_wrapper;
        // gles2_ctx->vtable->glActiveTexture = gl_active_texture_wrapper;
        // gles2_ctx->vtable->glDeleteTextures = gl_delete_textures_wrapper;
        // gles2_ctx->vtable->glBindTexture = gl_bind_texture_wrapper;
        // gles2_ctx->vtable->glTexImage2D = gl_tex_image_2d_wrapper;

        // gles2_ctx->shader_map =
        //     g_hash_table_new_full (g_direct_hash,
        //                         g_direct_equal,
        //                         NULL, /* key_destroy */
        //                         (GDestroyNotify) free_shader_data);
        // gles2_ctx->program_map =
        //     g_hash_table_new_full (g_direct_hash,
        //                         g_direct_equal,
        //                         NULL, /* key_destroy */
        //                         (GDestroyNotify) free_program_data);

        // gles2_ctx->texture_object_map =
        //     g_hash_table_new_full (g_direct_hash,
        //                         g_direct_equal,
        //                         NULL, /* key_destroy */
        //                         (GDestroyNotify) free_texture_object_data);

        // gles2_ctx->texture_units = g_array_new (false, /* not zero terminated */
        //                                         true, /* clear */
        //                                         sizeof (GLES2TextureUnitData));
        // gles2_ctx->current_texture_unit = 0;
        // g_array_set_size (gles2_ctx->texture_units, 1);

        // return _gles2_context_object_new (gles2_ctx);
        unimplemented!()
    }

    // /// Queries the OpenGLES 2.0 api fn pointers that should be
    // /// used for rendering with the given `self`.
    // ///
    // /// `<note>`You should not try to directly link to and use the symbols
    // /// provided by any system OpenGLES 2.0 driver.`</note>`
    // ///
    // ///
    // /// # Returns
    // ///
    // /// A pointer to a `GLES2Vtable` providing pointers
    // ///  to functions for the full OpenGLES 2.0 api.
    // pub fn vtable(&self) -> Option<GLES2Vtable> {
    //     return gles2_ctx->vtable;
    // }
}

impl fmt::Display for GLES2Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GLES2Context")
    }
}
