use crate::{IndexBuffer, IndicesType, Object, Context};
use std::fmt;

// * SECTION:cogl-indices
// * @short_description: Describe vertex indices stored in a #IndexBuffer.
// *
// * Indices allow you to avoid duplicating vertices in your vertex data
// * by virtualizing your data and instead providing a sequence of index
// * values that tell the GPU which data should be used for each vertex.
// *
// * If the GPU is given a sequence of indices it doesn't simply walk
// * through each vertex of your data in order it will instead walk
// * through the indices which can provide random access to the
// * underlying data.
// *
// * Since it's very common to have duplicate vertices when describing a
// * shape as a list of triangles it can often be a significant space
// * saving to describe geometry using indices. Reducing the size of
// * your models can make it cheaper to map them into the GPU by
// * reducing the demand on memory bandwidth and may help to make better
// * use of your GPUs internal vertex caching.
// *
// * For example, to describe a quadrilateral as 2 triangles for the GPU
// * you could either provide data with 6 vertices or instead with
// * indices you can provide vertex data for just 4 vertices and an
// * index buffer that specfies the 6 vertices by indexing the shared
// * vertices multiple times.
// *
// * |[
// *   Vertex2f quad_vertices[] = {
// *     {x0, y0}, //0 = top left
// *     {x1, y1}, //1 = bottom left
// *     {x2, y2}, //2 = bottom right
// *     {x3, y3}, //3 = top right
// *   };
// *   //tell the gpu how to interpret the quad as 2 triangles...
// *   unsigned char indices[] = {0, 1, 2, 0, 2, 3};
// * ]|
// *
// * Even in the above illustration we see a saving of 10bytes for one
// * quad compared to having data for 6 vertices and no indices but if
// * you need to draw 100s or 1000s of quads then its really quite
// * significant.
// *
// * Something else to consider is that often indices can be defined
// * once and remain static while the vertex data may change for
// * animations perhaps. That means you may be able to ignore the
// * negligable cost of mapping your indices into the GPU if they don't
// * ever change.
// *
// * The above illustration is actually a good example of static indices
// * because it's really common that developers have quad mesh data that
// * they need to display and we know exactly what that indices array
// * needs to look like depending on the number of quads that need to be
// * drawn. It doesn't matter how the quads might be animated and
// * changed the indices will remain the same.  even has a utility
// * (get_rectangle_indices()) to get access to re-useable indices
// * for drawing quads as above.

pub struct Indices {
    // Object _parent;

    // IndexBuffer *buffer;
    // size_t offset;
  
    // IndicesType type;
  
    // int immutable_ref;
    
}

impl Indices {
    pub fn new(context: &Context, type_: IndicesType, indices_data: &[u8], n_indices: i32) -> Indices {
        // size_t buffer_bytes = sizeof_indices_type (type) * n_indices;
        // IndexBuffer *index_buffer = index_buffer_new (context, buffer_bytes);
        // Buffer *buffer = COGL_BUFFER (index_buffer);
        // Indices *indices;
        // Error *ignore_error = NULL;
      
        // _buffer_set_data (buffer,
        //                        0,
        //                        indices_data,
        //                        buffer_bytes,
        //                        &ignore_error);
        // if (ignore_error)
        //   {
        //     error_free (ignore_error);
        //     object_unref (index_buffer);
        //     return NULL;
        //   }
      
        // indices = indices_new_for_buffer (type, index_buffer, 0);
        // object_unref (index_buffer);
      
        // return indices;
        unimplemented!()
    }

    pub fn new_for_buffer(type_: IndicesType, buffer: &IndexBuffer, offset: usize) -> Indices {
        // Indices *indices = g_slice_new (Indices);

        // indices->buffer = object_ref (buffer);
        // indices->offset = offset;

        // indices->type = type;

        // indices->immutable_ref = 0;

        // return _indices_object_new (indices);
        unimplemented!()
    }

    pub fn get_buffer(&self) -> Option<IndexBuffer> {
        // return indices->buffer;
        unimplemented!()
    }

    pub fn get_offset(&self) -> usize {
        // _COGL_RETURN_VAL_IF_FAIL (is_indices (indices), 0);

        // return indices->offset;
        unimplemented!()
    }

    pub fn get_type(&self) -> IndicesType {
        // _COGL_RETURN_VAL_IF_FAIL (is_indices (indices),
        //                     COGL_INDICES_TYPE_UNSIGNED_BYTE);
        // return indices->type;
        unimplemented!()
    }

    pub fn set_offset(&self, offset: usize) {
        // _COGL_RETURN_IF_FAIL (is_indices (indices));

        // if (G_UNLIKELY (indices->immutable_ref))
        //     warn_about_midscene_changes ();

        // indices->offset = offset;
        unimplemented!()
    }
}

impl fmt::Display for Indices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Indices")
    }
}
