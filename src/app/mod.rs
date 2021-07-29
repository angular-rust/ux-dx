//! Quickly create ux-dx applications.

#![allow(unused_variables)]

use std::{fmt::Debug, path::PathBuf};
pub use warmy::Store;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        AxisId, DeviceId, ElementState, KeyboardInput, ModifiersState, MouseButton,
        MouseScrollDelta, Touch, TouchPhase, VirtualKeyCode,
    },
    event_loop::ControlFlow,
    window::Theme,
};

pub use crate::{resource::key::Key, time::Time};

pub mod error;
pub mod runner;

/// Class of simple applications.
///
/// Simple application is basically just a single function that takes the current time and display something.
pub trait Application<Runner>: Sized {
    /// Context carried around with the application.
    type Context;

    /// Initialization error that might occur.
    type Error: Sized + Debug;

    /// Initialize the application with a given store.
    ///
    /// The runner is passed so that specific initialization is possible.
    fn init(
        runner: &mut Runner,
        store: &mut Store<Self::Context, Key>,
        context: &mut Self::Context,
    ) -> Result<Self, Self::Error>;

    /// Resize the application when the framebuffer gets resized.
    ///
    /// The runner is passed so that specific resizing is possible.
    fn resized(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        width: u32,
        height: u32,
    );

    // TODO: need to implement user-event logic for UserEvent(T)
    // Emitted when an event is sent from [`EventLoopProxy::send_event`](crate::event_loop::EventLoopProxy::send_event)

    /// Notify the application has been suspended.
    fn suspend(&mut self, runner: &mut Runner, context: &mut Self::Context) {}

    /// Notify the application has been resumed.
    fn resume(&mut self, runner: &mut Runner, context: &mut Self::Context) {}

    /// The position of the window has changed. Contains the window's new position.
    fn moved(&mut self, runner: &mut Runner, context: &mut Self::Context, x: i32, y: i32) {}

    /// The window has been requested to close.
    ///
    /// By default application will exit
    fn close(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        control_flow: &mut ControlFlow,
    ) {
        *control_flow = ControlFlow::Exit
    }

    /// The window has been destroyed.
    fn destroyed(&mut self, runner: &mut Runner, context: &mut Self::Context) {}

    /// A file has been dropped into the window.
    ///
    /// When the user drops multiple files at once, this event will be emitted for each file
    /// separately.
    fn dropped_file(&mut self, runner: &mut Runner, context: &mut Self::Context, path: &PathBuf) {}

    /// A file is being hovered over the window.
    ///
    /// When the user hovers multiple files at once, this event will be emitted for each file
    /// separately.
    fn hovered_file(&mut self, runner: &mut Runner, context: &mut Self::Context, path: &PathBuf) {}

    /// A file was hovered, but has exited the window.
    ///
    /// There will be a single `HoveredFileCancelled` event triggered even if multiple files were
    /// hovered.
    fn hovered_file_cancelled(&mut self, runner: &mut Runner, context: &mut Self::Context) {}

    /// The window received a unicode character.
    fn received_character(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        character: char,
    ) {
    }

    /// The window gained or lost focus.
    ///
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    fn focused(&mut self, runner: &mut Runner, context: &mut Self::Context, is_focused: bool) {}

    /// An event from the keyboard has been received.
    ///
    /// If `true`, the event was generated synthetically by winit
    /// in one of the following circumstances:
    ///
    /// * Synthetic key press events are generated for all keys pressed
    ///   when a window gains focus. Likewise, synthetic key release events
    ///   are generated for all keys pressed when a window goes out of focus.
    ///   ***Currently, this is only functional on X11 and Windows***
    ///
    /// Otherwise, this value is always `false`.
    fn keyboard_input(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        control_flow: &mut ControlFlow,
        device_id: &DeviceId,
        input: &KeyboardInput,
        is_synthetic: bool,
    ) {
        match input {
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    }

    /// The keyboard modifiers have changed.
    ///
    /// Platform-specific behavior:
    /// - **Web**: This API is currently unimplemented on the web. This isn't by design - it's an
    ///   issue, and it should get fixed - but it's the current state of the API.
    fn modifiers_changed(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        state: &ModifiersState,
    ) {
    }

    /// The cursor has moved on the window.
    ///
    /// (x,y) coords in pixels relative to the top-left corner of the window. Because the range of this data is
    /// limited by the display area and it may have been transformed by the OS to implement effects such as cursor
    /// acceleration, it should not be used to implement non-cursor-like interactions such as 3D camera control.
    fn cursor_moved(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        device_id: &DeviceId,
        position: &PhysicalPosition<f64>,
    ) {
    }

    /// The cursor has entered the window.
    fn cursor_entered(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        device_id: &DeviceId,
    ) {
    }

    /// The cursor has left the window.
    fn cursor_left(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        device_id: &DeviceId,
    ) {
    }

    /// A mouse wheel movement or touchpad scroll occurred.
    fn mouse_wheel(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
    ) {
    }

    /// An mouse button press has been received.
    fn mouse_input(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        device_id: &DeviceId,
        state: &ElementState,
        button: &MouseButton,
    ) {
    }

    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    /// The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
    /// is being pressed) and stage (integer representing the click level).
    fn touchpad_pressure(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        device_id: &DeviceId,
        pressure: f32,
        stage: i64,
    ) {
    }

    /// Motion on some analog axis. May report data redundant to other, more specific events.
    fn axis_motion(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        device_id: &DeviceId,
        axis: &AxisId,
        value: f64,
    ) {
    }

    /// Touch event has been received
    fn touch(&mut self, runner: &mut Runner, context: &mut Self::Context, touch: &Touch) {}

    /// The window's scale factor has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// * Changing the display's resolution.
    /// * Changing the display's scale factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different scale factor.
    ///
    /// After this event callback has been processed, the window will be resized to whatever value
    /// is pointed to by the `new_inner_size` reference. By default, this will contain the size suggested
    /// by the OS, but it can be changed to any value.
    ///
    /// For more information about DPI in general, see the [`dpi`](crate::dpi) module.
    fn scale_factor_changed(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        scale_factor: f64,
        new_inner_size: &&mut PhysicalSize<u32>,
    ) {
    }

    /// The system window theme has changed.
    ///
    /// Applications might wish to react to this to change the theme of the content of the window
    /// when the system changes the window theme.
    ///
    /// At the moment this is only supported on Windows.
    fn theme_changed(&mut self, runner: &mut Runner, context: &mut Self::Context, theme: &Theme) {}

    /// Render the application at a given time.
    ///
    /// The runner is passed so that specific rendering is possible.
    /// return `agressive` state. true for applications which FPS aggressive,
    /// like a games or animation/media oriented applications.
    /// also you can change the behaviour for your needs.
    fn render(
        &mut self,
        runner: &mut Runner,
        context: &mut Self::Context,
        t: Time,
        // back_buffer: &Backbuffer,
        // builder: Builder,
    ) -> bool;
}
