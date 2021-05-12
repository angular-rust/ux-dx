#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms
)]

use std::mem;
 
// * SECTION:cogl-poll
// * @short_description: Functions for integrating Cogl with an
// *   application's main loop
// *
// * Cogl needs to integrate with the application's main loop so that it
// * can internally handle some events from the driver. All Cogl
// * applications must use these functions. They provide enough
// * information to describe the state that Cogl will need to wake up
// * on. An application using the GLib main loop can instead use
// * cogl_source_new() which provides a #GSource ready to be added
// * to the main loop.

// struct _CoglPollSource
// {
//   int fd;
//   CoglPollPrepareCallback prepare;
//   CoglPollDispatchCallback dispatch;
//   void *user_data;
// };

// * CoglPollFD:
// * @fd: The file descriptor to block on
// * @events: A bitmask of events to block on
// * @revents: A bitmask of returned events
// *
// * A struct for describing the state of a file descriptor that Cogl
// * needs to block on. The @events field contains a bitmask of
// * #CoglPollFDEvent<!-- -->s that should cause the application to wake
// * up. After the application is woken up from idle it should pass back
// * an array of #CoglPollFD<!-- -->s to Cogl and update the @revents
// * mask to the actual events that occurred on the file descriptor.
// *
// * Note that CoglPollFD is deliberately exactly the same as struct
// * pollfd on Unix so that it can simply be cast when calling poll.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PollFD {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

