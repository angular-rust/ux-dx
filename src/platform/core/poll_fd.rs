#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms
)]

use std::mem;

// @short_description: Functions for integrating  with an
//   application's main loop
//
//  needs to integrate with the application's main loop so that it
// can internally handle some events from the driver. All
// applications must use these functions. They provide enough
// information to describe the state that  will need to wake up
// on. An application using the GLib main loop can instead use
// source_new() which provides a #GSource ready to be added
// to the main loop.

// struct _PollSource
// {
//   int fd;
//   PollPrepareCallback prepare;
//   PollDispatchCallback dispatch;
//   void *user_data;
// };

// PollFD:
// @fd: The file descriptor to block on
// @events: A bitmask of events to block on
// @revents: A bitmask of returned events
//
// A struct for describing the state of a file descriptor that
// needs to block on. The @events field contains a bitmask of
// #PollFDEvents that should cause the application to wake
// up. After the application is woken up from idle it should pass back
// an array of #PollFDs to  and update the @revents
// mask to the actual events that occurred on the file descriptor.
//
// Note that PollFD is deliberately exactly the same as struct
// pollfd on Unix so that it can simply be cast when calling poll.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PollFD {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}
