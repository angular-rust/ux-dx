use super::TextureVertex;

// * SECTION:cogl-primitives
// * @short_description: Functions that draw various primitive 3D shapes
// *
// * The primitives API provides utilities for drawing some
// * common 3D shapes in a more convenient way than the CoglVertexBuffer
// * API provides.

// typedef struct _TextureSlicedQuadState
// {
//   CoglFramebuffer *framebuffer;
//   CoglPipeline *pipeline;
//   CoglTexture *main_texture;
//   float tex_virtual_origin_x;
//   float tex_virtual_origin_y;
//   float quad_origin_x;
//   float quad_origin_y;
//   float v_to_q_scale_x;
//   float v_to_q_scale_y;
//   float quad_len_x;
//   float quad_len_y;
//   CoglBool flipped_x;
//   CoglBool flipped_y;
// } TextureSlicedQuadState;

// typedef struct _TextureSlicedPolygonState
// {
//   const CoglTextureVertex *vertices;
//   int n_vertices;
//   int stride;
//   CoglAttribute **attributes;
// } TextureSlicedPolygonState;

// typedef struct _ValidateFirstLayerState
// {
//   CoglPipeline *override_pipeline;
// } ValidateFirstLayerState;

// * cogl_rectangle:
// * @x_1: X coordinate of the top-left corner
// * @y_1: Y coordinate of the top-left corner
// * @x_2: X coordinate of the bottom-right corner
// * @y_2: Y coordinate of the bottom-right corner
// *
// * Fills a rectangle at the given coordinates with the current source material

pub fn rectangle(x_1: f32, y_1: f32, x_2: f32, y_2: f32) {
    // const float position[4] = {x_1, y_1, x_2, y_2};
    // CoglMultiTexturedRect rect;
  
    // /* XXX: All the cogl_rectangle* APIs normalize their input into an array of
    //  * CoglMultiTexturedRect rectangles and pass these on to our work horse;
    //  * _cogl_rectangles_with_multitexture_coords.
    //  */
  
    // rect.position = position;
    // rect.tex_coords = NULL;
    // rect.tex_coords_len = 0;
  
    // _cogl_rectangles_with_multitexture_coords (&rect, 1);
}

// * cogl_rectangle_with_texture_coords:
// * @x1: x coordinate upper left on screen.
// * @y1: y coordinate upper left on screen.
// * @x2: x coordinate lower right on screen.
// * @y2: y coordinate lower right on screen.
// * @tx1: x part of texture coordinate to use for upper left pixel
// * @ty1: y part of texture coordinate to use for upper left pixel
// * @tx2: x part of texture coordinate to use for lower right pixel
// * @ty2: y part of texture coordinate to use for left pixel
// *
// * Draw a rectangle using the current material and supply texture coordinates
// * to be used for the first texture layer of the material. To draw the entire
// * texture pass in @tx1=0.0 @ty1=0.0 @tx2=1.0 @ty2=1.0.
// *
// * Since: 1.0

pub fn rectangle_with_texture_coords(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    tx1: f32,
    ty1: f32,
    tx2: f32,
    ty2: f32,
) {
    // const float position[4] = {x_1, y_1, x_2, y_2};
    // const float tex_coords[4] = {tx_1, ty_1, tx_2, ty_2};
    // CoglMultiTexturedRect rect;

    // /* XXX: All the cogl_rectangle* APIs normalize their input into an array of
    // * CoglMultiTexturedRect rectangles and pass these on to our work horse;
    // * _cogl_rectangles_with_multitexture_coords.
    // */

    // rect.position = position;
    // rect.tex_coords = tex_coords;
    // rect.tex_coords_len = 4;

    // _cogl_rectangles_with_multitexture_coords (&rect, 1);
}

// * cogl_rectangle_with_multitexture_coords:
// * @x1: x coordinate upper left on screen.
// * @y1: y coordinate upper left on screen.
// * @x2: x coordinate lower right on screen.
// * @y2: y coordinate lower right on screen.
// * @tex_coords: (in) (array) (transfer none): An array containing groups of
// *   4 float values: [tx1, ty1, tx2, ty2] that are interpreted as two texture
// *   coordinates; one for the upper left texel, and one for the lower right
// *   texel. Each value should be between 0.0 and 1.0, where the coordinate
// *   (0.0, 0.0) represents the top left of the texture, and (1.0, 1.0) the
// *   bottom right.
// * @tex_coords_len: The length of the tex_coords array. (e.g. for one layer
// *   and one group of texture coordinates, this would be 4)
// *
// * This function draws a rectangle using the current source material to
// * texture or fill with. As a material may contain multiple texture layers
// * this interface lets you supply texture coordinates for each layer of the
// * material.
// *
// * The first pair of coordinates are for the first layer (with the smallest
// * layer index) and if you supply less texture coordinates than there are
// * layers in the current source material then default texture coordinates
// * (0.0, 0.0, 1.0, 1.0) are generated.
// *
// * Since: 1.0
pub fn rectangle_with_multitexture_coords(x1: f32, y1: f32, x2: f32, y2: f32, tex_coords: &[f32]) {
    // const float position[4] = {x_1, y_1, x_2, y_2};
    // CoglMultiTexturedRect rect;

    // /* XXX: All the cogl_rectangle* APIs normalize their input into an array of
    // * CoglMultiTexturedRect rectangles and pass these on to our work horse;
    // * _cogl_rectangles_with_multitexture_coords.
    // */

    // rect.position = position;
    // rect.tex_coords = user_tex_coords;
    // rect.tex_coords_len = user_tex_coords_len;

    // _cogl_rectangles_with_multitexture_coords (&rect, 1);
}

// * cogl_rectangles_with_texture_coords:
// * @verts: (in) (array) (transfer none): an array of vertices
// * @n_rects: number of rectangles to draw
// *
// * Draws a series of rectangles in the same way that
// * cogl_rectangle_with_texture_coords() does. In some situations it can give a
// * significant performance boost to use this function rather than
// * calling cogl_rectangle_with_texture_coords() separately for each rectangle.
// *
// * @verts should point to an array of #float<!-- -->s with
// * @n_rects * 8 elements. Each group of 8 values corresponds to the
// * parameters x1, y1, x2, y2, tx1, ty1, tx2 and ty2 and have the same
// * meaning as in cogl_rectangle_with_texture_coords().
// *
// * Since: 0.8.6
pub fn rectangles_with_texture_coords(verts: &[f32]) {
    // CoglMultiTexturedRect *rects;
    // int i;

    // /* XXX: All the cogl_rectangle* APIs normalize their input into an array of
    //  * CoglMultiTexturedRect rectangles and pass these on to our work horse;
    //  * _cogl_rectangles_with_multitexture_coords.
    //  */
    // rects = g_alloca (n_rects * sizeof (CoglMultiTexturedRect));

    // for (i = 0; i < n_rects; i++)
    //   {
    //     rects[i].position = &verts[i * 8];
    //     rects[i].tex_coords = &verts[i * 8 + 4];
    //     rects[i].tex_coords_len = 4;
    //   }

    // _cogl_rectangles_with_multitexture_coords (rects, n_rects);
    unimplemented!()
}

// * cogl_rectangles:
// * @verts: (in) (array) (transfer none): an array of vertices
// * @n_rects: number of rectangles to draw
// *
// * Draws a series of rectangles in the same way that
// * cogl_rectangle() does. In some situations it can give a
// * significant performance boost to use this function rather than
// * calling cogl_rectangle() separately for each rectangle.
// *
// * @verts should point to an array of #float<!-- -->s with
// * @n_rects * 4 elements. Each group of 4 values corresponds to the
// * parameters x1, y1, x2, and y2, and have the same
// * meaning as in cogl_rectangle().
// *
// * Since: 1.0

pub fn rectangles(verts: &[f32]) {
    // CoglMultiTexturedRect *rects;
    // int i;

    // /* XXX: All the cogl_rectangle* APIs normalize their input into an array of
    //  * CoglMultiTexturedRect rectangles and pass these on to our work horse;
    //  * _cogl_rectangles_with_multitexture_coords.
    //  */
    // rects = g_alloca (n_rects * sizeof (CoglMultiTexturedRect));

    // for (i = 0; i < n_rects; i++)
    //   {
    //     rects[i].position = &verts[i * 4];
    //     rects[i].tex_coords = NULL;
    //     rects[i].tex_coords_len = 0;
    //   }

    // _cogl_rectangles_with_multitexture_coords (rects, n_rects);
    unimplemented!()
}

// * cogl_polygon:
// * @vertices: An array of #CoglTextureVertex structs
// * @n_vertices: The length of the vertices array
// * @use_color: %TRUE if the color member of #CoglTextureVertex should be used
// *
// * Draws a convex polygon using the current source material to fill / texture
// * with according to the texture coordinates passed.
// *
// * If @use_color is %TRUE then the color will be changed for each vertex using
// * the value specified in the color member of #CoglTextureVertex. This can be
// * used for example to make the texture fade out by setting the alpha value of
// * the color.
// *
// * All of the texture coordinates must be in the range [0,1] and repeating the
// * texture is not supported.
// *
// * Because of the way this function is implemented it will currently
// * only work if either the texture is not sliced or the backend is not
// * OpenGL ES and the minifying and magnifying functions are both set
// * to COGL_MATERIAL_FILTER_NEAREST.
// *
// * Since: 1.0
pub fn polygon(vertices: &[TextureVertex], use_color: bool) {
    // CoglPipeline *pipeline;
    // ValidateState validate_state;
    // int n_layers;
    // int n_attributes;
    // CoglAttribute **attributes;
    // int i;
    // unsigned int stride;
    // size_t stride_bytes;
    // CoglAttributeBuffer *attribute_buffer;
    // float *v;

    // _COGL_GET_CONTEXT (ctx, NO_RETVAL);

    // pipeline = cogl_get_source ();

    // validate_state.original_pipeline = pipeline;
    // validate_state.pipeline = pipeline;
    // cogl_pipeline_foreach_layer (pipeline,
    //                             _cogl_polygon_validate_layer_cb,
    //                             &validate_state);
    // pipeline = validate_state.pipeline;

    // n_layers = cogl_pipeline_get_n_layers (pipeline);

    // n_attributes = 1 + n_layers + (use_color ? 1 : 0);
    // attributes = g_alloca (sizeof (CoglAttribute *) * n_attributes);

    // /* Our data is arranged like:
    // * [X, Y, Z, TX0, TY0, TX1, TY1..., R, G, B, A,...] */
    // stride = 3 + (2 * n_layers) + (use_color ? 1 : 0);
    // stride_bytes = stride * sizeof (float);

    // /* Make sure there is enough space in the global vertex array. This
    // * is used so we can render the polygon with a single call to OpenGL
    // * but still support any number of vertices */
    // g_array_set_size (ctx->polygon_vertices, n_vertices * stride);

    // attribute_buffer =
    //     cogl_attribute_buffer_new (ctx, n_vertices * stride_bytes, NULL);

    // attributes[0] = cogl_attribute_new (attribute_buffer,
    //                                     "cogl_position_in",
    //                                     stride_bytes,
    //                                     0,
    //                                     3,
    //                                     COGL_ATTRIBUTE_TYPE_FLOAT);

    // for (i = 0; i < n_layers; i++)
    //     {
    //     static const char *names[] = {
    //         "cogl_tex_coord0_in",
    //         "cogl_tex_coord1_in",
    //         "cogl_tex_coord2_in",
    //         "cogl_tex_coord3_in",
    //         "cogl_tex_coord4_in",
    //         "cogl_tex_coord5_in",
    //         "cogl_tex_coord6_in",
    //         "cogl_tex_coord7_in"
    //     };
    //     char *allocated_name = NULL;
    //     const char *name;

    //     if (i < 8)
    //         name = names[i];
    //     else
    //         name = allocated_name = g_strdup_printf ("cogl_tex_coord%d_in", i);

    //     attributes[i + 1] = cogl_attribute_new (attribute_buffer,
    //                                             name,
    //                                             stride_bytes,
    //                                             /* NB: [X,Y,Z,TX,TY...,R,G,B,A,...] */
    //                                             12 + 8 * i,
    //                                             2,
    //                                             COGL_ATTRIBUTE_TYPE_FLOAT);

    //     g_free (allocated_name);
    //     }

    // if (use_color)
    //     {
    //     attributes[n_attributes - 1] =
    //         cogl_attribute_new (attribute_buffer,
    //                             "cogl_color_in",
    //                             stride_bytes,
    //                             /* NB: [X,Y,Z,TX,TY...,R,G,B,A,...] */
    //                             12 + 8 * n_layers,
    //                             4,
    //                             COGL_ATTRIBUTE_TYPE_UNSIGNED_BYTE);
    //     }

    // /* Convert the vertices into an array of float vertex attributes */
    // v = (float *)ctx->polygon_vertices->data;
    // for (i = 0; i < n_vertices; i++)
    //     {
    //     AppendTexCoordsState append_tex_coords_state;
    //     uint8_t *c;

    //     /* NB: [X,Y,Z,TX,TY...,R,G,B,A,...] */
    //     v[0] = vertices[i].x;
    //     v[1] = vertices[i].y;
    //     v[2] = vertices[i].z;

    //     append_tex_coords_state.vertices_in = vertices;
    //     append_tex_coords_state.vertex = i;
    //     append_tex_coords_state.layer = 0;
    //     append_tex_coords_state.vertices_out = v;
    //     cogl_pipeline_foreach_layer (pipeline,
    //                                 append_tex_coord_attributes_cb,
    //                                 &append_tex_coords_state);

    //     if (use_color)
    //         {
    //         /* NB: [X,Y,Z,TX,TY...,R,G,B,A,...] */
    //         c = (uint8_t *) (v + 3 + 2 * n_layers);
    //         c[0] = cogl_color_get_red_byte (&vertices[i].color);
    //         c[1] = cogl_color_get_green_byte (&vertices[i].color);
    //         c[2] = cogl_color_get_blue_byte (&vertices[i].color);
    //         c[3] = cogl_color_get_alpha_byte (&vertices[i].color);
    //         }

    //     v += stride;
    //     }

    // v = (float *)ctx->polygon_vertices->data;
    // cogl_buffer_set_data (COGL_BUFFER (attribute_buffer),
    //                         0,
    //                         v,
    //                         ctx->polygon_vertices->len * sizeof (float));

    // /* XXX: although this may seem redundant, we need to do this since
    // * cogl_polygon() can be used with legacy state and its the source stack
    // * which track whether legacy state is enabled.
    // *
    // * (We only have a CoglDrawFlag to disable legacy state not one
    // *  to enable it) */
    // cogl_push_source (pipeline);

    // _cogl_framebuffer_draw_attributes (cogl_get_draw_framebuffer (),
    //                                     pipeline,
    //                                     COGL_VERTICES_MODE_TRIANGLE_FAN,
    //                                     0, n_vertices,
    //                                     attributes,
    //                                     n_attributes,
    //                                     0 /* no draw flags */);

    // cogl_pop_source ();

    // if (pipeline != validate_state.original_pipeline)
    //     cogl_object_unref (pipeline);

    // cogl_object_unref (attribute_buffer);

    // for (i = 0; i < n_attributes; i++)
    //     cogl_object_unref (attributes[i]);
}
