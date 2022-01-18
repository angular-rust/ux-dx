#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

//! Pure Rust Immediate Mode UI - WIP

use cgmath::Vector3;
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    f32::consts::PI,
    sync::{Arc, RwLock},
    time::SystemTime,
};
use winit::event::{ModifiersState, MouseButton, VirtualKeyCode};

use crate::prelude::color;

use crate::{
    engine::d2::{Font, Image, Painter},
    foundation::colorspace::Color,
};

mod ext;
pub use self::ext::*;

mod id;
pub use self::id::*;

mod inspect;
pub use self::inspect::*;

mod macros;
pub use macros::*;

mod nodes;
pub use self::nodes::*;

mod themes;
pub use self::themes::*;

pub struct Framebuffer {}

pub struct UiOptions {
    pub font: Font,
    pub theme: themes::Theme,
    pub window_id: i32,
    pub scale_factor: f32,
    pub auto_notify_input: bool,
    pub color_wheel: Option<Image>,
}

impl Default for UiOptions {
    fn default() -> Self {
        Self {
            font: Default::default(),
            theme: Themes::get(DEFAULT_DARK_THEME).unwrap(),
            window_id: 0,
            scale_factor: 1.0,
            auto_notify_input: true,
            color_wheel: None, //Image::new(0, 0, TextureFormat::Rgba8Uint, false),
        }
    }
}

pub struct Ui {
    pub is_scrolling: bool, // Use to limit other activities
    pub is_typing: bool,
    pub enabled: bool, // Current element state
    pub is_started: bool,
    pub is_pushed: bool,
    pub is_hovered: bool,
    pub is_released: bool,
    pub changed: bool, // Global elements change check
    pub image_invert_y: bool,
    pub scroll_enabled: bool,
    pub always_redraw: bool,       // Hurts performance
    pub highlight_on_select: bool, // Highlight text edit contents on selection
    pub tab_switch_enabled: bool,  // Allow switching focus to the next element by pressing tab
    pub window_border_top: u32,
    pub window_border_bottom: u32,
    pub window_border_left: u32,
    pub window_border_right: u32,
    highlight_full_row: bool,

    // static
    // pub current: Ui = None;
    // pub static
    on_border_hover: Option<Box<dyn Fn(&Handle, i32)>>, // Mouse over window border, use for resizing
    // pub static
    on_text_hover: Option<Box<dyn Fn()>>, // = None; // Mouse over text input, use to set I-cursor
    // pub static
    always_redraw_window: bool, // Redraw cached window texture each frame or on changes only
    // pub static
    key_repeat: bool, // Emulate key repeat for non-character keys
    // pub static
    dynamic_glyph_load: bool, // Allow text input fields to push new glyphs into the font atlas

    // static
    pub touch_controls: bool,

    touch_hold: bool,
    slider_tooltip: bool,
    slider_tooltip_x: f32,
    slider_tooltip_y: f32,
    slider_tooltip_w: f32,

    pub input_registered: bool,
    pub input_enabled: bool,
    pub input_x: f32, // Input position
    pub input_y: f32,
    pub input_started_x: f32,
    pub input_started_y: f32,
    pub input_dx: f32, // Delta
    pub input_dy: f32,
    pub input_wheel_delta: i32,
    pub input_started: bool, // Buttons
    pub input_started_r: bool,
    pub input_released: bool,
    pub input_released_r: bool,
    pub input_down: bool,
    pub input_down_r: bool,
    pub is_key_pressed: bool, // Keys
    pub is_key_down: bool,
    pub is_shift_down: bool,
    pub is_ctrl_down: bool,
    pub is_alt_down: bool,
    pub is_adown: bool,
    pub is_backspace_down: bool,
    pub is_delete_down: bool,
    pub is_escape_down: bool,
    pub is_return_down: bool,
    pub is_tab_down: bool,
    pub key: Option<VirtualKeyCode>,
    pub char: char,

    // static
    key_repeat_time: f32,
    // static
    text_to_paste: String,
    // static
    text_to_copy: String,
    // static
    is_cut: bool,
    // static
    is_copy: bool,
    // static
    is_paste: bool,
    // static
    // copy_receiver: Ui
    // static
    copy_frame: i32,

    input_started_time: f32,
    cursor_x: usize, // Text input
    highlight_anchor: usize,
    ratios: Vec<f32>, // Splitting rows
    cur_ratio: i32,
    x_before_split: f32,
    w_before_split: f32,

    pub painter: Painter, // Drawing
    pub theme: Theme,     // t
    pub ops: UiOptions,
    // rtTextPipeline: graphics4.PipelineState; // Rendering text into rendertargets
    font_size: f32,
    font_offset_y: f32, // Precalculated offsets
    arrow_offset_x: f32,
    arrow_offset_y: f32,
    title_offset_x: f32,
    button_offset_y: f32,
    check_offset_x: f32,
    check_offset_y: f32,
    check_select_offset_x: f32,
    check_select_offset_y: f32,
    radio_offset_x: f32,
    radio_offset_y: f32,
    radio_select_offset_x: f32,
    radio_select_offset_y: f32,
    scroll_align: f32,
    image_scroll_align: bool,

    // TODO: x,y,w,h should be private
    pub x: f32, // Cursor(stack) position
    pub y: f32,
    pub w: f32,
    pub h: f32,

    window_x: f32, // Window state
    window_y: f32,
    window_w: f32,
    window_h: f32,

    window_ended: bool, // = true
    window_header_w: f32,
    window_header_h: f32,
    restore_x: f32,
    restore_y: f32,

    current_window: Option<Handle>,
    scroll_handle: Option<Handle>, // Window or slider being scrolled
    drag_handle: Option<Handle>,   // Window being dragged
    text_selected_handle: Option<Handle>,
    submit_text_handle: Option<Handle>,
    tab_pressed_handle: Option<Handle>,
    combo_selected_handle: Option<Handle>,
    combo_selected_window: Option<Handle>,
    submit_combo_handle: Option<Handle>,
    tab_handle: Option<Handle>,
    wheel_selected_handle: Option<Handle>,
    text_selected: String,
    text_to_submit: String,
    tab_pressed: bool,

    combo_selected_align: Align,
    combo_selected_texts: Vec<String>,
    combo_selected_label: String,
    combo_selected_x: i32,
    combo_selected_y: i32,
    combo_selected_w: i32,

    combo_to_submit: i32,
    tooltip_text: String,
    tooltip_img: Option<Image>,
    tooltip_img_max_width: Option<i32>,
    tooltip_invert_y: bool,
    tooltip_x: f32,
    tooltip_y: f32,
    tooltip_shown: bool,
    tooltip_wait: bool,
    tooltip_time: f32,
    tab_names: Vec<String>, // Number of tab calls since window begin
    tab_colors: Vec<Option<Color>>,
    tab_scroll: f32,
    tab_vertical: bool,
    sticky: bool,
    scissor: bool,

    start_time: SystemTime,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            is_scrolling: false, // Use to limit other activities
            is_typing: false,
            enabled: true, // Current element state
            is_started: false,
            is_pushed: false,
            is_hovered: false,
            is_released: false,
            changed: false, // Global elements change check
            image_invert_y: false,
            scroll_enabled: true,
            always_redraw: false,      // Hurts performance
            highlight_on_select: true, // Highlight text edit contents on selection
            tab_switch_enabled: true,  // Allow switching focus to the next element by pressing tab
            window_border_top: 0,
            window_border_bottom: 0,
            window_border_left: 0,
            window_border_right: 0,
            highlight_full_row: false,

            // static
            // pub current: Ui = None;
            on_border_hover: None, // Mouse over window border, use for resizing
            on_text_hover: None,   // Mouse over text input, use to set I-cursor
            always_redraw_window: true, // Redraw cached window texture each frame or on changes only
            key_repeat: true,           // Emulate key repeat for non-character keys
            dynamic_glyph_load: true, // Allow text input fields to push new glyphs into the font atlas

            // TODO: add touch ui feature
            #[cfg(any(target_os = "android", target_os = "ios"))]
            touch_controls: true, // Pan with finger to scroll, hold finger for right click
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            touch_controls: false,

            touch_hold: false,
            slider_tooltip: false,
            slider_tooltip_x: 0.0,
            slider_tooltip_y: 0.0,
            slider_tooltip_w: 0.0,

            input_registered: false,
            input_enabled: true,
            input_x: 0.0, // Input position
            input_y: 0.0,
            input_started_x: 0.0,
            input_started_y: 0.0,
            input_dx: 0.0, // Delta
            input_dy: 0.0,
            input_wheel_delta: 0,
            input_started: false, // Buttons
            input_started_r: false,
            input_released: false,
            input_released_r: false,
            input_down: false,
            input_down_r: false,
            is_key_pressed: false, // Keys
            is_key_down: false,
            is_shift_down: false,
            is_ctrl_down: false,
            is_alt_down: false,
            is_adown: false,
            is_backspace_down: false,
            is_delete_down: false,
            is_escape_down: false,
            is_return_down: false,
            is_tab_down: false,
            key: None,
            char: Default::default(), // '\x00' Maybe use Option for that is more handy

            key_repeat_time: 0.0,
            text_to_paste: "".into(),
            text_to_copy: "".into(),
            is_cut: false,
            is_copy: false,
            is_paste: false,
            // copy_receiver: None,
            copy_frame: 0,

            input_started_time: 0.0,
            cursor_x: 0, // Text input
            highlight_anchor: 0,
            ratios: Vec::new(), // Splitting rows
            cur_ratio: -1,
            x_before_split: 0.0,
            w_before_split: 0.0,

            painter: Default::default(), // Drawing
            theme: Default::default(),
            ops: Default::default(),
            // rtTextPipeline: graphics4.PipelineState; // Rendering text into rendertargets
            font_size: 0.0,
            font_offset_y: 0.0, // Precalculated offsets
            arrow_offset_x: 0.0,
            arrow_offset_y: 0.0,
            title_offset_x: 0.0,
            button_offset_y: 0.0,
            check_offset_x: 0.0,
            check_offset_y: 0.0,
            check_select_offset_x: 0.0,
            check_select_offset_y: 0.0,
            radio_offset_x: 0.0,
            radio_offset_y: 0.0,
            radio_select_offset_x: 0.0,
            radio_select_offset_y: 0.0,
            scroll_align: 0.0,
            image_scroll_align: true,

            x: 0.0, // Cursor(stack) position
            y: 0.0,
            w: 0.0,
            h: 0.0,

            window_x: 0.0, // Window state
            window_y: 0.0,
            window_w: 0.0,
            window_h: 0.0,

            window_ended: true,
            window_header_w: 0.0,
            window_header_h: 0.0,
            restore_x: -1.0,
            restore_y: -1.0,

            current_window: None,
            scroll_handle: None, // Window or slider being scrolled
            drag_handle: None,   // Window being dragged
            text_selected_handle: None,
            submit_text_handle: None,
            tab_pressed_handle: None,
            combo_selected_handle: None,
            combo_selected_window: None,
            submit_combo_handle: None,
            tab_handle: None,
            wheel_selected_handle: None,
            text_selected: "".into(),
            text_to_submit: "".into(),
            tab_pressed: false,

            combo_selected_align: Align::Left,
            combo_selected_texts: Vec::new(),
            combo_selected_label: "".into(),
            combo_selected_x: 0,
            combo_selected_y: 0,
            combo_selected_w: 0,

            combo_to_submit: 0,
            tooltip_text: "".into(),
            tooltip_img: None,
            tooltip_img_max_width: None,
            tooltip_invert_y: false,
            tooltip_x: 0.0,
            tooltip_y: 0.0,
            tooltip_shown: false,
            tooltip_wait: false,
            tooltip_time: 0.0,
            tab_names: Vec::new(), // Number of tab calls since window begin
            tab_colors: Vec::new(),
            tab_scroll: 0.0,
            tab_vertical: false,
            sticky: false,
            scissor: false,
            start_time: SystemTime::now(),
        }
    }
}

impl Ui {
    pub fn new(ops: UiOptions) -> Self {
        let mut instance = Self {
            ops,
            ..Default::default()
        };

        instance.theme = instance.ops.theme.clone();
        instance.set_scale(instance.ops.scale_factor);

        if instance.ops.auto_notify_input {
            instance.register_input();
        }

        // if instance.copy_receiver.is_some() {
        // 	copy_receiver = this;
        // 	System.notifyOnCutCopyPaste(onCut, onCopy, onPaste);
        // 	System.notifyOnFrames(|frames: Vec<Framebuffer>| {
        // 		// Set isCopy to false on next frame
        // 		if (isCopy || isPaste) && ++copyFrame > 1 {
        // 			isCopy = isCut = isPaste = false;
        // 		} else if copyFrame > 1 && ++copyFrame > 2 {
        // 			// Clear unpasted text on next frame
        // 			copyFrame = 0;
        // 			text_to_paste = "";
        // 		}
        // 	});
        // }

        // let rtTextVS = graphics4.Graphics2.createTextVertexStructure();
        // instance.rtTextPipeline = graphics4.Graphics2.createTextPipeline(rtTextVS);
        // instance.rtTextPipeline.alphaBlendSource = BlendOne;
        // instance.rtTextPipeline.compile();
        instance
    }

    fn scheduler_time(&self) -> f32 {
        self.start_time.elapsed().unwrap().as_millis() as f32 / 1000.0
    }

    pub fn set_scale(&mut self, factor: f32) {
        self.ops.scale_factor = factor;
        self.font_size = self.font_size();

        let font_height = self.ops.font.height(self.font_size);

        self.font_offset_y = (self.element_h() - font_height) / 2.0; // Precalculate offsets
        self.arrow_offset_y = (self.element_h() - self.arrow_size()) / 2.0;
        self.arrow_offset_x = self.arrow_offset_y;
        self.title_offset_x = (self.arrow_offset_x * 2.0 + self.arrow_size()) / self.scale();
        self.button_offset_y = (self.element_h() - self.button_h()) / 2.0;
        self.check_offset_y = (self.element_h() - self.check_size()) / 2.0;
        self.check_offset_x = self.check_offset_y;
        self.check_select_offset_y = (self.check_size() - self.check_select_size()) / 2.0;
        self.check_select_offset_x = self.check_select_offset_y;
        self.radio_offset_y = (self.element_h() - self.check_size()) / 2.0;
        self.radio_offset_x = self.radio_offset_y;
        self.radio_select_offset_y = (self.check_size() - self.check_select_size()) / 2.0;
        self.radio_select_offset_x = self.radio_select_offset_y;
    }

    pub fn remove(&mut self) {
        // Clean up
        if self.ops.auto_notify_input {
            self.unregister_input();
        }
    }

    // Need register handlers:
    // on_mouse_down, on_mouse_up, on_mouse_move, on_mouse_wheel
    // on_key_down, on_key_up, on_key_press
    // on android and ios also need to register the:
    // on_touch_down, on_touch_up, on_touch_move
    pub fn register_input(&mut self) {
        // // Reset mouse delta on foreground
        // // System.notifyOnApplicationState(fn() { inputDX = inputDY = 0; }, None, None, None, None);
        self.input_registered = true;
    }

    pub fn unregister_input(&mut self) {
        // System.removeCutCopyPaste(onCut, onCopy, onPaste);
        self.end_input();
        self.is_shift_down = false;
        self.is_ctrl_down = false;
        self.is_alt_down = false;
        self.input_x = 0.0;
        self.input_y = 0.0;
        self.input_registered = false;
    }

    pub fn begin(&mut self /* , g: &Graphics*/) {
        // Begin UI drawing
        self.changed = false;

        self.x = 0.0; // Reset cursor
        self.y = 0.0;
        self.w = 0.0;
        self.h = 0.0;
    }

    pub fn end(&mut self, last: bool /* = true*/) {
        // End drawing
        if !self.window_ended {
            self.end_window(true);
        }

        self.draw_combo(); // Handle active combo
        self.draw_tooltip(true);
        self.tab_pressed_handle = None;
        if last {
            self.end_input();
        }
        self.painter.end();
    }

    pub fn begin_region(&mut self, x: i32, y: i32, w: i32) {
        self.changed = false;
        self.current_window = None;
        self.tooltip_text = "".into();
        self.tooltip_img = None;
        self.window_x = 0.0;
        self.window_y = 0.0;
        self.window_w = w as f32;
        self.x = x as f32;
        self.y = y as f32;
        self.w = w as f32;
    }

    pub fn end_region(&mut self, last: bool /* = true*/) {
        self.draw_tooltip(false);
        self.tab_pressed_handle = None;
        if last {
            self.end_input();
        }
    }

    // Sticky region ignores window scrolling
    pub fn begin_sticky(&mut self) {
        self.sticky = true;
        if let Some(ref current_window) = self.current_window {
            match current_window.props.read() {
                Ok(props) => {
                    self.y -= props.scroll_offset;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }
    }

    pub fn end_sticky(&mut self) {
        self.sticky = false;
        self.scissor = true;
        self.painter.scissor(
            0,
            self.y as i32,
            self.window_w as i32,
            (self.window_h - self.y) as i32,
        );
        self.window_header_h += self.y - self.window_header_h;

        if let Some(ref current_window) = self.current_window {
            match current_window.props.read() {
                Ok(props) => {
                    self.y += props.scroll_offset;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }
    }

    fn end_input(&mut self) {
        self.is_key_pressed = false;
        self.input_started = false;
        self.input_started_r = false;
        self.input_released = false;
        self.input_released_r = false;
        self.input_dx = 0.0;
        self.input_dy = 0.0;
        self.input_wheel_delta = 0;

        if self.key_repeat
            && self.is_key_down
            && self.scheduler_time() - self.key_repeat_time > 0.05
        {
            self.key.map(|code| {
                if code == VirtualKeyCode::Back
                    || code == VirtualKeyCode::Delete
                    || code == VirtualKeyCode::Left
                    || code == VirtualKeyCode::Right
                    || code == VirtualKeyCode::Up
                    || code == VirtualKeyCode::Down
                {
                    self.key_repeat_time = self.scheduler_time();
                    self.is_key_pressed = true;
                }
            });
        }

        if self.touch_controls
            && self.input_down
            && self.input_x == self.input_started_x
            && self.input_y == self.input_started_y
            && self.input_started_time > 0.0
            && self.scheduler_time() - self.input_started_time > 0.5
        {
            self.touch_hold = true;
            self.input_started_time = 0.0;
        }
    }

    fn input_changed(&self) -> bool {
        self.input_dx != 0.0
            || self.input_dy != 0.0
            || self.input_wheel_delta != 0
            || self.input_started
            || self.input_started_r
            || self.input_released
            || self.input_released_r
            || self.input_down
            || self.input_down_r
            || self.is_key_pressed
    }

    pub fn window_dirty(&self, handle: &Handle, x: i32, y: i32, w: i32, h: i32) -> bool {
        let (wx, wy) = match handle.props.read() {
            Ok(props) => (x + props.drag_x, y + props.drag_y),
            Err(e) => panic!("RwLock poisoned"),
        };

        let input_changed = self.input_in_rect(wx as f32, wy as f32, w as f32, h as f32, 1.0)
            && self.input_changed();
        self.always_redraw || self.is_scrolling || self.is_typing || input_changed
    }

    // Returns true if redraw is needed
    pub fn window(
        &mut self,
        handle: &Handle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        drag: bool, /*= false*/
    ) -> bool {
        let mut w = w;

        // if handle.texture.is_none() || w != handle.texture.width || h != handle.texture.height {
        //  // FIXME: Here we redirect whole into FB
        // 	self.resize(handle, w, h, ops.khaWindowId);
        // }

        if !self.window_ended {
            self.end_window(true); // End previous window if necessary
        }

        self.window_ended = false;

        // i think it normal to clone handle, coz we clone only id: u64 and Arc)))
        // but i think we need to made some research to use references here with lifetimes
        self.current_window = Some(handle.clone());

        match handle.props.read() {
            Ok(props) => {
                self.window_x = (x + props.drag_x) as f32;
                self.window_y = (y + props.drag_y) as f32;
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        self.window_w = w as f32;
        self.window_h = h as f32;
        self.window_header_w = 0.0;
        self.window_header_h = 0.0;

        {
            // just fill window bg
            self.painter.set_color(self.theme.window_bg_col); // Bg

            // Here we use push/pop transform for translation.
            // Originally the window is baked to texture and then shown.
            // Maybe it handy to setup some option for Ui to use one of that ways
            // which more usable for user
            self.painter.push_translation(x as f32, y as f32);

            self.painter
                .fill_rect(0.0, 0.0, self.window_w, self.window_h);
        }

        if self.window_dirty(&handle, x, y, w, h) {
            match handle.props.write() {
                Ok(mut props) => {
                    props.redraws = 2;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        if let Some(ref on_border_hover) = self.on_border_hover {
            if self.input_in_rect(self.window_x - 4.0, self.window_y, 8.0, self.window_h, 1.0) {
                on_border_hover(&handle, 0);
            } else if self.input_in_rect(
                self.window_x + self.window_w - 4.0,
                self.window_y,
                8.0,
                self.window_h,
                1.0,
            ) {
                on_border_hover(&handle, 1);
            } else if self.input_in_rect(
                self.window_x,
                self.window_y - 4.0,
                self.window_w,
                8.0,
                1.0,
            ) {
                on_border_hover(&handle, 2);
            } else if self.input_in_rect(
                self.window_x,
                self.window_y + self.window_h - 4.0,
                self.window_w,
                8.0,
                1.0,
            ) {
                on_border_hover(&handle, 3);
            }
        }

        match handle.props.read() {
            Ok(props) => {
                self.x = 0.0;
                self.y = props.scroll_offset;

                if props.layout == Layout::Horizontal {
                    w = self.element_w() as i32;
                }

                self.w = if !props.scroll_enabled {
                    w as f32
                } else {
                    (w - self.scroll_w()) as f32 // Exclude scrollbar if present
                };

                self.h = h as f32;
                self.tooltip_text = "".into();
                self.tooltip_img = None;
                self.tab_names.clear();

                if self.theme.fill_window_bg {
                    self.painter.begin(true, Some(self.theme.window_bg_col));
                } else {
                    self.painter.begin(true, Some(color::BLACK)); // FIXME: or none?
                    self.painter.set_color(self.theme.window_bg_col);
                    self.painter.fill_rect(
                        self.x,
                        self.y - props.scroll_offset,
                        props.last_max_x,
                        props.last_max_y,
                    );
                }
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        match handle.props.write() {
            Ok(mut props) => {
                props.drag_enabled = drag;
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        if drag {
            if self.input_started
                && self.input_in_rect(
                    self.window_x,
                    self.window_y,
                    self.window_w,
                    self.header_drag_h() as f32,
                    1.0,
                )
            {
                self.drag_handle = Some(handle.clone());
            } else if self.input_released {
                self.drag_handle = None;
            }

            if let Some(drag_handle) = &self.drag_handle {
                if handle == drag_handle {
                    match handle.props.write() {
                        Ok(mut props) => {
                            props.redraws = 2;
                            props.drag_x += self.input_dx as i32;
                            props.drag_y += self.input_dy as i32;
                        }
                        Err(e) => panic!("RwLock poisoned"),
                    }
                }
            }

            self.y += self.header_drag_h() as f32; // Header offset
            self.window_header_h += self.header_drag_h() as f32;
        }

        true
    }

    pub fn end_window(&mut self, bind_global_g: bool /* = true*/) {
        let header_drag_h = self.header_drag_h() as f32;

        if let Some(ref mut handle) = self.current_window {
            // FIXME: double muttable starts here
            match handle.props.write() {
                Ok(mut props) => {
                    if props.redraws > 0 || self.is_scrolling || self.is_typing {
                        if self.scissor {
                            self.scissor = false;
                            self.painter.disable_scissor();
                        }

                        // FIXME: double muttable
                        // if !self.tab_names.is_empty() {
                        //     self.draw_tabs();
                        // }

                        if props.drag_enabled {
                            // Draggable header
                            self.painter.set_color(self.theme.separator_col);
                            self.painter
                                .fill_rect(0.0, 0.0, self.window_w, header_drag_h);
                        }

                        let wh = self.window_h - self.window_header_h; // Exclude header
                        let fullHeight = self.y - props.scroll_offset - self.window_header_h;

                        if fullHeight < wh
                            || props.layout == Layout::Horizontal
                            || !self.scroll_enabled
                        {
                            // Disable scrollbar
                            props.scroll_enabled = false;
                            props.scroll_offset = 0.0;
                        } else {
                            // Draw window scrollbar if necessary
                            props.scroll_enabled = true;
                            if self.tab_scroll < 0.0 {
                                // Restore tab
                                props.scroll_offset = self.tab_scroll;
                                self.tab_scroll = 0.0;
                            }
                            let wy = self.window_y + self.window_header_h;
                            let amountToScroll = fullHeight - wh;
                            let amountScrolled = -props.scroll_offset;
                            let ratio = amountScrolled / amountToScroll;
                            let barH = wh * (wh / fullHeight).abs();

                            // FIXME: double muttable
                            // barH = barH.max(self.element_h());

                            let totalScrollableArea = wh - barH;
                            let e = amountToScroll / totalScrollableArea;
                            let barY = totalScrollableArea * ratio + self.window_header_h;

                            // FIXME: double muttable
                            // let barFocus = self.get_input_in_rect(
                            //     self.window_x + self.window_w - self.scroll_w() as f32,
                            //     barY + self.window_y,
                            //     self.scroll_w() as f32,
                            //     barH,
                            //     1.0,
                            // );

                            // if self.input_started && barFocus {
                            //     // Start scrolling
                            //     self.scroll_handle = Some(handle.clone());
                            //     self.is_scrolling = true;
                            // }

                            let mut scrollDelta: f32 = self.input_wheel_delta as f32;

                            if self.touch_controls && self.input_down && self.input_dy != 0.0 {
                                self.is_scrolling = true;
                                scrollDelta = -self.input_dy / 20.0;
                            }

                            if let Some(scroll_handle) = &self.scroll_handle {
                                // FIXME: double muttable
                                // if handle == scroll_handle {
                                //     // Scroll
                                //     self.scroll(self.input_dy * e, fullHeight);
                                // } else if scrollDelta != 0.0
                                //     && self.combo_selected_handle.is_some()
                                //     && self.get_input_in_rect(self.window_x, wy, self.window_w, wh, 1.0)
                                // {
                                //     // Wheel
                                //     self.scroll(scrollDelta * self.element_h(), fullHeight);
                                // }
                            }

                            // Stay in bounds
                            if props.scroll_offset > 0.0 {
                                props.scroll_offset = 0.0;
                            } else if fullHeight + props.scroll_offset < wh {
                                props.scroll_offset = wh - fullHeight;
                            }

                            self.painter.set_color(self.theme.window_bg_col); // Bg

                            // FIXME: double muttable
                            // self.painter.fill_rect(
                            //     self.window_w - self.scroll_w() as f32,
                            //     wy,
                            //     self.scroll_w() as f32,
                            //     wh,
                            // );

                            // self.painter.set_color(self.theme.accent_col); // Bar
                            // let scrollbarFocus = self.get_input_in_rect(
                            //     self.window_x + self.window_w - self.scroll_w() as f32,
                            //     wy,
                            //     self.scroll_w() as f32,
                            //     wh,
                            //     1.0,
                            // );

                            // if let Some(scroll_handle) = &self.scroll_handle {
                            //     let barW = if scrollbarFocus || handle == scroll_handle {
                            //         self.scroll_w() as f32
                            //     } else {
                            //         self.scroll_w() as f32  / 3.0
                            //     };

                            //     self.painter.fill_rect(
                            //         self.window_w - barW - self.scroll_align,
                            //         barY,
                            //         barW,
                            //         barH,
                            //     );
                            // }
                        }

                        props.last_max_x = self.x;
                        props.last_max_y = self.y;
                        if props.layout == Layout::Vertical {
                            props.last_max_x += self.window_w;
                        } else {
                            props.last_max_y += self.window_h;
                        }

                        props.redraws -= 1;

                        self.painter.end();
                    }

                    self.painter.pop_transformation(); // DV remove previous window transformation matrix
                    self.window_ended = true;

                    // Draw window texture
                    if self.always_redraw_window || props.redraws > -4 {
                        // FIXME: here we put window texture ((( WHY we need it?
                        // if bindGlobalG {
                        //     global_g.begin(false);
                        // }
                        // global_g.color = self.theme.window_tint_col;
                        // global_g.drawImage(props.texture, self.window_x, self.window_y);
                        // if bindGlobalG {
                        //     global_g.end();
                        // }

                        if props.redraws <= 0 {
                            props.redraws -= 1;
                        }
                    }
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        if self.window_ended {
            // DV
            self.current_window = None;
        }
    }

    fn scroll(&mut self, delta: f32, full_height: f32) {
        if let Some(ref mut current_window) = self.current_window {
            match current_window.props.write() {
                Ok(mut props) => {
                    props.scroll_offset -= delta;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }
    }

    pub fn tab(
        &mut self,
        handle: &Handle,
        text: &str,
        vertical: bool,       /*= false*/
        color: Option<Color>, /* = -1*/
    ) -> bool {
        if self.tab_names.len() == 0 {
            // First tab
            self.tab_handle = Some(handle.clone());
            self.tab_vertical = vertical;
            self.w -= if vertical {
                self.element_offset() + self.element_w() - 1.0 * self.scale()
            } else {
                0.0 // Shrink window area by width of vertical tabs
            };

            if vertical {
                self.window_header_w += self.element_w();
            } else {
                self.window_header_h +=
                    self.button_h() + self.button_offset_y + self.element_offset();
            }

            self.restore_x = self.input_x; // Mouse in tab header, disable clicks for tab content
            self.restore_y = self.input_y;
            if !vertical
                && self.input_in_rect(
                    self.window_x,
                    self.window_y,
                    self.window_w,
                    self.window_header_h,
                    1.0,
                )
            {
                self.input_x = -1.0;
                self.input_y = -1.0;
            }
            if vertical {
                self.x += self.window_header_w + 6.0;
                self.w -= 6.0;
            } else {
                self.y += self.window_header_h + 3.0;
            }
        }
        self.tab_names.push(text.into());
        self.tab_colors.push(color);

        match handle.props.read() {
            Ok(props) => props.position == self.tab_names.len() as i32 - 1,
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    fn draw_tabs(&mut self) {
        self.input_x = self.restore_x;
        self.input_y = self.restore_y;

        let mut tabX = 0.0;
        let mut tabY = 0.0;
        let tabHMin = self.button_h() * 1.1;

        let (header_h, current_window_scroll_offset) =
            if let Some(ref current_window) = self.current_window {
                match current_window.props.read() {
                    Ok(props) => {
                        let header_h = if props.drag_enabled {
                            self.header_drag_h()
                        } else {
                            0
                        };

                        (header_h, props.scroll_offset)
                    }
                    Err(e) => panic!("RwLock poisoned"),
                }
            } else {
                (0, 0.0)
            };

        let tabH = if self.theme.full_tabs && self.tab_vertical {
            (self.window_h - header_h as f32) / self.tab_names.len() as f32
        } else {
            tabHMin
        };

        let origy = self.y;
        self.y = header_h as f32;

        let mut tab_handle_position = 0;
        let mut tab_handle_changed = false;

        let released = self.released(tabH);
        let pushed = self.pushed(tabH);
        let hover = self.hover(tabH);

        let element_w = self.element_w();
        let scale_factor = self.scale();

        if let Some(ref mut tab_handle) = self.tab_handle {
            match tab_handle.props.write() {
                Ok(mut props) => {
                    props.changed = false;

                    // remember the position
                    tab_handle_position = props.position;

                    let tab_names_len = self.tab_names.len() as i32;

                    if self.is_ctrl_down && self.is_tab_down {
                        // Next tab
                        props.position += 1;
                        if props.position >= tab_names_len as i32 {
                            props.position = 0;
                        }
                        props.changed = true;
                        self.is_tab_down = false;
                    }

                    if props.position >= tab_names_len {
                        props.position = tab_names_len - 1;
                    }
                }
                Err(e) => panic!("RwLock poisoned"),
            }

            self.painter.set_color(self.theme.separator_col); // Tab background

            if self.tab_vertical {
                self.painter
                    .fill_rect(0.0, self.y, element_w, self.window_h);
            } else {
                self.painter.fill_rect(
                    0.0,
                    self.y,
                    self.window_w,
                    self.button_offset_y + tabH + 2.0,
                );
            }

            self.painter.set_color(self.theme.accent_col); // Underline tab buttons

            if self.tab_vertical {
                self.painter
                    .fill_rect(element_w, self.y, 1.0, self.window_h);
            } else {
                self.painter.fill_rect(
                    self.button_offset_y,
                    self.y + self.button_offset_y + tabH + 2.0,
                    self.window_w - self.button_offset_y * 2.0,
                    1.0,
                );
            }
        }

        let basey = if self.tab_vertical {
            self.y
        } else {
            self.y + 2.0
        };

        let mut current_window_redraws = None;

        for idx in 0..self.tab_names.len() {
            self.x = tabX;
            self.y = basey + tabY;
            self.w = if self.tab_vertical {
                element_w - 1.0 * scale_factor
            } else {
                if self.theme.full_tabs {
                    self.window_w / self.tab_names.len() as f32
                } else {
                    if let Some((width, _)) = self.painter.measure(self.tab_names[idx].as_str()) {
                        width + self.button_offset_y * 2.0 + 18.0 * scale_factor
                    } else {
                        self.window_w / self.tab_names.len() as f32
                    }
                }
            };

            if released {
                if let Some(ref mut tab_handle) = self.tab_handle {
                    let handle = tab_handle.nest(tab_handle_position as u64, None); // Restore tab scroll

                    match handle.props.write() {
                        Ok(mut props) => {
                            props.scroll_offset = current_window_scroll_offset;

                            // FIXME: SOME MAGIC HERE, URGENTLY DEAL WITH IT
                            // handle = tab_handle.nest(idx as u64, None);
                            // self.tab_scroll = handle.scroll_offset;

                            self.tab_scroll = props.scroll_offset; // DV
                        }
                        Err(e) => panic!("RwLock poisoned"),
                    }

                    tab_handle_position = idx as i32; // Set new tab
                    current_window_redraws = Some(3);
                    tab_handle_changed = true;
                }
            }

            let selected = tab_handle_position == idx as i32;

            {
                let color = if pushed || hover {
                    self.theme.button_hover_col
                } else {
                    // TODO: should check the bounds
                    if self.tab_colors[idx].is_some() {
                        self.tab_colors[idx].unwrap()
                    } else {
                        if selected {
                            self.theme.window_bg_col
                        } else {
                            self.theme.separator_col
                        }
                    }
                };
                self.painter.set_color(color);
            }

            if self.tab_vertical {
                tabY += tabH + 1.0;
            } else {
                tabX += self.w + 1.0;
            }

            self.draw_rect(
                true,
                self.x + self.button_offset_y,
                self.y + self.button_offset_y,
                self.w,
                tabH,
                0.0,
            );

            self.painter.set_color(if selected {
                self.theme.button_text_col
            } else {
                self.theme.label_col
            });

            self.draw_string(
                self.tab_names[idx].as_str(),
                None,
                (tabH - tabHMin) / 2.0,
                if self.theme.full_tabs {
                    Align::Center
                } else {
                    Align::Left
                },
                true,
            );

            if selected && !self.tab_vertical {
                // Hide underline for active tab
                self.painter.set_color(self.theme.window_bg_col);
                self.painter.fill_rect(
                    self.x + self.button_offset_y + 1.0,
                    self.y + self.button_offset_y + tabH,
                    self.w - 1.0,
                    1.0,
                );
            }
        }

        // update tab_handle
        if tab_handle_changed {
            if let Some(ref mut tab_handle) = self.tab_handle {
                match tab_handle.props.write() {
                    Ok(mut props) => {
                        props.changed = tab_handle_changed;
                        props.position = tab_handle_position;
                    }
                    Err(e) => panic!("RwLock poisoned"),
                }
            }
        }

        self.x = 0.0; // Restore positions
        self.y = origy;

        if let Some(current_window) = &self.current_window {
            match current_window.props.read() {
                Ok(props) => {
                    self.w = if !props.scroll_enabled {
                        self.window_w
                    } else {
                        self.window_w - self.scroll_w() as f32
                    }
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }
    }

    pub fn panel(
        &mut self,
        handle: &Handle,
        text: &str,
        is_tree: bool, /* = false*/
        filled: bool,  /* = true*/
        pack: bool,    /*= true*/
    ) -> bool {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);

            match handle.props.read() {
                Ok(props) => return props.selected,
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        if self.released(-1.0) {
            match handle.props.write() {
                Ok(mut props) => {
                    props.selected = !props.selected;
                    props.changed = true;
                    self.changed = true;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        if filled {
            self.painter.set_color(self.theme.panel_bg_col);
            self.draw_rect(true, self.x, self.y, self.w as f32, self.element_h(), 0.0);
        }

        match handle.props.read() {
            Ok(props) => {
                if is_tree {
                    self.draw_tree(props.selected);
                } else {
                    self.draw_arrow(props.selected);
                }

                self.painter.set_color(self.theme.label_col); // Title
                self.painter.set_opacity(1.0);
                self.draw_string(
                    text.into(),
                    Some(self.title_offset_x),
                    0.0,
                    Align::Left,
                    true,
                );

                self.end_element(None);
                if pack && !props.selected {
                    self.y -= self.element_offset();
                }

                props.selected
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn image(
        &mut self,
        image: &Image,
        tint: Color,    /* = 0xffffffff*/
        h: Option<f32>, /* = None*/
        sx: i32,        /* = 0*/
        sy: i32,        /* = 0*/
        sw: i32,        /* = 0*/
        sh: i32,        /*= 0*/
    ) -> State {
        let iw = (if sw > 0 {
            sw as f32
        } else {
            image.width as f32
        }) * self.scale();
        let ih = (if sh > 0 {
            sh as f32
        } else {
            image.height as f32
        }) * self.scale();

        let mut w = iw.min(self.w as f32);
        let mut x = self.x;

        let scroll = match self.current_window {
            Some(ref current_window) => match current_window.props.read() {
                Ok(props) => props.scroll_enabled,
                Err(e) => panic!("RwLock poisoned"),
            },
            None => false,
        };

        let r = if self.cur_ratio == -1 {
            1.0
        } else {
            self.ratio(self.ratios[self.cur_ratio as usize], 1.0)
        };

        if self.image_scroll_align {
            // Account for scrollbar size
            w = iw.min(self.w as f32 - self.button_offset_y * 2.0);
            x += self.button_offset_y;
            if !scroll {
                w -= self.scroll_w() as f32 * r;
                x += self.scroll_w() as f32 * r / 2.0;
            }
        } else if scroll {
            w += self.scroll_w() as f32 * r;
        }

        // Image size
        let ratio = if let Some(h) = h { h / ih } else { w / iw };

        let h = match h {
            Some(h) => {
                w = iw * ratio;
                h
            }
            None => ih * ratio,
        };

        if !self.is_visible(h) {
            self.end_element(Some(h));
            return State::Idle;
        }

        let mut started = self.started(h);
        let mut down = self.pushed(h);
        let mut released = self.released(h);
        let mut hover = self.hover(h);

        if self.cur_ratio == -1 && (started || down || released || hover) {
            if self.input_x < self.window_x + self.x || self.input_x > self.window_x + self.x + w {
                started = false;
                down = false;
                released = false;
                hover = false;
            }
        }

        self.painter.set_color(tint);

        if !self.enabled {
            self.fade_color();
        }

        let h_float: f32 = h; // TODO: hashlink fix
        if sw > 0 {
            // Source rect specified
            if self.image_invert_y {
                self.painter.draw_scaled_subimage(
                    image,
                    sx as f32,
                    sy as f32,
                    sw as f32,
                    sh as f32,
                    x,
                    self.y + h_float,
                    w,
                    -h_float,
                )
            } else {
                self.painter.draw_scaled_subimage(
                    image, sx as f32, sy as f32, sw as f32, sh as f32, x, self.y, w, h_float,
                );
            }
        } else {
            if self.image_invert_y {
                self.painter
                    .draw_scaled_image(image, x, self.y + h_float, w, -h_float)
            } else {
                self.painter.draw_scaled_image(image, x, self.y, w, h_float);
            }
        }

        self.end_element(Some(h));
        if started {
            State::Started
        } else {
            if released {
                State::Released
            } else {
                if down {
                    State::Down
                } else {
                    if hover {
                        State::Hovered
                    } else {
                        State::Idle
                    }
                }
            }
        }
    }

    pub fn text(&mut self, text: &str, align: Align, bg: Option<Color>) -> State {
        if text.contains("\n") {
            self.split_text(text, align, bg);
            return State::Idle;
        }

        let h = self.element_h().max(self.ops.font.height(self.font_size));

        if !self.is_visible(h) {
            self.end_element(Some(h + self.element_offset()));
            return State::Idle;
        }

        let started = self.started(h);
        let down = self.pushed(h);
        let released = self.released(h);
        let hover = self.hover(h);

        if let Some(bg) = bg {
            self.painter.set_color(bg);
            self.painter.fill_rect(
                self.x + self.button_offset_y,
                self.y + self.button_offset_y,
                self.w as f32 - self.button_offset_y * 2.0,
                self.button_h(),
            );
        }

        self.painter.set_color(self.theme.text_col);
        self.draw_string(text, None, 0.0, align, true);

        self.end_element(Some(h + self.element_offset()));

        if started {
            State::Started
        } else {
            if released {
                State::Released
            } else {
                if down {
                    State::Down
                } else {
                    State::Idle
                }
            }
        }
    }

    fn split_text(&mut self, lines: &str, align: Align, bg: Option<Color>) {
        for line in lines.split("\n") {
            self.text(line, align, bg);
        }
    }

    fn start_text_edit(&mut self, handle: &Handle) {
        self.is_typing = true;
        self.submit_text_handle = self.text_selected_handle.clone();
        self.text_to_submit = self.text_selected.clone();

        self.text_selected_handle = Some(handle.clone());

        match handle.props.read() {
            Ok(props) => {
                self.text_selected = props.text.clone();
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        self.cursor_x = self.text_selected.len();

        if self.tab_pressed {
            self.tab_pressed = false;
            self.is_key_pressed = false; // Prevent text deselect after tab press
        } else if !self.highlight_on_select {
            // Set cursor to click location
            let x = self.input_x - (self.window_x + self.x + self.text_offset());
            self.cursor_x = 0;

            while self.cursor_x < self.text_selected.len() && {
                // just measure the substring
                let substr = &self.text_selected[..self.cursor_x as usize];
                self.painter
                    .measure(substr)
                    .map(|(width, _)| width < x)
                    .unwrap_or_default()
            } {
                self.cursor_x += 1;
            }
        }

        self.tab_pressed_handle = Some(handle.clone());
        self.highlight_anchor = if self.highlight_on_select {
            0
        } else {
            self.cursor_x
        };

        // if Keyboard.get() != None {
        //     Keyboard.get().show();
        // }
    }

    fn submit_text_edit(&mut self) {
        if let Some(ref mut submit_text_handle) = self.submit_text_handle {
            match submit_text_handle.props.write() {
                Ok(mut props) => {
                    props.text = self.text_to_submit.clone();
                    props.changed = true;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        self.changed = true;
        self.submit_text_handle = None;
        self.text_to_submit = "".into();
        self.text_selected = "".into();
    }

    fn update_text_edit(
        &mut self,
        align: Align,   /* = Align::Left*/
        editable: bool, /* = true*/
    ) {
        let mut text = self.text_selected.clone();
        if self.is_key_pressed {
            // Process input
            if let Some(key) = self.key {
                match key {
                    VirtualKeyCode::Left => {
                        // Move cursor
                        if self.cursor_x > 0 {
                            self.cursor_x -= 1;
                        }
                    }
                    VirtualKeyCode::Right => {
                        if self.cursor_x < text.len() {
                            self.cursor_x += 1;
                        }
                    }
                    VirtualKeyCode::Back => {
                        if editable {
                            // Remove char
                            if self.cursor_x > 0 && self.highlight_anchor == self.cursor_x {
                                // text = text.substr(0, self.cursor_x - 1)
                                //     + text.substr(self.cursor_x, text.len());
                                self.cursor_x -= 1;
                            } else if self.highlight_anchor < self.cursor_x {
                                // text = text.substr(0, self.highlight_anchor)
                                //     + text.substr(self.cursor_x, text.len());
                                self.cursor_x = self.highlight_anchor;
                            } else {
                                // text = text.substr(0, self.cursor_x)
                                //     + text.substr(self.highlight_anchor, text.len());
                            }
                        }
                    }
                    VirtualKeyCode::Delete => {
                        if self.highlight_anchor == self.cursor_x {
                            // text = text.substr(0, self.cursor_x) + text.substr(self.cursor_x + 1);
                        } else if self.highlight_anchor < self.cursor_x {
                            // text = text.substr(0, self.highlight_anchor)
                            //     + text.substr(self.cursor_x, text.len());
                            self.cursor_x = self.highlight_anchor;
                        } else {
                            // text = text.substr(0, self.cursor_x)
                            //     + text.substr(self.highlight_anchor, text.len());
                        }
                    }
                    VirtualKeyCode::Return => {
                        // Deselect
                        self.deselect_text();
                    }
                    VirtualKeyCode::Escape => {
                        // Cancel
                        if let Some(text_selected_handle) = &self.text_selected_handle {
                            match text_selected_handle.props.read() {
                                Ok(props) => self.text_selected = props.text.clone(),
                                Err(e) => panic!("RwLock poisoned"),
                            }
                        }

                        self.deselect_text();
                    }
                    VirtualKeyCode::Tab => {
                        if self.tab_switch_enabled && !self.is_ctrl_down {
                            // Next field
                            self.tab_pressed = true;
                            self.deselect_text();
                            self.key = None;
                        }
                    }
                    VirtualKeyCode::Home => {
                        self.cursor_x = 0;
                    }
                    VirtualKeyCode::End => {
                        self.cursor_x = text.len();
                    }
                    _ => {
                        if self.is_ctrl_down && self.is_adown {
                            // Select all
                            self.cursor_x = text.len();
                            self.highlight_anchor = 0;
                        } else if editable && // Write
                            // key != VirtualKeyCode::Shift &&
                            // key != VirtualKeyCode::CapsLock &&
                            // key != VirtualKeyCode::Control &&
                            // key != VirtualKeyCode::Meta &&
                            // key != VirtualKeyCode::Alt &&
                            key != VirtualKeyCode::Up &&
                            key != VirtualKeyCode::Down
                        {
                            // IF
                            // self.char != None &&
                            // self.char != "" &&
                            // self.char.charCodeAt(0) >= 32

                            // text = text.substr(0, self.highlight_anchor)
                            //     + self.char
                            //     + text.substr(self.cursor_x);
                            self.cursor_x = if self.cursor_x + 1 > text.len() {
                                text.len()
                            } else {
                                self.cursor_x + 1
                            };

                            // if self.dynamic_glyph_load && self.char.charCodeAt(0) > 126 && Graphics.fontGlyphs.indexOf(self.char.charCodeAt(0)) == -1 {
                            // 	Graphics.fontGlyphs.push(self.char.charCodeAt(0));
                            // 	Graphics.fontGlyphs = Graphics.fontGlyphs.copy(); // Trigger atlas update
                            // }
                        }
                    }
                }
                let selecting = self.is_shift_down
                    && (key == VirtualKeyCode::Left || key == VirtualKeyCode::Right/*|| key == VirtualKeyCode::Shift*/);

                if !selecting && !self.is_ctrl_down {
                    self.highlight_anchor = self.cursor_x;
                }
            }
        }

        if self.text_to_paste != "" {
            // Process cut copy paste

            // text = text.substr(0, self.highlight_anchor)
            //     + self.text_to_paste
            //     + text.substr(self.cursor_x);

            self.cursor_x += self.text_to_paste.len();
            self.highlight_anchor = self.cursor_x;
            self.text_to_paste.clear();
            self.is_paste = false;
        }

        if self.highlight_anchor == self.cursor_x {
            self.text_to_copy = text.clone(); // Copy
        } else if self.highlight_anchor < self.cursor_x {
            self.text_to_copy = text[self.highlight_anchor..self.cursor_x].into();
        } else {
            self.text_to_copy = text[self.cursor_x..self.highlight_anchor].into();
        }

        if self.is_cut {
            // Cut
            if self.highlight_anchor == self.cursor_x {
                text.clear();
            } else if self.highlight_anchor < self.cursor_x {
                // text =
                //     text.substr(0, self.highlight_anchor) + text.substr(self.cursor_x, text.len());
                self.cursor_x = self.highlight_anchor;
            } else {
                // text =
                //     text.substr(0, self.cursor_x) + text.substr(self.highlight_anchor, text.len());
            }
        }

        let off = self.text_offset();
        let lineHeight = self.element_h();
        let cursorHeight = lineHeight - self.button_offset_y * 3.0;

        // Draw highlight
        if self.highlight_anchor != self.cursor_x {
            let mut istart = self.cursor_x;
            let mut iend = self.highlight_anchor;

            if self.highlight_anchor < self.cursor_x {
                istart = self.highlight_anchor;
                iend = self.cursor_x;
            }

            let hlstr = &text[istart..iend - istart];

            let hlstrw = self
                .painter
                .measure(hlstr)
                .map(|(width, _)| width)
                .unwrap_or_default();

            let startoff = self
                .painter
                .measure(&text[..istart])
                .map(|(width, _)| width)
                .unwrap_or_default();

            let mut hlStart = if align == Align::Left {
                self.x + startoff + off
            } else {
                self.x + self.w - hlstrw - off
            };

            if align == Align::Right {
                hlStart -= self
                    .painter
                    .measure(&text[iend..])
                    .map(|(width, _)| width)
                    .unwrap_or_default();
            }

            self.painter.set_color(self.theme.accent_select_col);
            self.painter.fill_rect(
                hlStart,
                self.y + self.button_offset_y * 1.5,
                hlstrw,
                cursorHeight,
            );
        }

        // Flash cursor
        let time = self.scheduler_time();
        if self.is_key_down || time % (self.flash_speed() * 2.0) < self.flash_speed() {
            let str = if align == Align::Left {
                &text[..self.cursor_x]
            } else {
                &text[self.cursor_x..text.len()]
            };

            let strw = self
                .painter
                .measure(str)
                .map(|(width, _)| width)
                .unwrap_or_default();

            let cursor_x = if align == Align::Left {
                self.x + strw + off
            } else {
                self.x + self.w - strw - off
            };
            self.painter.set_color(self.theme.text_col); // Cursor
            self.painter.fill_rect(
                cursor_x,
                self.y + self.button_offset_y * 1.5,
                1.0 * self.scale(),
                cursorHeight,
            );
        }

        self.text_selected = text;
    }

    pub fn text_input(
        &mut self,
        handle: &Handle,
        label: &str,    /*= ""*/
        align: Align,   /*= Align::Left*/
        editable: bool, /*= true*/
    ) -> String {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);

            match handle.props.read() {
                Ok(props) => return props.text.clone(),
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        let hover = self.hover(-1.0);
        if hover {
            if let Some(ref on_text_hover) = self.on_text_hover {
                on_text_hover();
            }
        }

        // Text bg
        self.painter.set_color(if hover {
            self.theme.accent_hover_col
        } else {
            self.theme.accent_col
        });

        self.draw_rect(
            self.theme.fill_accent_bg,
            self.x + self.button_offset_y,
            self.y + self.button_offset_y,
            self.w as f32 - self.button_offset_y * 2.0,
            self.button_h(),
            0.0,
        );

        let startEdit = self.released(-1.0) || self.tab_pressed;

        let is_text_selected_handle = match &self.text_selected_handle {
            Some(text_selected_handle) => text_selected_handle == handle,
            None => false,
        };

        if !is_text_selected_handle && startEdit {
            self.start_text_edit(handle);
        }

        if is_text_selected_handle {
            self.update_text_edit(align, editable);
        }

        if self
            .submit_text_handle
            .as_ref()
            .map(|h| h == handle)
            .unwrap_or_default()
        {
            self.submit_text_edit();
        } else {
            match handle.props.write() {
                Ok(mut props) => {
                    props.changed = false;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        if !label.is_empty() {
            self.painter.set_color(self.theme.label_col); // Label
            let labelAlign = if align == Align::Right {
                Align::Left
            } else {
                Align::Right
            };

            self.draw_string(
                label,
                if labelAlign == Align::Left {
                    None
                } else {
                    Some(0.0)
                },
                0.0,
                labelAlign,
                true,
            );
        }

        // Text
        self.painter.set_color(self.theme.text_col);

        if self
            .text_selected_handle
            .as_ref()
            .map(|h| h != handle)
            .unwrap_or_default()
        {
            match handle.props.read() {
                Ok(props) => self.draw_string(props.text.as_str(), None, 0.0, align, true),
                Err(e) => panic!("RwLock poisoned"),
            }
        } else {
            self.draw_string(self.text_selected.as_str(), None, 0.0, align, false);
        }

        self.end_element(None);

        match handle.props.read() {
            Ok(props) => props.text.clone(),
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    fn deselect_text(&mut self) {
        if self.text_selected_handle.is_none() {
            return;
        }

        self.submit_text_handle = self.text_selected_handle.clone();
        self.text_to_submit = self.text_selected.clone();
        self.text_selected_handle = None;
        self.is_typing = false;

        if let Some(ref mut current_window) = self.current_window {
            match current_window.props.write() {
                Ok(mut props) => {
                    props.redraws = 2;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        // if Keyboard.get() != None {
        //     Keyboard.get().hide();
        // }

        self.highlight_anchor = self.cursor_x;
    }

    pub fn button(
        &mut self,
        text: &str,
        align: Align, /*= Align::Center*/
        label: &str,  /* = ""*/
    ) -> bool {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);
            return false;
        }

        let released = self.released(-1.0);
        let pushed = self.pushed(-1.0);
        let hover = self.hover(-1.0);

        if released {
            self.changed = true;
        }

        self.painter.set_color(if pushed {
            self.theme.button_pressed_col
        } else {
            if hover {
                self.theme.button_hover_col
            } else {
                self.theme.button_col
            }
        });

        self.draw_rect(
            self.theme.fill_button_bg,
            self.x + self.button_offset_y,
            self.y + self.button_offset_y,
            self.w as f32 - self.button_offset_y * 2.0,
            self.button_h(),
            0.0,
        );

        self.painter.set_color(self.theme.button_text_col);
        self.draw_string(text, None, 0.0, align, true);

        if !label.is_empty() {
            self.painter.set_color(self.theme.label_col);
            self.draw_string(
                label,
                None,
                0.0,
                if align == Align::Right {
                    Align::Left
                } else {
                    Align::Right
                },
                true,
            );
        }

        self.end_element(None);

        released
    }

    pub fn check(&mut self, handle: &Handle, text: &str) -> bool {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);

            match handle.props.read() {
                Ok(props) => {
                    return props.selected;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        match handle.props.write() {
            Ok(mut props) => {
                if self.released(-1.0) {
                    props.selected = !props.selected;
                    props.changed = true;
                    self.changed = true;
                } else {
                    props.changed = false;
                }

                let hover = self.hover(-1.0);

                // Check
                self.draw_check(props.selected, hover);

                // Text
                self.painter.set_color(self.theme.text_col);
                self.draw_string(text, Some(self.title_offset_x), 0.0, Align::Left, true);

                self.end_element(None);

                props.selected
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn radio(&mut self, handle: &Handle, position: i32, text: &str) -> bool {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);

            match handle.props.read() {
                Ok(props) => {
                    return props.position == position;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        match handle.props.write() {
            Ok(mut props) => {
                if position == 0 {
                    props.changed = false;
                }

                if self.released(-1.0) {
                    props.position = position;
                    props.changed = true;
                    self.changed = true;
                }

                let hover = self.hover(-1.0);

                // Radio
                self.draw_radio(props.position == position, hover);

                // Text
                self.painter.set_color(self.theme.text_col);
                self.draw_string(text, Some(self.title_offset_x), 0.0, Align::Left, true);

                self.end_element(None);

                props.position == position
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn combo(
        &mut self,
        handle: &Handle,
        texts: Vec<String>,
        label: &str,      /*= ""*/
        show_label: bool, /*= false*/
        align: Align,     /*= Align::Left*/
    ) -> i32 {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);

            match handle.props.read() {
                Ok(props) => {
                    return props.position;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        if self.released(-1.0) {
            if self.combo_selected_handle.is_none() {
                self.input_enabled = false;
                self.combo_selected_handle = Some(handle.clone());
                self.combo_selected_window = self.current_window.clone();
                self.combo_selected_align = align;
                self.combo_selected_texts = texts.clone(); // FIXME: OOPS
                self.combo_selected_label = label.into();
                self.combo_selected_x = (self.x + self.window_x) as i32;
                self.combo_selected_y = (self.y + self.window_y + self.element_h()) as i32;
                self.combo_selected_w = self.w as i32;

                // Adapt combo list width to combo item width
                for row in texts.iter() {
                    if let Some((width, _)) = self.painter.measure(row.as_str()) {
                        let w = width as i32 + 10;

                        if self.combo_selected_w < w {
                            self.combo_selected_w = w;
                        }
                    }
                }

                if self.combo_selected_w > (self.w * 2.0) as i32 {
                    self.combo_selected_w = (self.w as f32 * 2.0) as i32;
                }

                if self.combo_selected_w > self.w as i32 {
                    self.combo_selected_w += self.text_offset() as i32;
                }

                match handle.props.read() {
                    Ok(props) => {
                        self.combo_to_submit = props.position;
                    }
                    Err(e) => panic!("RwLock poisoned"),
                }
            }
        }

        if let Some(submit_combo_handle) = &self.submit_combo_handle {
            if handle == submit_combo_handle {
                match handle.props.write() {
                    Ok(mut props) => {
                        props.changed = true;
                        props.position = self.combo_to_submit;
                    }
                    Err(e) => panic!("RwLock poisoned"),
                }
                self.submit_combo_handle = None;
                self.changed = true;
            } else {
                match handle.props.write() {
                    Ok(mut props) => props.changed = false,
                    Err(e) => panic!("RwLock poisoned"),
                }
            }
        }

        let hover = self.hover(-1.0);
        if hover {
            // Bg
            self.painter.set_color(self.theme.accent_hover_col);
            self.draw_rect(
                self.theme.fill_accent_bg,
                self.x + self.button_offset_y,
                self.y + self.button_offset_y,
                self.w as f32 - self.button_offset_y * 2.0,
                self.button_h(),
                0.0,
            );
        } else {
            self.painter.set_color(self.theme.accent_col);
            self.draw_rect(
                self.theme.fill_accent_bg,
                self.x + self.button_offset_y,
                self.y + self.button_offset_y,
                self.w as f32 - self.button_offset_y * 2.0,
                self.button_h(),
                0.0,
            );
        }

        let x = self.x + self.w as f32 - self.arrow_offset_x - 8.0;
        let y = self.y + self.arrow_offset_y + 3.0;
        self.painter.fill_triangle(
            x,
            y,
            x + self.arrow_size(),
            y,
            x + self.arrow_size() / 2.0,
            y + self.arrow_size() / 2.0,
        );

        if show_label && !label.is_empty() {
            if align == Align::Left {
                self.x -= 15.0;
            }
            self.painter.set_color(self.theme.label_col);
            self.draw_string(
                label,
                None,
                0.0,
                if align == Align::Left {
                    Align::Right
                } else {
                    Align::Left
                },
                true,
            );
            if align == Align::Left {
                self.x += 15.0;
            }
        }

        if align == Align::Right {
            self.x -= 15.0;
        }

        // Value
        self.painter.set_color(self.theme.text_col);

        match handle.props.read() {
            Ok(props) => {
                if props.position < texts.len() as i32 {
                    self.draw_string(
                        texts[props.position as usize].as_str(),
                        None,
                        0.0,
                        align,
                        true,
                    );
                }
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        if align == Align::Right {
            self.x += 15.0;
        }

        self.end_element(None);

        match handle.props.read() {
            Ok(props) => props.position,
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn slider(
        &mut self,
        handle: &Handle,
        text: &str,
        from: f32,           /*= 0.0*/
        to: f32,             /*= 1.0*/
        filled: bool,        /*= false*/
        precision: f32,      /* = 100.0*/
        display_value: bool, /*= true*/
        align: Align,        /* = Align::Right*/
        text_edit: bool,     /* = true*/
    ) -> f32 {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);

            match handle.props.read() {
                Ok(props) => {
                    return props.value;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        if self.started(-1.0) {
            self.scroll_handle = Some(handle.clone());
            self.is_scrolling = true;
            if self.touch_controls {
                self.slider_tooltip = true;
                self.slider_tooltip_x = self.x + self.window_x;
                self.slider_tooltip_y = self.y + self.window_y;
                self.slider_tooltip_w = self.w as f32;
            }
        }

        match handle.props.write() {
            Ok(mut props) => {
                props.changed = false;
                if let Some(scroll_handle) = &self.scroll_handle {
                    // Scroll
                    if handle == scroll_handle {
                        let range = to - from;
                        let sliderX = self.x + self.window_x + self.button_offset_y;
                        let sliderW = self.w as f32 - self.button_offset_y * 2.0;
                        let step = range / sliderW;
                        let value = from + (self.input_x - sliderX) * step;
                        props.value = (value * precision).round() / precision;

                        if props.value < from {
                            props.value = from; // Stay in bounds
                        } else if props.value > to {
                            props.value = to;
                        }

                        props.changed = true;
                        self.changed = true;
                    }
                }
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        match handle.props.read() {
            Ok(props) => {
                let hover = self.hover(-1.0);
                self.draw_slider(props.value, from, to, filled, hover); // Slider
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        // Text edit
        let startEdit = (self.released(-1.0) || self.tab_pressed) && text_edit;
        if startEdit {
            // Mouse did not move
            match handle.props.write() {
                Ok(mut props) => {
                    props.text = format!("{}", props.value);
                }
                Err(e) => panic!("RwLock poisoned"),
            }

            self.start_text_edit(&handle);

            match handle.props.write() {
                Ok(mut props) => {
                    props.changed = true;
                    self.changed = true;
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        let lalign = if align == Align::Left {
            Align::Right
        } else {
            Align::Left
        };

        if self
            .text_selected_handle
            .as_ref()
            .map(|h| h == handle)
            .unwrap_or_default()
        {
            self.update_text_edit(lalign, true)
        }

        if self
            .submit_text_handle
            .as_ref()
            .map(|h| h == handle)
            .unwrap_or_default()
        {
            self.submit_text_edit();

            // TODO: when compliled to javascript (but not only),
            // it is possible to eval values
            // something lika `0.1+0.5` should return 0.6
            // at the moment we just parse value and so

            match handle.props.write() {
                Ok(mut props) => match props.text.parse::<f32>() {
                    Ok(value) => {
                        props.value = value;
                        props.changed = true;
                        self.changed = true;
                    }
                    Err(e) => {
                        println!("slider value parse error");
                    }
                },
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        self.painter.set_color(self.theme.label_col); // Text
        self.draw_string(text, None, 0.0, align, true);

        if display_value {
            self.painter.set_color(self.theme.text_col); // Value

            if self
                .text_selected_handle
                .as_ref()
                .map(|h| h != handle)
                .unwrap_or_default()
            {
                self.draw_string(self.text_selected.as_str(), None, 0.0, lalign, true);
            } else {
                match handle.props.read() {
                    Ok(props) => {
                        let value = format!("{}", (props.value * precision).round() / precision);
                        self.draw_string(value.as_str(), None, 0.0, lalign, true);
                    }
                    Err(e) => panic!("RwLock poisoned"),
                }
            }
        }

        self.end_element(None);

        match handle.props.read() {
            Ok(props) => props.value,
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn separator(&mut self, h: u32 /* = 4*/, fill: bool /* = true*/) {
        if !self.is_visible(self.element_h()) {
            self.y += h as f32 * self.scale();
            return;
        }

        if fill {
            self.painter.set_color(self.theme.separator_col);
            self.painter
                .fill_rect(self.x, self.y, self.w as f32, h as f32 * self.scale());
        }

        self.y += h as f32 * self.scale();
    }

    pub fn tooltip(&mut self, text: &str) {
        self.tooltip_text = text.into();
        self.tooltip_y = self.y + self.window_y;
    }

    pub fn tooltip_image(&mut self, image: Image, max_width: Option<i32> /*= None*/) {
        self.tooltip_img = Some(image);
        self.tooltip_img_max_width = max_width;
        self.tooltip_invert_y = self.image_invert_y;
        self.tooltip_y = self.y + self.window_y;
    }

    fn draw_arrow(&self, selected: bool) {
        let trans_mat = self.painter.transformation();

        let x = self.x + self.arrow_offset_x;
        let y = self.y + self.arrow_offset_y;

        let p = trans_mat * Vector3::new(x, y, 1.0_f32);

        self.painter.set_color(self.theme.text_col);
        if selected {
            self.painter.fill_triangle(
                p.x,
                p.y,
                p.x + self.arrow_size(),
                p.y,
                p.x + self.arrow_size() / 2.0,
                p.y + self.arrow_size(),
            );
        } else {
            self.painter.fill_triangle(
                p.x,
                p.y,
                p.x,
                p.y + self.arrow_size(),
                p.x + self.arrow_size(),
                p.y + self.arrow_size() / 2.0,
            );
        }
        self.painter.end(); // DV should be automatic
    }

    fn draw_tree(&self, selected: bool) {
        let sign_w = 7.0 * self.scale();
        let x = self.x + self.arrow_offset_x + 1.0;
        let y = self.y + self.arrow_offset_y + 1.0;

        self.painter.set_color(self.theme.text_col);

        if selected {
            self.painter
                .fill_rect(x, y + sign_w / 2.0 - 1.0, sign_w, sign_w / 8.0);
        } else {
            self.painter
                .fill_rect(x, y + sign_w / 2.0 - 1.0, sign_w, sign_w / 8.0);
            self.painter
                .fill_rect(x + sign_w / 2.0 - 1.0, y, sign_w / 8.0, sign_w);
        }
    }

    fn draw_check(&self, selected: bool, hover: bool) {
        let trans_mat = self.painter.transformation();

        let x = self.x + self.check_offset_x;
        let y = self.y + self.check_offset_y;

        self.painter.set_color(if hover {
            self.theme.accent_hover_col
        } else {
            self.theme.accent_col
        });

        self.draw_rect(
            self.theme.fill_accent_bg,
            x,
            y,
            self.check_size(),
            self.check_size(),
            0.0,
        ); // Bg

        if selected {
            // Check
            self.painter.set_color(color::WHITE);

            if !self.enabled {
                self.fade_color();
            }

            let width = self.check_select_size() as u32;
            let hight = self.check_select_size() as u32;

            let dx = x + self.check_select_offset_x;
            let dy = y + self.check_select_offset_y;

            self.painter.set_color(self.theme.accent_select_col);

            self.painter.draw_line(
                dx,
                dy,
                dx + width as f32,
                dy + hight as f32,
                2.0 * self.scale(),
            );
            self.painter.draw_line(
                dx + width as f32,
                dy,
                dx,
                dy + hight as f32,
                2.0 * self.scale(),
            );
        }
    }

    fn draw_radio(&self, selected: bool, hover: bool) {
        let x = self.x + self.radio_offset_x;
        let y = self.y + self.radio_offset_y;

        self.painter.set_color(if hover {
            self.theme.accent_hover_col
        } else {
            self.theme.accent_col
        });

        self.draw_rect(
            self.theme.fill_accent_bg,
            x,
            y,
            self.check_size(),
            self.check_size(),
            0.0,
        ); // Bg

        if selected {
            // Check
            self.painter.set_color(self.theme.accent_select_col);

            if !self.enabled {
                self.fade_color();
            }

            self.painter.fill_rect(
                x + self.radio_select_offset_x,
                y + self.radio_select_offset_y,
                self.check_select_size(),
                self.check_select_size(),
            );
        }
    }

    fn draw_slider(&self, value: f32, from: f32, to: f32, filled: bool, hover: bool) {
        let x = self.x + self.button_offset_y;
        let y = self.y + self.button_offset_y;
        let w = self.w as f32 - self.button_offset_y * 2.0;

        self.painter.set_color(if hover {
            self.theme.accent_hover_col
        } else {
            self.theme.accent_col
        });

        // Draw slider Bg
        self.draw_rect(self.theme.fill_accent_bg, x, y, w, self.button_h(), 0.0);

        self.painter.set_color(if hover {
            self.theme.accent_hover_col
        } else {
            self.theme.accent_col
        });

        let offset = (value - from) / (to - from);

        // Unfilled bar
        let barW = 8.0 * self.scale();

        let mut sliderX = if filled { x } else { x + (w - barW) * offset };
        sliderX = sliderX.min(x + (w - barW)).max(x);

        let mut sliderW = if filled { w * offset } else { barW };
        sliderW = sliderW.min(w).max(0.0);

        self.draw_rect(true, sliderX, y, sliderW, self.button_h(), 0.0);
    }

    // static let comboFirst = true;

    fn draw_combo(&mut self) {
        if self.combo_selected_handle.is_none() {
            return;
        }

        // let _g = g;
        // global_g.set_color(self.theme.separator_col);
        // global_g.begin(false);

        let comboH = (self.combo_selected_texts.len() as f32
            + (if !self.combo_selected_label.is_empty() {
                1.0
            } else {
                0.0
            }))
            * self.element_h();
        let distTop = self.combo_selected_y as f32
            - comboH
            - self.element_h()
            - self.window_border_top as f32;

        // let distBottom = System.windowHeight()
        //     - self.window_border_bottom
        //     - (self.combo_selected_y + comboH);

        // let unrollUp = distBottom < 0 && distBottom < distTop;
        // self.begin_region(
        //     self.combo_selected_x,
        //     self.combo_selected_y,
        //     self.combo_selected_w,
        // );

        // if self.is_key_pressed || self.input_wheel_delta != 0 {
        //     let arrowUp = self.is_key_pressed && self.key == (if unrollUp { VirtualKeyCode::Down} else { VirtualKeyCode::Up});
        //     let arrowDown = self.is_key_pressed && self.key == (if unrollUp { VirtualKeyCode::Up} else { VirtualKeyCode::Down});
        //     let wheelUp = (unrollUp && self.input_wheel_delta > 0) || (!unrollUp && self.input_wheel_delta < 0);
        //     let wheelDown = (unrollUp && self.input_wheel_delta < 0) || (!unrollUp && self.input_wheel_delta > 0);

        //     if (arrowUp || wheelUp) && self.combo_to_submit > 0 {
        //     	self.combo_to_submit -= 1;
        //     	self.submit_combo_handle = self.combo_selected_handle;
        //     } else if (arrowDown || wheelDown) && self.combo_to_submit < self.combo_selected_texts.len() - 1 {
        //     	self.combo_to_submit += 1;
        //     	self.submit_combo_handle = self.combo_selected_handle;
        //     }

        //     if self.combo_selected_window.is_none() {
        //         self.combo_selected_window.redraws = 2;
        //     }
        // }

        self.input_enabled = true;
        let _BUTTON_COL = self.theme.button_col;
        let _ELEMENT_OFFSET = self.theme.element_offset;
        self.theme.element_offset = 0;

        // let unrollRight = if self.x + self.combo_selected_w * 2
        //     < System.windowWidth() - self.window_border_right
        // {
        //     1
        // } else {
        //     -1
        // };

        // for i in 0..self.combo_selected_texts.len() {
        //     if unrollUp {
        //         self.y -= self.element_h() * 2.0;
        //     }

        //     self.theme.button_col = if i == self.combo_selected_handle.position {
        //         self.theme.accent_select_col
        //     } else {
        //         self.theme.separator_col
        //     };
        //     self.fill(
        //         0,
        //         0,
        //         self.w / self.scale(),
        //         self.element_h() / self.scale(),
        //         self.theme.separator_col,
        //     );
        //     if self.button(self.combo_selected_texts[i], self.combo_selected_align, "") {
        //         self.combo_to_submit = i;
        //         self.submit_combo_handle = self.combo_selected_handle;
        //         if self.combo_selected_window.is_some() {
        //             self.combo_selected_window.redraws = 2;
        //         }
        //         break;
        //     }
        //     if self.y + self.element_h() > System.windowHeight() - self.window_border_bottom
        //         || self.y - self.element_h() * 2.0 < self.window_border_top
        //     {
        //         self.x += (self.combo_selected_w * unrollRight) as f32; // Next column
        //         self.y = self.combo_selected_y as f32;
        //     }
        // }
        // self.theme.button_col = _BUTTON_COL;
        // self.theme.element_offset = _ELEMENT_OFFSET;

        // if !self.combo_selected_label.is_empty() {
        //     // Unroll down
        //     if unrollUp {
        //         self.y -= self.element_h() * 2.0;
        //         self.fill(
        //             0.0,
        //             0.0,
        //             self.w as f32 / self.scale(),
        //             self.element_h() / self.scale(),
        //             self.theme.separator_col,
        //         );
        //         self.painter.set_color(self.theme.label_col);
        //         self.draw_string(
        //             self.combo_selected_label.as_str(),
        //             None,
        //             0.0,
        //             Align::Right,
        //             true,
        //         );
        //         self.y += self.element_h();
        //         self.fill(
        //             0,
        //             0,
        //             self.w / self.scale(),
        //             1 * self.scale(),
        //             self.theme.accent_select_col,
        //         ); // Separator
        //     } else {
        //         self.fill(
        //             0,
        //             0,
        //             self.w / self.scale(),
        //             self.element_h() / self.scale(),
        //             self.theme.separator_col,
        //         );
        //         self.fill(
        //             0,
        //             0,
        //             self.w / self.scale(),
        //             1 * self.scale(),
        //             self.theme.accent_select_col,
        //         ); // Separator
        //         self.painter.set_color(self.theme.label_col);
        //         self.draw_string(
        //             self.combo_selected_label.as_str(),
        //             None,
        //             0.0,
        //             Align::Right,
        //             true,
        //         );
        //     }
        // }

        // if (self.input_released || self.is_escape_down || self.is_return_down) && !self.combo_first {
        //     self.combo_selected_handle = None;
        //     self.combo_first = true;
        // } else {
        //     self.combo_first = false;
        // }

        self.input_enabled = self.combo_selected_handle.is_none();
        self.end_region(false);
        // global_g.end();
        // g = _g; // Restore
    }

    fn draw_tooltip(&mut self, bind_global_g: bool) {
        // if self.slider_tooltip {
        // 	if bind_global_g {
        //         global_g.begin(false);
        //     }

        // 	global_g.font = self.ops.font;
        // 	global_g.font_size = font_size * 2;
        // 	let text = ((self.scroll_handle.value * 100).round() / 100) + "";
        // FIXME: need font measure

        // if let Some((width, _)) = self.painter.measure("text here") {
        //     println!("TEXT DIMENSIONT {} {}", width, height);
        // } else {
        //     println!("CANT MEASURE THE TEXT");
        // }

        // 	let xoff = self.ops.font.width(global_g.font_size, text) / 2;
        // 	let yoff = self.ops.font.height(global_g.font_size);
        // 	let x = self.slider_tooltip_x.max(self.input_x).min(self.slider_tooltip_x + self.slider_tooltip_w);

        //     global_g.set_color(self.theme.accent_col);
        // 	global_g.fill_rect(x - xoff, self.slider_tooltip_y - yoff, xoff * 2, yoff);

        //     global_g.set_color(self.theme.text_col);
        // 	global_g.draw_string(text, x - xoff, self.slider_tooltip_y - yoff);
        // 	if bind_global_g {
        //         global_g.end();
        //     }
        // }

        // if self.tooltip_text != "" || self.tooltip_img.is_some() {
        // 	if self.input_changed() {
        // 		self.tooltip_shown = false;
        //         // Wait for movement before showing up again
        // 		self.tooltip_wait = self.input_dx == 0.0 && self.input_dy == 0.0;
        // 	}

        // 	if !self.tooltip_shown {
        // 		self.tooltip_shown = true;
        // 		self.tooltip_x = self.input_x;
        // 		self.tooltip_time = self.scheduler_time();
        // 	}
        // 	if !self.tooltip_wait && self.scheduler_time() - self.tooltip_time > self.tooltip_delay() {
        // 		if !self.tooltip_text.is_empty() {
        //             self.draw_tooltip_text(bind_global_g)
        //         } else {
        //             self.draw_tooltip_image(bind_global_g);
        //         }
        // 	}
        // } else {
        //     self.tooltip_shown = false;
        // }
    }

    fn draw_tooltip_text(&self, bind_global_g: bool) {
        // global_g.set_color(self.theme.text_col);
        // let lines = self.tooltip_text.split("\n");
        // let tooltipW = 0.0;
        // for line in lines {
        // FIXME: need font measure

        // if let Some((width, _)) = self.painter.measure("text here") {
        //     println!("TEXT DIMENSIONT {} {}", width, height);
        // } else {
        //     println!("CANT MEASURE THE TEXT");
        // }

        // 	let lineTooltipW = self.ops.font.width(self.font_size, line);
        // 	if lineTooltipW > tooltipW {
        //         tooltipW = lineTooltipW;
        //     }
        // }

        // self.tooltip_x = self.tooltip_x.min(System.windowWidth() - tooltipW - 20);
        // if bind_global_g {
        //     global_g.begin(false);
        // }

        // let fontHeight = self.ops.font.height(self.font_size);
        // global_g.fill_rect(self.tooltip_x, self.tooltip_y, tooltipW + 20, fontHeight * lines.len());
        // global_g.set_font(self.ops.font);
        // global_g.set_font_size(self.font_size);
        // global_g.set_color(self.theme.accent_col);

        // for i in 0..lines.len() {
        // 	global_g.draw_string(lines[i], self.tooltip_x + 5, self.tooltip_y + i * self.font_size);
        // }
        // if bind_global_g {
        //     global_g.end();
        // }
    }

    fn draw_tooltip_image(&self, bind_global_g: bool) {
        // let w = self.tooltip_img.width;
        // if self.tooltip_img_max_width != None && w > self.tooltip_img_max_width {
        //     w = self.tooltip_img_max_width;
        // }

        // let h = self.tooltip_img.height * (w / self.tooltip_img.width);
        // self.tooltip_x = self.tooltip_x.min(System.windowWidth() - w - 20);
        // self.tooltip_y = self.tooltip_y.min(System.windowHeight() - h - 20);

        // if bind_global_g {
        //     global_g.begin(false);
        // }

        // global_g.set_color(0xff000000);
        // global_g.fill_rect(self.tooltip_x, self.tooltip_y, w, h);
        // global_g.set_color(0xffffffff);

        // if self.tooltip_invert_y {
        // 	global_g.draw_scaled_image(self.tooltip_img, self.tooltip_x, self.tooltip_y + h, w, -h);
        // } else {
        // 	global_g.draw_scaled_image(self.tooltip_img, self.tooltip_x, self.tooltip_y, w, h);
        // }

        // if bind_global_g {
        //     global_g.end();
        // }
    }

    fn draw_string(
        &self,
        text: &str,
        x_offset: Option<f32>, /*= None*/
        y_offset: f32,         /*= 0*/
        align: Align,          /*= Align::Left*/
        truncation: bool,      /* = true*/
    ) {
        let mut text = text;
        if truncation {
            let original_len = text.len();

            let mut trimmed = false;
            let mut cursor_pos = original_len - 1;

            while cursor_pos > 0 && {
                let substr = &text[..cursor_pos];
                self.painter
                    .measure(substr)
                    .map(|(width, _)| width > self.w - 6.0)
                    .unwrap_or_default()
            } {
                // mean text is not filled
                // so we move cursor to start of text
                cursor_pos -= 1;
                trimmed = true;
            }

            if trimmed {
                if cursor_pos + 1 < original_len {
                    // if self.is_hovered {
                    //     self.tooltip(text); // show tooltip with text before trim
                    // }

                    // TODO: need to adjust it with some extra
                    // space for ".." suffix aka "read more.."
                    text = &text[..cursor_pos];
                }
            }
        }

        let mut x_offset = match x_offset {
            Some(value) => value,
            None => self.theme.text_offset as f32,
        };

        x_offset = x_offset * self.scale();
        self.painter.set_font(self.ops.font);
        self.painter.set_font_size(self.font_size as f32);

        // TODO: to enable alignment we should implement `measure` method for font
        if align == Align::Center {
            if let Some((width, _)) = self.painter.measure("text here") {
                x_offset = self.w / 2.0 - width / 2.0;
            }
        } else if align == Align::Right {
            if let Some((width, _)) = self.painter.measure(text) {
                x_offset = self.w - width - self.text_offset();
            }
        }

        if !self.enabled {
            self.fade_color();
        }

        // self.painter.set_pipeline(rtTextPipeline);
        self.painter.draw_string(
            text,
            self.x + x_offset,
            self.y + self.font_offset_y / 2.0 + y_offset,
        );
        // self.painter.set_pipeline(None);
    }

    fn end_element(&mut self, element_size: Option<f32> /* = None*/) {
        let element_size = match element_size {
            Some(val) => val,
            None => self.element_h() + self.element_offset(),
        };

        let condition = match self.current_window {
            Some(ref current_window) => match current_window.props.read() {
                Ok(props) => {
                    if props.layout == Layout::Vertical {
                        true
                    } else {
                        false
                    }
                }
                Err(e) => panic!("RwLock poisoned"),
            },
            None => true,
        };

        if condition {
            if self.cur_ratio == -1
                || (self.ratios.len() > 0 && self.cur_ratio == self.ratios.len() as i32 - 1)
            {
                // New line
                self.y += element_size;

                if self.ratios.len() > 0 && self.cur_ratio == self.ratios.len() as i32 - 1 {
                    // Last row element
                    self.cur_ratio = -1;
                    self.ratios.clear();
                    self.x = self.x_before_split;
                    self.w = self.w_before_split;
                    self.highlight_full_row = false;
                }
            } else {
                // Row
                self.cur_ratio += 1;
                self.x += self.w as f32; // More row elements to place
                self.w = self.ratio(
                    self.ratios[self.cur_ratio as usize] as f32,
                    self.w_before_split as f32,
                );
            }
        } else {
            // Horizontal
            self.x += self.w as f32 + self.element_offset();
        }
    }

    // Highlight all upcoming elements in the next row on a `mouse-over` event.
    #[inline]
    pub fn highlight_next_row(&mut self) {
        self.highlight_full_row = true;
    }

    #[inline]
    fn ratio(&self, ratio: f32, factor: f32) -> f32 {
        if ratio < 0.0 {
            -ratio
        } else {
            ratio * factor
        }
    }

    // Draw the upcoming elements in the same row.
    // Negative values will be treated as absolute, positive values as ratio to `window width`.
    pub fn row(&mut self, ratios: &[f32]) {
        self.ratios = ratios.into();
        self.cur_ratio = 0;
        self.x_before_split = self.x;
        self.w_before_split = self.w;
        self.w = self.ratio(self.ratios[self.cur_ratio as usize], self.w);
    }

    pub fn indent(&mut self, both_sides: bool /* = true*/) {
        self.x += self.tab_w() as f32;
        self.w -= self.tab_w() as f32;

        if both_sides {
            self.w -= self.tab_w() as f32;
        }
    }

    pub fn unindent(&mut self, both_sides: bool /* = true*/) {
        self.x -= self.tab_w() as f32;
        self.w += self.tab_w() as f32;

        if both_sides {
            self.w += self.tab_w() as f32;
        }
    }

    fn fade_color(&self) {
        let color = self.painter.color();

        self.painter.set_color(Color {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: 0.25,
        });
    }

    pub fn fill(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.painter.set_color(color);
        if !self.enabled {
            self.fade_color();
        }
        self.painter.fill_rect(
            self.x + x * self.scale(),
            self.y + y * self.scale() - 1.0,
            w * self.scale(),
            h * self.scale(),
        );
        self.painter.set_color(color::WHITE);
    }

    pub fn rect(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        strength: f32, /*  = 1.0*/
    ) {
        self.painter.set_color(color);
        if !self.enabled {
            self.fade_color();
        }
        self.painter.draw_rect(
            self.x + x * self.scale(),
            self.y + y * self.scale(),
            w * self.scale(),
            h * self.scale(),
            strength,
        );
        self.painter.set_color(color::WHITE);
    }

    #[inline]
    fn draw_rect(&self, fill: bool, x: f32, y: f32, w: f32, h: f32, strength: f32 /* = 0.0*/) {
        let strength = if strength == 0.0 { 1.0 } else { strength };

        if !self.enabled {
            self.fade_color();
        }

        if fill {
            self.painter.fill_rect(x, y - 1.0, w, h + 1.0);
        } else {
            self.painter.draw_rect(x, y, w, h, strength);
        }
    }

    fn is_visible(&self, elem_h: f32) -> bool {
        // if self.current_window.is_none() {
        //     return true;
        // }

        let tex_height = match self.current_window {
            Some(ref handle) => match handle.props.read() {
                Ok(props) => match props.texture {
                    Some(ref texture) => texture.height,
                    None => 0,
                },
                Err(e) => panic!("RwLock poisoned"),
            },
            None => 0,
        };

        // self.y + elem_h > self.window_header_h && self.y < tex_height as f32 // DV Wrong logic
        self.y + elem_h > self.window_header_h && self.y < self.window_h
    }

    fn released(&mut self, elem_h: f32 /*= -1.0*/) -> bool {
        // Input selection
        self.is_released = self.enabled
            && self.input_enabled
            && self.input_released
            && self.hover(elem_h)
            && self.initial_hover(elem_h);
        self.is_released
    }

    fn pushed(&mut self, elem_h: f32 /* = -1.0*/) -> bool {
        self.is_pushed = self.enabled
            && self.input_enabled
            && self.input_down
            && self.hover(elem_h)
            && self.initial_hover(elem_h);
        self.is_pushed
    }

    fn started(&mut self, elem_h: f32 /* = -1.0*/) -> bool {
        self.is_started =
            self.enabled && self.input_enabled && self.input_started && self.hover(elem_h);
        self.is_started
    }

    fn initial_hover(&self, elem_h: f32 /* = -1.0*/) -> bool {
        if self.scissor && self.input_y < self.window_y + self.window_header_h {
            return false;
        }

        let elem_h = if elem_h == -1.0 {
            self.element_h()
        } else {
            elem_h
        };

        self.enabled
            && self.input_enabled
            && self.input_started_x >= self.window_x + self.x
            && self.input_started_x < (self.window_x + self.x + self.w as f32)
            && self.input_started_y >= self.window_y + self.y
            && self.input_started_y < (self.window_y + self.y + elem_h)
    }

    fn hover(&mut self, elem_h: f32 /* = -1.0*/) -> bool {
        if self.scissor && self.input_y < self.window_y + self.window_header_h {
            return false;
        }

        let elem_h = if elem_h == -1.0 {
            self.element_h()
        } else {
            elem_h
        };

        self.is_hovered = self.enabled
            && self.input_enabled
            && self.input_x
                >= self.window_x
                    + (if self.highlight_full_row {
                        0_f32
                    } else {
                        self.x as f32
                    })
            && self.input_x
                < (self.window_x
                    + self.x
                    + (if self.highlight_full_row {
                        self.window_w
                    } else {
                        self.w as f32
                    }))
            && self.input_y >= self.window_y + self.y
            && self.input_y < (self.window_y + self.y + elem_h);

        self.is_hovered
    }

    fn input_in_rect(&self, x: f32, y: f32, w: f32, h: f32, scale: f32 /* = 1.0*/) -> bool {
        self.enabled
            && self.input_enabled
            && self.input_x >= x * scale
            && self.input_x < (x + w) * scale
            && self.input_y >= y * scale
            && self.input_y < (y + h) * scale
    }

    pub fn on_mouse_down(&mut self, button: &MouseButton) {
        // Input events
        match button {
            MouseButton::Left => {
                self.input_started = true;
                self.input_down = true;
            }
            _ => {
                self.input_started_r = true;
                self.input_down_r = true;
            }
        }

        self.input_started_time = self.scheduler_time();

        // TODO: should deal with input_position on android and ios;
        self.input_started_x = self.input_x;
        self.input_started_y = self.input_y;
    }

    pub fn on_mouse_up(&mut self, button: &MouseButton) {
        if self.is_scrolling {
            // Prevent action when scrolling is active
            self.is_scrolling = false;
            self.scroll_handle = None;
            self.slider_tooltip = false;

            if self.input_x as i32 == self.input_started_x as i32
                && self.input_y as i32 == self.input_started_y as i32
            {
                // Mouse not moved
                match button {
                    MouseButton::Left => self.input_released = true,
                    _ => self.input_released_r = true,
                }
            }
        } else {
            match button {
                MouseButton::Left => self.input_released = true,
                _ => self.input_released_r = true,
            }
        }

        match button {
            MouseButton::Left => self.input_down = false,
            _ => self.input_down_r = false,
        }

        // TODO: should deal with input_position on android and ios;
        self.deselect_text();

        if self.touch_hold {
            self.touch_hold = false;
            self.input_released = false;
            self.input_released_r = true;
        }
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32, movement_x: f32, movement_y: f32) {
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        self.set_input_position(x, y);
    }

    pub fn on_mouse_wheel(&mut self, delta: i32) {
        self.input_wheel_delta = delta;
    }

    fn set_input_position(&mut self, x: f32, y: f32) {
        self.input_dx += x - self.input_x;
        self.input_dy += y - self.input_y;
        self.input_x = x;
        self.input_y = y;
    }

    pub fn on_key_down(&mut self, code: VirtualKeyCode) {
        self.key = Some(code);
        self.is_key_pressed = true;
        self.is_key_down = true;
        self.key_repeat_time = self.scheduler_time() + 0.4;

        match code {
            VirtualKeyCode::Back => self.is_backspace_down = true,
            VirtualKeyCode::Delete => self.is_delete_down = true,
            VirtualKeyCode::Escape => self.is_escape_down = true,
            VirtualKeyCode::Return => self.is_return_down = true,
            VirtualKeyCode::Tab => self.is_tab_down = true,
            VirtualKeyCode::A => self.is_adown = true,
            VirtualKeyCode::Space => self.char = ' ',
            // #[cfg(feature = "rmb")] // Detect right mouse button on Android..
            // VirtualKeyCode::Back: if (!self.input_down_r) on_mouse_down(&MouseButton::Left);
            _ => {}
        }
    }

    pub fn on_key_up(&mut self, code: VirtualKeyCode) {
        self.is_key_down = false;

        match code {
            VirtualKeyCode::Back => self.is_backspace_down = false,
            VirtualKeyCode::Delete => self.is_delete_down = false,
            VirtualKeyCode::Escape => self.is_escape_down = false,
            VirtualKeyCode::Return => self.is_return_down = false,
            VirtualKeyCode::Tab => self.is_tab_down = false,
            VirtualKeyCode::A => self.is_adown = false,
            // #[cfg(feature = "rmb")]
            // VirtualKeyCode::Back: self.on_mouse_up(&MouseButton::Left),
            _ => {}
        }
    }

    pub fn on_modifier_changed(&mut self, state: &ModifiersState) {
        // println!("MODIFIER CHANGED");

        self.is_alt_down = state.alt();
        self.is_ctrl_down = state.ctrl();

        // #[cfg(target_os = "macos")]
        // self.is_ctrl_down = state.logo();

        self.is_shift_down = state.shift();
    }

    pub fn on_key_press(&mut self, char: char) {
        // println!("KEY RECEIVED");
        self.char = char;
        self.is_key_pressed = true;
    }

    // #[cfg(any(target_os = "android", target_os = "ios"))]
    pub fn on_touch_down(&mut self, id: u64, x: f32, y: f32) {
        // Reset movement delta on touch start
        if id == 0 {
            self.input_dx = 0.0;
            self.input_dy = 0.0;
            self.input_x = x;
            self.input_y = y;
        }

        // Two fingers down - right mouse button
        if id == 1 {
            self.on_mouse_up(&MouseButton::Left);
            self.on_mouse_down(&MouseButton::Right);
        }
    }

    // #[cfg(any(target_os = "android", target_os = "ios"))]
    pub fn on_touch_up(&mut self, id: u64, x: f32, y: f32) {
        if id == 1 {
            self.on_mouse_up(&MouseButton::Left);
        }
    }

    // #[cfg(any(target_os = "android", target_os = "ios"))]
    pub fn on_touch_move(&mut self, id: u64, x: f32, y: f32) {
        if id == 0 {
            self.set_input_position(x, y);
        }
    }

    pub fn on_cut(&mut self) -> String {
        self.is_cut = true;
        self.on_copy()
    }

    pub fn on_copy(&mut self) -> String {
        self.is_copy = true;
        self.text_to_copy.clone()
    }

    pub fn on_paste(&mut self, s: String) {
        self.is_paste = true;
        self.text_to_paste = s;
    }

    #[inline]
    pub fn element_w(&self) -> f32 {
        self.ops.theme.element_w as f32 * self.scale()
    }

    #[inline]
    pub fn element_h(&self) -> f32 {
        self.ops.theme.element_h as f32 * self.scale()
    }

    #[inline]
    pub fn element_offset(&self) -> f32 {
        self.ops.theme.element_offset as f32 * self.scale()
    }

    #[inline]
    pub fn arrow_size(&self) -> f32 {
        self.ops.theme.arrow_size as f32 * self.scale()
    }

    #[inline]
    pub fn button_h(&self) -> f32 {
        self.ops.theme.button_h as f32 * self.scale()
    }

    #[inline]
    pub fn check_size(&self) -> f32 {
        self.ops.theme.check_size as f32 * self.scale()
    }

    #[inline]
    pub fn check_select_size(&self) -> f32 {
        self.ops.theme.check_select_size as f32 * self.scale()
    }

    #[inline]
    pub fn font_size(&self) -> f32 {
        self.theme.font_size as f32 * self.scale()
    }

    #[inline]
    pub fn scroll_w(&self) -> i32 {
        (self.ops.theme.scroll_w as f32 * self.scale()) as i32
    }

    #[inline]
    pub fn text_offset(&self) -> f32 {
        self.ops.theme.text_offset as f32 * self.scale()
    }

    #[inline]
    pub fn tab_w(&self) -> i32 {
        (self.ops.theme.tab_w as f32 * self.scale()) as i32
    }

    #[inline]
    pub fn header_drag_h(&self) -> i32 {
        (15.0 * self.scale()) as i32
    }

    #[inline]
    pub fn scale(&self) -> f32 {
        self.ops.scale_factor
    }

    #[inline]
    fn flash_speed(&self) -> f32 {
        0.5
    }

    #[inline]
    fn tooltip_delay(&self) -> f32 {
        1.0
    }

    pub fn resize(&mut self, handle: Handle, w: i32, h: i32, window_id: i32 /* = 0*/) {
        match handle.props.write() {
            Ok(mut props) => {
                props.redraws = 2;
                if props.texture.is_some() {
                    // props.unload();
                }

                let w = if w < 1 { 1 } else { w };
                let h = if h < 1 { 1 } else { h };

                // props.texture = Image::create_render_target(
                //     w,
                //     h,
                //     graphics4.TextureFormat.RGBA32,
                //     graphics4.DepthStencilFormat.NoDepthAndStencil,
                //     1,
                //     khaWindowId,
                // );
                // props.texture.g2.imageScaleQuality = ImageScaleQuality::HIGH;
                // props.texture.g2.mipmapScaleQuality = ImageScaleQuality::HIGH;
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn inline_radio(
        &mut self,
        handle: &Handle,
        texts: &[&str],
        align: Align, /*= Left*/
    ) -> i32 {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);
            match handle.props.read() {
                Ok(props) => return props.position,
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        let step = self.w / texts.len() as f32;
        let mut hovered = -1;
        if self.hover(-1.0) {
            let ix = (self.input_x - self.x - self.window_x) as i32;
            for idx in 0..texts.len() as i32 {
                if (ix as f32) < (idx as f32 * step + step) {
                    hovered = idx;
                    break;
                }
            }
        }

        match handle.props.write() {
            Ok(mut props) => {
                if self.released(-1.0) {
                    props.position = hovered;
                    props.changed = true;
                    self.changed = true;
                } else {
                    props.changed = false;
                }
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        match handle.props.read() {
            Ok(props) => {
                for idx in 0..texts.len() {
                    if props.position == idx as i32 {
                        self.painter.set_color(self.theme.accent_hover_col);
                        if !self.enabled {
                            self.fade_color();
                        }
                        self.painter.fill_rect(
                            self.x + step * idx as f32,
                            self.y + self.button_offset_y,
                            step,
                            self.button_h(),
                        );
                    } else if hovered == idx as i32 {
                        self.painter.set_color(self.theme.accent_col);
                        if !self.enabled {
                            self.fade_color();
                        }
                        self.painter.draw_rect(
                            self.x + step * idx as f32,
                            self.y + self.button_offset_y,
                            step,
                            self.button_h(),
                            1.0,
                        );
                    }

                    self.painter.set_color(self.theme.text_col); // Text
                    self.x += step * idx as f32;

                    let w = self.w;
                    self.w = step as f32;
                    self.draw_string(texts[idx], None, 0 as f32, align, true);
                    self.x -= step * idx as f32;
                    self.w = w;
                }

                self.end_element(None);
                props.position
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn color_wheel(
        &mut self,
        handle: &Handle,
        alpha: bool,         /* = false*/
        w: Option<f32>,      /* = None*/
        color_preview: bool, /* = true*/
    ) -> Color {
        let mut w = match w {
            Some(w) => w,
            None => self.w,
        };

        let color = match handle.props.read() {
            Ok(props) => props.color,
            Err(e) => panic!("RwLock poisoned"),
        };

        let (mut chue, mut csat, cval) = Self::rgb_to_hsv(color.red, color.green, color.blue);
        let mut calpha = color.alpha;

        // Wheel
        let mut px = self.x;
        let py = self.y;

        let scroll = if let Some(current_window) = &self.current_window {
            match current_window.props.read() {
                Ok(props) => props.scroll_enabled,
                Err(e) => panic!("RwLock poisoned"),
            }
        } else {
            false
        };

        if !scroll {
            w -= self.scroll_w() as f32;
            px += self.scroll_w() as f32 / 2.0;
        }

        if let Some(image) = self.ops.color_wheel {
            self.image(&image, Color::new(cval, cval, cval, 1.0), None, 0, 0, 0, 0);
        }

        // Picker
        let ph = self.y - py;
        let ox = px + w / 2.0;
        let oy = py + ph / 2.0;
        let cw = w * 0.7;
        let cwh = cw / 2.0;
        let mut cx = ox;
        let mut cy = oy + csat * cwh; // Sat is distance from center

        // Rotate around origin by hue
        let theta = chue * (PI * 2.0);
        let cx2 = theta.cos() * (cx - ox) - theta.sin() * (cy - oy) + ox;
        let cy2 = theta.sin() * (cx - ox) + theta.cos() * (cy - oy) + oy;
        cx = cx2;
        cy = cy2;

        self.painter.set_color(color::BLACK);
        self.painter.fill_rect(
            cx - 3.0 * self.scale(),
            cy - 3.0 * self.scale(),
            6.0 * self.scale(),
            6.0 * self.scale(),
        );
        self.painter.set_color(color::WHITE);
        self.painter.fill_rect(
            cx - 2.0 * self.scale(),
            cy - 2.0 * self.scale(),
            4.0 * self.scale(),
            4.0 * self.scale(),
        );

        if alpha {
            let alpha_handle = handle.nest(
                1,
                Some(HandleOptions {
                    value: Some((calpha * 100.0).round() / 100.0),
                    ..Default::default()
                }),
            );

            calpha = self.slider(
                &alpha_handle,
                "Alpha",
                0.0,
                1.0,
                true,
                100.0,
                true,
                Align::Right,
                true,
            );

            match handle.props.read() {
                Ok(alpha_props) => {
                    if alpha_props.changed {
                        match handle.props.write() {
                            Ok(mut props) => props.changed = true,
                            Err(e) => panic!("RwLock poisoned"),
                        }

                        self.changed = true;
                    }
                }
                Err(e) => panic!("RwLock poisoned"),
            };
        }

        // Mouse picking
        let gx = ox + self.window_x;
        let gy = oy + self.window_y;

        if self.input_started && self.input_in_rect(gx - cwh, gy - cwh, cw, cw, 1.0) {
            self.wheel_selected_handle = Some(handle.clone()); // FIXME: OOPS OVERHEAD
        }

        if self.input_released {
            self.wheel_selected_handle = None;
        }

        if let Some(ref wheel_selected_handle) = self.wheel_selected_handle {
            if self.input_down && wheel_selected_handle == handle {
                csat = Self::dist(gx, gy, self.input_x, self.input_y).min(cwh) / cwh;
                let mut angle = (self.input_x - gx).atan2(self.input_y - gy);

                if angle < 0.0 {
                    angle = PI + (PI - angle.abs());
                }

                angle = PI * 2.0 - angle;
                chue = angle / (PI * 2.0);

                match handle.props.write() {
                    Ok(mut props) => {
                        props.changed = true;
                        self.changed = true;
                    }
                    Err(e) => panic!("RwLock poisoned"),
                }
            }
        }

        // Save as rgb
        let (red, green, blue) = Self::hsv_to_rgb(chue, csat, cval);
        let color = Color {
            red,
            green,
            blue,
            alpha: calpha,
        };

        match handle.props.write() {
            Ok(mut props) => props.color = color.clone(),
            Err(e) => panic!("RwLock poisoned"),
        }

        if color_preview {
            self.text("", Align::Right, Some(color));
        }

        let pos = self.inline_radio(
            &Id::handle(handle.id + 0xFF, None),
            &["RGB", "HSV", "Hex"],
            Align::Left,
        );
        let h0 = handle.nest(0, None).nest(0, None);
        let h1 = handle.nest(0, None).nest(1, None);
        let h2 = handle.nest(0, None).nest(2, None);

        if pos == 0 {
            let color = match handle.props.read() {
                Ok(props) => props.color,
                Err(e) => panic!("RwLock poisoned"),
            };

            match h0.props.write() {
                Ok(mut props) => props.value = color.red,
                Err(e) => panic!("RwLock poisoned"),
            }

            let red = self.slider(&h0, "R", 0.0, 1.0, true, 100.0, true, Align::Right, true);

            match h1.props.write() {
                Ok(mut props) => props.value = color.green,
                Err(e) => panic!("RwLock poisoned"),
            }

            let green = self.slider(&h1, "G", 0.0, 1.0, true, 100.0, true, Align::Right, true);

            match h2.props.write() {
                Ok(mut props) => props.value = color.blue,
                Err(e) => panic!("RwLock poisoned"),
            }

            let blue = self.slider(&h2, "B", 0.0, 1.0, true, 100.0, true, Align::Right, true);

            match handle.props.write() {
                Ok(mut props) => props.color = Color::new(red, green, blue, 1.0),
                Err(e) => panic!("RwLock poisoned"),
            }
        } else if pos == 1 {
            let (hue, sat, val) = match handle.props.read() {
                Ok(props) => Self::rgb_to_hsv(props.color.red, props.color.green, props.color.blue),
                Err(e) => panic!("RwLock poisoned"),
            };

            match h0.props.write() {
                Ok(mut props) => props.value = hue,
                Err(e) => panic!("RwLock poisoned"),
            }

            match h1.props.write() {
                Ok(mut props) => props.value = sat,
                Err(e) => panic!("RwLock poisoned"),
            }

            match h2.props.write() {
                Ok(mut props) => props.value = val,
                Err(e) => panic!("RwLock poisoned"),
            }

            let chue = self.slider(&h0, "H", 0.0, 1.0, true, 100.0, true, Align::Right, true);
            let csat = self.slider(&h1, "S", 0.0, 1.0, true, 100.0, true, Align::Right, true);
            let cval = self.slider(&h2, "V", 0.0, 1.0, true, 100.0, true, Align::Right, true);

            let (red, green, blue) = Self::hsv_to_rgb(chue, csat, cval);

            match handle.props.write() {
                Ok(mut props) => props.color = Color::new(red, green, blue, 1.0),
                Err(e) => panic!("RwLock poisoned"),
            }
        } else if pos == 2 {
            // #cfg[() js]
            // {
            //     handle.text = untyped (handle.color >>> 0).toString(16);
            //     handle.color = untyped parseInt(ui.textInput(handle, "#"), 16);
            // }
        }
        let changed = {
            (match h0.props.read() {
                Ok(props) => props.changed,
                Err(e) => panic!("RwLock poisoned"),
            } || match h1.props.read() {
                Ok(props) => props.changed,
                Err(e) => panic!("RwLock poisoned"),
            } || match h2.props.read() {
                Ok(props) => props.changed,
                Err(e) => panic!("RwLock poisoned"),
            })
        };

        match handle.props.write() {
            Ok(mut props) => {
                if changed {
                    props.changed = true;
                    self.changed = true;
                }
                props.color
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    fn init_path(handle: &Handle) {
        use directories::UserDirs;
        let path = match UserDirs::new() {
            Some(user_dirs) => match user_dirs.home_dir().as_os_str().to_str() {
                Some(path) => path.to_owned(),
                None => {
                    log::error!("Cant convert %HomePath% to UTF8 string");
                    return;
                }
            },
            None => {
                log::error!("Cant detect %HomePath%");
                return;
            }
        };

        match handle.props.write() {
            Ok(mut props) => {
                props.text = path;
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn file_browser(&mut self, handle: &Handle, folders_only: bool /*= false*/) -> String {
        use std::path::MAIN_SEPARATOR;
        // let sep = "/";

        #[cfg(not(feature = "webgl"))]
        let files: Vec<String> = {
            let empty = match handle.props.read() {
                Ok(props) => props.text.is_empty(),
                Err(e) => panic!("RwLock poisoned"),
            };

            if empty {
                Self::init_path(handle);
            }

            // if sys.FileSystem.isDirectory(handle.text) {
            //     sys.FileSystem.readDirectory(handle.text)
            // } else {
            //     Vec::new()
            // }
            Vec::new()
        };

        #[cfg(feature = "webgl")]
        let files: Vec<String> = Vec::new();

        // Up directory
        let nested = match handle.props.write() {
            Ok(mut props) => {
                let textlen = props.text.len();
                props.changed = false;

                props
                    .text
                    .chars()
                    .position(|c| c == MAIN_SEPARATOR)
                    .map(|pos| textlen - 1 > pos)
                    .unwrap_or_default()
            }
            Err(e) => panic!("RwLock poisoned"),
        };

        if nested && self.button("..", Align::Left, "") {
            match handle.props.write() {
                Ok(mut props) => {
                    props.changed = true;
                    self.changed = true;

                    match props.text.rfind(MAIN_SEPARATOR) {
                        Some(pos) => {
                            props.text = props.text[..pos].to_string();
                        }
                        None => {
                            log::warn!("Cant find path separator");
                        }
                    }

                    // Drive root
                    if props.text.len() == 2 && props.text.chars().nth(1) == Some(':') {
                        props.text.push(MAIN_SEPARATOR);
                    }
                }
                Err(e) => panic!("RwLock poisoned"),
            }
        }

        // Directory contents
        for f in files {
            if f == "" || f.chars().nth(0) == Some('.') {
                continue; // Skip hidden
            }

            if self.button(f.as_str(), Align::Left, "") {
                match handle.props.write() {
                    Ok(mut props) => {
                        props.changed = true;
                        self.changed = true;

                        if props.text.chars().nth(props.text.len() - 1) != Some(MAIN_SEPARATOR) {
                            props.text.push(MAIN_SEPARATOR);
                        }

                        props.text.push_str(f.as_str());
                    }
                    Err(e) => panic!("RwLock poisoned"),
                }
            }
        }

        match handle.props.read() {
            Ok(props) => props.text.clone(),
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn float_input(
        &mut self,
        handle: &Handle,
        label: &str,    /* = ""*/
        align: Align,   /* = Left*/
        precision: f32, /* = 1000.0*/
    ) -> f32 {
        match handle.props.write() {
            Ok(mut props) => {
                props.text = format!("{}", (props.value * precision).round() / precision);
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        let text = self.text_input(handle, label, align, true);

        match handle.props.write() {
            Ok(mut props) => match text.parse::<f32>() {
                Ok(val) => {
                    props.value = val;
                    props.value
                }
                Err(e) => {
                    println!("{}", e);
                    0.0
                }
            },
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        let vx = x1 - x2;
        let vy = y1 - y2;
        (vx * vx + vy * vy).sqrt()
    }

    #[deprecated]
    fn fract(f: f32) -> f32 {
        f.fract()
    }

    fn mix(x: f32, y: f32, a: f32) -> f32 {
        x * (1.0 - a) + y * a
    }

    #[deprecated]
    fn clamp(x: f32, min_val: f32, max_val: f32) -> f32 {
        x.clamp(min_val, max_val)
    }

    fn step(edge: f32, x: f32) -> f32 {
        if x < edge {
            0.0
        } else {
            1.0
        }
    }

    fn hsv_to_rgb(cr: f32, cg: f32, cb: f32) -> (f32, f32, f32) {
        const KX: f32 = 1.0;
        const KY: f32 = 2.0 / 3.0;
        const KZ: f32 = 1.0 / 3.0;
        const KW: f32 = 3.0;

        let px = ((cr + KX).fract() * 6.0 - KW).abs();
        let py = ((cr + KY).fract() * 6.0 - KW).abs();
        let pz = ((cr + KZ).fract() * 6.0 - KW).abs();
        let r = cb * Self::mix(KX, (px - KX).clamp(0.0, 1.0), cg);
        let g = cb * Self::mix(KX, (py - KX).clamp(0.0, 1.0), cg);
        let b = cb * Self::mix(KX, (pz - KX).clamp(0.0, 1.0), cg);
        (r, g, b)
    }

    fn rgb_to_hsv(cr: f32, cg: f32, cb: f32) -> (f32, f32, f32) {
        const KX: f32 = 0.0;
        const KY: f32 = -1.0 / 3.0;
        const KZ: f32 = 2.0 / 3.0;
        const KW: f32 = -1.0;
        const E: f32 = 1.0e-10;

        let px = Self::mix(cb, cg, Self::step(cb, cg));
        let py = Self::mix(cg, cb, Self::step(cb, cg));
        let pz = Self::mix(KW, KX, Self::step(cb, cg));
        let pw = Self::mix(KZ, KY, Self::step(cb, cg));
        let qx = Self::mix(px, cr, Self::step(px, cr));
        let qy = Self::mix(py, py, Self::step(px, cr));
        let qz = Self::mix(pw, pz, Self::step(px, cr));
        let qw = Self::mix(cr, px, Self::step(px, cr));
        let d = qx - qw.min(qy);
        let h = (qz + (qw - qy) / (6.0 * d + E)).abs();
        let s = d / (qx + E);
        let v = qx;
        (h, s, v)
    }
}

#[derive(Default)]
pub struct HandleOptions {
    pub selected: Option<bool>,
    pub position: Option<i32>,
    pub value: Option<f32>,
    pub text: Option<String>,
    pub color: Option<Color>,
    pub layout: Option<Layout>,
}

#[derive(Clone)]
pub struct HandleProps {
    pub selected: bool,
    pub position: i32,
    pub color: Color,
    pub value: f32,
    pub text: String,
    pub texture: Option<Image>,
    pub redraws: i32,
    pub scroll_offset: f32,
    pub scroll_enabled: bool,
    pub layout: Layout,
    pub last_max_x: f32,
    pub last_max_y: f32,
    pub drag_enabled: bool,
    pub drag_x: i32,
    pub drag_y: i32,
    pub changed: bool,
    children: HashMap<u64, Handle>,
}

impl Default for HandleProps {
    fn default() -> Self {
        Self {
            selected: false,
            position: 0,
            color: color::WHITE, // = Color.White,
            value: 0.0,
            text: "".into(),
            texture: None,
            redraws: 2,
            scroll_offset: 0.0,
            scroll_enabled: false,
            layout: Layout::Vertical,
            last_max_x: 0.0,
            last_max_y: 0.0,
            drag_enabled: false,
            drag_x: 0,
            drag_y: 0,
            changed: false,
            children: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct Handle {
    id: u64,
    props: Arc<RwLock<HandleProps>>,
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Handle {
    pub fn new(id: u64, options: Option<HandleOptions>) -> Self {
        match options {
            Some(options) => {
                let mut props = HandleProps::default();

                if let Some(selected) = options.selected {
                    props.selected = selected;
                }

                if let Some(position) = options.position {
                    props.position = position;
                }

                if let Some(value) = options.value {
                    props.value = value;
                }

                if let Some(text) = options.text {
                    props.text = text;
                }

                if let Some(color) = options.color {
                    props.color = color;
                }

                if let Some(layout) = options.layout {
                    props.layout = layout;
                }

                Self {
                    id,
                    props: Arc::new(RwLock::new(props)),
                }
            }
            None => Self {
                id,
                props: Default::default(),
            },
        }
    }

    pub fn global() -> &'static Self {
        static ROOT_HANDLE: OnceCell<Handle> = OnceCell::new();
        ROOT_HANDLE.get_or_init(|| Self::new(0, None))
    }

    pub fn nest(&self, id: u64, ops: Option<HandleOptions>) -> Handle {
        match self.props.write() {
            Ok(mut props) => match props.children.get(&id) {
                Some(child) => child.clone(),
                None => {
                    // TODO: should prevent the override ROOT_HANDLE
                    let child = Handle::new(id as u64, ops);
                    props.children.insert(id, child.clone());
                    child
                }
            },
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn unnest(&self, idx: u64) -> Option<Handle> {
        match self.props.write() {
            Ok(mut props) => {
                if props.children.contains_key(&idx) {
                    props.children.remove(&idx)
                } else {
                    None
                }
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Layout {
    Vertical = 0,
    Horizontal = 1,
}

impl Default for Layout {
    fn default() -> Self {
        Self::Vertical
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Align {
    Left = 0,
    Center = 1,
    Right = 2,
}

impl Default for Align {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Idle = 0,
    Started = 1,
    Down = 2,
    Released = 3,
    Hovered = 4,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}
