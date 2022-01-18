use std::mem;
use std::ptr;

pub enum FenceType {
    //   FENCE_TYPE_PENDING,
// #ifdef GL_ARB_sync
//   FENCE_TYPE_GL_ARB,
// #endif
//   FENCE_TYPE_WINSYS,
//   FENCE_TYPE_ERROR
}

// FenceClosure:
//
// An opaque type representing one future callback to be made when the
// GPU command stream has passed a certain point.
//
// Since: 2.0
// Stability: Unstable
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FenceClosure {
    // List link;
// Framebuffer *framebuffer;

// FenceType type;
// void *fence_obj;

// FenceCallback callback;
// void *user_data;
}

//*
//  * frame_closure_get_user_data:
//  * @closure: A #FenceClosure returned from framebuffer_add_fence()
//  *
//  * Returns the user_data submitted to framebuffer_add_fence() which
//  * returned a given #FenceClosure.
//  *
//  * Since: 2.0
//  * Stability: Unstable
//  */
//  void *
//  fence_closure_get_user_data (FenceClosure *closure);

// void *
// fence_closure_get_user_data (FenceClosure *closure)
// {
//   return closure->user_data;
// }

//  /**
//   * framebuffer_add_fence_callback:
//   * @framebuffer: The #Framebuffer the commands have been submitted to
//   * @callback: (scope notified): A #FenceCallback to be called when
//   *            all commands submitted to  have been executed
//   * @user_data: (closure): Private data that will be passed to the callback
//   *
//   * Calls the provided callback when all previously-submitted commands have
//   * been executed by the GPU.
//   *
//   * Returns non-NULL if the fence succeeded, or %NULL if it was unable to
//   * be inserted and the callback will never be called.  The user does not
//   * need to free the closure; it will be freed automatically when the
//   * callback is called, or cancelled.
//   *
//   * Since: 2.0
//   * Stability: Unstable
//   */
//  FenceClosure *
//  framebuffer_add_fence_callback (Framebuffer *framebuffer,
//                                       FenceCallback callback,
//                                       void *user_data);

//  /**
//   * framebuffer_cancel_fence_callback:
//   * @framebuffer: The #Framebuffer the commands were submitted to
//   * @closure: The #FenceClosure returned from
//   *           framebuffer_add_fence_callback()
//   *
//   * Removes a fence previously submitted with
//   * framebuffer_add_fence_callback(); the callback will not be
//   * called.
//   *
//   * Since: 2.0
//   * Stability: Unstable
//   */
//  void
//  framebuffer_cancel_fence_callback (Framebuffer *framebuffer,
//                                          FenceClosure *closure);

// FenceClosure *
// framebuffer_add_fence_callback (Framebuffer *framebuffer,
//                                      FenceCallback callback,
//                                      void *user_data)
// {
//   Context *context = framebuffer->context;
//   Journal *journal = framebuffer->journal;
//   FenceClosure *fence;

//   if (!FLAGS_GET (context->features, FEATURE_ID_FENCE))
//     return NULL;

//   fence = g_slice_new (FenceClosure);
//   fence->framebuffer = framebuffer;
//   fence->callback = callback;
//   fence->user_data = user_data;
//   fence->fence_obj = NULL;

//   if (journal->entries->len)
//     {
//       _list_insert (journal->pending_fences.prev, &fence->link);
//       fence->type = FENCE_TYPE_PENDING;
//     }
//   else
//     _fence_submit (fence);

//   return fence;
// }

// void
// framebuffer_cancel_fence_callback (Framebuffer *framebuffer,
//                                         FenceClosure *fence)
// {
//   Context *context = framebuffer->context;

//   if (fence->type == FENCE_TYPE_PENDING)
//     {
//       _list_remove (&fence->link);
//     }
//   else
//     {
//       _list_remove (&fence->link);

//       if (fence->type == FENCE_TYPE_WINSYS)
//         {
//           const WinsysVtable *winsys = _context_get_winsys (context);

//           winsys->fence_destroy (context, fence->fence_obj);
//         }
// #ifdef GL_ARB_sync
//       else if (fence->type == FENCE_TYPE_GL_ARB)
//         {
//           context->glDeleteSync (fence->fence_obj);
//         }
// #endif
//     }

//   g_slice_free (FenceClosure, fence);
// }
