use crate::{Display, Object, Renderer};
use std::fmt;

pub struct TextureGLVertex {
    //   GLfloat v[3];
//   GLfloat t[2];
//   GLubyte c[4];
}

// * CoglFeatureID:
// * @TEXTURE_NPOT_BASIC: The hardware supports non power
// *     of two textures, but you also need to check the
// *     %TEXTURE_NPOT_MIPMAP and %TEXTURE_NPOT_REPEAT
// *     features to know if the hardware supports npot texture mipmaps
// *     or repeat modes other than
// *     %COGL_PIPELINE_WRAP_MODE_CLAMP_TO_EDGE respectively.
// * @TEXTURE_NPOT_MIPMAP: Mipmapping is supported in
// *     conjuntion with non power of two textures.
// * @TEXTURE_NPOT_REPEAT: Repeat modes other than
// *     %COGL_PIPELINE_WRAP_MODE_CLAMP_TO_EDGE are supported by the
// *     hardware.
// * @TEXTURE_NPOT: Non power of two textures are supported
// *    by the hardware. This is a equivalent to the
// *    %TEXTURE_NPOT_BASIC, %TEXTURE_NPOT_MIPMAP
// *    and %TEXTURE_NPOT_REPEAT features combined.
// * @TEXTURE_RECTANGLE: Support for rectangular
// *    textures with non-normalized texture coordinates.
// * @TEXTURE_RG: Support for
// *    %COGL_TEXTURE_COMPONENTS_RG as the internal components of a
// *    texture.
// * @TEXTURE_3D: 3D texture support
// * @OFFSCREEN: Offscreen rendering support
// * @OFFSCREEN_MULTISAMPLE: Multisample support for
// *    offscreen framebuffers
// * @ONSCREEN_MULTIPLE: Multiple onscreen framebuffers
// *    supported.
// * @GLSL: GLSL support
// * @ARBFP: ARBFP support
// * @UNSIGNED_INT_INDICES: Set if
// *     %COGL_INDICES_TYPE_UNSIGNED_INT is supported in
// *     cogl_indices_new().
// * @DEPTH_RANGE: cogl_pipeline_set_depth_range() support
// * @POINT_SPRITE: Whether
// *     cogl_pipeline_set_layer_point_sprite_coords_enabled() is supported.
// * @PER_VERTEX_POINT_SIZE: Whether cogl_point_size_in
// *     can be used as an attribute to set a per-vertex point size.
// * @MAP_BUFFER_FOR_READ: Whether cogl_buffer_map() is
// *     supported with CoglBufferAccess including read support.
// * @MAP_BUFFER_FOR_WRITE: Whether cogl_buffer_map() is
// *     supported with CoglBufferAccess including write support.
// * @MIRRORED_REPEAT: Whether
// *    %COGL_PIPELINE_WRAP_MODE_MIRRORED_REPEAT is supported.
// * @SWAP_BUFFERS_EVENT:
// *     Available if the window system supports reporting an event
// *     for swap buffer completions.
// * @BUFFER_AGE: Available if the age of #CoglOnscreen back
// *    buffers are tracked and so cogl_onscreen_get_buffer_age() can be
// *    expected to return age values other than 0.
// * @GLES2_CONTEXT: Whether creating new GLES2 contexts is
// *    suported.
// * @DEPTH_TEXTURE: Whether #CoglFramebuffer support rendering
// *     the depth buffer to a texture.
// * @PRESENTATION_TIME: Whether frame presentation
// *    time stamps will be recorded in #CoglFrameInfo objects.
// *
// * All the capabilities that can vary between different GPUs supported
// * by Cogl. Applications that depend on any of these features should explicitly
// * check for them using cogl_has_feature() or cogl_has_features().
// *
// * Since: 1.10

pub enum FeatureID {
//  TEXTURE_NPOT_BASIC = 1,
//  TEXTURE_NPOT_MIPMAP,
//  TEXTURE_NPOT_REPEAT,
//  TEXTURE_NPOT,
//  TEXTURE_RECTANGLE,
//  TEXTURE_3D,
//  GLSL,
//  ARBFP,
//  OFFSCREEN,
//  OFFSCREEN_MULTISAMPLE,
//  ONSCREEN_MULTIPLE,
//  UNSIGNED_INT_INDICES,
//  DEPTH_RANGE,
//  POINT_SPRITE,
//  MAP_BUFFER_FOR_READ,
//  MAP_BUFFER_FOR_WRITE,
//  MIRRORED_REPEAT,
//  SWAP_BUFFERS_EVENT,
//  GLES2_CONTEXT,
//  DEPTH_TEXTURE,
//  PRESENTATION_TIME,
//  FENCE,
//  PER_VERTEX_POINT_SIZE,
//  TEXTURE_RG,
//  BUFFER_AGE,

//  //< private >
//  _COGL_N_FEATURE_IDS
}

// * SECTION:cogl-context
// * @short_description: The top level application context.
// *
// * A #CoglContext is the top most sandbox of Cogl state for an
// * application or toolkit. Its main purpose is to act as a sandbox
// * for the memory management of state objects. Normally an application
// * will only create a single context since there is no way to share
// * resources between contexts.
// *
// * For those familiar with OpenGL or perhaps Cairo it should be
// * understood that unlike these APIs a Cogl context isn't a rendering
// * context as such. In other words Cogl doesn't aim to provide a state
// * machine style model for configuring rendering parameters. Most
// * rendering state in Cogl is directly associated with user managed
// * objects called pipelines and geometry is drawn with a specific
// * pipeline object to a framebuffer object and those 3 things fully
// * define the state for drawing. This is an important part of Cogl's
// * design since it helps you write orthogonal rendering components
// * that can all access the same GPU without having to worry about
// * what state other components have left you with.
// *
// * <note><para>Cogl does not maintain internal references to the context for
// * resources that depend on the context so applications. This is to
// * help applications control the lifetime a context without us needing to
// * introduce special api to handle the breakup of internal circular
// * references due to internal resources and caches associated with the
// * context.
// *
// * One a context has been destroyed then all directly or indirectly
// * dependant resources will be in an inconsistent state and should not
// * be manipulated or queried in any way.
// *
// * For applications that rely on the operating system to clean up
// * resources this policy shouldn't affect them, but for applications
// * that need to carefully destroy and re-create Cogl contexts multiple
// * times throughout their lifetime (such as Android applications) they
// * should be careful to destroy all context dependant resources, such as
// * framebuffers or textures etc before unrefing and destroying the
// * context.</para></note>
pub struct Context {
    //     CoglObject _parent;

//     CoglDisplay *display;

//     CoglDriver driver;

//     /* Information about the GPU and driver which we can use to
//        determine certain workarounds */
//     CoglGpuInfo gpu;

//     /* vtables for the driver functions */
//     const CoglDriverVtable *driver_vtable;
//     const CoglTextureDriver *texture_driver;

//     int glsl_major;
//     int glsl_minor;

//     /* This is the GLSL version that we will claim that snippets are
//      * written against using the #version pragma. This will be the
//      * largest version that is less than or equal to the version
//      * provided by the driver without massively altering the syntax. Eg,
//      * we wouldn't use version 1.3 even if it is available because that
//      * removes the ‘attribute’ and ‘varying’ keywords. */
//     int glsl_version_to_use;

//     /* Features cache */
//     unsigned long features[COGL_FLAGS_N_LONGS_FOR_SIZE (_COGL_N_FEATURE_IDS)];
//     CoglFeatureFlags feature_flags; /* legacy/deprecated feature flags */
//     unsigned long private_features
//       [COGL_FLAGS_N_LONGS_FOR_SIZE (COGL_N_PRIVATE_FEATURES)];

//     CoglBool needs_viewport_scissor_workaround;
//     CoglFramebuffer *viewport_scissor_workaround_framebuffer;

//     CoglPipeline *default_pipeline;
//     CoglPipelineLayer *default_layer_0;
//     CoglPipelineLayer *default_layer_n;
//     CoglPipelineLayer *dummy_layer_dependant;

//     GHashTable *attribute_name_states_hash;
//     GArray *attribute_name_index_map;
//     int n_attribute_names;

//     CoglBitmask       enabled_builtin_attributes;
//     CoglBitmask       enabled_texcoord_attributes;
//     CoglBitmask       enabled_custom_attributes;

//     /* These are temporary bitmasks that are used when disabling
//      * builtin,texcoord and custom attribute arrays. They are here just
//      * to avoid allocating new ones each time */
//     CoglBitmask       enable_builtin_attributes_tmp;
//     CoglBitmask       enable_texcoord_attributes_tmp;
//     CoglBitmask       enable_custom_attributes_tmp;
//     CoglBitmask       changed_bits_tmp;

//     CoglBool          legacy_backface_culling_enabled;

//     /* A few handy matrix constants */
//     CoglMatrix        identity_matrix;
//     CoglMatrix        y_flip_matrix;

//     /* Value that was last used when calling glMatrixMode to avoid
//        calling it multiple times */
//     CoglMatrixMode    flushed_matrix_mode;

//     /* The matrix stack entries that should be flushed during the next
//      * pipeline state flush */
//     CoglMatrixEntry *current_projection_entry;
//     CoglMatrixEntry *current_modelview_entry;

//     CoglMatrixEntry identity_entry;

//     /* A cache of the last (immutable) matrix stack entries that were
//      * flushed to the GL matrix builtins */
//     CoglMatrixEntryCache builtin_flushed_projection;
//     CoglMatrixEntryCache builtin_flushed_modelview;

//     GArray           *texture_units;
//     int               active_texture_unit;

//     CoglPipelineFogState legacy_fog_state;

//     /* Pipelines */
//     CoglPipeline     *opaque_color_pipeline; /* used for set_source_color */
//     CoglPipeline     *blended_color_pipeline; /* used for set_source_color */
//     CoglPipeline     *texture_pipeline; /* used for set_source_texture */
//     GString          *codegen_header_buffer;
//     GString          *codegen_source_buffer;
//     GString          *codegen_boilerplate_buffer;
//     GList            *source_stack;

//     int               legacy_state_set;

//     CoglPipelineCache *pipeline_cache;

//     /* Textures */
//     CoglTexture2D *default_gl_texture_2d_tex;
//     CoglTexture3D *default_gl_texture_3d_tex;
//     CoglTextureRectangle *default_gl_texture_rect_tex;

//     /* Central list of all framebuffers so all journals can be flushed
//      * at any time. */
//     GList            *framebuffers;

//     /* Global journal buffers */
//     GArray           *journal_flush_attributes_array;
//     GArray           *journal_clip_bounds;

//     GArray           *polygon_vertices;

//     /* Some simple caching, to minimize state changes... */
//     CoglPipeline     *current_pipeline;
//     unsigned long     current_pipeline_changes_since_flush;
//     CoglBool          current_pipeline_with_color_attrib;
//     CoglBool          current_pipeline_unknown_color_alpha;
//     unsigned long     current_pipeline_age;

//     CoglBool          gl_blend_enable_cache;

//     CoglBool              depth_test_enabled_cache;
//     CoglDepthTestFunction depth_test_function_cache;
//     CoglBool              depth_writing_enabled_cache;
//     float                 depth_range_near_cache;
//     float                 depth_range_far_cache;

//     CoglBool              legacy_depth_test_enabled;

//     CoglBuffer       *current_buffer[COGL_BUFFER_BIND_TARGET_COUNT];

//     /* Framebuffers */
//     GSList           *framebuffer_stack;
//     CoglFramebuffer  *window_buffer;
//     unsigned long     current_draw_buffer_state_flushed;
//     unsigned long     current_draw_buffer_changes;
//     CoglFramebuffer  *current_draw_buffer;
//     CoglFramebuffer  *current_read_buffer;

//     gboolean have_last_offscreen_allocate_flags;
//     CoglOffscreenAllocateFlags last_offscreen_allocate_flags;

//     GHashTable *swap_callback_closures;
//     int next_swap_callback_id;

//     CoglList onscreen_events_queue;
//     CoglList onscreen_dirty_queue;
//     CoglClosure *onscreen_dispatch_idle;

//     CoglGLES2Context *current_gles2_context;
//     GQueue gles2_context_stack;

//     /* This becomes TRUE the first time the context is bound to an
//      * onscreen buffer. This is used by cogl-framebuffer-gl to determine
//      * when to initialise the glDrawBuffer state */
//     CoglBool was_bound_to_onscreen;

//     /* Primitives */
//     CoglPath         *current_path;
//     CoglPipeline     *stencil_pipeline;

//     /* Pre-generated VBOs containing indices to generate GL_TRIANGLES
//        out of a vertex array of quads */
//     CoglIndices      *quad_buffer_indices_byte;
//     unsigned int      quad_buffer_indices_len;
//     CoglIndices      *quad_buffer_indices;

//     CoglIndices      *rectangle_byte_indices;
//     CoglIndices      *rectangle_short_indices;
//     int               rectangle_short_indices_len;

//     CoglBool          in_begin_gl_block;

//     CoglPipeline     *texture_download_pipeline;
//     CoglPipeline     *blit_texture_pipeline;

//     GSList           *atlases;
//     GHookList         atlas_reorganize_callbacks;

//     /* This debugging variable is used to pick a colour for visually
//        displaying the quad batches. It needs to be global so that it can
//        be reset by cogl_clear. It needs to be reset to increase the
//        chances of getting the same colour during an animation */
//     uint8_t            journal_rectangles_color;

//     /* Cached values for GL_MAX_TEXTURE_[IMAGE_]UNITS to avoid calling
//        glGetInteger too often */
//     GLint             max_texture_units;
//     GLint             max_texture_image_units;
//     GLint             max_activateable_texture_units;

//     /* Fragment processing programs */
//     CoglHandle              current_program;

//     CoglPipelineProgramType current_fragment_program_type;
//     CoglPipelineProgramType current_vertex_program_type;
//     GLuint                  current_gl_program;

//     CoglBool current_gl_dither_enabled;
//     CoglColorMask current_gl_color_mask;
//     GLenum current_gl_draw_buffer;

//     /* Clipping */
//     /* TRUE if we have a valid clipping stack flushed. In that case
//        current_clip_stack will describe what the current state is. If
//        this is FALSE then the current clip stack is completely unknown
//        so it will need to be reflushed. In that case current_clip_stack
//        doesn't need to be a valid pointer. We can't just use NULL in
//        current_clip_stack to mark a dirty state because NULL is a valid
//        stack (meaning no clipping) */
//     CoglBool          current_clip_stack_valid;
//     /* The clip state that was flushed. This isn't intended to be used
//        as a stack to push and pop new entries. Instead the current stack
//        that the user wants is part of the framebuffer state. This is
//        just used to record the flush state so we can avoid flushing the
//        same state multiple times. When the clip state is flushed this
//        will hold a reference */
//     CoglClipStack    *current_clip_stack;
//     /* Whether the stencil buffer was used as part of the current clip
//        state. If TRUE then any further use of the stencil buffer (such
//        as for drawing paths) would need to be merged with the existing
//        stencil buffer */
//     CoglBool          current_clip_stack_uses_stencil;

//     /* This is used as a temporary buffer to fill a CoglBuffer when
//        cogl_buffer_map fails and we only want to map to fill it with new
//        data */
//     GByteArray       *buffer_map_fallback_array;
//     CoglBool          buffer_map_fallback_in_use;
//     size_t            buffer_map_fallback_offset;

//     CoglWinsysRectangleState rectangle_state;

//     CoglSamplerCache *sampler_cache;

//     /* FIXME: remove these when we remove the last xlib based clutter
//      * backend. they should be tracked as part of the renderer but e.g.
//      * the eglx backend doesn't yet have a corresponding Cogl winsys
//      * and so we wont have a renderer in that case. */
//   #ifdef COGL_HAS_XLIB_SUPPORT
//     int damage_base;
//     /* List of callback functions that will be given every Xlib event */
//     GSList *event_filters;
//     /* Current top of the XError trap state stack. The actual memory for
//        these is expected to be allocated on the stack by the caller */
//     CoglXlibTrapState *trap_state;
//   #endif

//     unsigned long winsys_features
//       [COGL_FLAGS_N_LONGS_FOR_SIZE (COGL_WINSYS_FEATURE_N_FEATURES)];
//     void *winsys;

//     /* Array of names of uniforms. These are used like quarks to give a
//        unique number to each uniform name except that we ensure that
//        they increase sequentially so that we can use the id as an index
//        into a bitfield representing the uniforms that a pipeline
//        overrides from its parent. */
//     GPtrArray *uniform_names;
//     /* A hash table to quickly get an index given an existing name. The
//        name strings are owned by the uniform_names array. The values are
//        the uniform location cast to a pointer. */
//     GHashTable *uniform_name_hash;
//     int n_uniform_names;

//     CoglPollSource *fences_poll_source;
//     CoglList fences;

//     /* This defines a list of function pointers that Cogl uses from
//        either GL or GLES. All functions are accessed indirectly through
//        these pointers rather than linking to them directly */
//   #ifndef APIENTRY
//   #define APIENTRY
//   #endif

//   #define COGL_EXT_BEGIN(name, \
//                          min_gl_major, min_gl_minor, \
//                          gles_availability, \
//                          extension_suffixes, extension_names)
//   #define COGL_EXT_FUNCTION(ret, name, args) \
//     ret (APIENTRY * name) args;
//   #define COGL_EXT_END()

//   #include "gl-prototypes/cogl-all-functions.h"

//   #undef COGL_EXT_BEGIN
//   #undef COGL_EXT_FUNCTION
//   #undef COGL_EXT_END
}

impl Context {
    /// Creates a new `Context` which acts as an application sandbox
    /// for any state objects that are allocated.
    /// ## `display`
    /// A `Display` pointer
    ///
    /// # Returns
    ///
    /// A newly allocated `Context`
    pub fn new(display: Option<&Display>) -> Context {
        // CoglContext *context;
        // uint8_t white_pixel[] = { 0xff, 0xff, 0xff, 0xff };
        // CoglBitmap *white_pixel_bitmap;
        // const CoglWinsysVtable *winsys;
        // int i;
        // CoglError *internal_error = NULL;

        // _cogl_init ();

        // #ifdef COGL_ENABLE_PROFILE
        // /* We need to be absolutely sure that uprof has been initialized
        // * before calling _cogl_uprof_init. uprof_init (NULL, NULL)
        // * will be a NOP if it has been initialized but it will also
        // * mean subsequent parsing of the UProf GOptionGroup will have no
        // * affect.
        // *
        // * Sadly GOptionGroup based library initialization is extremely
        // * fragile by design because GOptionGroups have no notion of
        // * dependencies and so the order things are initialized isn't
        // * currently under tight control.
        // */
        // uprof_init (NULL, NULL);
        // _cogl_uprof_init ();
        // #endif

        // /* Allocate context memory */
        // context = g_malloc0 (sizeof (CoglContext));

        // /* Convert the context into an object immediately in case any of the
        //     code below wants to verify that the context pointer is a valid
        //     object */
        // _cogl_context_object_new (context);

        // /* XXX: Gross hack!
        // * Currently everything in Cogl just assumes there is a default
        // * context which it can access via _COGL_GET_CONTEXT() including
        // * code used to construct a CoglContext. Until all of that code
        // * has been updated to take an explicit context argument we have
        // * to immediately make our pointer the default context.
        // */
        // _cogl_context = context;

        // /* Init default values */
        // memset (context->features, 0, sizeof (context->features));
        // context->feature_flags = 0;
        // memset (context->private_features, 0, sizeof (context->private_features));

        // context->rectangle_state = COGL_WINSYS_RECTANGLE_STATE_UNKNOWN;

        // memset (context->winsys_features, 0, sizeof (context->winsys_features));

        // if (!display)
        //     {
        //     CoglRenderer *renderer = cogl_renderer_new ();
        //     if (!cogl_renderer_connect (renderer, error))
        //         {
        //         g_free (context);
        //         return NULL;
        //         }

        //     display = cogl_display_new (renderer, NULL);
        //     cogl_object_unref(renderer);
        //     }
        // else
        //     cogl_object_ref (display);

        // if (!cogl_display_setup (display, error))
        //     {
        //     cogl_object_unref (display);
        //     g_free (context);
        //     return NULL;
        //     }

        // context->display = display;

        // /* This is duplicated data, but it's much more convenient to have
        //     the driver attached to the context and the value is accessed a
        //     lot throughout Cogl */
        // context->driver = display->renderer->driver;

        // /* Again this is duplicated data, but it convenient to be able
        // * access these from the context. */
        // context->driver_vtable = display->renderer->driver_vtable;
        // context->texture_driver = display->renderer->texture_driver;

        // for (i = 0; i < G_N_ELEMENTS (context->private_features); i++)
        //     context->private_features[i] |= display->renderer->private_features[i];

        // winsys = _cogl_context_get_winsys (context);
        // if (!winsys->context_init (context, error))
        //     {
        //     cogl_object_unref (display);
        //     g_free (context);
        //     return NULL;
        //     }

        // context->attribute_name_states_hash =
        //     g_hash_table_new_full (g_str_hash, g_str_equal, g_free, g_free);
        // context->attribute_name_index_map = NULL;
        // context->n_attribute_names = 0;

        // /* The "cogl_color_in" attribute needs a deterministic name_index
        // * so we make sure it's the first attribute name we register */
        // _cogl_attribute_register_attribute_name (context, "cogl_color_in");


        // context->uniform_names =
        //     g_ptr_array_new_with_free_func ((GDestroyNotify) g_free);
        // context->uniform_name_hash = g_hash_table_new (g_str_hash, g_str_equal);
        // context->n_uniform_names = 0;

        // /* Initialise the driver specific state */
        // _cogl_init_feature_overrides (context);

        // /* XXX: ONGOING BUG: Intel viewport scissor
        // *
        // * Intel gen6 drivers don't currently correctly handle offset
        // * viewports, since primitives aren't clipped within the bounds of
        // * the viewport.  To workaround this we push our own clip for the
        // * viewport that will use scissoring to ensure we clip as expected.
        // *
        // * TODO: file a bug upstream!
        // */
        // if (context->gpu.driver_package == COGL_GPU_INFO_DRIVER_PACKAGE_MESA &&
        //     context->gpu.architecture == COGL_GPU_INFO_ARCHITECTURE_SANDYBRIDGE &&
        //     !getenv ("COGL_DISABLE_INTEL_VIEWPORT_SCISSORT_WORKAROUND"))
        //     context->needs_viewport_scissor_workaround = TRUE;
        // else
        //     context->needs_viewport_scissor_workaround = FALSE;

        // context->sampler_cache = _cogl_sampler_cache_new (context);

        // _cogl_pipeline_init_default_pipeline ();
        // _cogl_pipeline_init_default_layers ();
        // _cogl_pipeline_init_state_hash_functions ();
        // _cogl_pipeline_init_layer_state_hash_functions ();

        // context->current_clip_stack_valid = FALSE;
        // context->current_clip_stack = NULL;

        // context->legacy_backface_culling_enabled = FALSE;

        // cogl_matrix_init_identity (&context->identity_matrix);
        // cogl_matrix_init_identity (&context->y_flip_matrix);
        // cogl_matrix_scale (&context->y_flip_matrix, 1, -1, 1);

        // context->flushed_matrix_mode = COGL_MATRIX_MODELVIEW;

        // context->texture_units =
        //     g_array_new (FALSE, FALSE, sizeof (CoglTextureUnit));

        // if (_cogl_has_private_feature (context, COGL_PRIVATE_FEATURE_ANY_GL))
        //     {
        //     /* See cogl-pipeline.c for more details about why we leave texture unit 1
        //     * active by default... */
        //     context->active_texture_unit = 1;
        //     GE (context, glActiveTexture (GL_TEXTURE1));
        //     }

        // context->legacy_fog_state.enabled = FALSE;

        // context->opaque_color_pipeline = cogl_pipeline_new (context);
        // context->blended_color_pipeline = cogl_pipeline_new (context);
        // context->texture_pipeline = cogl_pipeline_new (context);
        // context->codegen_header_buffer = g_string_new ("");
        // context->codegen_source_buffer = g_string_new ("");
        // context->codegen_boilerplate_buffer = g_string_new ("");
        // context->source_stack = NULL;

        // context->legacy_state_set = 0;

        // context->default_gl_texture_2d_tex = NULL;
        // context->default_gl_texture_3d_tex = NULL;
        // context->default_gl_texture_rect_tex = NULL;

        // context->framebuffers = NULL;
        // context->current_draw_buffer = NULL;
        // context->current_read_buffer = NULL;
        // context->current_draw_buffer_state_flushed = 0;
        // context->current_draw_buffer_changes = COGL_FRAMEBUFFER_STATE_ALL;

        // context->swap_callback_closures =
        //     g_hash_table_new (g_direct_hash, g_direct_equal);

        // _cogl_list_init (&context->onscreen_events_queue);
        // _cogl_list_init (&context->onscreen_dirty_queue);

        // g_queue_init (&context->gles2_context_stack);

        // context->journal_flush_attributes_array =
        //     g_array_new (TRUE, FALSE, sizeof (CoglAttribute *));
        // context->journal_clip_bounds = NULL;

        // context->polygon_vertices = g_array_new (FALSE, FALSE, sizeof (float));

        // context->current_pipeline = NULL;
        // context->current_pipeline_changes_since_flush = 0;
        // context->current_pipeline_with_color_attrib = FALSE;

        // _cogl_bitmask_init (&context->enabled_builtin_attributes);
        // _cogl_bitmask_init (&context->enable_builtin_attributes_tmp);
        // _cogl_bitmask_init (&context->enabled_texcoord_attributes);
        // _cogl_bitmask_init (&context->enable_texcoord_attributes_tmp);
        // _cogl_bitmask_init (&context->enabled_custom_attributes);
        // _cogl_bitmask_init (&context->enable_custom_attributes_tmp);
        // _cogl_bitmask_init (&context->changed_bits_tmp);

        // context->max_texture_units = -1;
        // context->max_activateable_texture_units = -1;

        // context->current_fragment_program_type = COGL_PIPELINE_PROGRAM_TYPE_FIXED;
        // context->current_vertex_program_type = COGL_PIPELINE_PROGRAM_TYPE_FIXED;
        // context->current_gl_program = 0;

        // context->current_gl_dither_enabled = TRUE;
        // context->current_gl_color_mask = COGL_COLOR_MASK_ALL;

        // context->gl_blend_enable_cache = FALSE;

        // context->depth_test_enabled_cache = FALSE;
        // context->depth_test_function_cache = COGL_DEPTH_TEST_FUNCTION_LESS;
        // context->depth_writing_enabled_cache = TRUE;
        // context->depth_range_near_cache = 0;
        // context->depth_range_far_cache = 1;

        // context->legacy_depth_test_enabled = FALSE;

        // context->pipeline_cache = _cogl_pipeline_cache_new ();

        // for (i = 0; i < COGL_BUFFER_BIND_TARGET_COUNT; i++)
        //     context->current_buffer[i] = NULL;

        // context->window_buffer = NULL;
        // context->framebuffer_stack = _cogl_create_framebuffer_stack ();

        // /* XXX: In this case the Clutter backend is still responsible for
        // * the OpenGL binding API and for creating onscreen framebuffers and
        // * so we have to add a dummy framebuffer to represent the backend
        // * owned window... */
        // if (_cogl_context_get_winsys (context) == _cogl_winsys_stub_get_vtable ())
        //     {
        //     CoglOnscreen *window = _cogl_onscreen_new ();
        //     cogl_set_framebuffer (COGL_FRAMEBUFFER (window));
        //     cogl_object_unref (COGL_FRAMEBUFFER (window));
        //     }

        // context->current_path = NULL;
        // context->stencil_pipeline = cogl_pipeline_new (context);

        // context->in_begin_gl_block = FALSE;

        // context->quad_buffer_indices_byte = NULL;
        // context->quad_buffer_indices = NULL;
        // context->quad_buffer_indices_len = 0;

        // context->rectangle_byte_indices = NULL;
        // context->rectangle_short_indices = NULL;
        // context->rectangle_short_indices_len = 0;

        // context->texture_download_pipeline = NULL;
        // context->blit_texture_pipeline = NULL;

        // #if defined (HAVE_COGL_GL) || defined (HAVE_COGL_GLES)
        // if (_cogl_has_private_feature (context, COGL_PRIVATE_FEATURE_ALPHA_TEST))
        //     /* The default for GL_ALPHA_TEST is to always pass which is equivalent to
        //     * the test being disabled therefore we assume that for all drivers there
        //     * will be no performance impact if we always leave the test enabled which
        //     * makes things a bit simpler for us. Under GLES2 the alpha test is
        //     * implemented in the fragment shader so there is no enable for it
        //     */
        //     GE (context, glEnable (GL_ALPHA_TEST));
        // #endif

        // #if defined (HAVE_COGL_GL)
        // if ((context->driver == COGL_DRIVER_GL3))
        //     {
        //     GLuint vertex_array;

        //     /* In a forward compatible context, GL 3 doesn't support rendering
        //     * using the default vertex array object. Cogl doesn't use vertex
        //     * array objects yet so for now we just create a dummy array
        //     * object that we will use as our own default object. Eventually
        //     * it could be good to attach the vertex array objects to
        //     * CoglPrimitives */
        //     context->glGenVertexArrays (1, &vertex_array);
        //     context->glBindVertexArray (vertex_array);
        //     }
        // #endif

        // context->current_modelview_entry = NULL;
        // context->current_projection_entry = NULL;
        // _cogl_matrix_entry_identity_init (&context->identity_entry);
        // _cogl_matrix_entry_cache_init (&context->builtin_flushed_projection);
        // _cogl_matrix_entry_cache_init (&context->builtin_flushed_modelview);

        // /* Create default textures used for fall backs */
        // context->default_gl_texture_2d_tex =
        //     cogl_texture_2d_new_from_data (context,
        //                                 1, 1,
        //                                 COGL_PIXEL_FORMAT_RGBA_8888_PRE,
        //                                 0, /* rowstride */
        //                                 white_pixel,
        //                                 NULL); /* abort on error */

        // /* If 3D or rectangle textures aren't supported then these will
        // * return errors that we can simply ignore. */
        // internal_error = NULL;
        // context->default_gl_texture_3d_tex =
        //     cogl_texture_3d_new_from_data (context,
        //                                 1, 1, 1, /* width, height, depth */
        //                                 COGL_PIXEL_FORMAT_RGBA_8888_PRE,
        //                                 0, /* rowstride */
        //                                 0, /* image stride */
        //                                 white_pixel,
        //                                 &internal_error);
        // if (internal_error)
        //     cogl_error_free (internal_error);

        // /* TODO: add cogl_texture_rectangle_new_from_data() */
        // white_pixel_bitmap =
        //     cogl_bitmap_new_for_data (context,
        //                             1, 1, /* width/height */
        //                             COGL_PIXEL_FORMAT_RGBA_8888_PRE,
        //                             4, /* rowstride */
        //                             white_pixel);

        // internal_error = NULL;
        // context->default_gl_texture_rect_tex =
        //     cogl_texture_rectangle_new_from_bitmap (white_pixel_bitmap);

        // /* XXX: we need to allocate the texture now because the white_pixel
        // * data is on the stack */
        // cogl_texture_allocate (COGL_TEXTURE (context->default_gl_texture_rect_tex),
        //                         &internal_error);
        // if (internal_error)
        //     cogl_error_free (internal_error);

        // cogl_object_unref (white_pixel_bitmap);

        // cogl_push_source (context->opaque_color_pipeline);

        // context->atlases = NULL;
        // g_hook_list_init (&context->atlas_reorganize_callbacks, sizeof (GHook));

        // context->buffer_map_fallback_array = g_byte_array_new ();
        // context->buffer_map_fallback_in_use = FALSE;

        // /* As far as I can tell, GL_POINT_SPRITE doesn't have any effect
        //     unless GL_COORD_REPLACE is enabled for an individual layer.
        //     Therefore it seems like it should be ok to just leave it enabled
        //     all the time instead of having to have a set property on each
        //     pipeline to track whether any layers have point sprite coords
        //     enabled. We don't need to do this for GL3 or GLES2 because point
        //     sprites are handled using a builtin varying in the shader. */
        // if (_cogl_has_private_feature (context, COGL_PRIVATE_FEATURE_GL_FIXED) &&
        //     cogl_has_feature (context, COGL_FEATURE_ID_POINT_SPRITE))
        //     GE (context, glEnable (GL_POINT_SPRITE));

        // _cogl_list_init (&context->fences);

        // return context;
        unimplemented!()
    }

    /// Retrieves the `Display` that is internally associated with the
    /// given `self`. This will return the same `Display` that was
    /// passed to `Context::new` or if `None` was passed to
    /// `Context::new` then this function returns a pointer to the
    /// display that was automatically setup internally.
    ///
    /// # Returns
    ///
    /// The `Display` associated with the
    ///  given `self`.
    pub fn get_display(&self) -> Option<Display> {
        // return context->display;
        unimplemented!()
    }

    /// Retrieves the `Renderer` that is internally associated with the
    /// given `self`. This will return the same `Renderer` that was
    /// passed to `Display::new` or if `None` was passed to
    /// `Display::new` or `Context::new` then this function returns
    /// a pointer to the renderer that was automatically connected
    /// internally.
    ///
    /// # Returns
    ///
    /// The `Renderer` associated with the
    ///  given `self`.
    pub fn get_renderer(&self) -> Option<Renderer> {
        // return context->display->renderer;
        unimplemented!()
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context")
    }
}
