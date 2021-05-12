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

// * CoglFenceClosure:
// *
// * An opaque type representing one future callback to be made when the
// * GPU command stream has passed a certain point.
// *
// * Since: 2.0
// * Stability: Unstable
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FenceClosure {
    // CoglList link;
    // CoglFramebuffer *framebuffer;
  
    // CoglFenceType type;
    // void *fence_obj;
  
    // CoglFenceCallback callback;
    // void *user_data;
}

// /**
//  * cogl_frame_closure_get_user_data:
//  * @closure: A #CoglFenceClosure returned from cogl_framebuffer_add_fence()
//  *
//  * Returns the user_data submitted to cogl_framebuffer_add_fence() which
//  * returned a given #CoglFenceClosure.
//  *
//  * Since: 2.0
//  * Stability: Unstable
//  */
//  void *
//  cogl_fence_closure_get_user_data (CoglFenceClosure *closure);

// void *
// cogl_fence_closure_get_user_data (CoglFenceClosure *closure)
// {
//   return closure->user_data;
// }

//  /**
//   * cogl_framebuffer_add_fence_callback:
//   * @framebuffer: The #CoglFramebuffer the commands have been submitted to
//   * @callback: (scope notified): A #CoglFenceCallback to be called when
//   *            all commands submitted to Cogl have been executed
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
//  CoglFenceClosure *
//  cogl_framebuffer_add_fence_callback (CoglFramebuffer *framebuffer,
//                                       CoglFenceCallback callback,
//                                       void *user_data);
 
//  /**
//   * cogl_framebuffer_cancel_fence_callback:
//   * @framebuffer: The #CoglFramebuffer the commands were submitted to
//   * @closure: The #CoglFenceClosure returned from
//   *           cogl_framebuffer_add_fence_callback()
//   *
//   * Removes a fence previously submitted with
//   * cogl_framebuffer_add_fence_callback(); the callback will not be
//   * called.
//   *
//   * Since: 2.0
//   * Stability: Unstable
//   */
//  void
//  cogl_framebuffer_cancel_fence_callback (CoglFramebuffer *framebuffer,
//                                          CoglFenceClosure *closure);

// CoglFenceClosure *
// cogl_framebuffer_add_fence_callback (CoglFramebuffer *framebuffer,
//                                      CoglFenceCallback callback,
//                                      void *user_data)
// {
//   CoglContext *context = framebuffer->context;
//   CoglJournal *journal = framebuffer->journal;
//   CoglFenceClosure *fence;

//   if (!COGL_FLAGS_GET (context->features, COGL_FEATURE_ID_FENCE))
//     return NULL;

//   fence = g_slice_new (CoglFenceClosure);
//   fence->framebuffer = framebuffer;
//   fence->callback = callback;
//   fence->user_data = user_data;
//   fence->fence_obj = NULL;

//   if (journal->entries->len)
//     {
//       _cogl_list_insert (journal->pending_fences.prev, &fence->link);
//       fence->type = FENCE_TYPE_PENDING;
//     }
//   else
//     _cogl_fence_submit (fence);

//   return fence;
// }

// void
// cogl_framebuffer_cancel_fence_callback (CoglFramebuffer *framebuffer,
//                                         CoglFenceClosure *fence)
// {
//   CoglContext *context = framebuffer->context;

//   if (fence->type == FENCE_TYPE_PENDING)
//     {
//       _cogl_list_remove (&fence->link);
//     }
//   else
//     {
//       _cogl_list_remove (&fence->link);

//       if (fence->type == FENCE_TYPE_WINSYS)
//         {
//           const CoglWinsysVtable *winsys = _cogl_context_get_winsys (context);

//           winsys->fence_destroy (context, fence->fence_obj);
//         }
// #ifdef GL_ARB_sync
//       else if (fence->type == FENCE_TYPE_GL_ARB)
//         {
//           context->glDeleteSync (fence->fence_obj);
//         }
// #endif
//     }

//   g_slice_free (CoglFenceClosure, fence);
// }
