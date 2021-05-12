use crate::{
    Color, ColorMask, Context, DepthState, Handle, Matrix, Object, PipelineAlphaFunc,
    PipelineCullFaceMode, PipelineFilter, PipelineWrapMode, Snippet, Texture, TextureType, Winding,
};

use std::{fmt, ptr};

// typedef struct
// {
//   GList *entries;
// } CoglPipelineSnippetList;

// /* Arguments to pass to _cogl_pipeline_snippet_generate_code() */
// typedef struct
// {
//   CoglPipelineSnippetList *snippets;

//   /* Only snippets at this hook point will be used */
//   CoglSnippetHook hook;

//   /* The final function to chain on to after all of the snippets code
//      has been run */
//   const char *chain_function;

//   /* The name of the final generated function */
//   const char *final_name;

//   /* A prefix to insert before each generate function name */
//   const char *function_prefix;

//   /* The return type of all of the functions, or NULL to use void */
//   const char *return_type;

//   /* A variable to return from the functions. The snippets are
//      expected to modify this variable. Ignored if return_type is
//      NULL */
//   const char *return_variable;

//   /* If this is TRUE then it won't allocate a separate variable for
//      the return value. Instead it is expected that the snippet will
//      modify one of the argument variables directly and that will be
//      returned */
//   CoglBool return_variable_is_argument;

//   /* The argument names or NULL if there are none */
//   const char *arguments;

//   /* The argument types or NULL */
//   const char *argument_declarations;

//   /* The string to generate the source into */
//   GString *source_buf;
// } CoglPipelineSnippetData;

// /* XXX: should I rename these as
//  * COGL_PIPELINE_STATE_INDEX_XYZ... ?
//  */
// typedef enum
// {
//   /* sparse state */
//   COGL_PIPELINE_STATE_COLOR_INDEX,
//   COGL_PIPELINE_STATE_BLEND_ENABLE_INDEX,
//   COGL_PIPELINE_STATE_LAYERS_INDEX,
//   COGL_PIPELINE_STATE_LIGHTING_INDEX,
//   COGL_PIPELINE_STATE_ALPHA_FUNC_INDEX,
//   COGL_PIPELINE_STATE_ALPHA_FUNC_REFERENCE_INDEX,
//   COGL_PIPELINE_STATE_BLEND_INDEX,
//   COGL_PIPELINE_STATE_USER_SHADER_INDEX,
//   COGL_PIPELINE_STATE_DEPTH_INDEX,
//   COGL_PIPELINE_STATE_FOG_INDEX,
//   COGL_PIPELINE_STATE_NON_ZERO_POINT_SIZE_INDEX,
//   COGL_PIPELINE_STATE_POINT_SIZE_INDEX,
//   COGL_PIPELINE_STATE_PER_VERTEX_POINT_SIZE_INDEX,
//   COGL_PIPELINE_STATE_LOGIC_OPS_INDEX,
//   COGL_PIPELINE_STATE_CULL_FACE_INDEX,
//   COGL_PIPELINE_STATE_UNIFORMS_INDEX,
//   COGL_PIPELINE_STATE_VERTEX_SNIPPETS_INDEX,
//   COGL_PIPELINE_STATE_FRAGMENT_SNIPPETS_INDEX,

//   /* non-sparse */
//   COGL_PIPELINE_STATE_REAL_BLEND_ENABLE_INDEX,

//   COGL_PIPELINE_STATE_COUNT
// } CoglPipelineStateIndex;

// /* Used in pipeline->differences masks and for notifying pipeline
//  * state changes.
//  *
//  * XXX: If you add or remove state groups here you may need to update
//  * some of the state masks following this enum too!
//  *
//  * FIXME: perhaps it would be better to rename this enum to
//  * CoglPipelineStateGroup to better convey the fact that a single enum
//  * here can map to multiple properties.
//  */
//  typedef enum _CoglPipelineState
//  {
//    COGL_PIPELINE_STATE_COLOR =
//      1L<<COGL_PIPELINE_STATE_COLOR_INDEX,
//    COGL_PIPELINE_STATE_BLEND_ENABLE =
//      1L<<COGL_PIPELINE_STATE_BLEND_ENABLE_INDEX,
//    COGL_PIPELINE_STATE_LAYERS =
//      1L<<COGL_PIPELINE_STATE_LAYERS_INDEX,

//    COGL_PIPELINE_STATE_LIGHTING =
//      1L<<COGL_PIPELINE_STATE_LIGHTING_INDEX,
//    COGL_PIPELINE_STATE_ALPHA_FUNC =
//      1L<<COGL_PIPELINE_STATE_ALPHA_FUNC_INDEX,
//    COGL_PIPELINE_STATE_ALPHA_FUNC_REFERENCE =
//      1L<<COGL_PIPELINE_STATE_ALPHA_FUNC_REFERENCE_INDEX,
//    COGL_PIPELINE_STATE_BLEND =
//      1L<<COGL_PIPELINE_STATE_BLEND_INDEX,
//    COGL_PIPELINE_STATE_USER_SHADER =
//      1L<<COGL_PIPELINE_STATE_USER_SHADER_INDEX,
//    COGL_PIPELINE_STATE_DEPTH =
//      1L<<COGL_PIPELINE_STATE_DEPTH_INDEX,
//    COGL_PIPELINE_STATE_FOG =
//      1L<<COGL_PIPELINE_STATE_FOG_INDEX,
//    COGL_PIPELINE_STATE_NON_ZERO_POINT_SIZE =
//      1L<<COGL_PIPELINE_STATE_NON_ZERO_POINT_SIZE_INDEX,
//    COGL_PIPELINE_STATE_POINT_SIZE =
//      1L<<COGL_PIPELINE_STATE_POINT_SIZE_INDEX,
//    COGL_PIPELINE_STATE_PER_VERTEX_POINT_SIZE =
//      1L<<COGL_PIPELINE_STATE_PER_VERTEX_POINT_SIZE_INDEX,
//    COGL_PIPELINE_STATE_LOGIC_OPS =
//      1L<<COGL_PIPELINE_STATE_LOGIC_OPS_INDEX,
//    COGL_PIPELINE_STATE_CULL_FACE =
//      1L<<COGL_PIPELINE_STATE_CULL_FACE_INDEX,
//    COGL_PIPELINE_STATE_UNIFORMS =
//      1L<<COGL_PIPELINE_STATE_UNIFORMS_INDEX,
//    COGL_PIPELINE_STATE_VERTEX_SNIPPETS =
//      1L<<COGL_PIPELINE_STATE_VERTEX_SNIPPETS_INDEX,
//    COGL_PIPELINE_STATE_FRAGMENT_SNIPPETS =
//      1L<<COGL_PIPELINE_STATE_FRAGMENT_SNIPPETS_INDEX,

//    COGL_PIPELINE_STATE_REAL_BLEND_ENABLE =
//      1L<<COGL_PIPELINE_STATE_REAL_BLEND_ENABLE_INDEX,

//  } CoglPipelineState;

//  typedef enum
// {
//   COGL_PIPELINE_LIGHTING_STATE_PROPERTY_AMBIENT = 1,
//   COGL_PIPELINE_LIGHTING_STATE_PROPERTY_DIFFUSE,
//   COGL_PIPELINE_LIGHTING_STATE_PROPERTY_SPECULAR,
//   COGL_PIPELINE_LIGHTING_STATE_PROPERTY_EMISSION,
//   COGL_PIPELINE_LIGHTING_STATE_PROPERTY_SHININESS
// } CoglPipelineLightingStateProperty;

// typedef struct
// {
//   /* Standard OpenGL lighting model attributes */
//   float ambient[4];
//   float diffuse[4];
//   float specular[4];
//   float emission[4];
//   float shininess;
// } CoglPipelineLightingState;

// typedef struct
// {
//   /* Determines what fragments are discarded based on their alpha */
//   CoglPipelineAlphaFunc alpha_func;
//   float		        alpha_func_reference;
// } CoglPipelineAlphaFuncState;

// typedef enum _CoglPipelineBlendEnable
// {
//   /* XXX: we want to detect users mistakenly using TRUE or FALSE
//    * so start the enum at 2. */
//   COGL_PIPELINE_BLEND_ENABLE_ENABLED = 2,
//   COGL_PIPELINE_BLEND_ENABLE_DISABLED,
//   COGL_PIPELINE_BLEND_ENABLE_AUTOMATIC
// } CoglPipelineBlendEnable;

// typedef struct
// {
//   /* Determines how this pipeline is blended with other primitives */
// #if defined(HAVE_COGL_GLES2) || defined(HAVE_COGL_GL)
//   GLenum    blend_equation_rgb;
//   GLenum    blend_equation_alpha;
//   GLint     blend_src_factor_alpha;
//   GLint     blend_dst_factor_alpha;
//   CoglColor blend_constant;
// #endif
//   GLint     blend_src_factor_rgb;
//   GLint     blend_dst_factor_rgb;
// } CoglPipelineBlendState;

// typedef struct
// {
//   CoglBool        enabled;
//   CoglColor       color;
//   CoglFogMode     mode;
//   float           density;
//   float           z_near;
//   float           z_far;
// } CoglPipelineFogState;

// typedef struct
// {
//   CoglColorMask color_mask;
// } CoglPipelineLogicOpsState;

// typedef struct
// {
//   CoglPipelineCullFaceMode mode;
//   CoglWinding front_winding;
// } CoglPipelineCullFaceState;

// typedef struct
// {
//   CoglBitmask override_mask;

//   /* This is an array of values. Only the uniforms that have a bit set
//      in override_mask have a corresponding value here. The uniform's
//      location is implicit from the order in this array */
//   CoglBoxedValue *override_values;

//   /* Uniforms that have been modified since this pipeline was last
//      flushed */
//   CoglBitmask changed_mask;
// } CoglPipelineUniformsState;

// typedef struct
// {
//   CoglPipelineLightingState lighting_state;
//   CoglPipelineAlphaFuncState alpha_state;
//   CoglPipelineBlendState blend_state;
//   CoglHandle user_program;
//   CoglDepthState depth_state;
//   CoglPipelineFogState fog_state;
//   float point_size;
//   unsigned int non_zero_point_size : 1;
//   unsigned int per_vertex_point_size : 1;
//   CoglPipelineLogicOpsState logic_ops_state;
//   CoglPipelineCullFaceState cull_face_state;
//   CoglPipelineUniformsState uniforms_state;
//   CoglPipelineSnippetList vertex_snippets;
//   CoglPipelineSnippetList fragment_snippets;
// } CoglPipelineBigState;

// typedef struct
// {
//   CoglPipeline *owner;
//   CoglPipelineLayer *layer;
// } CoglPipelineLayerCacheEntry;

// typedef struct _CoglPipelineHashState
// {
//   unsigned long layer_differences;
//   CoglPipelineEvalFlags flags;
//   unsigned int hash;
// } CoglPipelineHashState;

// typedef struct _CoglPipelineFragend
// {
//   void (*start) (CoglPipeline *pipeline,
//                  int n_layers,
//                  unsigned long pipelines_difference);
//   CoglBool (*add_layer) (CoglPipeline *pipeline,
//                          CoglPipelineLayer *layer,
//                          unsigned long layers_difference);
//   CoglBool (*passthrough) (CoglPipeline *pipeline);
//   CoglBool (*end) (CoglPipeline *pipeline,
//                    unsigned long pipelines_difference);

//   void (*pipeline_pre_change_notify) (CoglPipeline *pipeline,
//                                       CoglPipelineState change,
//                                       const CoglColor *new_color);
//   void (*pipeline_set_parent_notify) (CoglPipeline *pipeline);
//   void (*layer_pre_change_notify) (CoglPipeline *owner,
//                                    CoglPipelineLayer *layer,
//                                    CoglPipelineLayerState change);
// } CoglPipelineFragend;

// typedef struct _CoglPipelineVertend
// {
//   void (*start) (CoglPipeline *pipeline,
//                  int n_layers,
//                  unsigned long pipelines_difference);
//   CoglBool (*add_layer) (CoglPipeline *pipeline,
//                          CoglPipelineLayer *layer,
//                          unsigned long layers_difference,
//                          CoglFramebuffer *framebuffer);
//   CoglBool (*end) (CoglPipeline *pipeline,
//                    unsigned long pipelines_difference);

//   void (*pipeline_pre_change_notify) (CoglPipeline *pipeline,
//                                       CoglPipelineState change,
//                                       const CoglColor *new_color);
//   void (*layer_pre_change_notify) (CoglPipeline *owner,
//                                    CoglPipelineLayer *layer,
//                                    CoglPipelineLayerState change);
// } CoglPipelineVertend;

// typedef struct
// {
//   int vertend;
//   int fragend;
//   CoglBool (*start) (CoglPipeline *pipeline);
//   void (*end) (CoglPipeline *pipeline,
//                unsigned long pipelines_difference);
//   void (*pipeline_pre_change_notify) (CoglPipeline *pipeline,
//                                       CoglPipelineState change,
//                                       const CoglColor *new_color);
//   void (*layer_pre_change_notify) (CoglPipeline *owner,
//                                    CoglPipelineLayer *layer,
//                                    CoglPipelineLayerState change);
//   /* This is called after all of the other functions whenever the
//      pipeline is flushed, even if the pipeline hasn't changed since
//      the last flush */
//   void (* pre_paint) (CoglPipeline *pipeline, CoglFramebuffer *framebuffer);
// } CoglPipelineProgend;

// typedef enum
// {
//   COGL_PIPELINE_PROGRAM_TYPE_GLSL = 1,
//   COGL_PIPELINE_PROGRAM_TYPE_ARBFP,
//   COGL_PIPELINE_PROGRAM_TYPE_FIXED
// } CoglPipelineProgramType;

// typedef enum
// {
//   COGL_PIPELINE_GET_LAYER_NO_CREATE = 1<<0
// } CoglPipelineGetLayerFlags;

// /*
//  * CoglPipelineFlushFlag:
//  * @COGL_PIPELINE_FLUSH_FALLBACK_MASK: The fallback_layers member is set to
//  *      a uint32_t mask of the layers that can't be supported with the user
//  *      supplied texture and need to be replaced with fallback textures. (1 =
//  *      fallback, and the least significant bit = layer 0)
//  * @COGL_PIPELINE_FLUSH_DISABLE_MASK: The disable_layers member is set to
//  *      a uint32_t mask of the layers that you want to completly disable
//  *      texturing for (1 = fallback, and the least significant bit = layer 0)
//  * @COGL_PIPELINE_FLUSH_LAYER0_OVERRIDE: The layer0_override_texture member is
//  *      set to a GLuint OpenGL texture name to override the texture used for
//  *      layer 0 of the pipeline. This is intended for dealing with sliced
//  *      textures where you will need to point to each of the texture slices in
//  *      turn when drawing your geometry.  Passing a value of 0 is the same as
//  *      not passing the option at all.
//  * @COGL_PIPELINE_FLUSH_SKIP_GL_COLOR: When flushing the GL state for the
//  *      pipeline don't call glColor.
//  */
//  typedef enum _CoglPipelineFlushFlag
//  {
//    COGL_PIPELINE_FLUSH_FALLBACK_MASK       = 1L<<0,
//    COGL_PIPELINE_FLUSH_DISABLE_MASK        = 1L<<1,
//    COGL_PIPELINE_FLUSH_LAYER0_OVERRIDE     = 1L<<2,
//    COGL_PIPELINE_FLUSH_SKIP_GL_COLOR       = 1L<<3
//  } CoglPipelineFlushFlag;

//  /*
//  * CoglPipelineFlushOptions:
//  *
//  */
// typedef struct _CoglPipelineFlushOptions
// {
//   CoglPipelineFlushFlag flags;

//   uint32_t fallback_layers;
//   uint32_t disable_layers;
//   CoglTexture *layer0_override_texture;
// } CoglPipelineFlushOptions;

// typedef struct
// {
//   /* Total number of pipelines that were ever added to the hash. This
//    * is not decremented when a pipeline is removed. It is only used to
//    * generate a warning if an unusually high number of pipelines are
//    * generated */
//   int n_unique_pipelines;

//   /* This is the expected minimum size we could prune the hash table
//    * to if we were to remove all pipelines that are not in use. This
//    * is only updated after we prune the table */
//   int expected_min_size;

//   /* String that will be used to describe the usage of this hash table
//    * in the debug warning when too many pipelines are generated. This
//    * must be a static string because it won't be copied or freed */
//   const char *debug_string;

//   unsigned int main_state;
//   unsigned int layer_state;

//   GHashTable *table;
// } CoglPipelineHashTable;

// typedef struct
// {
//   CoglPipeline *pipeline;

//   /* Number of usages of this template. If this drops to zero then it
//    * will be a candidate for removal from the cache */
//   int usage_count;
// } CoglPipelineCacheEntry;

// /* XXX: should I rename these as
//  * COGL_PIPELINE_LAYER_STATE_INDEX_XYZ... ?
//  */
//  typedef enum
//  {
//    /* sparse state */
//    COGL_PIPELINE_LAYER_STATE_UNIT_INDEX,
//    COGL_PIPELINE_LAYER_STATE_TEXTURE_TYPE_INDEX,
//    COGL_PIPELINE_LAYER_STATE_TEXTURE_DATA_INDEX,
//    COGL_PIPELINE_LAYER_STATE_SAMPLER_INDEX,
//    COGL_PIPELINE_LAYER_STATE_COMBINE_INDEX,
//    COGL_PIPELINE_LAYER_STATE_COMBINE_CONSTANT_INDEX,
//    COGL_PIPELINE_LAYER_STATE_USER_MATRIX_INDEX,
//    COGL_PIPELINE_LAYER_STATE_POINT_SPRITE_COORDS_INDEX,
//    COGL_PIPELINE_LAYER_STATE_VERTEX_SNIPPETS_INDEX,
//    COGL_PIPELINE_LAYER_STATE_FRAGMENT_SNIPPETS_INDEX,

//    /* note: layers don't currently have any non-sparse state */
//    COGL_PIPELINE_LAYER_STATE_SPARSE_COUNT,
//    COGL_PIPELINE_LAYER_STATE_COUNT = COGL_PIPELINE_LAYER_STATE_SPARSE_COUNT
//  } CoglPipelineLayerStateIndex;

//  /* XXX: If you add or remove state groups here you may need to update
//   * some of the state masks following this enum too!
//   *
//   * FIXME: perhaps it would be better to rename this enum to
//   * CoglPipelineLayerStateGroup to better convey the fact that a single
//   * enum here can map to multiple properties.
//   */
//  typedef enum
//  {
//    COGL_PIPELINE_LAYER_STATE_UNIT =
//      1L<<COGL_PIPELINE_LAYER_STATE_UNIT_INDEX,
//    COGL_PIPELINE_LAYER_STATE_TEXTURE_TYPE =
//      1L<<COGL_PIPELINE_LAYER_STATE_TEXTURE_TYPE_INDEX,
//    COGL_PIPELINE_LAYER_STATE_TEXTURE_DATA =
//      1L<<COGL_PIPELINE_LAYER_STATE_TEXTURE_DATA_INDEX,
//    COGL_PIPELINE_LAYER_STATE_SAMPLER =
//      1L<<COGL_PIPELINE_LAYER_STATE_SAMPLER_INDEX,

//    COGL_PIPELINE_LAYER_STATE_COMBINE =
//      1L<<COGL_PIPELINE_LAYER_STATE_COMBINE_INDEX,
//    COGL_PIPELINE_LAYER_STATE_COMBINE_CONSTANT =
//      1L<<COGL_PIPELINE_LAYER_STATE_COMBINE_CONSTANT_INDEX,
//    COGL_PIPELINE_LAYER_STATE_USER_MATRIX =
//      1L<<COGL_PIPELINE_LAYER_STATE_USER_MATRIX_INDEX,

//    COGL_PIPELINE_LAYER_STATE_POINT_SPRITE_COORDS =
//      1L<<COGL_PIPELINE_LAYER_STATE_POINT_SPRITE_COORDS_INDEX,

//    COGL_PIPELINE_LAYER_STATE_VERTEX_SNIPPETS =
//      1L<<COGL_PIPELINE_LAYER_STATE_VERTEX_SNIPPETS_INDEX,
//    COGL_PIPELINE_LAYER_STATE_FRAGMENT_SNIPPETS =
//      1L<<COGL_PIPELINE_LAYER_STATE_FRAGMENT_SNIPPETS_INDEX,

//    /* COGL_PIPELINE_LAYER_STATE_TEXTURE_INTERN   = 1L<<8, */
//  } CoglPipelineLayerState;

//  typedef enum
// {
//   /* These are the same values as GL */
//   COGL_PIPELINE_COMBINE_FUNC_ADD         = 0x0104,
//   COGL_PIPELINE_COMBINE_FUNC_ADD_SIGNED  = 0x8574,
//   COGL_PIPELINE_COMBINE_FUNC_SUBTRACT    = 0x84E7,
//   COGL_PIPELINE_COMBINE_FUNC_INTERPOLATE = 0x8575,
//   COGL_PIPELINE_COMBINE_FUNC_REPLACE     = 0x1E01,
//   COGL_PIPELINE_COMBINE_FUNC_MODULATE    = 0x2100,
//   COGL_PIPELINE_COMBINE_FUNC_DOT3_RGB    = 0x86AE,
//   COGL_PIPELINE_COMBINE_FUNC_DOT3_RGBA   = 0x86AF
// } CoglPipelineCombineFunc;

// typedef enum
// {
//   /* Note that these numbers are deliberately not the same as the GL
//      numbers so that we can reserve all numbers > TEXTURE0 to store
//      very large layer numbers */
//   COGL_PIPELINE_COMBINE_SOURCE_TEXTURE,
//   COGL_PIPELINE_COMBINE_SOURCE_CONSTANT,
//   COGL_PIPELINE_COMBINE_SOURCE_PRIMARY_COLOR,
//   COGL_PIPELINE_COMBINE_SOURCE_PREVIOUS,
//   COGL_PIPELINE_COMBINE_SOURCE_TEXTURE0
// } CoglPipelineCombineSource;

// typedef enum
// {
//   /* These are the same values as GL */
//   COGL_PIPELINE_COMBINE_OP_SRC_COLOR           = 0x0300,
//   COGL_PIPELINE_COMBINE_OP_ONE_MINUS_SRC_COLOR = 0x0301,
//   COGL_PIPELINE_COMBINE_OP_SRC_ALPHA           = 0x0302,
//   COGL_PIPELINE_COMBINE_OP_ONE_MINUS_SRC_ALPHA = 0x0303
// } CoglPipelineCombineOp;

// typedef struct
// {
//   /* The texture combine state determines how the color of individual
//    * texture fragments are calculated. */
//   CoglPipelineCombineFunc texture_combine_rgb_func;
//   CoglPipelineCombineSource texture_combine_rgb_src[3];
//   CoglPipelineCombineOp texture_combine_rgb_op[3];

//   CoglPipelineCombineFunc texture_combine_alpha_func;
//   CoglPipelineCombineSource texture_combine_alpha_src[3];
//   CoglPipelineCombineOp texture_combine_alpha_op[3];

//   float texture_combine_constant[4];

//   /* The texture matrix dscribes how to transform texture coordinates */
//   CoglMatrix matrix;

//   CoglPipelineSnippetList vertex_snippets;
//   CoglPipelineSnippetList fragment_snippets;

//   CoglBool point_sprite_coords;
// } CoglPipelineLayerBigState;

// struct _CoglPipelineLayer
// {
//   /* XXX: Please think twice about adding members that *have* be
//    * initialized during a _cogl_pipeline_layer_copy. We are aiming
//    * to have copies be as cheap as possible and copies may be
//    * done by the primitives APIs which means they may happen
//    * in performance critical code paths.
//    *
//    * XXX: If you are extending the state we track please consider if
//    * the state is expected to vary frequently across many pipelines or
//    * if the state can be shared among many derived pipelines instead.
//    * This will determine if the state should be added directly to this
//    * structure which will increase the memory overhead for *all*
//    * layers or if instead it can go under ->big_state.
//    */
//   /* Layers represent their state in a tree structure where some of
//    * the state relating to a given pipeline or layer may actually be
//    * owned by one if is ancestors in the tree. We have a common data
//    * type to track the tree heirachy so we can share code... */
//   CoglNode _parent;

//   /* Some layers have a pipeline owner, which is to say that the layer
//    * is referenced in that pipelines->layer_differences list.  A layer
//    * doesn't always have an owner and may simply be an ancestor for
//    * other layers that keeps track of some shared state. */
//   CoglPipeline      *owner;

//   /* The lowest index is blended first then others on top */
//   int	             index;

//   /* A mask of which state groups are different in this layer
//    * in comparison to its parent. */
//   unsigned int       differences;

//   /* Common differences
//    *
//    * As a basic way to reduce memory usage we divide the layer
//    * state into two groups; the minimal state modified in 90% of
//    * all layers and the rest, so that the second group can
//    * be allocated dynamically when required.
//    */
//   /* Each layer is directly associated with a single texture unit */
//   int                        unit_index;

//   /* The type of the texture. This is always set even if the texture
//      is NULL and it will be used to determine what type of texture
//      lookups to use in any shaders generated by the pipeline
//      backends. */
//   CoglTextureType            texture_type;
//   /* The texture for this layer, or NULL for an empty
//    * layer */
//   CoglTexture               *texture;

//   const CoglSamplerCacheEntry *sampler_cache_entry;

//   /* Infrequent differences aren't currently tracked in
//    * a separate, dynamically allocated structure as they are
//    * for pipelines... */
//   CoglPipelineLayerBigState *big_state;

//   /* bitfields */
//   /* Determines if layer->big_state is valid */
//   unsigned int          has_big_state:1;

// };

// typedef enum {
//     COGL_PIPELINE_LAYER_TYPE_TEXTURE
// } CoglPipelineLayerType;

// /**
//  * CoglPipelineFilter:
//  * @COGL_PIPELINE_FILTER_NEAREST: Measuring in manhatten distance from the,
//  *   current pixel center, use the nearest texture texel
//  * @COGL_PIPELINE_FILTER_LINEAR: Use the weighted average of the 4 texels
//  *   nearest the current pixel center
//  * @COGL_PIPELINE_FILTER_NEAREST_MIPMAP_NEAREST: Select the mimap level whose
//  *   texel size most closely matches the current pixel, and use the
//  *   %COGL_PIPELINE_FILTER_NEAREST criterion
//  * @COGL_PIPELINE_FILTER_LINEAR_MIPMAP_NEAREST: Select the mimap level whose
//  *   texel size most closely matches the current pixel, and use the
//  *   %COGL_PIPELINE_FILTER_LINEAR criterion
//  * @COGL_PIPELINE_FILTER_NEAREST_MIPMAP_LINEAR: Select the two mimap levels
//  *   whose texel size most closely matches the current pixel, use
//  *   the %COGL_PIPELINE_FILTER_NEAREST criterion on each one and take
//  *   their weighted average
//  * @COGL_PIPELINE_FILTER_LINEAR_MIPMAP_LINEAR: Select the two mimap levels
//  *   whose texel size most closely matches the current pixel, use
//  *   the %COGL_PIPELINE_FILTER_LINEAR criterion on each one and take
//  *   their weighted average
//  *
//  * Texture filtering is used whenever the current pixel maps either to more
//  * than one texture element (texel) or less than one. These filter enums
//  * correspond to different strategies used to come up with a pixel color, by
//  * possibly referring to multiple neighbouring texels and taking a weighted
//  * average or simply using the nearest texel.
//  */
//  typedef enum {
//     COGL_PIPELINE_FILTER_NEAREST = 0x2600,
//     COGL_PIPELINE_FILTER_LINEAR = 0x2601,
//     COGL_PIPELINE_FILTER_NEAREST_MIPMAP_NEAREST = 0x2700,
//     COGL_PIPELINE_FILTER_LINEAR_MIPMAP_NEAREST = 0x2701,
//     COGL_PIPELINE_FILTER_NEAREST_MIPMAP_LINEAR = 0x2702,
//     COGL_PIPELINE_FILTER_LINEAR_MIPMAP_LINEAR = 0x2703
//   } CoglPipelineFilter;
//   /* NB: these values come from the equivalents in gl.h */
//   /**
//    * CoglPipelineWrapMode:
//    * @COGL_PIPELINE_WRAP_MODE_REPEAT: The texture will be repeated. This
//    *   is useful for example to draw a tiled background.
//    * @COGL_PIPELINE_WRAP_MODE_CLAMP_TO_EDGE: The coordinates outside the
//    *   range 0→1 will sample copies of the edge pixels of the
//    *   texture. This is useful to avoid artifacts if only one copy of
//    *   the texture is being rendered.
//    * @COGL_PIPELINE_WRAP_MODE_AUTOMATIC: Cogl will try to automatically
//    *   decide which of the above two to use. For cogl_rectangle(), it
//    *   will use repeat mode if any of the texture coordinates are
//    *   outside the range 0→1, otherwise it will use clamp to edge. For
//    *   cogl_polygon() it will always use repeat mode. For
//    *   cogl_vertex_buffer_draw() it will use repeat mode except for
//    *   layers that have point sprite coordinate generation enabled. This
//    *   is the default value.
//    *
//    * The wrap mode specifies what happens when texture coordinates
//    * outside the range 0→1 are used. Note that if the filter mode is
//    * anything but %COGL_PIPELINE_FILTER_NEAREST then texels outside the
//    * range 0→1 might be used even when the coordinate is exactly 0 or 1
//    * because OpenGL will try to sample neighbouring pixels. For example
//    * if you are trying to render the full texture then you may get
//    * artifacts around the edges when the pixels from the other side are
//    * merged in if the wrap mode is set to repeat.
//    *
//    * Since: 2.0
//    */
//   /* GL_ALWAYS is just used here as a value that is known not to clash
//    * with any valid GL wrap modes
//    *
//    * XXX: keep the values in sync with the CoglPipelineWrapModeInternal
//    * enum so no conversion is actually needed.
//    */
//   typedef enum {
//     COGL_PIPELINE_WRAP_MODE_REPEAT = 0x2901,
//     COGL_PIPELINE_WRAP_MODE_MIRRORED_REPEAT = 0x8370,
//     COGL_PIPELINE_WRAP_MODE_CLAMP_TO_EDGE = 0x812F,
//     COGL_PIPELINE_WRAP_MODE_AUTOMATIC = 0x0207 /* GL_ALWAYS */
//   } CoglPipelineWrapMode;

pub struct Pipeline {
    //     /* XXX: Please think twice about adding members that *have* be
//    * initialized during a cogl_pipeline_copy. We are aiming to have
//    * copies be as cheap as possible and copies may be done by the
//    * primitives APIs which means they may happen in performance
//    * critical code paths.
//    *
//    * XXX: If you are extending the state we track please consider if
//    * the state is expected to vary frequently across many pipelines or
//    * if the state can be shared among many derived pipelines instead.
//    * This will determine if the state should be added directly to this
//    * structure which will increase the memory overhead for *all*
//    * pipelines or if instead it can go under ->big_state.
//    */

//   /* Layers represent their state in a tree structure where some of
//    * the state relating to a given pipeline or layer may actually be
//    * owned by one if is ancestors in the tree. We have a common data
//    * type to track the tree heirachy so we can share code... */
//    CoglNode _parent;

//    /* When weak pipelines are destroyed the user is notified via this
//     * callback */
//    CoglPipelineDestroyCallback destroy_callback;

//    /* When notifying that a weak pipeline has been destroyed this
//     * private data is passed to the above callback */
//    void *destroy_data;

//    /* We need to track if a pipeline is referenced in the journal
//     * because we can't allow modification to these pipelines without
//     * flushing the journal first */
//    unsigned int journal_ref_count;

//    /* A mask of which sparse state groups are different in this
//     * pipeline in comparison to its parent. */
//    unsigned int differences;

//    /* Whenever a pipeline is modified we increment the age. There's no
//     * guarantee that it won't wrap but it can nevertheless be a
//     * convenient mechanism to determine when a pipeline has been
//     * changed to you can invalidate some some associated cache that
//     * depends on the old state. */
//    unsigned int age;

//    /* This is the primary color of the pipeline.
//     *
//     * This is a sparse property, ref COGL_PIPELINE_STATE_COLOR */
//    CoglColor color;

//    /* A pipeline may be made up with multiple layers used to combine
//     * textures together.
//     *
//     * This is sparse state, ref COGL_PIPELINE_STATE_LAYERS */
//    unsigned int     n_layers;
//    GList	          *layer_differences;

//    /* As a basic way to reduce memory usage we divide the pipeline
//     * state into two groups; the minimal state modified in 90% of
//     * all pipelines and the rest, so that the second group can
//     * be allocated dynamically when required... */
//    CoglPipelineBigState *big_state;

//  #ifdef COGL_DEBUG_ENABLED
//    /* For debugging purposes it's possible to associate a static const
//     * string with a pipeline which can be an aid when trying to trace
//     * where the pipeline originates from */
//    const char      *static_breadcrumb;
//  #endif

//    /* Cached state... */

//    /* A cached, complete list of the layers this pipeline depends
//     * on sorted by layer->unit_index. */
//    CoglPipelineLayer   **layers_cache;
//    /* To avoid a separate ->layers_cache allocation for common
//     * pipelines with only a few layers... */
//    CoglPipelineLayer    *short_layers_cache[3];

//    /* The deprecated cogl_pipeline_get_layers() API returns a
//     * const GList of layers, which we track here... */
//    GList                *deprecated_get_layers_list;

//    /* XXX: consider adding an authorities cache to speed up sparse
//     * property value lookups:
//     * CoglPipeline *authorities_cache[COGL_PIPELINE_N_SPARSE_PROPERTIES];
//     * and corresponding authorities_cache_dirty:1 bitfield
//     */

//    /* bitfields */

//    /* Weak pipelines don't count as dependants on their parents which
//     * means that the parent pipeline can be modified without
//     * considering how the modifications may affect the weak pipeline.
//     */
//    unsigned int          is_weak:1;

//    /* Determines if pipeline->big_state is valid */
//    unsigned int          has_big_state:1;

//    /* By default blending is enabled automatically depending on the
//     * unlit color, the lighting colors or the texture format. The user
//     * can override this to explicitly enable or disable blending.
//     *
//     * This is a sparse property */
//    unsigned int          blend_enable:3;

//    /* There are many factors that can determine if we need to enable
//     * blending, this holds our final decision */
//    unsigned int          real_blend_enable:1;

//    /* Since the code for deciding if blending really needs to be
//     * enabled for a particular pipeline is quite expensive we update
//     * the real_blend_enable flag lazily when flushing a pipeline if
//     * this dirty flag has been set. */
//    unsigned int          dirty_real_blend_enable:1;

//    /* Whenever a pipeline is flushed we keep track of whether the
//     * pipeline was used with a color attribute where we don't know
//     * whether the colors are opaque. The real_blend_enable state
//     * depends on this, and must be updated whenever this changes (even
//     * if dirty_real_blend_enable isn't set) */
//    unsigned int          unknown_color_alpha:1;

//    unsigned int          layers_cache_dirty:1;
//    unsigned int          deprecated_get_layers_list_dirty:1;

//  #ifdef COGL_DEBUG_ENABLED
//    /* For debugging purposes it's possible to associate a static const
//     * string with a pipeline which can be an aid when trying to trace
//     * where the pipeline originates from */
//    unsigned int          has_static_breadcrumb:1;
//  #endif

//    /* There are multiple fragment and vertex processing backends for
//     * CoglPipeline, glsl, arbfp and fixed that are bundled under a
//     * "progend". This identifies the backend being used for the
//     * pipeline. */
//    unsigned int          progend:3;
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
        // CoglPipeline *new;

        // new = cogl_pipeline_copy (context->default_pipeline);
        // #ifdef COGL_DEBUG_ENABLED
        // _cogl_pipeline_set_static_breadcrumb (new, "new");
        // #endif
        // return new;
        unimplemented!()
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
        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));
        // _COGL_RETURN_IF_FAIL (cogl_is_snippet (snippet));
        // _COGL_RETURN_IF_FAIL (snippet->hook >= COGL_SNIPPET_FIRST_LAYER_HOOK);

        // if (snippet->hook < COGL_SNIPPET_FIRST_LAYER_FRAGMENT_HOOK)
        //     _cogl_pipeline_layer_add_vertex_snippet (pipeline,
        //                                             layer_index,
        //                                             snippet);
        // else
        //     _cogl_pipeline_layer_add_fragment_snippet (pipeline,
        //                                             layer_index,
        //                                             snippet);
        unimplemented!()
    }

    /// Adds a shader snippet to `self`. The snippet will wrap around or
    /// replace some part of the pipeline as defined by the hook point in
    /// `snippet`. Note that some hook points are specific to a layer and
    /// must be added with `Pipeline::add_layer_snippet` instead.
    /// ## `snippet`
    /// The `Snippet` to add to the vertex processing hook
    pub fn add_snippet(&self, snippet: &Snippet) {
        // g_return_if_fail (cogl_is_pipeline (pipeline));
        // g_return_if_fail (cogl_is_snippet (snippet));
        // g_return_if_fail (snippet->hook < COGL_SNIPPET_FIRST_LAYER_HOOK);

        // if (snippet->hook < COGL_SNIPPET_FIRST_PIPELINE_FRAGMENT_HOOK)
        //   _cogl_pipeline_add_vertex_snippet (pipeline, snippet);
        // else
        //   _cogl_pipeline_add_fragment_snippet (pipeline, snippet);
        unimplemented!()
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
        // CoglPipeline *pipeline = g_slice_new (CoglPipeline);

        // _cogl_pipeline_node_init (COGL_NODE (pipeline));

        // pipeline->is_weak = is_weak;

        // pipeline->journal_ref_count = 0;

        // pipeline->differences = 0;

        // pipeline->has_big_state = FALSE;

        // /* NB: real_blend_enable isn't a sparse property, it's valid for
        // * every pipeline node so we have fast access to it. */
        // pipeline->real_blend_enable = src->real_blend_enable;
        // pipeline->dirty_real_blend_enable = src->dirty_real_blend_enable;
        // pipeline->unknown_color_alpha = src->unknown_color_alpha;

        // /* XXX:
        // * consider generalizing the idea of "cached" properties. These
        // * would still have an authority like other sparse properties but
        // * you wouldn't have to walk up the ancestry to find the authority
        // * because the value would be cached directly in each pipeline.
        // */
        // pipeline->layers_cache_dirty = TRUE;
        // pipeline->deprecated_get_layers_list = NULL;
        // pipeline->deprecated_get_layers_list_dirty = TRUE;

        // pipeline->progend = src->progend;

        // pipeline->has_static_breadcrumb = FALSE;

        // pipeline->age = 0;

        // _cogl_pipeline_set_parent (pipeline, src, !is_weak);

        // /* The semantics for copying a weak pipeline are that we promote all
        // * weak ancestors to temporarily become strong pipelines until the
        // * copy is freed. */
        // if (!is_weak)
        //     _cogl_pipeline_promote_weak_ancestors (pipeline);

        // return _cogl_pipeline_object_new (pipeline);
        unimplemented!()
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
        // CoglPipeline *authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LAYERS);
        // AppendLayerIndexState state;
        // CoglBool cont;
        // int i;

        // /* XXX: We don't know what the user is going to want to do to the layers
        // * but any modification of layers can result in the layer graph changing
        // * which could confuse _cogl_pipeline_foreach_layer_internal(). We first
        // * get a list of layer indices which will remain valid so long as the
        // * user doesn't remove layers. */
        // state.i = 0;
        // state.indices = g_alloca (authority->n_layers * sizeof (int));

        // _cogl_pipeline_foreach_layer_internal (pipeline,
        //                                         append_layer_index_cb,
        //                                         &state);

        // for (i = 0, cont = TRUE; i < authority->n_layers && cont; i++)
        //     cont = callback (pipeline, state.indices[i], user_data);
        unimplemented!()
    }

    ///
    ///
    /// # Returns
    ///
    /// The alpha test function of `self`.
    pub fn get_alpha_test_function(&self) -> PipelineAlphaFunc {
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), 0);

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_ALPHA_FUNC);

        // return authority->big_state->alpha_state.alpha_func;
        unimplemented!()
    }

    ///
    ///
    /// # Returns
    ///
    /// The alpha test reference value of `self`.
    pub fn get_alpha_test_reference(&self) -> f32 {
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), 0.0f);

        // authority =
        //     _cogl_pipeline_get_authority (pipeline,
        //                                 COGL_PIPELINE_STATE_ALPHA_FUNC_REFERENCE);

        // return authority->big_state->alpha_state.alpha_func_reference;
        unimplemented!()
    }

    /// Retrieves the current ambient color for `self`
    ///
    /// ## `ambient`
    /// The location to store the ambient color
    pub fn get_ambient(&self, ambient: &mut Color) {
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LIGHTING);

        // cogl_color_init_from_4fv (ambient,
        //                             authority->big_state->lighting_state.ambient);
        unimplemented!()
    }

    /// Retrieves the current pipeline color.
    ///
    /// ## `color`
    /// The location to store the color
    pub fn get_color(&self) -> Color {
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_COLOR);

        // *color = authority->color;
        unimplemented!()
    }

    /// Gets the current `ColorMask` of which channels would be written to the
    /// current framebuffer. Each bit set in the mask means that the
    /// corresponding color would be written.
    ///
    /// # Returns
    ///
    /// A `ColorMask`
    pub fn get_color_mask(&self) -> ColorMask {
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), 0);

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LOGIC_OPS);

        // return authority->big_state->logic_ops_state.color_mask;
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_CULL_FACE;
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline),
        //                             COGL_PIPELINE_CULL_FACE_MODE_NONE);

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // return authority->big_state->cull_face_state.mode;
        unimplemented!()
    }

    /// Retrieves the current depth state configuration for the given
    /// `self` as previously set using `Pipeline::set_depth_state`.
    ///
    /// ## `state_out`
    /// A destination `DepthState` struct
    pub fn get_depth_state(&self) -> DepthState {
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority =
        //   _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_DEPTH);
        // *state = authority->big_state->depth_state;
        unimplemented!()
    }

    /// Retrieves the current diffuse color for `self`
    ///
    /// ## `diffuse`
    /// The location to store the diffuse color
    pub fn get_diffuse(&self, diffuse: &mut Color) {
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LIGHTING);

        // cogl_color_init_from_4fv (diffuse,
        //                             authority->big_state->lighting_state.diffuse);
        unimplemented!()
    }

    /// Retrieves the pipelines current emission color.
    ///
    /// ## `emission`
    /// The location to store the emission color
    pub fn get_emission(&self, emission: &mut Color) {
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LIGHTING);

        // cogl_color_init_from_4fv (emission,
        //                             authority->big_state->lighting_state.emission);
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_CULL_FACE;
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline),
        //                             COGL_PIPELINE_CULL_FACE_MODE_NONE);

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // return authority->big_state->cull_face_state.front_winding;
        unimplemented!()
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
        // CoglPipelineFilter min_filter;
        // CoglPipelineFilter mag_filter;

        // _cogl_pipeline_get_layer_filters (pipeline, layer_index,
        //                                     &min_filter, &mag_filter);
        // return mag_filter;
        unimplemented!()
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
        // CoglPipelineFilter min_filter;
        // CoglPipelineFilter mag_filter;

        // _cogl_pipeline_get_layer_filters (pipeline, layer_index,
        //                                     &min_filter, &mag_filter);
        // return min_filter;
        unimplemented!()
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
        // CoglPipelineLayerState       change =
        //     COGL_PIPELINE_LAYER_STATE_POINT_SPRITE_COORDS;
        // CoglPipelineLayer *layer;
        // CoglPipelineLayer *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);
        // /* FIXME: we shouldn't ever construct a layer in a getter function */
        // authority = _cogl_pipeline_layer_get_authority (layer, change);

        // return authority->big_state->point_sprite_coords;
        unimplemented!()
    }

    /// ## `layer_index`
    /// the index of the layer
    ///
    /// # Returns
    ///
    /// the texture that was set for the
    ///  given layer of the pipeline or `None` if no texture was set.
    pub fn get_layer_texture(&self, layer_index: i32) -> Option<Texture> {
        // CoglPipelineLayer *layer =
        //     _cogl_pipeline_get_layer (pipeline, layer_index);
        // return _cogl_pipeline_layer_get_texture (layer);
        unimplemented!()
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
        // CoglPipelineLayer *layer;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // return _cogl_pipeline_layer_get_wrap_mode_p (layer);
        unimplemented!()
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
        // CoglPipelineLayer *layer;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);
        // /* FIXME: we shouldn't ever construct a layer in a getter function */
        // return _cogl_pipeline_layer_get_wrap_mode_s (layer);
        unimplemented!()
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
        // CoglPipelineLayer *layer;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);
        // /* FIXME: we shouldn't ever construct a layer in a getter function */
        // return _cogl_pipeline_layer_get_wrap_mode_t (layer);
        unimplemented!()
    }

    /// Retrieves the number of layers defined for the given `self`
    ///
    ///
    /// # Returns
    ///
    /// the number of layers
    pub fn get_n_layers(&self) -> i32 {
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), 0);

        // authority =
        //   _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LAYERS);

        // return authority->n_layers;
        unimplemented!()
    }

    ///
    ///
    /// # Returns
    ///
    /// `true` if the pipeline has per-vertex point size
    ///  enabled or `false` otherwise. The per-vertex point size can be
    ///  enabled with `Pipeline::set_per_vertex_point_size`.
    pub fn get_per_vertex_point_size(&self) -> bool {
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // authority =
        //     _cogl_pipeline_get_authority (pipeline,
        //                                 COGL_PIPELINE_STATE_PER_VERTEX_POINT_SIZE);

        // return authority->big_state->per_vertex_point_size;
        unimplemented!()
    }

    /// Get the size of points drawn when `VerticesMode::Points` is
    /// used with the vertex buffer API.
    ///
    ///
    /// # Returns
    ///
    /// the point size of the `self`.
    pub fn get_point_size(&self) -> f32 {
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_POINT_SIZE);

        // return authority->big_state->point_size;
        unimplemented!()
    }

    /// Retrieves the pipelines current emission color.
    ///
    ///
    /// # Returns
    ///
    /// The pipelines current shininess value
    pub fn get_shininess(&self) -> f32 {
        // CoglPipeline *authority;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), 0);

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LIGHTING);

        // return authority->big_state->lighting_state.shininess;
        unimplemented!()
    }

    /// Retrieves the pipelines current specular color.
    ///
    /// ## `specular`
    /// The location to store the specular color
    pub fn get_specular(&self, specular: &mut Color) {
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LIGHTING);

        // cogl_color_init_from_4fv (specular,
        //                             authority->big_state->lighting_state.specular);
        unimplemented!()
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
        // void *location_ptr;
        // char *uniform_name_copy;

        // _COGL_GET_CONTEXT (ctx, -1);

        // /* This API is designed as if the uniform locations are specific to
        //     a pipeline but they are actually unique across a whole
        //     CoglContext. Potentially this could just be
        //     cogl_context_get_uniform_location but it seems to make sense to
        //     keep the API this way so that we can change the internals if need
        //     be. */
        // /* Look for an existing uniform with this name */
        // if (g_hash_table_lookup_extended (ctx->uniform_name_hash,
        //                                     uniform_name,
        //                                     NULL,
        //                                     &location_ptr))
        //     return GPOINTER_TO_INT (location_ptr);

        // uniform_name_copy = g_strdup (uniform_name);
        // g_ptr_array_add (ctx->uniform_names, uniform_name_copy);
        // g_hash_table_insert (ctx->uniform_name_hash,
        //                     uniform_name_copy,
        //                     GINT_TO_POINTER (ctx->n_uniform_names));

        // return ctx->n_uniform_names++;
        unimplemented!()
    }

    //pub fn get_user_program(&self) -> /*Unimplemented*/Option<Handle> {
    //    unsafe { TODO: call cogl_sys:cogl_pipeline_get_user_program() }
    //}

    /// This function removes a layer from your pipeline
    /// ## `layer_index`
    /// Specifies the layer you want to remove
    pub fn remove_layer(&self, layer_index: i32) {
        // CoglPipeline         *authority;
        // CoglPipelineLayerInfo layer_info;
        // int                   i;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, COGL_PIPELINE_STATE_LAYERS);

        // /* The layer index of the layer we want info about */
        // layer_info.layer_index = layer_index;

        // /* This will be updated with a reference to the layer being removed
        // * if it can be found. */
        // layer_info.layer = NULL;

        // /* This will be filled in with a list of layers that need to be
        // * dropped down to a lower texture unit to fill the gap of the
        // * removed layer. */
        // layer_info.layers_to_shift =
        //     g_alloca (sizeof (CoglPipelineLayer *) * authority->n_layers);
        // layer_info.n_layers_to_shift = 0;

        // /* Unlike when we query layer info when adding a layer we must
        // * always have a complete layers_to_shift list... */
        // layer_info.ignore_shift_layers_if_found = FALSE;

        // _cogl_pipeline_get_layer_info (authority, &layer_info);

        // if (layer_info.layer == NULL)
        //     return;

        // for (i = 0; i < layer_info.n_layers_to_shift; i++)
        //     {
        //     CoglPipelineLayer *shift_layer = layer_info.layers_to_shift[i];
        //     int unit_index = _cogl_pipeline_layer_get_unit_index (shift_layer);
        //     _cogl_pipeline_set_layer_unit (pipeline, shift_layer, unit_index - 1);
        //     /* NB: shift_layer may not be writeable so _set_layer_unit()
        //     * will allocate a derived layer internally which will become
        //     * owned by pipeline. Check the return value if we need to do
        //     * anything else with this layer. */
        //     }

        // _cogl_pipeline_remove_layer_difference (pipeline, layer_info.layer, TRUE);
        // _cogl_pipeline_try_reverting_layers_authority (pipeline, NULL);

        // pipeline->dirty_real_blend_enable = TRUE;
        unimplemented!()
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
        // _cogl_pipeline_set_alpha_test_function (pipeline, alpha_func);
        // _cogl_pipeline_set_alpha_test_function_reference (pipeline, alpha_reference);
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_LIGHTING;
        // CoglPipeline *authority;
        // CoglPipelineLightingState *lighting_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // lighting_state = &authority->big_state->lighting_state;
        // if (cogl_color_equal (ambient, &lighting_state->ambient))
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // lighting_state = &pipeline->big_state->lighting_state;
        // lighting_state->ambient[0] = cogl_color_get_red_float (ambient);
        // lighting_state->ambient[1] = cogl_color_get_green_float (ambient);
        // lighting_state->ambient[2] = cogl_color_get_blue_float (ambient);
        // lighting_state->ambient[3] = cogl_color_get_alpha_float (ambient);

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_lighting_state_equal);

        // pipeline->dirty_real_blend_enable = TRUE;
        unimplemented!()
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
        // cogl_pipeline_set_ambient (pipeline, color);
        // cogl_pipeline_set_diffuse (pipeline, color);
        unimplemented!()
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
    pub fn set_blend(&self, blend_string: &str) -> bool {
        // CoglPipelineState state = COGL_PIPELINE_STATE_BLEND;
        // CoglPipeline *authority;
        // CoglBlendStringStatement statements[2];
        // CoglBlendStringStatement *rgb;
        // CoglBlendStringStatement *a;
        // int count;
        // CoglPipelineBlendState *blend_state;

        // _COGL_GET_CONTEXT (ctx, FALSE);

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // count =
        //     _cogl_blend_string_compile (blend_description,
        //                                 COGL_BLEND_STRING_CONTEXT_BLENDING,
        //                                 statements,
        //                                 error);
        // if (!count)
        //     return FALSE;

        // if (count == 1)
        //     rgb = a = statements;
        // else
        //     {
        //     rgb = &statements[0];
        //     a = &statements[1];
        //     }

        // authority =
        //     _cogl_pipeline_get_authority (pipeline, state);

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // blend_state = &pipeline->big_state->blend_state;

        // setup_blend_state (rgb,
        //                     &blend_state->blend_equation_rgb,
        //                     &blend_state->blend_src_factor_rgb,
        //                     &blend_state->blend_dst_factor_rgb);
        // setup_blend_state (a,
        //                     &blend_state->blend_equation_alpha,
        //                     &blend_state->blend_src_factor_alpha,
        //                     &blend_state->blend_dst_factor_alpha);

        // /* If we are the current authority see if we can revert to one of our
        // * ancestors being the authority */
        // if (pipeline == authority &&
        //     _cogl_pipeline_get_parent (authority) != NULL)
        //     {
        //     CoglPipeline *parent = _cogl_pipeline_get_parent (authority);
        //     CoglPipeline *old_authority =
        //         _cogl_pipeline_get_authority (parent, state);

        //     if (_cogl_pipeline_blend_state_equal (authority, old_authority))
        //         pipeline->differences &= ~state;
        //     }

        // /* If we weren't previously the authority on this state then we need
        // * to extended our differences mask and so it's possible that some
        // * of our ancestry will now become redundant, so we aim to reparent
        // * ourselves if that's true... */
        // if (pipeline != authority)
        //     {
        //     pipeline->differences |= state;
        //     _cogl_pipeline_prune_redundant_ancestry (pipeline);
        //     }

        // pipeline->dirty_real_blend_enable = TRUE;

        // return TRUE;
        unimplemented!()
    }

    /// When blending is setup to reference a CONSTANT blend factor then
    /// blending will depend on the constant set with this function.
    ///
    /// ## `constant_color`
    /// The constant color you want
    pub fn set_blend_constant(&self, constant_color: &Color) {
        // _COGL_GET_CONTEXT (ctx, NO_RETVAL);

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // if (!_cogl_has_private_feature (ctx, COGL_PRIVATE_FEATURE_BLEND_CONSTANT))
        //     return;

        // #if defined(HAVE_COGL_GLES2) || defined(HAVE_COGL_GL)
        // {
        //     CoglPipelineState state = COGL_PIPELINE_STATE_BLEND;
        //     CoglPipeline *authority;
        //     CoglPipelineBlendState *blend_state;

        //     authority = _cogl_pipeline_get_authority (pipeline, state);

        //     blend_state = &authority->big_state->blend_state;
        //     if (cogl_color_equal (constant_color, &blend_state->blend_constant))
        //     return;

        //     /* - Flush journal primitives referencing the current state.
        //     * - Make sure the pipeline has no dependants so it may be modified.
        //     * - If the pipeline isn't currently an authority for the state being
        //     *   changed, then initialize that state from the current authority.
        //     */
        //     _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        //     blend_state = &pipeline->big_state->blend_state;
        //     blend_state->blend_constant = *constant_color;

        //     _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                     _cogl_pipeline_blend_state_equal);

        //     pipeline->dirty_real_blend_enable = TRUE;
        // }
        // #endif
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_COLOR;
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // if (cogl_color_equal (color, &authority->color))
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, color, FALSE);

        // pipeline->color = *color;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_color_equal);

        // pipeline->dirty_real_blend_enable = TRUE;
        unimplemented!()
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
        // CoglColor color;
        // cogl_color_init_from_4f (&color, red, green, blue, alpha);
        // cogl_pipeline_set_color (pipeline, &color);
        unimplemented!()
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
        // CoglColor color;
        // cogl_color_init_from_4ub (&color, red, green, blue, alpha);
        // cogl_pipeline_set_color (pipeline, &color);
        unimplemented!()
    }

    /// Defines a bit mask of which color channels should be written to the
    /// current framebuffer. If a bit is set in `color_mask` that means that
    /// color will be written.
    /// ## `color_mask`
    /// A `ColorMask` of which color channels to write to
    ///  the current framebuffer.
    pub fn set_color_mask(&self, color_mask: ColorMask) {
        // CoglPipelineState state = COGL_PIPELINE_STATE_LOGIC_OPS;
        // CoglPipeline *authority;
        // CoglPipelineLogicOpsState *logic_ops_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // logic_ops_state = &authority->big_state->logic_ops_state;
        // if (logic_ops_state->color_mask == color_mask)
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // logic_ops_state = &pipeline->big_state->logic_ops_state;
        // logic_ops_state->color_mask = color_mask;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_logic_ops_state_equal);
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_CULL_FACE;
        // CoglPipeline *authority;
        // CoglPipelineCullFaceState *cull_face_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // cull_face_state = &authority->big_state->cull_face_state;

        // if (cull_face_state->mode == cull_face_mode)
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // pipeline->big_state->cull_face_state.mode = cull_face_mode;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_cull_face_state_equal);
        unimplemented!()
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
    pub fn set_depth_state(&self, state: &DepthState) -> bool {
        // CoglPipelineState state = COGL_PIPELINE_STATE_DEPTH;
        // CoglPipeline *authority;
        // CoglDepthState *orig_state;

        // _COGL_GET_CONTEXT (ctx, FALSE);

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);
        // _COGL_RETURN_VAL_IF_FAIL (depth_state->magic == COGL_DEPTH_STATE_MAGIC, FALSE);

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // orig_state = &authority->big_state->depth_state;
        // if (orig_state->test_enabled == depth_state->test_enabled &&
        //     orig_state->write_enabled == depth_state->write_enabled &&
        //     orig_state->test_function == depth_state->test_function &&
        //     orig_state->range_near == depth_state->range_near &&
        //     orig_state->range_far == depth_state->range_far)
        //     return TRUE;

        // if (ctx->driver == COGL_DRIVER_GLES1 &&
        //     (depth_state->range_near != 0 ||
        //     depth_state->range_far != 1))
        //     {
        //     _cogl_set_error (error,
        //                     COGL_SYSTEM_ERROR,
        //                     COGL_SYSTEM_ERROR_UNSUPPORTED,
        //                     "glDepthRange not available on GLES 1");
        //     return FALSE;
        //     }

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // pipeline->big_state->depth_state = *depth_state;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_depth_state_equal);

        // return TRUE;
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_LIGHTING;
        // CoglPipeline *authority;
        // CoglPipelineLightingState *lighting_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // lighting_state = &authority->big_state->lighting_state;
        // if (cogl_color_equal (diffuse, &lighting_state->diffuse))
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // lighting_state = &pipeline->big_state->lighting_state;
        // lighting_state->diffuse[0] = cogl_color_get_red_float (diffuse);
        // lighting_state->diffuse[1] = cogl_color_get_green_float (diffuse);
        // lighting_state->diffuse[2] = cogl_color_get_blue_float (diffuse);
        // lighting_state->diffuse[3] = cogl_color_get_alpha_float (diffuse);

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_lighting_state_equal);

        // pipeline->dirty_real_blend_enable = TRUE;
        unimplemented!()
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
        // CoglPipeline *authority;
        // CoglPipelineState state = COGL_PIPELINE_STATE_LIGHTING;
        // CoglPipelineLightingState *lighting_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // lighting_state = &authority->big_state->lighting_state;
        // if (cogl_color_equal (emission, &lighting_state->emission))
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // lighting_state = &pipeline->big_state->lighting_state;
        // lighting_state->emission[0] = cogl_color_get_red_float (emission);
        // lighting_state->emission[1] = cogl_color_get_green_float (emission);
        // lighting_state->emission[2] = cogl_color_get_blue_float (emission);
        // lighting_state->emission[3] = cogl_color_get_alpha_float (emission);

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_lighting_state_equal);

        // pipeline->dirty_real_blend_enable = TRUE;
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_CULL_FACE;
        // CoglPipeline *authority;
        // CoglPipelineCullFaceState *cull_face_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // cull_face_state = &authority->big_state->cull_face_state;

        // if (cull_face_state->front_winding == front_winding)
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // pipeline->big_state->cull_face_state.front_winding = front_winding;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_cull_face_state_equal);
        unimplemented!()
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
    ) -> bool {
        // CoglPipelineLayerState state = COGL_PIPELINE_LAYER_STATE_COMBINE;
        // CoglPipelineLayer *authority;
        // CoglPipelineLayer *layer;
        // CoglBlendStringStatement statements[2];
        // CoglBlendStringStatement split[2];
        // CoglBlendStringStatement *rgb;
        // CoglBlendStringStatement *a;
        // int count;

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, state);

        // count =
        //     _cogl_blend_string_compile (combine_description,
        //                                 COGL_BLEND_STRING_CONTEXT_TEXTURE_COMBINE,
        //                                 statements,
        //                                 error);
        // if (!count)
        //     return FALSE;

        // if (statements[0].mask == COGL_BLEND_STRING_CHANNEL_MASK_RGBA)
        //     {
        //     _cogl_blend_string_split_rgba_statement (statements,
        //                                             &split[0], &split[1]);
        //     rgb = &split[0];
        //     a = &split[1];
        //     }
        // else
        //     {
        //     rgb = &statements[0];
        //     a = &statements[1];
        //     }

        // /* FIXME: compare the new state with the current state! */
        // /* possibly flush primitives referencing the current state... */
        // layer = _cogl_pipeline_layer_pre_change_notify (pipeline, layer, state);

        // setup_texture_combine_state (rgb,
        //                             &layer->big_state->texture_combine_rgb_func,
        //                             layer->big_state->texture_combine_rgb_src,
        //                             layer->big_state->texture_combine_rgb_op);

        // setup_texture_combine_state (a,
        //                             &layer->big_state->texture_combine_alpha_func,
        //                             layer->big_state->texture_combine_alpha_src,
        //                             layer->big_state->texture_combine_alpha_op);

        // /* If the original layer we found is currently the authority on
        // * the state we are changing see if we can revert to one of our
        // * ancestors being the authority. */
        // if (layer == authority &&
        //     _cogl_pipeline_layer_get_parent (authority) != NULL)
        //     {
        //     CoglPipelineLayer *parent = _cogl_pipeline_layer_get_parent (authority);
        //     CoglPipelineLayer *old_authority =
        //         _cogl_pipeline_layer_get_authority (parent, state);

        //     if (_cogl_pipeline_layer_combine_state_equal (authority,
        //                                                     old_authority))
        //         {
        //         layer->differences &= ~state;

        //         g_assert (layer->owner == pipeline);
        //         if (layer->differences == 0)
        //             _cogl_pipeline_prune_empty_layer_difference (pipeline,
        //                                                         layer);
        //         goto changed;
        //         }
        //     }

        // /* If we weren't previously the authority on this state then we need
        // * to extended our differences mask and so it's possible that some
        // * of our ancestry will now become redundant, so we aim to reparent
        // * ourselves if that's true... */
        // if (layer != authority)
        //     {
        //     layer->differences |= state;
        //     _cogl_pipeline_layer_prune_redundant_ancestry (layer);
        //     }

        // changed:

        // pipeline->dirty_real_blend_enable = TRUE;
        // return TRUE;
        unimplemented!()
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
        // CoglPipelineLayerState state = COGL_PIPELINE_LAYER_STATE_COMBINE_CONSTANT;
        // CoglPipelineLayer     *layer;
        // CoglPipelineLayer     *authority;
        // CoglPipelineLayer     *new;
        // float                  color_as_floats[4];

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, state);

        // color_as_floats[0] = cogl_color_get_red_float (constant_color);
        // color_as_floats[1] = cogl_color_get_green_float (constant_color);
        // color_as_floats[2] = cogl_color_get_blue_float (constant_color);
        // color_as_floats[3] = cogl_color_get_alpha_float (constant_color);

        // if (memcmp (authority->big_state->texture_combine_constant,
        //             color_as_floats, sizeof (float) * 4) == 0)
        //     return;

        // new = _cogl_pipeline_layer_pre_change_notify (pipeline, layer, state);
        // if (new != layer)
        //     layer = new;
        // else
        //     {
        //     /* If the original layer we found is currently the authority on
        //     * the state we are changing see if we can revert to one of our
        //     * ancestors being the authority. */
        //     if (layer == authority &&
        //         _cogl_pipeline_layer_get_parent (authority) != NULL)
        //         {
        //         CoglPipelineLayer *parent =
        //             _cogl_pipeline_layer_get_parent (authority);
        //         CoglPipelineLayer *old_authority =
        //             _cogl_pipeline_layer_get_authority (parent, state);
        //         CoglPipelineLayerBigState *old_big_state = old_authority->big_state;

        //         if (memcmp (old_big_state->texture_combine_constant,
        //                     color_as_floats, sizeof (float) * 4) == 0)
        //             {
        //             layer->differences &= ~state;

        //             g_assert (layer->owner == pipeline);
        //             if (layer->differences == 0)
        //                 _cogl_pipeline_prune_empty_layer_difference (pipeline,
        //                                                             layer);
        //             goto changed;
        //             }
        //         }
        //     }

        // memcpy (layer->big_state->texture_combine_constant,
        //         color_as_floats,
        //         sizeof (color_as_floats));

        // /* If we weren't previously the authority on this state then we need
        // * to extended our differences mask and so it's possible that some
        // * of our ancestry will now become redundant, so we aim to reparent
        // * ourselves if that's true... */
        // if (layer != authority)
        //     {
        //     layer->differences |= state;
        //     _cogl_pipeline_layer_prune_redundant_ancestry (layer);
        //     }

        // changed:

        // pipeline->dirty_real_blend_enable = TRUE;
        unimplemented!()
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
        // CoglPipelineLayerState state = COGL_PIPELINE_LAYER_STATE_SAMPLER;
        // CoglPipelineLayer *layer;
        // CoglPipelineLayer *authority;
        // const CoglSamplerCacheEntry *sampler_state;

        // _COGL_GET_CONTEXT (ctx, NO_RETVAL);

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // _COGL_RETURN_IF_FAIL (mag_filter == COGL_PIPELINE_FILTER_NEAREST ||
        //                         mag_filter == COGL_PIPELINE_FILTER_LINEAR);

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, state);

        // sampler_state =
        //     _cogl_sampler_cache_update_filters (ctx->sampler_cache,
        //                                         authority->sampler_cache_entry,
        //                                         min_filter,
        //                                         mag_filter);
        // _cogl_pipeline_set_layer_sampler_state (pipeline,
        //                                         layer,
        //                                         authority,
        //                                         sampler_state);
        unimplemented!()
    }

    /// This function lets you set a matrix that can be used to e.g. translate
    /// and rotate a single layer of a pipeline used to fill your geometry.
    /// ## `layer_index`
    /// the index for the layer inside `self`
    /// ## `matrix`
    /// the transformation matrix for the layer
    pub fn set_layer_matrix(&self, layer_index: i32, matrix: &Matrix) {
        // CoglPipelineLayerState state = COGL_PIPELINE_LAYER_STATE_USER_MATRIX;
        // CoglPipelineLayer     *layer;
        // CoglPipelineLayer     *authority;
        // CoglPipelineLayer     *new;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, state);

        // if (cogl_matrix_equal (matrix, &authority->big_state->matrix))
        //     return;

        // new = _cogl_pipeline_layer_pre_change_notify (pipeline, layer, state);
        // if (new != layer)
        //     layer = new;
        // else
        //     {
        //     /* If the original layer we found is currently the authority on
        //     * the state we are changing see if we can revert to one of our
        //     * ancestors being the authority. */
        //     if (layer == authority &&
        //         _cogl_pipeline_layer_get_parent (authority) != NULL)
        //         {
        //         CoglPipelineLayer *parent =
        //             _cogl_pipeline_layer_get_parent (authority);
        //         CoglPipelineLayer *old_authority =
        //             _cogl_pipeline_layer_get_authority (parent, state);

        //         if (cogl_matrix_equal (matrix, &old_authority->big_state->matrix))
        //             {
        //             layer->differences &= ~state;

        //             g_assert (layer->owner == pipeline);
        //             if (layer->differences == 0)
        //                 _cogl_pipeline_prune_empty_layer_difference (pipeline,
        //                                                             layer);
        //             return;
        //             }
        //         }
        //     }

        // layer->big_state->matrix = *matrix;

        // /* If we weren't previously the authority on this state then we need
        // * to extended our differences mask and so it's possible that some
        // * of our ancestry will now become redundant, so we aim to reparent
        // * ourselves if that's true... */
        // if (layer != authority)
        //     {
        //     layer->differences |= state;
        //     _cogl_pipeline_layer_prune_redundant_ancestry (layer);
        //     }
        unimplemented!()
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
        // CoglContext *ctx = _cogl_context_get_default ();

        // /* Disallow setting texture types that aren't supported */
        // switch (texture_type)
        //     {
        //     case COGL_TEXTURE_TYPE_2D:
        //     break;

        //     case COGL_TEXTURE_TYPE_3D:
        //     if (ctx->default_gl_texture_3d_tex == NULL)
        //         {
        //         g_warning ("The default 3D texture was set on a pipeline but "
        //                     "3D textures are not supported");
        //         texture_type = COGL_TEXTURE_TYPE_2D;
        //         return;
        //         }
        //     break;

        //     case COGL_TEXTURE_TYPE_RECTANGLE:
        //     if (ctx->default_gl_texture_rect_tex == NULL)
        //         {
        //         g_warning ("The default rectangle texture was set on a pipeline but "
        //                     "rectangle textures are not supported");
        //         texture_type = COGL_TEXTURE_TYPE_2D;
        //         }
        //     break;
        //     }

        // _cogl_pipeline_set_layer_texture_type (pipeline, layer_index, texture_type);
        // _cogl_pipeline_set_layer_texture_data (pipeline, layer_index, NULL);
        unimplemented!()
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
    ) -> bool {
        // CoglPipelineLayerState       change =
        //     COGL_PIPELINE_LAYER_STATE_POINT_SPRITE_COORDS;
        // CoglPipelineLayer           *layer;
        // CoglPipelineLayer           *new;
        // CoglPipelineLayer           *authority;

        // _COGL_GET_CONTEXT (ctx, FALSE);

        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // /* Don't allow point sprite coordinates to be enabled if the driver
        //     doesn't support it */
        // if (enable && !cogl_has_feature (ctx, COGL_FEATURE_ID_POINT_SPRITE))
        //     {
        //     if (error)
        //         {
        //         _cogl_set_error (error,
        //                         COGL_SYSTEM_ERROR,
        //                         COGL_SYSTEM_ERROR_UNSUPPORTED,
        //                         "Point sprite texture coordinates are enabled for "
        //                         "a layer but the GL driver does not support it.");
        //         }
        //     else
        //         {
        //         static CoglBool warning_seen = FALSE;
        //         if (!warning_seen)
        //             g_warning ("Point sprite texture coordinates are enabled "
        //                     "for a layer but the GL driver does not support it.");
        //         warning_seen = TRUE;
        //         }

        //     return FALSE;
        //     }

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, change);

        // if (authority->big_state->point_sprite_coords == enable)
        //     return TRUE;

        // new = _cogl_pipeline_layer_pre_change_notify (pipeline, layer, change);
        // if (new != layer)
        //     layer = new;
        // else
        //     {
        //     /* If the original layer we found is currently the authority on
        //     * the state we are changing see if we can revert to one of our
        //     * ancestors being the authority. */
        //     if (layer == authority &&
        //         _cogl_pipeline_layer_get_parent (authority) != NULL)
        //         {
        //         CoglPipelineLayer *parent =
        //             _cogl_pipeline_layer_get_parent (authority);
        //         CoglPipelineLayer *old_authority =
        //             _cogl_pipeline_layer_get_authority (parent, change);

        //         if (old_authority->big_state->point_sprite_coords == enable)
        //             {
        //             layer->differences &= ~change;

        //             g_assert (layer->owner == pipeline);
        //             if (layer->differences == 0)
        //                 _cogl_pipeline_prune_empty_layer_difference (pipeline,
        //                                                             layer);
        //             return TRUE;
        //             }
        //         }
        //     }

        // layer->big_state->point_sprite_coords = enable;

        // /* If we weren't previously the authority on this state then we need
        // * to extended our differences mask and so it's possible that some
        // * of our ancestry will now become redundant, so we aim to reparent
        // * ourselves if that's true... */
        // if (layer != authority)
        //     {
        //     layer->differences |= change;
        //     _cogl_pipeline_layer_prune_redundant_ancestry (layer);
        //     }

        // return TRUE;
        unimplemented!()
    }

    // pub fn set_layer_texture<P: Is<Texture>>(&self, layer_index: i32, texture: &P) {
    //     ffi::cogl_pipeline_set_layer_texture
    // }

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
        // CoglPipelineLayerState       change = COGL_PIPELINE_LAYER_STATE_SAMPLER;
        // CoglPipelineLayer           *layer;
        // CoglPipelineLayer           *authority;
        // CoglSamplerCacheWrapMode     internal_mode =
        //     public_to_internal_wrap_mode (mode);
        // const CoglSamplerCacheEntry *sampler_state;

        // _COGL_GET_CONTEXT (ctx, NO_RETVAL);

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, change);

        // sampler_state =
        //     _cogl_sampler_cache_update_wrap_modes (ctx->sampler_cache,
        //                                         authority->sampler_cache_entry,
        //                                         internal_mode,
        //                                         internal_mode,
        //                                         internal_mode);
        // _cogl_pipeline_set_layer_sampler_state (pipeline,
        //                                         layer,
        //                                         authority,
        //                                         sampler_state);
        // /* XXX: I wonder if we should really be duplicating the mode into
        // * the 'p' wrap mode too? */
        unimplemented!()
    }

    /// Sets the wrap mode for the 'p' coordinate of texture lookups on
    /// this layer. 'p' is the third coordinate.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `mode`
    /// the new wrap mode
    pub fn set_layer_wrap_mode_p(&self, layer_index: i32, mode: PipelineWrapMode) {
        // CoglPipelineLayerState       change = COGL_PIPELINE_LAYER_STATE_SAMPLER;
        // CoglPipelineLayer           *layer;
        // CoglPipelineLayer           *authority;
        // CoglSamplerCacheWrapMode     internal_mode =
        //     public_to_internal_wrap_mode (mode);
        // const CoglSamplerCacheEntry *sampler_state;

        // _COGL_GET_CONTEXT (ctx, NO_RETVAL);

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, change);

        // sampler_state =
        //     _cogl_sampler_cache_update_wrap_modes (ctx->sampler_cache,
        //                                         authority->sampler_cache_entry,
        //                                         authority->sampler_cache_entry->
        //                                         wrap_mode_s,
        //                                         authority->sampler_cache_entry->
        //                                         wrap_mode_t,
        //                                         internal_mode);
        // _cogl_pipeline_set_layer_sampler_state (pipeline,
        //                                         layer,
        //                                         authority,
        //                                         sampler_state);
        unimplemented!()
    }

    /// Sets the wrap mode for the 's' coordinate of texture lookups on this layer.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `mode`
    /// the new wrap mode
    pub fn set_layer_wrap_mode_s(&self, layer_index: i32, mode: PipelineWrapMode) {
        // CoglPipelineLayerState       change = COGL_PIPELINE_LAYER_STATE_SAMPLER;
        // CoglPipelineLayer           *layer;
        // CoglPipelineLayer           *authority;
        // CoglSamplerCacheWrapMode     internal_mode =
        //     public_to_internal_wrap_mode (mode);
        // const CoglSamplerCacheEntry *sampler_state;

        // _COGL_GET_CONTEXT (ctx, NO_RETVAL);

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, change);

        // sampler_state =
        //     _cogl_sampler_cache_update_wrap_modes (ctx->sampler_cache,
        //                                         authority->sampler_cache_entry,
        //                                         internal_mode,
        //                                         authority->sampler_cache_entry->
        //                                         wrap_mode_t,
        //                                         authority->sampler_cache_entry->
        //                                         wrap_mode_p);
        // _cogl_pipeline_set_layer_sampler_state (pipeline,
        //                                         layer,
        //                                         authority,
        //                                         sampler_state);
        unimplemented!()
    }

    /// Sets the wrap mode for the 't' coordinate of texture lookups on this layer.
    ///
    /// ## `layer_index`
    /// the layer number to change.
    /// ## `mode`
    /// the new wrap mode
    pub fn set_layer_wrap_mode_t(&self, layer_index: i32, mode: PipelineWrapMode) {
        // CoglPipelineLayerState       change = COGL_PIPELINE_LAYER_STATE_SAMPLER;
        // CoglPipelineLayer           *layer;
        // CoglPipelineLayer           *authority;
        // CoglSamplerCacheWrapMode     internal_mode =
        //     public_to_internal_wrap_mode (mode);
        // const CoglSamplerCacheEntry *sampler_state;

        // _COGL_GET_CONTEXT (ctx, NO_RETVAL);

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // /* Note: this will ensure that the layer exists, creating one if it
        // * doesn't already.
        // *
        // * Note: If the layer already existed it's possibly owned by another
        // * pipeline. If the layer is created then it will be owned by
        // * pipeline. */
        // layer = _cogl_pipeline_get_layer (pipeline, layer_index);

        // /* Now find the ancestor of the layer that is the authority for the
        // * state we want to change */
        // authority = _cogl_pipeline_layer_get_authority (layer, change);

        // sampler_state =
        //     _cogl_sampler_cache_update_wrap_modes (ctx->sampler_cache,
        //                                         authority->sampler_cache_entry,
        //                                         authority->sampler_cache_entry->
        //                                         wrap_mode_s,
        //                                         internal_mode,
        //                                         authority->sampler_cache_entry->
        //                                         wrap_mode_p);
        // _cogl_pipeline_set_layer_sampler_state (pipeline,
        //                                         layer,
        //                                         authority,
        //                                         sampler_state);
        unimplemented!()
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
    pub fn set_per_vertex_point_size(&self, enable: bool) -> bool {
        // CoglPipelineState state = COGL_PIPELINE_STATE_PER_VERTEX_POINT_SIZE;
        // CoglPipeline *authority;

        // _COGL_GET_CONTEXT (ctx, FALSE);
        // _COGL_RETURN_VAL_IF_FAIL (cogl_is_pipeline (pipeline), FALSE);

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // enable = !!enable;

        // if (authority->big_state->per_vertex_point_size == enable)
        //     return TRUE;

        // if (enable && !cogl_has_feature (ctx, COGL_FEATURE_ID_PER_VERTEX_POINT_SIZE))
        //     {
        //     _cogl_set_error (error,
        //                     COGL_SYSTEM_ERROR,
        //                     COGL_SYSTEM_ERROR_UNSUPPORTED,
        //                     "Per-vertex point size is not supported");

        //     return FALSE;
        //     }

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // pipeline->big_state->per_vertex_point_size = enable;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_point_size_equal);

        // return TRUE;
        unimplemented!()
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
        // CoglPipelineState state = COGL_PIPELINE_STATE_POINT_SIZE;
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // if (authority->big_state->point_size == point_size)
        //     return;

        // /* Changing the point size may additionally modify
        // * COGL_PIPELINE_STATE_NON_ZERO_POINT_SIZE. */
        // if ((authority->big_state->point_size > 0.0f) != (point_size > 0.0f))
        //     _cogl_pipeline_set_non_zero_point_size (pipeline, point_size > 0.0f);

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // pipeline->big_state->point_size = point_size;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_point_size_equal);
        unimplemented!()
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
        // CoglPipeline *authority;
        // CoglPipelineState state = COGL_PIPELINE_STATE_LIGHTING;
        // CoglPipelineLightingState *lighting_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // if (shininess < 0.0)
        //     {
        //     g_warning ("Out of range shininess %f supplied for pipeline\n",
        //                 shininess);
        //     return;
        //     }

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // lighting_state = &authority->big_state->lighting_state;

        // if (lighting_state->shininess == shininess)
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // lighting_state = &pipeline->big_state->lighting_state;
        // lighting_state->shininess = shininess;

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_lighting_state_equal);
        unimplemented!()
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
        // CoglPipeline *authority;
        // CoglPipelineState state = COGL_PIPELINE_STATE_LIGHTING;
        // CoglPipelineLightingState *lighting_state;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // lighting_state = &authority->big_state->lighting_state;
        // if (cogl_color_equal (specular, &lighting_state->specular))
        //     return;

        // /* - Flush journal primitives referencing the current state.
        // * - Make sure the pipeline has no dependants so it may be modified.
        // * - If the pipeline isn't currently an authority for the state being
        // *   changed, then initialize that state from the current authority.
        // */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // lighting_state = &pipeline->big_state->lighting_state;
        // lighting_state->specular[0] = cogl_color_get_red_float (specular);
        // lighting_state->specular[1] = cogl_color_get_green_float (specular);
        // lighting_state->specular[2] = cogl_color_get_blue_float (specular);
        // lighting_state->specular[3] = cogl_color_get_alpha_float (specular);

        // _cogl_pipeline_update_authority (pipeline, authority, state,
        //                                 _cogl_pipeline_lighting_state_equal);

        // pipeline->dirty_real_blend_enable = TRUE;
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
        // CoglBoxedValue *boxed_value;

        // boxed_value = _cogl_pipeline_override_uniform (pipeline, uniform_location);

        // _cogl_boxed_value_set_1f (boxed_value, value);
        unimplemented!()
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
        // CoglBoxedValue *boxed_value;

        // boxed_value = _cogl_pipeline_override_uniform (pipeline, uniform_location);

        // _cogl_boxed_value_set_1i (boxed_value, value);
        unimplemented!()
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
        // CoglBoxedValue *boxed_value;

        // boxed_value = _cogl_pipeline_override_uniform (pipeline, uniform_location);

        // _cogl_boxed_value_set_float (boxed_value, n_components, count, value);
        unimplemented!()
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
        // CoglBoxedValue *boxed_value;

        // boxed_value = _cogl_pipeline_override_uniform (pipeline, uniform_location);

        // _cogl_boxed_value_set_int (boxed_value, n_components, count, value);
        unimplemented!()
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
        // CoglBoxedValue *boxed_value;

        // boxed_value = _cogl_pipeline_override_uniform (pipeline, uniform_location);

        // _cogl_boxed_value_set_matrix (boxed_value,
        //                                 dimensions,
        //                                 count,
        //                                 transpose,
        //                                 value);
        unimplemented!()
    }

    pub fn set_user_program(&self, program: Handle) {
        // CoglPipelineState state = COGL_PIPELINE_STATE_USER_SHADER;
        // CoglPipeline *authority;

        // _COGL_RETURN_IF_FAIL (cogl_is_pipeline (pipeline));

        // authority = _cogl_pipeline_get_authority (pipeline, state);

        // if (authority->big_state->user_program == program)
        //   return;

        // /* - Flush journal primitives referencing the current state.
        //  * - Make sure the pipeline has no dependants so it may be modified.
        //  * - If the pipeline isn't currently an authority for the state being
        //  *   changed, then initialize that state from the current authority.
        //  */
        // _cogl_pipeline_pre_change_notify (pipeline, state, NULL, FALSE);

        // if (program != COGL_INVALID_HANDLE)
        //   _cogl_pipeline_set_progend (pipeline, COGL_PIPELINE_PROGEND_UNDEFINED);

        // /* If we are the current authority see if we can revert to one of our
        //  * ancestors being the authority */
        // if (pipeline == authority &&
        //     _cogl_pipeline_get_parent (authority) != NULL)
        //   {
        //     CoglPipeline *parent = _cogl_pipeline_get_parent (authority);
        //     CoglPipeline *old_authority =
        //       _cogl_pipeline_get_authority (parent, state);

        //     if (old_authority->big_state->user_program == program)
        //       pipeline->differences &= ~state;
        //   }
        // else if (pipeline != authority)
        //   {
        //     /* If we weren't previously the authority on this state then we
        //      * need to extended our differences mask and so it's possible
        //      * that some of our ancestry will now become redundant, so we
        //      * aim to reparent ourselves if that's true... */
        //     pipeline->differences |= state;
        //     _cogl_pipeline_prune_redundant_ancestry (pipeline);
        //   }

        // if (program != COGL_INVALID_HANDLE)
        //   cogl_handle_ref (program);
        // if (authority == pipeline &&
        //     pipeline->big_state->user_program != COGL_INVALID_HANDLE)
        //   cogl_handle_unref (pipeline->big_state->user_program);
        // pipeline->big_state->user_program = program;

        // pipeline->dirty_real_blend_enable = TRUE;
        unimplemented!()
    }
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pipeline")
    }
}
