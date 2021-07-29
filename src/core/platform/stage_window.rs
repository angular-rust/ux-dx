// use crate::pure::platform::cogl::StageCogl;
struct StageWindowProps {
    // parent_instance: StageCogl,

    // xwin: Window,
    xwin_width: u32,
    xwin_height: u32, // FIXME target_width / height

    title: Option<String>,

    clipped_redraws_cool_off: u32,

    // wm_state: StageX11State,
    scale_factor: f32,

    is_foreign_xwin: bool,
    fullscreening: bool,
    is_cursor_visible: bool,
    viewport_initialized: bool,
    accept_focus: bool,
    fullscreen_on_realize: bool,
}
#[derive(Debug, Clone)]
pub struct StageWindow {}

impl StageWindow {}
