use std::mem;
use std::ptr;

// * SECTION:cogl-fence
// * @short_description: Functions for notification of command completion
// *
// *  allows notification of GPU command completion; users may mark
// * points in the GPU command stream and receive notification when the GPU
// * has executed to that point.


// /**
//  * FenceCallback:
//  * @fence: Unused. In the future this parameter may be used to pass
//  *   extra information about the fence completion but for now it
//  *   should be ignored.
//  * @user_data: The private data passed to framebuffer_add_fence_callback()
//  *
//  * The callback prototype used with
//  * framebuffer_add_fence_callback() for notification of GPU
//  * command completion.
//  *
//  * Since: 2.0
//  * Stability: Unstable
//  */
//  typedef void (* FenceCallback) (Fence *fence,
//     void *user_data);

// * Fence:
// *
// * An opaque object representing a fence. This type is currently
// * unused but in the future may be used to pass extra information
// * about the fence completion.
// *
// * Since: 2.0
// * Stability: Unstable
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fence {

}

