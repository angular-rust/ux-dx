use crate::Matrix;
use std::mem;

// typedef enum _CoglMatrixOp
// {
//   COGL_MATRIX_OP_LOAD_IDENTITY,
//   COGL_MATRIX_OP_TRANSLATE,
//   COGL_MATRIX_OP_ROTATE,
//   COGL_MATRIX_OP_ROTATE_QUATERNION,
//   COGL_MATRIX_OP_ROTATE_EULER,
//   COGL_MATRIX_OP_SCALE,
//   COGL_MATRIX_OP_MULTIPLY,
//   COGL_MATRIX_OP_LOAD,
//   COGL_MATRIX_OP_SAVE,
// } CoglMatrixOp;

// typedef struct _CoglMatrixEntryTranslate
// {
//   CoglMatrixEntry _parent_data;

//   float x;
//   float y;
//   float z;

// } CoglMatrixEntryTranslate;

// typedef struct _CoglMatrixEntryRotate
// {
//   CoglMatrixEntry _parent_data;

//   float angle;
//   float x;
//   float y;
//   float z;

// } CoglMatrixEntryRotate;

// typedef struct _CoglMatrixEntryRotateEuler
// {
//   CoglMatrixEntry _parent_data;

//   /* This doesn't store an actual CoglEuler in order to avoid the
//    * padding */
//   float heading;
//   float pitch;
//   float roll;
// } CoglMatrixEntryRotateEuler;

// typedef struct _CoglMatrixEntryRotateQuaternion
// {
//   CoglMatrixEntry _parent_data;

//   /* This doesn't store an actual CoglQuaternion in order to avoid the
//    * padding */
//   float values[4];
// } CoglMatrixEntryRotateQuaternion;

// typedef struct _CoglMatrixEntryScale
// {
//   CoglMatrixEntry _parent_data;

//   float x;
//   float y;
//   float z;

// } CoglMatrixEntryScale;

// typedef struct _CoglMatrixEntryMultiply
// {
//   CoglMatrixEntry _parent_data;

//   CoglMatrix *matrix;

// } CoglMatrixEntryMultiply;

// typedef struct _CoglMatrixEntryLoad
// {
//   CoglMatrixEntry _parent_data;

//   CoglMatrix *matrix;

// } CoglMatrixEntryLoad;

// typedef struct _CoglMatrixEntrySave
// {
//   CoglMatrixEntry _parent_data;

//   CoglMatrix *cache;
//   CoglBool cache_valid;

// } CoglMatrixEntrySave;

// typedef union _CoglMatrixEntryFull
// {
//   CoglMatrixEntry any;
//   CoglMatrixEntryTranslate translate;
//   CoglMatrixEntryRotate rotate;
//   CoglMatrixEntryRotateEuler rotate_euler;
//   CoglMatrixEntryRotateQuaternion rotate_quaternion;
//   CoglMatrixEntryScale scale;
//   CoglMatrixEntryMultiply multiply;
//   CoglMatrixEntryLoad load;
//   CoglMatrixEntrySave save;
// } CoglMatrixEntryFull;

// typedef struct _CoglMatrixEntryCache
// {
//   CoglMatrixEntry *entry;
//   CoglBool flushed_identity;
//   CoglBool flipped;
// } CoglMatrixEntryCache;

// void
// _cogl_matrix_entry_identity_init (CoglMatrixEntry *entry);

// typedef enum {
//   COGL_MATRIX_MODELVIEW,
//   COGL_MATRIX_PROJECTION,
//   COGL_MATRIX_TEXTURE
// } CoglMatrixMode;

// * CoglMatrixEntry:
// *
// * Represents a single immutable transformation that was retrieved
// * from a #CoglMatrixStack using cogl_matrix_stack_get_entry().
// *
// * Internally a #CoglMatrixEntry represents a single matrix
// * operation (such as "rotate", "scale", "translate") which is applied
// * to the transform of a single parent entry.
// *
// * Using the #CoglMatrixStack api effectively builds up a graph of
// * these immutable #CoglMatrixEntry structures whereby operations
// * that can be shared between multiple transformations will result
// * in shared #CoglMatrixEntry nodes in the graph.
// *
// * When a #CoglMatrixStack is first created it references one
// * #CoglMatrixEntry that represents a single "load identity"
// * operation. This serves as the root entry and all operations
// * that are then applied to the stack will extend the graph
// * starting from this root "load identity" entry.
// *
// * Given the typical usage model for a #CoglMatrixStack and the way
// * the entries are built up while traversing a scenegraph then in most
// * cases where an application is interested in comparing two
// * transformations for equality then it is enough to simply compare
// * two #CoglMatrixEntry pointers directly. Technically this can lead
// * to false negatives that could be identified with a deeper
// * comparison but often these false negatives are unlikely and
// * don't matter anyway so this enables extremely cheap comparisons.
// *
// * <note>#CoglMatrixEntry<!-- -->s are reference counted using
// * cogl_matrix_entry_ref() and cogl_matrix_entry_unref() not with
// * cogl_object_ref() and cogl_object_unref().</note>
#[derive(Debug, PartialOrd, Ord)] // Hash
pub struct MatrixEntry {
//     CoglMatrixEntry *parent;
//     CoglMatrixOp op;
//     unsigned int ref_count;
  
//   #ifdef COGL_DEBUG_ENABLED
//     /* used for performance tracing */
//     int composite_gets;
//   #endif
}

impl MatrixEntry {
    /// Determines if the only difference between two transforms is a
    /// translation and if so returns what the `x`, `y`, and `z` components of
    /// the translation are.
    ///
    /// If the difference between the two translations involves anything
    /// other than a translation then the function returns `false`.
    /// ## `other`
    /// A second reference transform
    /// ## `x`
    /// The destination for the x-component of the translation
    /// ## `y`
    /// The destination for the y-component of the translation
    /// ## `z`
    /// The destination for the z-component of the translation
    ///
    /// # Returns
    ///
    /// `true` if the only difference between the transform of
    ///  `self` and the transform of `other` is a translation,
    ///  otherwise `false`.
    pub fn calculate_translation(&self, other: &MatrixEntry) -> (bool, f32, f32, f32) {
        // GSList *head0 = NULL;
        // GSList *head1 = NULL;
        // CoglMatrixEntry *node0;
        // CoglMatrixEntry *node1;
        // int len0 = 0;
        // int len1 = 0;
        // int count;
        // GSList *common_ancestor0;
        // GSList *common_ancestor1;

        // /* Algorithm:
        // *
        // * 1) Ignoring _OP_SAVE entries walk the ancestors of each entry to
        // *    the root node or any non-translation node, adding a pointer to
        // *    each ancestor node to two linked lists.
        // *
        // * 2) Compare the lists to find the nodes where they start to
        // *    differ marking the common_ancestor node for each list.
        // *
        // * 3) For the list corresponding to entry0, start iterating after
        // *    the common ancestor applying the negative of all translations
        // *    to x, y and z.
        // *
        // * 4) For the list corresponding to entry1, start iterating after
        // *    the common ancestor applying the positive of all translations
        // *    to x, y and z.
        // *
        // * If we come across any non-translation operations during 3) or 4)
        // * then bail out returning FALSE.
        // */

        // for (node0 = entry0; node0; node0 = node0->parent)
        //     {
        //     GSList *link;

        //     if (node0->op == COGL_MATRIX_OP_SAVE)
        //         continue;

        //     link = alloca (sizeof (GSList));
        //     link->next = head0;
        //     link->data = node0;
        //     head0 = link;
        //     len0++;

        //     if (node0->op != COGL_MATRIX_OP_TRANSLATE)
        //         break;
        //     }
        // for (node1 = entry1; node1; node1 = node1->parent)
        //     {
        //     GSList *link;

        //     if (node1->op == COGL_MATRIX_OP_SAVE)
        //         continue;

        //     link = alloca (sizeof (GSList));
        //     link->next = head1;
        //     link->data = node1;
        //     head1 = link;
        //     len1++;

        //     if (node1->op != COGL_MATRIX_OP_TRANSLATE)
        //         break;
        //     }

        // if (head0->data != head1->data)
        //     return FALSE;

        // common_ancestor0 = head0;
        // common_ancestor1 = head1;
        // head0 = head0->next;
        // head1 = head1->next;
        // count = MIN (len0, len1) - 1;
        // while (count--)
        //     {
        //     if (head0->data != head1->data)
        //         break;
        //     common_ancestor0 = head0;
        //     common_ancestor1 = head1;
        //     head0 = head0->next;
        //     head1 = head1->next;
        //     }

        // *x = 0;
        // *y = 0;
        // *z = 0;

        // for (head0 = common_ancestor0->next; head0; head0 = head0->next)
        //     {
        //     CoglMatrixEntryTranslate *translate;

        //     node0 = head0->data;

        //     if (node0->op != COGL_MATRIX_OP_TRANSLATE)
        //         return FALSE;

        //     translate = (CoglMatrixEntryTranslate *)node0;

        //     *x = *x - translate->x;
        //     *y = *y - translate->y;
        //     *z = *z - translate->z;
        //     }
        // for (head1 = common_ancestor1->next; head1; head1 = head1->next)
        //     {
        //     CoglMatrixEntryTranslate *translate;

        //     node1 = head1->data;

        //     if (node1->op != COGL_MATRIX_OP_TRANSLATE)
        //         return FALSE;

        //     translate = (CoglMatrixEntryTranslate *)node1;

        //     *x = *x + translate->x;
        //     *y = *y + translate->y;
        //     *z = *z + translate->z;
        //     }

        // return TRUE;
        unimplemented!()
    }

    /// Compares two arbitrary `MatrixEntry` transforms for equality
    /// returning `true` if they are equal or `false` otherwise.
    ///
    /// `<note>`In many cases it is unnecessary to use this api and instead
    /// direct pointer comparisons of entries are good enough and much
    /// cheaper too.`</note>`
    /// ## `other`
    /// A second `MatrixEntry` to compare
    ///
    /// # Returns
    ///
    /// `true` if `self` represents the same transform as
    ///  `other`, otherwise `false`.
    fn equal(&self, other: &Self) -> bool {
        // for (; entry0 && entry1; entry0 = entry0->parent, entry1 = entry1->parent)
        //  {
        //    entry0 = _cogl_matrix_entry_skip_saves (entry0);
        //    entry1 = _cogl_matrix_entry_skip_saves (entry1);
     
        //    if (entry0 == entry1)
        //      return TRUE;
     
        //    if (entry0->op != entry1->op)
        //      return FALSE;
     
        //    switch (entry0->op)
        //      {
        //      case COGL_MATRIX_OP_LOAD_IDENTITY:
        //        return TRUE;
        //      case COGL_MATRIX_OP_TRANSLATE:
        //        {
        //          CoglMatrixEntryTranslate *translate0 =
        //            (CoglMatrixEntryTranslate *)entry0;
        //          CoglMatrixEntryTranslate *translate1 =
        //            (CoglMatrixEntryTranslate *)entry1;
        //          /* We could perhaps use an epsilon to compare here?
        //           * I expect the false negatives are probaly never going to
        //           * be a problem and this is a bit cheaper. */
        //          if (translate0->x != translate1->x ||
        //              translate0->y != translate1->y ||
        //              translate0->z != translate1->z)
        //            return FALSE;
        //        }
        //        break;
        //      case COGL_MATRIX_OP_ROTATE:
        //        {
        //          CoglMatrixEntryRotate *rotate0 =
        //            (CoglMatrixEntryRotate *)entry0;
        //          CoglMatrixEntryRotate *rotate1 =
        //            (CoglMatrixEntryRotate *)entry1;
        //          if (rotate0->angle != rotate1->angle ||
        //              rotate0->x != rotate1->x ||
        //              rotate0->y != rotate1->y ||
        //              rotate0->z != rotate1->z)
        //            return FALSE;
        //        }
        //        break;
        //      case COGL_MATRIX_OP_ROTATE_QUATERNION:
        //        {
        //          CoglMatrixEntryRotateQuaternion *rotate0 =
        //            (CoglMatrixEntryRotateQuaternion *)entry0;
        //          CoglMatrixEntryRotateQuaternion *rotate1 =
        //            (CoglMatrixEntryRotateQuaternion *)entry1;
        //          int i;
        //          for (i = 0; i < 4; i++)
        //            if (rotate0->values[i] != rotate1->values[i])
        //              return FALSE;
        //        }
        //        break;
        //      case COGL_MATRIX_OP_ROTATE_EULER:
        //        {
        //          CoglMatrixEntryRotateEuler *rotate0 =
        //            (CoglMatrixEntryRotateEuler *)entry0;
        //          CoglMatrixEntryRotateEuler *rotate1 =
        //            (CoglMatrixEntryRotateEuler *)entry1;
     
        //          if (rotate0->heading != rotate1->heading ||
        //              rotate0->pitch != rotate1->pitch ||
        //              rotate0->roll != rotate1->roll)
        //            return FALSE;
        //        }
        //        break;
        //      case COGL_MATRIX_OP_SCALE:
        //        {
        //          CoglMatrixEntryScale *scale0 = (CoglMatrixEntryScale *)entry0;
        //          CoglMatrixEntryScale *scale1 = (CoglMatrixEntryScale *)entry1;
        //          if (scale0->x != scale1->x ||
        //              scale0->y != scale1->y ||
        //              scale0->z != scale1->z)
        //            return FALSE;
        //        }
        //        break;
        //      case COGL_MATRIX_OP_MULTIPLY:
        //        {
        //          CoglMatrixEntryMultiply *mult0 = (CoglMatrixEntryMultiply *)entry0;
        //          CoglMatrixEntryMultiply *mult1 = (CoglMatrixEntryMultiply *)entry1;
        //          if (!cogl_matrix_equal (mult0->matrix, mult1->matrix))
        //            return FALSE;
        //        }
        //        break;
        //      case COGL_MATRIX_OP_LOAD:
        //        {
        //          CoglMatrixEntryLoad *load0 = (CoglMatrixEntryLoad *)entry0;
        //          CoglMatrixEntryLoad *load1 = (CoglMatrixEntryLoad *)entry1;
        //          /* There's no need to check any further since an
        //           * _OP_LOAD makes all the ancestors redundant as far as
        //           * the final matrix value is concerned. */
        //          return cogl_matrix_equal (load0->matrix, load1->matrix);
        //        }
        //      case COGL_MATRIX_OP_SAVE:
        //        /* We skip over saves above so we shouldn't see save entries */
        //        g_warn_if_reached ();
        //      }
        //  }
     
        // return FALSE;
        unimplemented!()
    }

    /// Resolves the current `self` transform into a `Matrix` by
    /// combining the sequence of operations that have been applied to
    /// build up the current transform.
    ///
    /// There are two possible ways that this function may return its
    /// result depending on whether it's possible to directly point
    /// to an internal `Matrix` or whether the result needs to be
    /// composed of multiple operations.
    ///
    /// If an internal matrix contains the required result then this
    /// function will directly return a pointer to that matrix, otherwise
    /// if the function returns `None` then `matrix` will be initialized
    /// to match the transform of `self`.
    ///
    /// `<note>``matrix` will be left untouched if a direct pointer is
    /// returned.`</note>`
    /// ## `matrix`
    /// The potential destination for the transform as
    ///  a matrix
    ///
    /// # Returns
    ///
    /// A direct pointer to a `Matrix` transform or `None`
    ///  and in that case `matrix` will be initialized with
    ///  the effective transform represented by `self`.
    pub fn get(&self) -> (Matrix, Matrix) {
        // int depth;
        // CoglMatrixEntry *current;
        // CoglMatrixEntry **children;
        // int i;

        // for (depth = 0, current = entry;
        //     current;
        //     current = current->parent, depth++)
        //     {
        //     switch (current->op)
        //         {
        //         case COGL_MATRIX_OP_LOAD_IDENTITY:
        //         cogl_matrix_init_identity (matrix);
        //         goto initialized;
        //         case COGL_MATRIX_OP_LOAD:
        //         {
        //             CoglMatrixEntryLoad *load = (CoglMatrixEntryLoad *)current;
        //             _cogl_matrix_init_from_matrix_without_inverse (matrix,
        //                                                         load->matrix);
        //             goto initialized;
        //         }
        //         case COGL_MATRIX_OP_SAVE:
        //         {
        //             CoglMatrixEntrySave *save = (CoglMatrixEntrySave *)current;
        //             if (!save->cache_valid)
        //             {
        //                 CoglMagazine *matrices_magazine =
        //                 cogl_matrix_stack_matrices_magazine;
        //                 save->cache = _cogl_magazine_chunk_alloc (matrices_magazine);
        //                 cogl_matrix_entry_get (current->parent, save->cache);
        //                 save->cache_valid = TRUE;
        //             }
        //             _cogl_matrix_init_from_matrix_without_inverse (matrix, save->cache);
        //             goto initialized;
        //         }
        //         default:
        //         continue;
        //         }
        //     }

        // initialized:

        // if (depth == 0)
        //     {
        //     switch (entry->op)
        //         {
        //         case COGL_MATRIX_OP_LOAD_IDENTITY:
        //         case COGL_MATRIX_OP_TRANSLATE:
        //         case COGL_MATRIX_OP_ROTATE:
        //         case COGL_MATRIX_OP_ROTATE_QUATERNION:
        //         case COGL_MATRIX_OP_ROTATE_EULER:
        //         case COGL_MATRIX_OP_SCALE:
        //         case COGL_MATRIX_OP_MULTIPLY:
        //         return NULL;

        //         case COGL_MATRIX_OP_LOAD:
        //         {
        //             CoglMatrixEntryLoad *load = (CoglMatrixEntryLoad *)entry;
        //             return load->matrix;
        //         }
        //         case COGL_MATRIX_OP_SAVE:
        //         {
        //             CoglMatrixEntrySave *save = (CoglMatrixEntrySave *)entry;
        //             return save->cache;
        //         }
        //         }
        //     g_warn_if_reached ();
        //     return NULL;
        //     }

        // #ifdef COGL_ENABLE_DEBUG
        // if (!current)
        //     {
        //     g_warning ("Inconsistent matrix stack");
        //     return NULL;
        //     }

        // entry->composite_gets++;
        // #endif

        // children = g_alloca (sizeof (CoglMatrixEntry) * depth);

        // /* We need walk the list of entries from the init/load/save entry
        // * back towards the leaf node but the nodes don't link to their
        // * children so we need to re-walk them here to add to a separate
        // * array. */
        // for (i = depth - 1, current = entry;
        //     i >= 0 && current;
        //     i--, current = current->parent)
        //     {
        //     children[i] = current;
        //     }

        // #ifdef COGL_ENABLE_DEBUG
        // if (COGL_DEBUG_ENABLED (COGL_DEBUG_PERFORMANCE) &&
        //     entry->composite_gets >= 2)
        //     {
        //     COGL_NOTE (PERFORMANCE,
        //                 "Re-composing a matrix stack entry multiple times");
        //     }
        // #endif

        // for (i = 0; i < depth; i++)
        //     {
        //     switch (children[i]->op)
        //         {
        //         case COGL_MATRIX_OP_TRANSLATE:
        //         {
        //             CoglMatrixEntryTranslate *translate =
        //             (CoglMatrixEntryTranslate *)children[i];
        //             cogl_matrix_translate (matrix,
        //                                 translate->x,
        //                                 translate->y,
        //                                 translate->z);
        //             continue;
        //         }
        //         case COGL_MATRIX_OP_ROTATE:
        //         {
        //             CoglMatrixEntryRotate *rotate=
        //             (CoglMatrixEntryRotate *)children[i];
        //             cogl_matrix_rotate (matrix,
        //                                 rotate->angle,
        //                                 rotate->x,
        //                                 rotate->y,
        //                                 rotate->z);
        //             continue;
        //         }
        //         case COGL_MATRIX_OP_ROTATE_EULER:
        //         {
        //             CoglMatrixEntryRotateEuler *rotate =
        //             (CoglMatrixEntryRotateEuler *)children[i];
        //             CoglEuler euler;
        //             cogl_euler_init (&euler,
        //                             rotate->heading,
        //                             rotate->pitch,
        //                             rotate->roll);
        //             cogl_matrix_rotate_euler (matrix,
        //                                     &euler);
        //             continue;
        //         }
        //         case COGL_MATRIX_OP_ROTATE_QUATERNION:
        //         {
        //             CoglMatrixEntryRotateQuaternion *rotate =
        //             (CoglMatrixEntryRotateQuaternion *)children[i];
        //             CoglQuaternion quaternion;
        //             cogl_quaternion_init_from_array (&quaternion, rotate->values);
        //             cogl_matrix_rotate_quaternion (matrix, &quaternion);
        //             continue;
        //         }
        //         case COGL_MATRIX_OP_SCALE:
        //         {
        //             CoglMatrixEntryScale *scale =
        //             (CoglMatrixEntryScale *)children[i];
        //             cogl_matrix_scale (matrix,
        //                             scale->x,
        //                             scale->y,
        //                             scale->z);
        //             continue;
        //         }
        //         case COGL_MATRIX_OP_MULTIPLY:
        //         {
        //             CoglMatrixEntryMultiply *multiply =
        //             (CoglMatrixEntryMultiply *)children[i];
        //             cogl_matrix_multiply (matrix, matrix, multiply->matrix);
        //             continue;
        //         }

        //         case COGL_MATRIX_OP_LOAD_IDENTITY:
        //         case COGL_MATRIX_OP_LOAD:
        //         case COGL_MATRIX_OP_SAVE:
        //         g_warn_if_reached ();
        //         continue;
        //         }
        //     }

        // return NULL;
        unimplemented!()
    }

    /// Determines whether `self` is known to represent an identity
    /// transform.
    ///
    /// If this returns `true` then the entry is definitely the identity
    /// matrix. If it returns `false` it may or may not be the identity
    /// matrix but no expensive comparison is performed to verify it.
    ///
    /// # Returns
    ///
    /// `true` if `self` is definitely an identity transform,
    ///  otherwise `false`.
    pub fn is_identity(&self) -> bool {
        // return entry ? entry->op == COGL_MATRIX_OP_LOAD_IDENTITY : FALSE;
        unimplemented!()
    }
}

impl PartialEq for MatrixEntry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        MatrixEntry::equal(self, other)
    }
}

impl Eq for MatrixEntry {}
