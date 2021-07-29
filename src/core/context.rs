#![allow(clippy::new_without_default)]
use super::{ColorMask, Display, Driver, FeatureFlags, Renderer};
use once_cell::sync::OnceCell;
use std::{
    fmt,
    sync::{Arc, Mutex},
};

pub struct TextureGLVertex {
    //   GLfloat v[3];
//   GLfloat t[2];
//   GLubyte c[4];
}

#[derive(Debug)]
pub struct ContextProps {
    display: Option<Display>,
    driver: Driver,

    // Information about the GPU and driver which we can use to
    // determine certain workarounds
    // GpuInfo gpu;

    // vtables for the driver functions

    // const DriverVtable *driver_vtable;
    // const TextureDriver *texture_driver;
    glsl_major: i32,
    glsl_minor: i32,

    // This is the GLSL version that we will claim that snippets are
    // written against using the #version pragma. This will be the
    // largest version that is less than or equal to the version
    // provided by the driver without massively altering the syntax. Eg,
    // we wouldn't use version 1.3 even if it is available because that
    // removes the ‘attribute’ and ‘varying’ keywords.
    glsl_version_to_use: i32,

    // Features cache
    // unsigned long features[FLAGS_N_LONGS_FOR_SIZE (_N_FEATURE_IDS)];

    // legacy/deprecated feature flags
    feature_flags: FeatureFlags,
    //     unsigned long private_features [FLAGS_N_LONGS_FOR_SIZE (N_PRIVATE_FEATURES)];
    needs_viewport_scissor_workaround: bool,

    //     Framebuffer *viewport_scissor_workaround_framebuffer;

    //     Pipeline *default_pipeline;
    //     PipelineLayer *default_layer_0;
    //     PipelineLayer *default_layer_n;
    //     PipelineLayer *dummy_layer_dependant;

    //     GHashTable *attribute_name_states_hash;
    //     GArray *attribute_name_index_map;
    n_attribute_names: i32,

    //     Bitmask       enabled_builtin_attributes;
    //     Bitmask       enabled_texcoord_attributes;
    //     Bitmask       enabled_custom_attributes;

    // These are temporary bitmasks that are used when disabling
    // builtin,texcoord and custom attribute arrays. They are here just
    // to avoid allocating new ones each time

    //     Bitmask       enable_builtin_attributes_tmp;
    //     Bitmask       enable_texcoord_attributes_tmp;
    //     Bitmask       enable_custom_attributes_tmp;
    //     Bitmask       changed_bits_tmp;
    legacy_backface_culling_enabled: bool,

    // A few handy matrix constants
    //     Matrix        identity_matrix;
    //     Matrix        y_flip_matrix;

    // Value that was last used when calling glMatrixMode to avoid
    // calling it multiple times

    //     MatrixMode    flushed_matrix_mode;

    // The matrix stack entries that should be flushed during the next
    // pipeline state flush

    //     MatrixEntry *current_projection_entry;
    //     MatrixEntry *current_modelview_entry;

    //     MatrixEntry identity_entry;

    // A cache of the last (immutable) matrix stack entries that were
    // flushed to the GL matrix builtins

    //     MatrixEntryCache builtin_flushed_projection;
    //     MatrixEntryCache builtin_flushed_modelview;

    //     GArray           *texture_units;
    active_texture_unit: i32,

    //     PipelineFogState legacy_fog_state;

    // Pipelines
    //     Pipeline     *opaque_color_pipeline; /* used for set_source_color */
    //     Pipeline     *blended_color_pipeline; /* used for set_source_color */
    //     Pipeline     *texture_pipeline; /* used for set_source_texture */
    //     GString          *codegen_header_buffer;
    //     GString          *codegen_source_buffer;
    //     GString          *codegen_boilerplate_buffer;
    //     GList            *source_stack;
    legacy_state_set: i32,

    //     PipelineCache *pipeline_cache;

    // Textures

    //     Texture2D *default_gl_texture_2d_tex;
    //     Texture3D *default_gl_texture_3d_tex;
    //     TextureRectangle *default_gl_texture_rect_tex;

    // Central list of all framebuffers so all journals can be flushed
    // at any time
    //     GList            *framebuffers;

    // Global journal buffers
    //     GArray           *journal_flush_attributes_array;
    //     GArray           *journal_clip_bounds;

    //     GArray           *polygon_vertices;

    // Some simple caching, to minimize state changes...

    //     Pipeline     *current_pipeline;
    current_pipeline_changes_since_flush: u64,
    current_pipeline_with_color_attrib: bool,
    current_pipeline_unknown_color_alpha: bool,
    current_pipeline_age: u64,

    gl_blend_enable_cache: bool,

    depth_test_enabled_cache: bool,
    //     DepthTestFunction depth_test_function_cache;
    depth_writing_enabled_cache: bool,
    depth_range_near_cache: f32,
    depth_range_far_cache: f32,

    legacy_depth_test_enabled: bool,

    //     Buffer       *current_buffer[BUFFER_BIND_TARGET_COUNT];

    // Framebuffers

    //     GSList           *framebuffer_stack;
    //     Framebuffer  *window_buffer;
    //     unsigned long     current_draw_buffer_state_flushed;
    //     unsigned long     current_draw_buffer_changes;
    //     Framebuffer  *current_draw_buffer;
    //     Framebuffer  *current_read_buffer;
    have_last_offscreen_allocate_flags: bool,
    //     OffscreenAllocateFlags last_offscreen_allocate_flags;

    //     GHashTable *swap_callback_closures;
    next_swap_callback_id: i32,

    //     List onscreen_events_queue;
    //     List onscreen_dirty_queue;
    //     Closure *onscreen_dispatch_idle;

    //     GLES2Context *current_gles2_context;
    //     GQueue gles2_context_stack;

    // This becomes true the first time the context is bound to an
    // onscreen buffer. This is used by framebuffer-gl to determine
    // when to initialise the glDrawBuffer state
    was_bound_to_onscreen: bool,

    // Primitives

    //     Path         *current_path;
    //     Pipeline     *stencil_pipeline;

    // Pre-generated VBOs containing indices to generate GL_TRIANGLES
    // out of a vertex array of quads

    //     Indices      *quad_buffer_indices_byte;
    //     unsigned int      quad_buffer_indices_len;
    //     Indices      *quad_buffer_indices;

    //     Indices      *rectangle_byte_indices;
    //     Indices      *rectangle_short_indices;
    rectangle_short_indices_len: i32,

    in_begin_gl_block: bool,

    //     Pipeline     *texture_download_pipeline;
    //     Pipeline     *blit_texture_pipeline;

    //     GSList           *atlases;
    //     GHookList         atlas_reorganize_callbacks;

    // This debugging variable is used to pick a colour for visually
    // displaying the quad batches. It needs to be global so that it can
    // be reset by clear. It needs to be reset to increase the
    // chances of getting the same colour during an animation

    //     uint8_t            journal_rectangles_color;

    // Cached values for GL_MAX_TEXTURE_[IMAGE_]UNITS to avoid calling
    // glGetInteger too often

    //     GLint             max_texture_units;
    //     GLint             max_texture_image_units;
    //     GLint             max_activateable_texture_units;

    // Fragment processing programs

    //     Handle              current_program;

    //     PipelineProgramType current_fragment_program_type;
    //     PipelineProgramType current_vertex_program_type;
    //     GLuint                  current_gl_program;
    current_gl_dither_enabled: bool,
    current_gl_color_mask: ColorMask,
    //     GLenum current_gl_draw_buffer;

    // Clipping
    // true if we have a valid clipping stack flushed. In that case
    // current_clip_stack will describe what the current state is. If
    // this is false then the current clip stack is completely unknown
    // so it will need to be reflushed. In that case current_clip_stack
    // doesn't need to be a valid pointer. We can't just use NULL in
    // current_clip_stack to mark a dirty state because NULL is a valid
    // stack (meaning no clipping)
    current_clip_stack_valid: bool,

    // The clip state that was flushed. This isn't intended to be used as a
    // stack to push and pop new entries. Instead the current stack that the
    // user wants is part of the framebuffer state. This is just used to
    // record the flush state so we can avoid flushing the same state multiple
    // times. When the clip state is flushed this will hold a reference

    //     ClipStack    *current_clip_stack;

    // Whether the stencil buffer was used as part of the current clip state.
    // If true then any further use of the stencil buffer (such as for drawing
    // paths) would need to be merged with the existing stencil buffer
    current_clip_stack_uses_stencil: bool,

    // This is used as a temporary buffer to fill a Buffer when
    // buffer_map fails and we only want to map to fill it with new data

    //     GByteArray       *buffer_map_fallback_array;
    buffer_map_fallback_in_use: bool,
    //     size_t            buffer_map_fallback_offset;

    //     WinsysRectangleState rectangle_state;

    //     SamplerCache *sampler_cache;

    // FIXME: remove these when we remove the last xlib based clutter
    // backend. they should be tracked as part of the renderer but e.g.
    // the eglx backend doesn't yet have a corresponding  winsys
    // and so we wont have a renderer in that case.

    //   #ifdef HAS_XLIB_SUPPORT
    //     int damage_base;
    //     /* List of callback functions that will be given every Xlib event */
    //     GSList *event_filters;
    //     /* Current top of the XError trap state stack. The actual memory for
    //        these is expected to be allocated on the stack by the caller */
    //     XlibTrapState *trap_state;
    //   #endif

    //     unsigned long winsys_features[FLAGS_N_LONGS_FOR_SIZE (WINSYS_FEATURE_N_FEATURES)];
    //     void *winsys;

    // Array of names of uniforms. These are used like quarks to give a
    // unique number to each uniform name except that we ensure that
    // they increase sequentially so that we can use the id as an index
    // into a bitfield representing the uniforms that a pipeline
    // overrides from its parent.

    //     GPtrArray *uniform_names;

    // A hash table to quickly get an index given an existing name. The
    // name strings are owned by the uniform_names array. The values are
    // the uniform location cast to a pointer.

    //     GHashTable *uniform_name_hash;
    //     int n_uniform_names;

    //     PollSource *fences_poll_source;
    //     List fences;

    // This defines a list of function pointers that  uses from
    // either GL or GLES. All functions are accessed indirectly through
    // these pointers rather than linking to them directly

    //   #ifndef APIENTRY
    //   #define APIENTRY
    //   #endif

    //   #define EXT_BEGIN(name, \
    //                          min_gl_major, min_gl_minor, \
    //                          gles_availability, \
    //                          extension_suffixes, extension_names)
    //   #define EXT_FUNCTION(ret, name, args) \
    //     ret (APIENTRY * name) args;
    //   #define EXT_END()

    //   #include "gl-prototypes/cogl-all-functions.h"

    //   #undef EXT_BEGIN
    //   #undef EXT_FUNCTION
    //   #undef EXT_END
}

impl Default for ContextProps {
    fn default() -> Self {
        Self {
            display: None,
            driver: Driver::default(),

            // Information about the GPU and driver which we can use to
            // determine certain workarounds
            // GpuInfo gpu;

            // vtables for the driver functions

            // const DriverVtable *driver_vtable;
            // const TextureDriver *texture_driver;
            glsl_major: 0,
            glsl_minor: 0,

            // This is the GLSL version that we will claim that snippets are
            // written against using the #version pragma. This will be the
            // largest version that is less than or equal to the version
            // provided by the driver without massively altering the syntax. Eg,
            // we wouldn't use version 1.3 even if it is available because that
            // removes the ‘attribute’ and ‘varying’ keywords.
            glsl_version_to_use: 0,

            // Features cache
            // unsigned long features[FLAGS_N_LONGS_FOR_SIZE (_N_FEATURE_IDS)];

            // legacy/deprecated feature flags
            feature_flags: FeatureFlags::empty(),
            //     unsigned long private_features [FLAGS_N_LONGS_FOR_SIZE (N_PRIVATE_FEATURES)];
            needs_viewport_scissor_workaround: false,

            //     Framebuffer *viewport_scissor_workaround_framebuffer;

            //     Pipeline *default_pipeline;
            //     PipelineLayer *default_layer_0;
            //     PipelineLayer *default_layer_n;
            //     PipelineLayer *dummy_layer_dependant;

            //     GHashTable *attribute_name_states_hash;
            //     GArray *attribute_name_index_map;
            n_attribute_names: 0,

            //     Bitmask       enabled_builtin_attributes;
            //     Bitmask       enabled_texcoord_attributes;
            //     Bitmask       enabled_custom_attributes;

            // These are temporary bitmasks that are used when disabling
            // builtin,texcoord and custom attribute arrays. They are here just
            // to avoid allocating new ones each time

            //     Bitmask       enable_builtin_attributes_tmp;
            //     Bitmask       enable_texcoord_attributes_tmp;
            //     Bitmask       enable_custom_attributes_tmp;
            //     Bitmask       changed_bits_tmp;
            legacy_backface_culling_enabled: false,

            // A few handy matrix constants
            //     Matrix        identity_matrix;
            //     Matrix        y_flip_matrix;

            // Value that was last used when calling glMatrixMode to avoid
            // calling it multiple times

            //     MatrixMode    flushed_matrix_mode;

            // The matrix stack entries that should be flushed during the next
            // pipeline state flush

            //     MatrixEntry *current_projection_entry;
            //     MatrixEntry *current_modelview_entry;

            //     MatrixEntry identity_entry;

            // A cache of the last (immutable) matrix stack entries that were
            // flushed to the GL matrix builtins

            //     MatrixEntryCache builtin_flushed_projection;
            //     MatrixEntryCache builtin_flushed_modelview;

            //     GArray           *texture_units;
            active_texture_unit: 0,

            //     PipelineFogState legacy_fog_state;

            // Pipelines
            //     Pipeline     *opaque_color_pipeline; /* used for set_source_color */
            //     Pipeline     *blended_color_pipeline; /* used for set_source_color */
            //     Pipeline     *texture_pipeline; /* used for set_source_texture */
            //     GString          *codegen_header_buffer;
            //     GString          *codegen_source_buffer;
            //     GString          *codegen_boilerplate_buffer;
            //     GList            *source_stack;
            legacy_state_set: 0,

            //     PipelineCache *pipeline_cache;

            // Textures

            //     Texture2D *default_gl_texture_2d_tex;
            //     Texture3D *default_gl_texture_3d_tex;
            //     TextureRectangle *default_gl_texture_rect_tex;

            // Central list of all framebuffers so all journals can be flushed
            // at any time
            //     GList            *framebuffers;

            // Global journal buffers
            //     GArray           *journal_flush_attributes_array;
            //     GArray           *journal_clip_bounds;

            //     GArray           *polygon_vertices;

            // Some simple caching, to minimize state changes...

            //     Pipeline     *current_pipeline;
            current_pipeline_changes_since_flush: 0,
            current_pipeline_with_color_attrib: false,
            current_pipeline_unknown_color_alpha: false,
            current_pipeline_age: 0,

            gl_blend_enable_cache: false,

            depth_test_enabled_cache: false,
            //     DepthTestFunction depth_test_function_cache;
            depth_writing_enabled_cache: false,
            depth_range_near_cache: 0.0,
            depth_range_far_cache: 0.0,

            legacy_depth_test_enabled: false,

            //     Buffer       *current_buffer[BUFFER_BIND_TARGET_COUNT];

            // Framebuffers

            //     GSList           *framebuffer_stack;
            //     Framebuffer  *window_buffer;
            //     unsigned long     current_draw_buffer_state_flushed;
            //     unsigned long     current_draw_buffer_changes;
            //     Framebuffer  *current_draw_buffer;
            //     Framebuffer  *current_read_buffer;
            have_last_offscreen_allocate_flags: false,
            //     OffscreenAllocateFlags last_offscreen_allocate_flags;

            //     GHashTable *swap_callback_closures;
            next_swap_callback_id: 0,

            //     List onscreen_events_queue;
            //     List onscreen_dirty_queue;
            //     Closure *onscreen_dispatch_idle;

            //     GLES2Context *current_gles2_context;
            //     GQueue gles2_context_stack;

            // This becomes true the first time the context is bound to an
            // onscreen buffer. This is used by framebuffer-gl to determine
            // when to initialise the glDrawBuffer state
            was_bound_to_onscreen: false,

            // Primitives

            //     Path         *current_path;
            //     Pipeline     *stencil_pipeline;

            // Pre-generated VBOs containing indices to generate GL_TRIANGLES
            // out of a vertex array of quads

            //     Indices      *quad_buffer_indices_byte;
            //     unsigned int      quad_buffer_indices_len;
            //     Indices      *quad_buffer_indices;

            //     Indices      *rectangle_byte_indices;
            //     Indices      *rectangle_short_indices;
            rectangle_short_indices_len: 0,

            in_begin_gl_block: false,

            //     Pipeline     *texture_download_pipeline;
            //     Pipeline     *blit_texture_pipeline;

            //     GSList           *atlases;
            //     GHookList         atlas_reorganize_callbacks;

            // This debugging variable is used to pick a colour for visually
            // displaying the quad batches. It needs to be global so that it can
            // be reset by clear. It needs to be reset to increase the
            // chances of getting the same colour during an animation

            //     uint8_t            journal_rectangles_color;

            // Cached values for GL_MAX_TEXTURE_[IMAGE_]UNITS to avoid calling
            // glGetInteger too often

            //     GLint             max_texture_units;
            //     GLint             max_texture_image_units;
            //     GLint             max_activateable_texture_units;

            // Fragment processing programs

            //     Handle              current_program;

            //     PipelineProgramType current_fragment_program_type;
            //     PipelineProgramType current_vertex_program_type;
            //     GLuint                  current_gl_program;
            current_gl_dither_enabled: false,
            current_gl_color_mask: ColorMask::NONE,
            //     GLenum current_gl_draw_buffer;

            // Clipping
            // true if we have a valid clipping stack flushed. In that case
            // current_clip_stack will describe what the current state is. If
            // this is false then the current clip stack is completely unknown
            // so it will need to be reflushed. In that case current_clip_stack
            // doesn't need to be a valid pointer. We can't just use NULL in
            // current_clip_stack to mark a dirty state because NULL is a valid
            // stack (meaning no clipping)
            current_clip_stack_valid: false,

            // The clip state that was flushed. This isn't intended to be used as a
            // stack to push and pop new entries. Instead the current stack that the
            // user wants is part of the framebuffer state. This is just used to
            // record the flush state so we can avoid flushing the same state multiple
            // times. When the clip state is flushed this will hold a reference

            //     ClipStack    *current_clip_stack;

            // Whether the stencil buffer was used as part of the current clip state.
            // If true then any further use of the stencil buffer (such as for drawing
            // paths) would need to be merged with the existing stencil buffer
            current_clip_stack_uses_stencil: false,

            // This is used as a temporary buffer to fill a Buffer when
            // buffer_map fails and we only want to map to fill it with new data

            //     GByteArray       *buffer_map_fallback_array;
            buffer_map_fallback_in_use: false,
            //     size_t            buffer_map_fallback_offset;

            //     WinsysRectangleState rectangle_state;

            //     SamplerCache *sampler_cache;

            // FIXME: remove these when we remove the last xlib based clutter
            // backend. they should be tracked as part of the renderer but e.g.
            // the eglx backend doesn't yet have a corresponding  winsys
            // and so we wont have a renderer in that case.

            //   #ifdef HAS_XLIB_SUPPORT
            //     int damage_base;
            //     /* List of callback functions that will be given every Xlib event */
            //     GSList *event_filters;
            //     /* Current top of the XError trap state stack. The actual memory for
            //        these is expected to be allocated on the stack by the caller */
            //     XlibTrapState *trap_state;
            //   #endif

            //     unsigned long winsys_features[FLAGS_N_LONGS_FOR_SIZE (WINSYS_FEATURE_N_FEATURES)];
            //     void *winsys;

            // Array of names of uniforms. These are used like quarks to give a
            // unique number to each uniform name except that we ensure that
            // they increase sequentially so that we can use the id as an index
            // into a bitfield representing the uniforms that a pipeline
            // overrides from its parent.

            //     GPtrArray *uniform_names;

            // A hash table to quickly get an index given an existing name. The
            // name strings are owned by the uniform_names array. The values are
            // the uniform location cast to a pointer.

            //     GHashTable *uniform_name_hash;
            //     int n_uniform_names;

            //     PollSource *fences_poll_source;
            //     List fences;

            // This defines a list of function pointers that  uses from
            // either GL or GLES. All functions are accessed indirectly through
            // these pointers rather than linking to them directly

            //   #ifndef APIENTRY
            //   #define APIENTRY
            //   #endif

            //   #define EXT_BEGIN(name, \
            //                          min_gl_major, min_gl_minor, \
            //                          gles_availability, \
            //                          extension_suffixes, extension_names)
            //   #define EXT_FUNCTION(ret, name, args) \
            //     ret (APIENTRY * name) args;
            //   #define EXT_END()

            //   #include "gl-prototypes/cogl-all-functions.h"

            //   #undef EXT_BEGIN
            //   #undef EXT_FUNCTION
            //   #undef EXT_END
        }
    }
}
unsafe impl Send for ContextProps {}

// SECTION:context
// @short_description: The top level application context.
//
// A #Context is the top most sandbox of  state for an
// application or toolkit. Its main purpose is to act as a sandbox
// for the memory management of state objects. Normally an application
// will only create a single context since there is no way to share
// resources between contexts.
//
// For those familiar with OpenGL or perhaps Cairo it should be
// understood that unlike these APIs a  context isn't a rendering
// context as such. In other words  doesn't aim to provide a state
// machine style model for configuring rendering parameters. Most
// rendering state in  is directly associated with user managed
// objects called pipelines and geometry is drawn with a specific
// pipeline object to a framebuffer object and those 3 things fully
// define the state for drawing. This is an important part of 's
// design since it helps you write orthogonal rendering components
// that can all access the same GPU without having to worry about
// what state other components have left you with.
//
// <note><para> does not maintain internal references to the context for
// resources that depend on the context so applications. This is to
// help applications control the lifetime a context without us needing to
// introduce special api to handle the breakup of internal circular
// references due to internal resources and caches associated with the
// context.
//
// One a context has been destroyed then all directly or indirectly
// dependant resources will be in an inconsistent state and should not
// be manipulated or queried in any way.
//
// For applications that rely on the operating system to clean up
// resources this policy shouldn't affect them, but for applications
// that need to carefully destroy and re-create  contexts multiple
// times throughout their lifetime (such as Android applications) they
// should be careful to destroy all context dependant resources, such as
// framebuffers or textures etc before unrefing and destroying the
// context.</para></note>
#[derive(Debug)]
pub struct Context {
    props: Mutex<ContextProps>,
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
    // display: Option<&Display>
    fn new() -> Context {
        println!("CREATE CONTEXT INSTANCE");

        // Context *context;
        // uint8_t white_pixel[] = { 0xff, 0xff, 0xff, 0xff };
        // Bitmap *white_pixel_bitmap;
        // const WinsysVtable *winsys;
        // int i;
        // Error *internal_error = NULL;

        super::init();

        // We need to be absolutely sure that uprof has been initialized
        // before calling _uprof_init. uprof_init (NULL, NULL)
        // will be a NOP if it has been initialized but it will also
        // mean subsequent parsing of the UProf GOptionGroup will have no
        // affect.
        //
        // Sadly GOptionGroup based library initialization is extremely
        // fragile by design because GOptionGroups have no notion of
        // dependencies and so the order things are initialized isn't
        // currently under tight control.

        #[cfg(feature = "prof")]
        {
            uprof_init(NULL, NULL);
            _uprof_init();
        }

        // Allocate context memory

        // context = g_malloc0 (sizeof (Context));

        // Convert the context into an object immediately in case any of the
        // code below wants to verify that the context pointer is a valid object

        // _context_object_new (context);

        // XXX: Gross hack!
        // Currently everything in  just assumes there is a default
        // context which it can access via _GET_CONTEXT() including
        // code used to construct a Context. Until all of that code
        // has been updated to take an explicit context argument we have
        // to immediately make our pointer the default context.

        // _context = context;

        // Init default values
        // memset (props.features, 0, sizeof (props.features));
        // props.feature_flags = 0;
        // memset (props.private_features, 0, sizeof (props.private_features));

        // props.rectangle_state = WINSYS_RECTANGLE_STATE_UNKNOWN;

        // memset (props.winsys_features, 0, sizeof (props.winsys_features));

        // if !display {
        //     Renderer *renderer = renderer_new ();
        //     if !renderer_connect (renderer, error) {
        //         g_free (context);
        //         return NULL;
        //         }

        //     display = display_new (renderer, NULL);
        //     object_unref(renderer);
        // } else {
        //     object_ref (display);
        // }

        // if !display_setup (display, error) {
        //     object_unref (display);
        //     g_free (context);
        //     return NULL;
        // }

        // props.display = display;

        // This is duplicated data, but it's much more convenient to have
        // the driver attached to the context and the value is accessed a
        // lot throughout

        // props.driver = display->renderer->driver;

        // Again this is duplicated data, but it convenient to be able
        // access these from the context.

        // props.driver_vtable = display->renderer->driver_vtable;
        // props.texture_driver = display->renderer->texture_driver;

        // for (i = 0; i < G_N_ELEMENTS (props.private_features); i++) {
        //     props.private_features[i] |= display->renderer->private_features[i];
        // }

        // winsys = _context_get_winsys (context);
        // if !winsys->context_init (context, error) {
        //     object_unref (display);
        //     g_free (context);
        //     return NULL;
        // }

        // props.attribute_name_states_hash =
        //     g_hash_table_new_full (g_str_hash, g_str_equal, g_free, g_free);
        // props.attribute_name_index_map = NULL;
        // props.n_attribute_names = 0;

        // The "color_in" attribute needs a deterministic name_index
        // so we make sure it's the first attribute name we register

        // _attribute_register_attribute_name (context, "color_in");

        // props.uniform_names =
        //     g_ptr_array_new_with_free_func ((GDestroyNotify) g_free);
        // props.uniform_name_hash = g_hash_table_new (g_str_hash, g_str_equal);
        // props.n_uniform_names = 0;

        // Initialise the driver specific state

        // _init_feature_overrides (context);

        // XXX: ONGOING BUG: Intel viewport scissor
        //
        // Intel gen6 drivers don't currently correctly handle offset
        // viewports, since primitives aren't clipped within the bounds of
        // the viewport.  To workaround this we push our own clip for the
        // viewport that will use scissoring to ensure we clip as expected.
        //
        // TODO: file a bug upstream!

        // if props.gpu.driver_package == GPU_INFO_DRIVER_PACKAGE_MESA &&
        //     props.gpu.architecture == GPU_INFO_ARCHITECTURE_SANDYBRIDGE &&
        //     !getenv ("DISABLE_INTEL_VIEWPORT_SCISSORT_WORKAROUND") {
        //     props.needs_viewport_scissor_workaround = true;
        // } else {
        //     props.needs_viewport_scissor_workaround = false;
        // }

        // props.sampler_cache = _sampler_cache_new (context);

        // _pipeline_init_default_pipeline ();
        // _pipeline_init_default_layers ();
        // _pipeline_init_state_hash_functions ();
        // _pipeline_init_layer_state_hash_functions ();

        // props.current_clip_stack_valid = false;
        // props.current_clip_stack = NULL;

        // props.legacy_backface_culling_enabled = false;

        // matrix_init_identity (&props.identity_matrix);
        // matrix_init_identity (&props.y_flip_matrix);
        // matrix_scale (&props.y_flip_matrix, 1, -1, 1);

        // props.flushed_matrix_mode = MATRIX_MODELVIEW;

        // props.texture_units =
        //     g_array_new (false, false, sizeof (TextureUnit));

        // if _has_private_feature(context, PRIVATE_FEATURE_ANY_GL) {
        //     See cogl-pipeline.c for more details about why we leave texture unit 1
        //     active by default...
        //
        //     props.active_texture_unit = 1;
        //     GE (context, glActiveTexture (GL_TEXTURE1));
        // }

        // props.legacy_fog_state.enabled = false;

        // props.opaque_color_pipeline = pipeline_new (context);
        // props.blended_color_pipeline = pipeline_new (context);
        // props.texture_pipeline = pipeline_new (context);
        // props.codegen_header_buffer = g_string_new ("");
        // props.codegen_source_buffer = g_string_new ("");
        // props.codegen_boilerplate_buffer = g_string_new ("");
        // props.source_stack = NULL;

        // props.legacy_state_set = 0;

        // props.default_gl_texture_2d_tex = NULL;
        // props.default_gl_texture_3d_tex = NULL;
        // props.default_gl_texture_rect_tex = NULL;

        // props.framebuffers = NULL;
        // props.current_draw_buffer = NULL;
        // props.current_read_buffer = NULL;
        // props.current_draw_buffer_state_flushed = 0;
        // props.current_draw_buffer_changes = FRAMEBUFFER_STATE_ALL;

        // props.swap_callback_closures =
        //     g_hash_table_new (g_direct_hash, g_direct_equal);

        // _list_init (&props.onscreen_events_queue);
        // _list_init (&props.onscreen_dirty_queue);

        // g_queue_init (&props.gles2_context_stack);

        // props.journal_flush_attributes_array =
        //     g_array_new (true, false, sizeof (Attribute *));
        // props.journal_clip_bounds = NULL;

        // props.polygon_vertices = g_array_new (false, false, sizeof (float));

        // props.current_pipeline = NULL;
        // props.current_pipeline_changes_since_flush = 0;
        // props.current_pipeline_with_color_attrib = false;

        // _bitmask_init(&props.enabled_builtin_attributes);
        // _bitmask_init(&props.enable_builtin_attributes_tmp);
        // _bitmask_init(&props.enabled_texcoord_attributes);
        // _bitmask_init(&props.enable_texcoord_attributes_tmp);
        // _bitmask_init(&props.enabled_custom_attributes);
        // _bitmask_init(&props.enable_custom_attributes_tmp);
        // _bitmask_init(&props.changed_bits_tmp);

        // props.max_texture_units = -1;
        // props.max_activateable_texture_units = -1;

        // props.current_fragment_program_type = PIPELINE_PROGRAM_TYPE_FIXED;
        // props.current_vertex_program_type = PIPELINE_PROGRAM_TYPE_FIXED;
        // props.current_gl_program = 0;

        // props.current_gl_dither_enabled = true;
        // props.current_gl_color_mask = COLOR_MASK_ALL;

        // props.gl_blend_enable_cache = false;

        // props.depth_test_enabled_cache = false;
        // props.depth_test_function_cache = DEPTH_TEST_FUNCTION_LESS;
        // props.depth_writing_enabled_cache = true;
        // props.depth_range_near_cache = 0;
        // props.depth_range_far_cache = 1;

        // props.legacy_depth_test_enabled = false;

        // props.pipeline_cache = _pipeline_cache_new ();

        // for (i = 0; i < BUFFER_BIND_TARGET_COUNT; i++) {
        //     props.current_buffer[i] = NULL;
        // }

        // props.window_buffer = NULL;
        // props.framebuffer_stack = _create_framebuffer_stack ();

        // XXX: In this case the Clutter backend is still responsible for
        // the OpenGL binding API and for creating onscreen framebuffers and
        // so we have to add a dummy framebuffer to represent the backend
        // owned window...

        // if _context_get_winsys(context) == _winsys_stub_get_vtable () {
        //     Onscreen *window = _onscreen_new ();
        //     set_framebuffer (FRAMEBUFFER (window));
        //     object_unref (FRAMEBUFFER (window));
        // }

        // props.current_path = NULL;
        // props.stencil_pipeline = pipeline_new (context);

        // props.in_begin_gl_block = false;

        // props.quad_buffer_indices_byte = NULL;
        // props.quad_buffer_indices = NULL;
        // props.quad_buffer_indices_len = 0;

        // props.rectangle_byte_indices = NULL;
        // props.rectangle_short_indices = NULL;
        // props.rectangle_short_indices_len = 0;

        // props.texture_download_pipeline = NULL;
        // props.blit_texture_pipeline = NULL;

        // #if defined (HAVE_GL) || defined (HAVE_GLES)
        // if _has_private_feature(context, PRIVATE_FEATURE_ALPHA_TEST) {
        //     /* The default for GL_ALPHA_TEST is to always pass which is equivalent to
        //     * the test being disabled therefore we assume that for all drivers there
        //     * will be no performance impact if we always leave the test enabled which
        //     * makes things a bit simpler for us. Under GLES2 the alpha test is
        //     * implemented in the fragment shader so there is no enable for it
        //     */
        //     GE (context, glEnable (GL_ALPHA_TEST));
        // }
        // #endif

        // #if defined(HAVE_GL)
        // if props.driver == DRIVER_GL3 {
        //     GLuint vertex_array;

        //     In a forward compatible context, GL 3 doesn't support rendering
        //     using the default vertex array object.  doesn't use vertex
        //     array objects yet so for now we just create a dummy array
        //     object that we will use as our own default object. Eventually
        //     it could be good to attach the vertex array objects to Primitives

        //     props.glGenVertexArrays (1, &vertex_array);
        //     props.glBindVertexArray (vertex_array);
        // }
        // #endif

        // props.current_modelview_entry = NULL;
        // props.current_projection_entry = NULL;
        // _matrix_entry_identity_init (&props.identity_entry);
        // _matrix_entry_cache_init (&props.builtin_flushed_projection);
        // _matrix_entry_cache_init (&props.builtin_flushed_modelview);

        // Create default textures used for fall backs

        // props.default_gl_texture_2d_tex =
        //     texture_2d_new_from_data (context,
        //                                 1, 1,
        //                                 PIXEL_FORMAT_RGBA_8888_PRE,
        //                                 0, // rowstride
        //                                 white_pixel,
        //                                 NULL); // abort on error

        // If 3D or rectangle textures aren't supported then these will
        // return errors that we can simply ignore.

        // internal_error = NULL;
        // props.default_gl_texture_3d_tex =
        //     texture_3d_new_from_data (context,
        //                                 1, 1, 1, // width, height, depth
        //                                 PIXEL_FORMAT_RGBA_8888_PRE,
        //                                 0, // rowstride
        //                                 0, // image stride
        //                                 white_pixel,
        //                                 &internal_error);
        // if internal_error {
        //     error_free (internal_error);
        // }

        // TODO: add texture_rectangle_new_from_data()

        // white_pixel_bitmap =
        //     bitmap_new_for_data (context,
        //                             1, 1, // width/height
        //                             PIXEL_FORMAT_RGBA_8888_PRE,
        //                             4, // rowstride
        //                             white_pixel);

        // internal_error = NULL;
        // props.default_gl_texture_rect_tex =
        //     texture_rectangle_new_from_bitmap (white_pixel_bitmap);

        // XXX: we need to allocate the texture now because the white_pixel
        // data is on the stack

        // texture_allocate (TEXTURE (props.default_gl_texture_rect_tex),
        //                         &internal_error);
        // if internal_error {
        //     error_free (internal_error);
        // }

        // object_unref (white_pixel_bitmap);

        // push_source (props.opaque_color_pipeline);

        // props.atlases = NULL;
        // g_hook_list_init (&props.atlas_reorganize_callbacks, sizeof (GHook));

        // props.buffer_map_fallback_array = g_byte_array_new ();
        // props.buffer_map_fallback_in_use = false;

        // As far as I can tell, GL_POINT_SPRITE doesn't have any effect
        // unless GL_COORD_REPLACE is enabled for an individual layer.
        // Therefore it seems like it should be ok to just leave it enabled
        // all the time instead of having to have a set property on each
        // pipeline to track whether any layers have point sprite coords
        // enabled. We don't need to do this for GL3 or GLES2 because point
        // sprites are handled using a builtin varying in the shader. */
        // if _has_private_feature (context, PRIVATE_FEATURE_GL_FIXED) &&
        //     has_feature (context, FEATURE_ID_POINT_SPRITE) {
        //     GE (context, glEnable (GL_POINT_SPRITE));
        // }

        // _list_init (&props.fences);

        Self {
            props: Mutex::new(Default::default()),
        }
    }

    /// Retrieves the singleton instance of `Settings`
    ///
    /// # Returns
    ///
    /// the instance of `Settings`. The
    ///  returned object is owned by internals and it should not be unreferenced
    ///  directly
    pub fn global() -> &'static Self {
        static CONTEXT_INSTANCE: OnceCell<Context> = OnceCell::new();
        CONTEXT_INSTANCE.get_or_init(Self::new)
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
        match self.props.lock() {
            Ok(props) => props.display.as_ref().cloned(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
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
        match self.props.lock() {
            Ok(props) => match &props.display {
                Some(display) => display.get_renderer(),
                None => None,
            },
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    // There no reassign ability for that method
    pub fn init(&self, display: Option<Display>) -> Result<Display, ContextError> {
        println!("Init context");
        match self.props.lock() {
            Ok(props) => match &props.display {
                Some(display) => Ok(display.clone()),
                None => {
                    // Ensure display
                    let display = match display {
                        Some(display) => display,
                        None => {
                            // Crate the default display
                            let renderer = Renderer::new();
                            if !renderer.connect() {
                                return Err(ContextError::CantConnectRenderer);
                            }

                            Display::new(Some(renderer), None)
                        }
                    };
                    Ok(display)
                }
            },
            Err(_) => Err(ContextError::Poisoned),
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context")
    }
}

pub enum ContextError {
    Poisoned,
    NoDisplay,
    CantConnectRenderer,
}
