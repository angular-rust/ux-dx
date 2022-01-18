use super::{AttributeBuffer, AttributeType, Context};
use std::{cell::RefCell, fmt};

pub enum AttributeNameID {
    PositionArray,
    ColorArray,
    TextureCoordArray,
    NormalArray,
    PointSizeArray,
    CustomArray,
}

pub struct AttributeNameState {
    name: Option<String>,
    name_id: AttributeNameID,
    name_index: i32,
    normalized_default: bool,
    layer_number: i32,
}

pub enum DrawFlags {
    SkipJournalFlush = 1 << 0,
    SkipPipelineValidation = 1 << 1,
    SkipFramebufferFlush = 1 << 2,
    SkipLegacyState = 1 << 3,
    // By default the vertex attribute drawing code will assume that if
    // there is a color attribute array enabled then we can't determine
    // if the colors will be opaque so we need to enabling
    // blending. However when drawing from the journal we know what the
    // contents of the color array is so we can override this by passing
    // this flag.
    ColorAttributeIsOpaque = 1 << 4,
    // This forcibly disables the debug option to divert all drawing to wireframes
    SkipDebugWireframe = 1 << 5,
}

struct AttributeProps {
    // const AttributeNameState *name_state;
    normalized: bool,
    is_buffered: bool,

    // union {
    //     struct {
    //         AttributeBuffer *attribute_buffer;
    //         size_t stride;
    //         size_t offset;
    //         int n_components;
    //         AttributeType type;
    //     } buffered;
    //     struct {
    //         Context *context;
    //         BoxedValue boxed;
    //     } constant;
    // } d;
    immutable_ref: i32,
}
pub struct Attribute {
    props: RefCell<AttributeProps>,
}

impl Attribute {
    // attribute_new: (constructor)
    // @attribute_buffer: The #AttributeBuffer containing the actual
    //                    attribute data
    // @name: The name of the attribute (used to reference it from GLSL)
    // @stride: The number of bytes to jump to get to the next attribute
    //          value for the next vertex. (Usually
    //          <literal>sizeof (MyVertex)</literal>)
    // @offset: The byte offset from the start of @attribute_buffer for
    //          the first attribute value. (Usually
    //          <literal>offsetof (MyVertex, component0)</literal>
    // @components: The number of components (e.g. 4 for an rgba color or
    //              3 for and (x,y,z) position)
    // @type: FIXME
    /// Describes the layout for a list of vertex attribute values (For
    /// example, a list of texture coordinates or colors).
    ///
    /// The `name` is used to access the attribute inside a GLSL vertex
    /// shader and there are some special names you should use if they are
    /// applicable:
    ///  `<itemizedlist>`
    ///  `<listitem>`"position_in" (used for vertex positions)`</listitem>`
    ///  `<listitem>`"color_in" (used for vertex colors)`</listitem>`
    ///  `<listitem>`"tex_coord0_in", "tex_coord1", ...
    /// (used for vertex texture coordinates)`</listitem>`
    ///  `<listitem>`"normal_in" (used for vertex normals)`</listitem>`
    ///  `<listitem>`"point_size_in" (used to set the size of points
    ///  per-vertex. Note this can only be used if
    ///  `FEATURE_ID_POINT_SIZE_ATTRIBUTE` is advertised and
    ///  `Pipeline::set_per_vertex_point_size` is called on the pipeline.
    ///  `</listitem>`
    ///  `</itemizedlist>`
    ///
    /// The attribute values corresponding to different vertices can either
    /// be tightly packed or interleaved with other attribute values. For
    /// example it's common to define a structure for a single vertex like:
    ///
    /// ```text
    /// typedef struct
    /// {
    ///   float x, y, z; /<!-- -->* position attribute *<!-- -->/
    ///   float s, t; /<!-- -->* texture coordinate attribute *<!-- -->/
    /// } MyVertex;
    /// ```
    ///
    /// And then create an array of vertex data something like:
    ///
    /// ```text
    /// MyVertex vertices[100] = { .... }
    /// ```
    ///
    /// In this case, to describe either the position or texture coordinate
    /// attribute you have to move `<literal>`sizeof (MyVertex)`</literal>` bytes to
    /// move from one vertex to the next. This is called the attribute
    /// `stride`. If you weren't interleving attributes and you instead had
    /// a packed array of float x, y pairs then the attribute stride would
    /// be `<literal>`(2 * sizeof (float))`</literal>`. So the `stride` is the number of
    /// bytes to move to find the attribute value of the next vertex.
    ///
    /// Normally a list of attributes starts at the beginning of an array.
    /// So for the `<literal>`MyVertex`</literal>` example above the `offset` is the
    /// offset inside the `<literal>`MyVertex`</literal>` structure to the first
    /// component of the attribute. For the texture coordinate attribute
    /// the offset would be `<literal>`offsetof (MyVertex, s)`</literal>` or instead of
    /// using the offsetof macro you could use `<literal>`sizeof (float) *
    /// 3`</literal>`. If you've divided your `array` into blocks of non-interleved
    /// attributes then you will need to calculate the `offset` as the number of
    /// bytes in blocks preceding the attribute you're describing.
    ///
    /// An attribute often has more than one component. For example a color
    /// is often comprised of 4 red, green, blue and alpha `components`, and a
    /// position may be comprised of 2 x and y `components`. You should aim
    /// to keep the number of components to a minimum as more components
    /// means more data needs to be mapped into the GPU which can be a
    /// bottlneck when dealing with a large number of vertices.
    ///
    /// Finally you need to specify the component data type. Here you
    /// should aim to use the smallest type that meets your precision
    /// requirements. Again the larger the type then more data needs to be
    /// mapped into the GPU which can be a bottlneck when dealing with
    /// a large number of vertices.
    /// ## `attribute_buffer`
    /// The `AttributeBuffer` containing the actual
    ///  attribute data
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `stride`
    /// The number of bytes to jump to get to the next attribute
    ///  value for the next vertex. (Usually
    ///  `<literal>`sizeof (MyVertex)`</literal>`)
    /// ## `offset`
    /// The byte offset from the start of `attribute_buffer` for
    ///  the first attribute value. (Usually
    ///  `<literal>`offsetof (MyVertex, component0)`</literal>`
    /// ## `components`
    /// The number of components (e.g. 4 for an rgba color or
    ///  3 for and (x,y,z) position)
    /// ## `type_`
    /// FIXME
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  describing the layout for a list of attribute values
    ///  stored in `array`.
    pub fn new(
        attribute_buffer: &AttributeBuffer,
        name: &str,
        stride: usize,
        offset: usize,
        components: i32,
        type_: AttributeType,
    ) -> Attribute {
        // Attribute *attribute = g_slice_new (Attribute);
        // Buffer *buffer = BUFFER (attribute_buffer);
        // Context *ctx = buffer->context;

        // attribute->is_buffered = true;

        // attribute->name_state =
        //     g_hash_table_lookup (ctx->attribute_name_states_hash, name);
        // if (!attribute->name_state)
        //     {
        //     AttributeNameState *name_state =
        //         _attribute_register_attribute_name (ctx, name);
        //     if (!name_state)
        //         goto error;
        //     attribute->name_state = name_state;
        //     }

        // attribute->d.buffered.attribute_buffer = object_ref (attribute_buffer);
        // attribute->d.buffered.stride = stride;
        // attribute->d.buffered.offset = offset;
        // attribute->d.buffered.n_components = n_components;
        // attribute->d.buffered.type = type;

        // attribute->immutable_ref = 0;

        // if (attribute->name_state->name_id != CUSTOM_ARRAY)
        //     {
        //     if (!validate_n_components (attribute->name_state, n_components))
        //         return NULL;
        //     attribute->normalized =
        //         attribute->name_state->normalized_default;
        //     }
        // else
        //     attribute->normalized = false;

        // return _attribute_object_new (attribute);

        // error:
        // _attribute_free (attribute);
        // return NULL;
        unimplemented!()
    }

    // attribute_new_const_1f:
    // @context: A #Context
    // @name: The name of the attribute (used to reference it from GLSL)
    // @value: The constant value for the attribute
    /// Creates a new, single component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constant `value` is a single precision floating point scalar
    /// which should have a corresponding declaration in GLSL code like:
    ///
    /// [|
    /// attribute float name;
    /// |]
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// The constant value for the attribute
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant `value`.
    pub fn new_const_1f(context: &Context, name: &str, value: f32) -> Attribute {
        // return _attribute_new_const (context,
        //     name,
        //     1, /* n_components */
        //     1, /* 1 column vector */
        //     false, /* no transpose */
        //     &value);
        unimplemented!()
    }

    /// Creates a new, 2 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (`component0`, `component1`) represent a 2 component
    /// float vector which should have a corresponding declaration in GLSL
    /// code like:
    ///
    /// [|
    /// attribute vec2 name;
    /// |]
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `component0`
    /// The first component of a 2 component vector
    /// ## `component1`
    /// The second component of a 2 component vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_2f(
        context: &Context,
        name: &str,
        component0: f32,
        component1: f32,
    ) -> Attribute {
        // float vec2[2] = { component0, component1 };
        // return _attribute_new_const (context,
        //                                   name,
        //                                   2, /* n_components */
        //                                   1, /* 1 column vector */
        //                                   false, /* no transpose */
        //                                   vec2);
        unimplemented!()
    }

    /// Creates a new, 2 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (value[0], value[1]) represent a 2 component float
    /// vector which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute vec2 name;
    /// |]
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// A pointer to a 2 component float vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_2fv(context: &Context, name: &str, value: &[f32; 2]) -> Attribute {
        // return _attribute_new_const (context,
        //     name,
        //     2, /* n_components */
        //     1, /* 1 column vector */
        //     false, /* no transpose */
        //     value);
        unimplemented!()
    }

    /// Creates a new matrix attribute whose value remains constant
    /// across all the vertices of a primitive without needing to duplicate
    /// the value for each vertex.
    ///
    /// `matrix2x2` represent a square 2 by 2 matrix specified in
    /// column-major order (each pair of consecutive numbers represents a
    /// column) which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute mat2 name;
    /// |]
    ///
    /// If `transpose` is `true` then all matrix components are rotated
    /// around the diagonal of the matrix such that the first column
    /// becomes the first row and the second column becomes the second row.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `matrix2x2`
    /// A pointer to a 2 by 2 matrix
    /// ## `transpose`
    /// Whether the matrix should be transposed on upload or
    ///  not
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant matrix.
    pub fn new_const_2x2fv(
        context: &Context,
        name: &str,
        matrix2x2: &[f32; 4],
        transpose: bool,
    ) -> Attribute {
        // return _attribute_new_const (context,
        //     name,
        //     2, /* n_components */
        //     2, /* 2 column vector */
        //     false, /* no transpose */
        //     matrix2x2);
        unimplemented!()
    }

    /// Creates a new, 3 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (`component0`, `component1`, `component2`) represent a 3
    /// component float vector which should have a corresponding
    /// declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec3 name;
    /// |]
    ///
    /// unless the built in name "normal_in" is being used where no
    /// explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `component0`
    /// The first component of a 3 component vector
    /// ## `component1`
    /// The second component of a 3 component vector
    /// ## `component2`
    /// The third component of a 3 component vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_3f(
        context: &Context,
        name: &str,
        component0: f32,
        component1: f32,
        component2: f32,
    ) -> Attribute {
        // float vec3[3] = { component0, component1, component2 };
        // return _attribute_new_const (context,
        //                                   name,
        //                                   3, /* n_components */
        //                                   1, /* 1 column vector */
        //                                   false, /* no transpose */
        //                                   vec3);
        unimplemented!()
    }

    /// Creates a new, 3 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (value[0], value[1], value[2]) represent a 3
    /// component float vector which should have a corresponding
    /// declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec3 name;
    /// |]
    ///
    /// unless the built in name "normal_in" is being used where no
    /// explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// A pointer to a 3 component float vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_3fv(context: &Context, name: &str, value: &[f32; 3]) -> Attribute {
        // return _attribute_new_const (context,
        //     name,
        //     3, /* n_components */
        //     1, /* 1 column vector */
        //     false, /* no transpose */
        //     value);
        unimplemented!()
    }

    /// Creates a new matrix attribute whose value remains constant
    /// across all the vertices of a primitive without needing to duplicate
    /// the value for each vertex.
    ///
    /// `matrix3x3` represent a square 3 by 3 matrix specified in
    /// column-major order (each triple of consecutive numbers represents a
    /// column) which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute mat3 name;
    /// |]
    ///
    /// If `transpose` is `true` then all matrix components are rotated
    /// around the diagonal of the matrix such that the first column
    /// becomes the first row and the second column becomes the second row
    /// etc.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `matrix3x3`
    /// A pointer to a 3 by 3 matrix
    /// ## `transpose`
    /// Whether the matrix should be transposed on upload or
    ///  not
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant matrix.
    pub fn new_const_3x3fv(
        context: &Context,
        name: &str,
        matrix3x3: &[f32; 9],
        transpose: bool,
    ) -> Attribute {
        // return _attribute_new_const (context,
        //     name,
        //     3, /* n_components */
        //     3, /* 3 column vector */
        //     false, /* no transpose */
        //     matrix3x3);
        unimplemented!()
    }

    /// Creates a new, 4 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (`component0`, `component1`, `component2`, `constant3`)
    /// represent a 4 component float vector which should have a
    /// corresponding declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec4 name;
    /// |]
    ///
    /// unless one of the built in names "color_in",
    /// "tex_coord0_in or "tex_coord1_in" etc is being used where
    /// no explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `component0`
    /// The first component of a 4 component vector
    /// ## `component1`
    /// The second component of a 4 component vector
    /// ## `component2`
    /// The third component of a 4 component vector
    /// ## `component3`
    /// The fourth component of a 4 component vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_4f(
        context: &Context,
        name: &str,
        component0: f32,
        component1: f32,
        component2: f32,
        component3: f32,
    ) -> Attribute {
        // float vec4[4] = { component0, component1, component2, component3 };
        // return _attribute_new_const (context,
        //                                   name,
        //                                   4, /* n_components */
        //                                   1, /* 1 column vector */
        //                                   false, /* no transpose */
        //                                   vec4);
        unimplemented!()
    }

    /// Creates a new, 4 component, attribute whose value remains
    /// constant across all the vertices of a primitive without needing to
    /// duplicate the value for each vertex.
    ///
    /// The constants (value[0], value[1], value[2], value[3]) represent a
    /// 4 component float vector which should have a corresponding
    /// declaration in GLSL code like:
    ///
    /// [|
    /// attribute vec4 name;
    /// |]
    ///
    /// unless one of the built in names "color_in",
    /// "tex_coord0_in or "tex_coord1_in" etc is being used where
    /// no explicit GLSL declaration need be made.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `value`
    /// A pointer to a 4 component float vector
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant vector.
    pub fn new_const_4fv(context: &Context, name: &str, value: &[f32; 4]) -> Attribute {
        // return _attribute_new_const (context,
        //     name,
        //     4, /* n_components */
        //     1, /* 1 column vector */
        //     false, /* no transpose */
        //     value);
        unimplemented!()
    }

    /// Creates a new matrix attribute whose value remains constant
    /// across all the vertices of a primitive without needing to duplicate
    /// the value for each vertex.
    ///
    /// `matrix4x4` represent a square 4 by 4 matrix specified in
    /// column-major order (each 4-tuple of consecutive numbers represents a
    /// column) which should have a corresponding declaration in GLSL code
    /// like:
    ///
    /// [|
    /// attribute mat4 name;
    /// |]
    ///
    /// If `transpose` is `true` then all matrix components are rotated
    /// around the diagonal of the matrix such that the first column
    /// becomes the first row and the second column becomes the second row
    /// etc.
    /// ## `context`
    /// A `Context`
    /// ## `name`
    /// The name of the attribute (used to reference it from GLSL)
    /// ## `matrix4x4`
    /// A pointer to a 4 by 4 matrix
    /// ## `transpose`
    /// Whether the matrix should be transposed on upload or
    ///  not
    ///
    /// # Returns
    ///
    /// A newly allocated `Attribute`
    ///  representing the given constant matrix.
    pub fn new_const_4x4fv(
        context: &Context,
        name: &str,
        matrix4x4: &[f32; 16],
        transpose: bool,
    ) -> Attribute {
        // return _attribute_new_const (context,
        //     name,
        //     4, /* n_components */
        //     4, /* 4 column vector */
        //     false, /* no transpose */
        //     matrix4x4);
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the `AttributeBuffer` that was
    ///  set with `Attribute::set_buffer` or `Attribute::new`.
    pub fn buffer(&self) -> Option<AttributeBuffer> {
        // _RETURN_VAL_IF_FAIL (is_attribute (attribute), NULL);
        // _RETURN_VAL_IF_FAIL (attribute->is_buffered, NULL);

        // return attribute->d.buffered.attribute_buffer;
        unimplemented!()
    }

    ///
    /// # Returns
    ///
    /// the value of the normalized property set with
    /// `Attribute::set_normalized`.
    pub fn normalized(&self) -> bool {
        let props = self.props.borrow();
        props.normalized
    }

    /// Sets a new `AttributeBuffer` for the attribute.
    /// ## `attribute_buffer`
    /// A `AttributeBuffer`
    pub fn set_buffer(&self, attribute_buffer: &AttributeBuffer) {
        // _RETURN_IF_FAIL (is_attribute (attribute));
        // _RETURN_IF_FAIL (attribute->is_buffered);

        // if (G_UNLIKELY (attribute->immutable_ref))
        //   warn_about_midscene_changes ();

        // object_ref (attribute_buffer);

        // object_unref (attribute->d.buffered.attribute_buffer);
        // attribute->d.buffered.attribute_buffer = attribute_buffer;
        unimplemented!()
    }

    /// Sets whether fixed point attribute types are mapped to the range
    /// 0→1. For example when this property is true and a
    /// `AttributeType::UnsignedByte` type is used then the value 255
    /// will be mapped to 1.0.
    ///
    /// The default value of this property depends on the name of the
    /// attribute. For the builtin properties color_in and
    /// normal_in it will default to true and for all other names it
    /// will default to false.
    /// ## `normalized`
    /// The new value for the normalized property.
    pub fn set_normalized(&self, normalized: bool) {
        // _RETURN_IF_FAIL (is_attribute (attribute));

        // if (G_UNLIKELY (attribute->immutable_ref))
        //   warn_about_midscene_changes ();

        // attribute->normalized = normalized;
        unimplemented!()
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attribute")
    }
}
