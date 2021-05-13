use super::{Context, Euler, Matrix, MatrixEntry, Quaternion};
use std::fmt;

// * SECTION:cogl-matrix-stack
// * @short_description: Functions for efficiently tracking many
// *                     related transformations
// *
// * Matrices can be used (for example) to describe the model-view
// * transforms of objects, texture transforms, and projective
// * transforms.
// *
// * The #Matrix api provides a good way to manipulate individual
// * matrices representing a single transformation but if you need to
// * track many-many such transformations for many objects that are
// * organized in a scenegraph for example then using a separate
// * #Matrix for each object may not be the most efficient way.
// *
// * A #MatrixStack enables applications to track lots of
// * transformations that are related to each other in some kind of
// * hierarchy.  In a scenegraph for example if you want to know how to
// * transform a particular node then you usually have to walk up
// * through the ancestors and accumulate their transforms before
// * finally applying the transform of the node itself. In this model
// * things are grouped together spatially according to their ancestry
// * and all siblings with the same parent share the same initial
// * transformation. The #MatrixStack API is suited to tracking lots
// * of transformations that fit this kind of model.
// *
// * Compared to using the #Matrix api directly to track many
// * related transforms, these can be some advantages to using a
// * #MatrixStack:
// * <itemizedlist>
// *   <listitem>Faster equality comparisons of transformations</listitem>
// *   <listitem>Efficient comparisons of the differences between arbitrary
// *   transformations</listitem>
// *   <listitem>Avoid redundant arithmetic related to common transforms
// *   </listitem>
// *   <listitem>Can be more space efficient (not always though)</listitem>
// * </itemizedlist>
// *
// * For reference (to give an idea of when a #MatrixStack can
// * provide a space saving) a #Matrix can be expected to take 72
// * bytes whereas a single #MatrixEntry in a #MatrixStack is
// * currently around 32 bytes on a 32bit CPU or 36 bytes on a 64bit
// * CPU. An entry is needed for each individual operation applied to
// * the stack (such as rotate, scale, translate) so if most of your
// * leaf node transformations only need one or two simple operations
// * relative to their parent then a matrix stack will likely take less
// * space than having a #Matrix for each node.
// *
// * Even without any space saving though the ability to perform fast
// * comparisons and avoid redundant arithmetic (especially sine and
// * cosine calculations for rotations) can make using a matrix stack
// * worthwhile.

// * MatrixStack:
// *
// * Tracks your current position within a hierarchy and lets you build
// * up a graph of transformations as you traverse through a hierarchy
// * such as a scenegraph.
// *
// * A #MatrixStack always maintains a reference to a single
// * transformation at any point in time, representing the
// * transformation at the current position in the hierarchy. You can
// * get a reference to the current transformation by calling
// * matrix_stack_get_entry().
// *
// * When a #MatrixStack is first created with
// * matrix_stack_new() then it is conceptually positioned at the
// * root of your hierarchy and the current transformation simply
// * represents an identity transformation.
// *
// * As you traverse your object hierarchy (your scenegraph) then you
// * should call matrix_stack_push() whenever you move down one
// * level and call matrix_stack_pop() whenever you move back up
// * one level towards the root.
// *
// * At any time you can apply a set of operations, such as "rotate",
// * "scale", "translate" on top of the current transformation of a
// * #MatrixStack using functions such as
// * matrix_stack_rotate(), matrix_stack_scale() and
// * matrix_stack_translate(). These operations will derive a new
// * current transformation and will never affect a transformation
// * that you have referenced using matrix_stack_get_entry().
// *
// * Internally applying operations to a #MatrixStack builds up a
// * graph of #MatrixEntry structures which each represent a single
// * immutable transform.
pub struct MatrixStack {
    // Object _parent;

// Context *context;

// MatrixEntry *last_entry;
}

impl MatrixStack {
    /// Allocates a new `MatrixStack` that can be used to build up
    /// transformations relating to objects in a scenegraph like hierarchy.
    /// (See the description of `MatrixStack` and `MatrixEntry` for
    /// more details of what a matrix stack is best suited for)
    ///
    /// When a `MatrixStack` is first allocated it is conceptually
    /// positioned at the root of your scenegraph hierarchy. As you
    /// traverse your scenegraph then you should call
    /// `MatrixStack::push` whenever you move down a level and
    /// `MatrixStack::pop` whenever you move back up a level towards
    /// the root.
    ///
    /// Once you have allocated a `MatrixStack` you can get a reference
    /// to the current transformation for the current position in the
    /// hierarchy by calling `MatrixStack::get_entry`.
    ///
    /// Once you have allocated a `MatrixStack` you can apply operations
    /// such as rotate, scale and translate to modify the current transform
    /// for the current position in the hierarchy by calling
    /// `MatrixStack::rotate`, `MatrixStack::scale` and
    /// `MatrixStack::translate`.
    /// ## `ctx`
    /// A `Context`
    ///
    /// # Returns
    ///
    /// A newly allocated `MatrixStack`
    pub fn new(ctx: &Context) -> MatrixStack {
        // MatrixStack *stack = g_slice_new (MatrixStack);

        // if (G_UNLIKELY (matrix_stack_magazine == NULL))
        //     {
        //     matrix_stack_magazine =
        //         _magazine_new (sizeof (MatrixEntryFull), 20);
        //     matrix_stack_matrices_magazine =
        //         _magazine_new (sizeof (Matrix), 20);
        //     }

        // stack->context = ctx;
        // stack->last_entry = NULL;

        // matrix_entry_ref (&ctx->identity_entry);
        // _matrix_stack_push_entry (stack, &ctx->identity_entry);

        // return _matrix_stack_object_new (stack);
        unimplemented!()
    }

    /// Replaces the current matrix with a perspective matrix for a given
    /// viewing frustum defined by 4 side clip planes that all cross
    /// through the origin and 2 near and far clip planes.
    /// ## `left`
    /// X position of the left clipping plane where it
    ///  intersects the near clipping plane
    /// ## `right`
    /// X position of the right clipping plane where it
    ///  intersects the near clipping plane
    /// ## `bottom`
    /// Y position of the bottom clipping plane where it
    ///  intersects the near clipping plane
    /// ## `top`
    /// Y position of the top clipping plane where it intersects
    ///  the near clipping plane
    /// ## `z_near`
    /// The distance to the near clipping plane (Must be positive)
    /// ## `z_far`
    /// The distance to the far clipping plane (Must be positive)
    pub fn frustum(&self, left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) {
        // MatrixEntryLoad *entry;

        // entry =
        //     _matrix_stack_push_replacement_entry (stack,
        //                                             COGL_MATRIX_OP_LOAD);

        // entry->matrix =
        //     _magazine_chunk_alloc (matrix_stack_matrices_magazine);

        // matrix_init_identity (entry->matrix);
        // matrix_frustum (entry->matrix,
        //                     left, right, bottom, top,
        //                     z_near, z_far);
    }

    /// Resolves the current `self` transform into a `Matrix` by
    /// combining the operations that have been applied to build up the
    /// current transform.
    ///
    /// There are two possible ways that this function may return its
    /// result depending on whether the stack is able to directly point
    /// to an internal `Matrix` or whether the result needs to be
    /// composed of multiple operations.
    ///
    /// If an internal matrix contains the required result then this
    /// function will directly return a pointer to that matrix, otherwise
    /// if the function returns `None` then `matrix` will be initialized
    /// to match the current transform of `self`.
    ///
    /// `<note>``matrix` will be left untouched if a direct pointer is
    /// returned.`</note>`
    /// ## `matrix`
    /// The potential destination for the current matrix
    ///
    /// # Returns
    ///
    /// A direct pointer to the current transform or `None`
    ///  and in that case `matrix` will be initialized with
    ///  the value of the current transform.
    pub fn get(&self) -> (Matrix, Matrix) {
        // return matrix_entry_get (stack->last_entry, matrix);
        unimplemented!()
    }

    /// Gets a reference to the current transform represented by a
    /// `MatrixEntry` pointer.
    ///
    /// `<note>`The transform represented by a `MatrixEntry` is
    /// immutable.`</note>`
    ///
    /// `<note>``MatrixEntry`<!-- -->s are reference counted using
    /// `MatrixEntry::ref` and `MatrixEntry::unref` and you
    /// should call `MatrixEntry::unref` when you are finished with
    /// and entry you get via `MatrixStack::get_entry`.`</note>`
    ///
    /// # Returns
    ///
    /// A pointer to the `MatrixEntry`
    ///  representing the current matrix stack transform.
    pub fn get_entry(&self) -> Option<MatrixEntry> {
        // return stack->last_entry;
        unimplemented!()
    }

    /// Gets the inverse transform of the current matrix and uses it to
    /// initialize a new `Matrix`.
    /// ## `inverse`
    /// The destination for a 4x4 inverse transformation matrix
    ///
    /// # Returns
    ///
    /// `true` if the inverse was successfully calculated or `false`
    ///  for degenerate transformations that can't be inverted (in this case the
    ///  `inverse` matrix will simply be initialized with the identity matrix)
    pub fn get_inverse(&self) -> (bool, Matrix) {
        // Matrix matrix;
        // Matrix *internal = matrix_stack_get (stack, &matrix);

        // if (internal)
        //     return matrix_get_inverse (internal, inverse);
        // else
        //     return matrix_get_inverse (&matrix, inverse);
        unimplemented!()
    }

    /// Resets the current matrix to the identity matrix.
    pub fn load_identity(&self) {
        // _matrix_stack_push_replacement_entry (stack,
        //     COGL_MATRIX_OP_LOAD_IDENTITY);
        unimplemented!()
    }

    /// Multiplies the current matrix by the given matrix.
    /// ## `matrix`
    /// the matrix to multiply with the current model-view
    pub fn multiply(&self, matrix: &Matrix) {
        // MatrixEntryMultiply *entry;

        // entry = _matrix_stack_push_operation (stack, COGL_MATRIX_OP_MULTIPLY);

        // entry->matrix =
        //     _magazine_chunk_alloc (matrix_stack_matrices_magazine);

        // matrix_init_from_array (entry->matrix, (float *)matrix);
        unimplemented!()
    }

    /// Replaces the current matrix with an orthographic projection matrix.
    /// ## `x_1`
    /// The x coordinate for the first vertical clipping plane
    /// ## `y_1`
    /// The y coordinate for the first horizontal clipping plane
    /// ## `x_2`
    /// The x coordinate for the second vertical clipping plane
    /// ## `y_2`
    /// The y coordinate for the second horizontal clipping plane
    /// ## `near`
    /// The `<emphasis>`distance`</emphasis>` to the near clipping
    ///  plane (will be `<emphasis>`negative`</emphasis>` if the plane is
    ///  behind the viewer)
    /// ## `far`
    /// The `<emphasis>`distance`</emphasis>` to the far clipping
    ///  plane (will be `<emphasis>`negative`</emphasis>` if the plane is
    ///  behind the viewer)
    pub fn orthographic(&self, x_1: f32, y_1: f32, x_2: f32, y_2: f32, near: f32, far: f32) {
        // MatrixEntryLoad *entry;

        // entry =
        //     _matrix_stack_push_replacement_entry (stack,
        //                                             COGL_MATRIX_OP_LOAD);

        // entry->matrix =
        //     _magazine_chunk_alloc (matrix_stack_matrices_magazine);

        // matrix_init_identity (entry->matrix);
        // matrix_orthographic (entry->matrix,
        //                             x_1, y_1, x_2, y_2, near, far);
        unimplemented!()
    }

    /// Replaces the current matrix with a perspective matrix based on the
    /// provided values.
    ///
    /// `<note>`You should be careful not to have too great a `z_far` / `z_near`
    /// ratio since that will reduce the effectiveness of depth testing
    /// since there wont be enough precision to identify the depth of
    /// objects near to each other.`</note>`
    /// ## `fov_y`
    /// Vertical field of view angle in degrees.
    /// ## `aspect`
    /// The (width over height) aspect ratio for display
    /// ## `z_near`
    /// The distance to the near clipping plane (Must be positive,
    ///  and must not be 0)
    /// ## `z_far`
    /// The distance to the far clipping plane (Must be positive)
    pub fn perspective(&self, fov_y: f32, aspect: f32, z_near: f32, z_far: f32) {
        // MatrixEntryLoad *entry;

        // entry =
        //     _matrix_stack_push_replacement_entry (stack,
        //                                             COGL_MATRIX_OP_LOAD);

        // entry->matrix =
        //     _magazine_chunk_alloc (matrix_stack_matrices_magazine);

        // matrix_init_identity (entry->matrix);
        // matrix_perspective (entry->matrix,
        //                         fov_y, aspect, z_near, z_far);
        unimplemented!()
    }

    /// Restores the previous transform that was last saved by calling
    /// `MatrixStack::push`.
    ///
    /// This is usually called while traversing a scenegraph whenever you
    /// return up one level in the graph towards the root node.
    pub fn pop(&self) {
        // MatrixEntry *old_top;
        // MatrixEntry *new_top;

        // _COGL_RETURN_IF_FAIL (stack != NULL);

        // old_top = stack->last_entry;
        // _COGL_RETURN_IF_FAIL (old_top != NULL);

        // /* To pop we are moving the top of the stack to the old top's parent
        // * node. The stack always needs to have a reference to the top entry
        // * so we must take a reference to the new top. The stack would have
        // * previously had a reference to the old top so we need to decrease
        // * the ref count on that. We need to ref the new head first in case
        // * this stack was the only thing referencing the old top. In that
        // * case the call to matrix_entry_unref will unref the parent.
        // */

        // /* Find the last save operation and remove it */

        // /* XXX: it would be an error to pop to the very beginning of the
        // * stack so we don't need to check for NULL pointer dereferencing. */
        // for (new_top = old_top;
        //     new_top->op != COGL_MATRIX_OP_SAVE;
        //     new_top = new_top->parent)
        //     ;

        // new_top = new_top->parent;
        // matrix_entry_ref (new_top);

        // matrix_entry_unref (old_top);

        // stack->last_entry = new_top;
    }

    /// Saves the current transform and starts a new transform that derives
    /// from the current transform.
    ///
    /// This is usually called while traversing a scenegraph whenever you
    /// traverse one level deeper. `MatrixStack::pop` can then be
    /// called when going back up one layer to restore the previous
    /// transform of an ancestor.
    pub fn push(&self) {
        // MatrixEntrySave *entry;

        // entry = _matrix_stack_push_operation (stack, COGL_MATRIX_OP_SAVE);

        // entry->cache_valid = FALSE;
    }

    /// Multiplies the current matrix by one that rotates the around the
    /// axis-vector specified by `x`, `y` and `z`. The rotation follows the
    /// right-hand thumb rule so for example rotating by 10 degrees about
    /// the axis-vector (0, 0, 1) causes a small counter-clockwise
    /// rotation.
    /// ## `angle`
    /// Angle in degrees to rotate.
    /// ## `x`
    /// X-component of vertex to rotate around.
    /// ## `y`
    /// Y-component of vertex to rotate around.
    /// ## `z`
    /// Z-component of vertex to rotate around.
    pub fn rotate(&self, angle: f32, x: f32, y: f32, z: f32) {
        // MatrixEntryRotate *entry;

        // entry = _matrix_stack_push_operation (stack, COGL_MATRIX_OP_ROTATE);

        // entry->angle = angle;
        // entry->x = x;
        // entry->y = y;
        // entry->z = z;
        unimplemented!()
    }

    /// Multiplies the current matrix by one that rotates according to the
    /// rotation described by `euler`.
    ///
    /// ## `euler`
    /// A `Euler`
    pub fn rotate_euler(&self, euler: &Euler) {
        // MatrixEntryRotateEuler *entry;

        // entry = _matrix_stack_push_operation (stack,
        //                                            COGL_MATRIX_OP_ROTATE_EULER);

        // entry->heading = euler->heading;
        // entry->pitch = euler->pitch;
        // entry->roll = euler->roll;
        unimplemented!()
    }

    /// Multiplies the current matrix by one that rotates according to the
    /// rotation described by `quaternion`.
    /// ## `quaternion`
    /// A `Quaternion`
    pub fn rotate_quaternion(&self, quaternion: &Quaternion) {
        // MatrixEntryRotateQuaternion *entry;

        // entry = _matrix_stack_push_operation (stack,
        //                                            COGL_MATRIX_OP_ROTATE_QUATERNION);

        // entry->values[0] = quaternion->w;
        // entry->values[1] = quaternion->x;
        // entry->values[2] = quaternion->y;
        // entry->values[3] = quaternion->z;
        unimplemented!()
    }

    /// Multiplies the current matrix by one that scales the x, y and z
    /// axes by the given values.
    /// ## `x`
    /// Amount to scale along the x-axis
    /// ## `y`
    /// Amount to scale along the y-axis
    /// ## `z`
    /// Amount to scale along the z-axis
    pub fn scale(&self, x: f32, y: f32, z: f32) {
        // MatrixEntryScale *entry;

        // entry = _matrix_stack_push_operation (stack, COGL_MATRIX_OP_SCALE);

        // entry->x = x;
        // entry->y = y;
        // entry->z = z;
        unimplemented!()
    }

    /// Replaces the current `self` matrix value with the value of `matrix`.
    /// This effectively discards any other operations that were applied
    /// since the last time `MatrixStack::push` was called or since
    /// the stack was initialized.
    /// ## `matrix`
    /// A `Matrix` replace the current matrix value with
    pub fn set(&self, matrix: &Matrix) {
        // MatrixEntryLoad *entry;

        // entry =
        //   _matrix_stack_push_replacement_entry (stack,
        //                                              COGL_MATRIX_OP_LOAD);

        // entry->matrix =
        //   _magazine_chunk_alloc (matrix_stack_matrices_magazine);

        // matrix_init_from_array (entry->matrix, (float *)matrix);
        unimplemented!()
    }

    /// Multiplies the current matrix by one that translates along all
    /// three axes according to the given values.
    /// ## `x`
    /// Distance to translate along the x-axis
    /// ## `y`
    /// Distance to translate along the y-axis
    /// ## `z`
    /// Distance to translate along the z-axis
    pub fn translate(&self, x: f32, y: f32, z: f32) {
        // MatrixEntryTranslate *entry;

        // entry = _matrix_stack_push_operation (stack, COGL_MATRIX_OP_TRANSLATE);

        // entry->x = x;
        // entry->y = y;
        // entry->z = z;
        unimplemented!()
    }
}

impl fmt::Display for MatrixStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MatrixStack")
    }
}
