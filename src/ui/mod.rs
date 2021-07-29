#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

//! Pure Rust Immediate mode UI - WIP

use primitives::color;
use std::collections::HashMap;

// Immediate Mode UI Library

mod ext;
pub use self::ext::*;

mod id;
pub use self::id::*;

mod nodes;
pub use self::nodes::*;

mod painter;
pub use self::painter::*;

mod themes;
pub use self::themes::*;

use crate::Color;

pub struct Framebuffer {
    // pub g1: Graphics,
// pub g2: Graphics,
// pub g3: Graphics,
// pub g4: Graphics,
}

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
            theme: DARK_THEME,
            window_id: 0,
            scale_factor: 1.0,
            auto_notify_input: true,
            color_wheel: None,
        }
    }
}

#[derive(Default)]
pub struct Ui {
    pub is_scrolling: bool,        // = false; // Use to limit other activities
    pub is_typing: bool,           // = false;
    pub enabled: bool,             // = true; // Current element state
    pub is_started: bool,          // = false;
    pub is_pushed: bool,           // = false;
    pub is_hovered: bool,          // = false;
    pub is_released: bool,         // = false;
    pub changed: bool,             // = false; // Global elements change check
    pub image_invert_y: bool,      // = false;
    pub scroll_enabled: bool,      // = true;
    pub always_redraw: bool,       // = false; // Hurts performance
    pub highlight_on_select: bool, // = true; // Highlight text edit contents on selection
    pub tab_switch_enabled: bool, // = true; // Allow switching focus to the next element by pressing tab
    pub window_border_top: u32,   // = 0;
    pub window_border_bottom: u32, // = 0;
    pub window_border_left: u32,  // = 0;
    pub window_border_right: u32, // = 0;
    highlight_full_row: bool,     // = false;

    // pub static current: Ui = None;
    // pub static
    on_border_hover: Option<Box<dyn Fn(&Handle, i32)>>, //->Void = None; // Mouse over window border, use for resizing
    // pub static
    on_text_hover: Option<Box<dyn Fn()>>, // = None; // Mouse over text input, use to set I-cursor
    // pub static
    always_redraw_window: bool, // = true; // Redraw cached window texture each frame or on changes only
    // pub static
    key_repeat: bool, // = true; // Emulate key repeat for non-character keys
    // pub static
    dynamic_glyph_load: bool, // = true; // Allow text input fields to push new glyphs into the font atlas

    // #if (kha_android || kha_ios || zui_touchui)
    // pub static touch_controls = true; // Pan with finger to scroll, hold finger for right click
    // #else
    // static
    pub touch_controls: bool, // = false;
    // #end
    touch_hold: bool,      // = false;
    slider_tooltip: bool,  // = false;
    slider_tooltip_x: f32, // = 0.0;
    slider_tooltip_y: f32, // = 0.0;
    slider_tooltip_w: f32, // = 0.0;

    pub input_registered: bool, // = false;
    pub input_enabled: bool,    // = true;
    pub input_x: f32,           // Input position
    pub input_y: f32,
    pub input_started_x: f32,
    pub input_started_y: f32,
    pub input_dx: f32, // Delta
    pub input_dy: f32,
    pub input_wheel_delta: i32, // = 0;
    pub input_started: bool,    // Buttons
    pub input_started_r: bool,
    pub input_released: bool,
    pub input_released_r: bool,
    pub input_down: bool,
    pub input_down_r: bool,
    pub is_key_pressed: bool,    // = false; // Keys
    pub is_key_down: bool,       // = false;
    pub is_shift_down: bool,     // = false;
    pub is_ctrl_down: bool,      // = false;
    pub is_alt_down: bool,       // = false;
    pub is_adown: bool,          // = false;
    pub is_backspace_down: bool, // = false;
    pub is_delete_down: bool,    // = false;
    pub is_escape_down: bool,    // = false;
    pub is_return_down: bool,    // = false;
    pub is_tab_down: bool,       // = false;
    // pub key: Option<KeyCode> = None;
    pub char: String,

    // static
    key_repeat_time: f32, // = 0.0;
    // static
    text_to_paste: String, // = "";
    // static
    text_to_copy: String, // = "";
    // static
    is_cut: bool, // = false;
    // static
    is_copy: bool, // = false;
    // static
    is_paste: bool, // = false;
    // static
    // copy_receiver: Ui = None;
    // static
    copy_frame: i32, // = 0;

    input_started_time: f32, // = 0.0;
    cursor_x: i32,           // = 0; // Text input
    highlight_anchor: i32,   // = 0;
    ratios: Vec<f32>,        // Splitting rows
    cur_ratio: i32,          // = -1;
    x_before_split: f32,
    w_before_split: i32,

    pub painter: Painter, // Drawing
    pub theme: Theme,     // t
    pub ops: UiOptions,
    // global_g: Graphics;
    // rtTextPipeline: graphics4.PipelineState; // Rendering text into rendertargets
    font_size: i32,
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
    scroll_align: f32,        // = 0.0;
    image_scroll_align: bool, // = true;

    _x: f32, // Cursor(stack) position
    _y: f32,
    _w: i32,
    _h: i32,

    _window_x: f32, // = 0.0; // Window state
    _window_y: f32, // = 0.0;
    _window_w: f32,
    _window_h: f32,
    window_ended: bool,   // = true
    window_header_w: f32, // = 0.0;
    window_header_h: f32, // = 0.0;
    restore_x: f32,       // = -1.0;
    restore_y: f32,       // = -1.0;

    current_window: Option<Handle>,
    scroll_handle: Option<Handle>, // = None, // Window or slider being scrolled
    drag_handle: Option<Handle>,   // = None, // Window being dragged
    text_selected_handle: Option<Handle>, // = None;
    submit_text_handle: Option<Handle>, // = None;
    tab_pressed_handle: Option<Handle>, // = None;
    combo_selected_handle: Option<Handle>, // = None;
    combo_selected_window: Option<Handle>, // = None;
    submit_combo_handle: Option<Handle>, // = None;
    tab_handle: Option<Handle>,    // = None;
    text_selected: String,
    text_to_submit: String, // = "";
    tab_pressed: bool,      // = false;

    combo_selected_align: Align,
    combo_selected_texts: Vec<String>,
    combo_selected_label: String,
    combo_selected_x: i32,
    combo_selected_y: i32,
    combo_selected_w: i32,

    combo_to_submit: i32,               // = 0;
    tooltip_text: String,               // = "";
    tooltip_img: Option<Image>,         // = None;
    tooltip_img_max_width: Option<i32>, // = None;
    tooltip_invert_y: bool,             // = false;
    tooltip_x: f32,                     // = 0.0;
    tooltip_y: f32,                     // = 0.0;
    tooltip_shown: bool,                // = false;
    tooltip_wait: bool,                 // = false;
    tooltip_time: f32,                  // = 0.0;
    tab_names: Vec<String>,             // = None; // Number of tab calls since window begin
    tab_colors: Vec<i32>,               // = None;
    tab_scroll: f32,                    // = 0.0;
    tab_vertical: bool,                 // = false;
    sticky: bool,                       // = false;
    scissor: bool,                      // = false;

    elements_baked: bool,              // = false;
    check_select_image: Option<Image>, // = None;
}

impl Ui {
    pub fn new(ops: UiOptions) -> Self {
        let mut instance = Self {
            ops,
            ..Default::default()
        };
        // instance.theme = ops.theme;
        instance.set_scale(instance.ops.scale_factor);

        if instance.ops.auto_notify_input {
            instance.register_input();
        }

        // if instance.copy_receiver == None {
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
        self.elements_baked = false;
    }

    fn bake_elements(&self) {
        if self.check_select_image.is_some() {
            // 	self.check_select_image.unload();
        }

        // self.check_select_image = Image.createRenderTarget(self.check_select_size() as i32, self.check_select_size() as i32, None, NoDepthAndStencil, 1, ops.window_id);
        // let painter = self.check_select_image.g2;

        // painter.begin(true, 0x00000000);
        // painter.color = self.theme.accent_select_col;
        // painter.drawLine(0, 0, self.check_select_image.width, self.check_select_image.height, 2 * self.scale());
        // painter.drawLine(self.check_select_image.width, 0, 0, self.check_select_image.height, 2 * self.scale());
        // painter.end();

        // self.elements_baked = true;
    }

    pub fn remove(&mut self) {
        // Clean up
        if self.ops.auto_notify_input {
            self.unregister_input();
        }
    }

    pub fn register_input(&mut self) {
        // Mouse.get().notifyWindowed(ops.khaWindowId, onMouseDown, onMouseUp, onMouseMove, onMouseWheel);
        // Keyboard.get().notify(onKeyDown, onKeyUp, onKeyPress);

        // // #if (kha_android || kha_ios)
        // // if (Surface.get() != None) Surface.get().notify(onTouchDown, onTouchUp, onTouchMove);
        // // #end

        // // Reset mouse delta on foreground
        // // System.notifyOnApplicationState(function() { inputDX = inputDY = 0; }, None, None, None, None);
        self.input_registered = true;
    }

    pub fn unregister_input(&mut self) {
        // Mouse.get().removeWindowed(ops.khaWindowId, onMouseDown, onMouseUp, onMouseMove, onMouseWheel);
        // Keyboard.get().remove(onKeyDown, onKeyUp, onKeyPress);

        // #if (kha_android || kha_ios)
        // if (Surface.get() != None) Surface.get().remove(onTouchDown, onTouchUp, onTouchMove);
        // #end
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
        if !self.elements_baked {
            self.bake_elements();
        }

        self.changed = false;
        // global_g = g;
        // current = this;
        self._x = 0.0; // Reset cursor
        self._y = 0.0;
        self._w = 0;
        self._h = 0;
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
    }

    pub fn begin_region(&mut self, /*g: Graphics,*/ x: i32, y: i32, w: i32) {
        if !self.elements_baked {
            self.painter.end();
            self.bake_elements();
            self.painter.begin(false, None);
        }
        self.changed = false;
        // self.global_g = g;
        // self.g = g;
        self.current_window = None;
        self.tooltip_text = "".into();
        self.tooltip_img = None;
        self._window_x = 0.0;
        self._window_y = 0.0;
        self._window_w = w as f32;
        self._x = x as f32;
        self._y = y as f32;
        self._w = w;
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
            self._y -= current_window.scroll_offset;
        }
    }

    pub fn end_sticky(&mut self) {
        self.sticky = false;
        self.scissor = true;
        self.painter.scissor(
            0,
            self._y as i32,
            self._window_w as i32,
            (self._window_h - self._y) as i32,
        );
        self.window_header_h += self._y - self.window_header_h;

        if let Some(ref current_window) = self.current_window {
            self._y += current_window.scroll_offset;
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

        if self.key_repeat && self.is_key_down
        // 	&& Scheduler.time() - self.key_repeat_time > 0.05
        {
            //     if self.key == KeyCode::Backspace
            //         || self.key == KeyCode::Delete
            //         || self.key == KeyCode::Left
            //         || self.key == KeyCode::Right
            //         || self.key == KeyCode::Up
            //         || self.key == KeyCode::Down
            //     {
            //         self.keyRepeatTime = Scheduler.time();
            //         self.is_key_pressed = true;
            //     }
        }

        if self.touch_controls
            && self.input_down
            && self.input_x == self.input_started_x
            && self.input_y == self.input_started_y
            && self.input_started_time > 0.0
        //     && Scheduler.time() - self.input_started_time > 0.5
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
        let wx = x + handle.drag_x;
        let wy = y + handle.drag_y;
        let input_changed = self.get_input_in_rect(wx as f32, wy as f32, w as f32, h as f32, 1.0)
            && self.input_changed();
        self.always_redraw || self.is_scrolling || self.is_typing || input_changed
    }

    // Returns true if redraw is needed
    pub fn window(
        &mut self,
        handle: &mut Handle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        drag: bool, /*= false*/
    ) -> bool {
        let mut handle = handle;
        let mut w = w;

        // if handle.texture.is_none() || w != handle.texture.width || h != handle.texture.height {
        // 	self.resize(handle, w, h, ops.khaWindowId);
        // }

        if !self.window_ended {
            self.end_window(true); // End previous window if necessary
        }

        self.window_ended = false;

        // g = handle.texture.g2; // Set g

        // self.current_window = Some(handle);
        self._window_x = (x + handle.drag_x) as f32;
        self._window_y = (y + handle.drag_y) as f32;
        self._window_w = w as f32;
        self._window_h = h as f32;
        self.window_header_w = 0.0;
        self.window_header_h = 0.0;

        if self.window_dirty(&handle, x, y, w, h) {
            handle.redraws = 2;
        }

        if let Some(ref on_border_hover) = self.on_border_hover {
            if self.get_input_in_rect(
                self._window_x - 4.0,
                self._window_y,
                8.0,
                self._window_h,
                1.0,
            ) {
                on_border_hover(&handle, 0);
            } else if self.get_input_in_rect(
                self._window_x + self._window_w - 4.0,
                self._window_y,
                8.0,
                self._window_h,
                1.0,
            ) {
                on_border_hover(&handle, 1);
            } else if self.get_input_in_rect(
                self._window_x,
                self._window_y - 4.0,
                self._window_w,
                8.0,
                1.0,
            ) {
                on_border_hover(&handle, 2);
            } else if self.get_input_in_rect(
                self._window_x,
                self._window_y + self._window_h - 4.0,
                self._window_w,
                8.0,
                1.0,
            ) {
                on_border_hover(&handle, 3);
            }
        }

        if handle.redraws <= 0 {
            return false;
        }

        self._x = 0.0;
        self._y = handle.scroll_offset;

        if handle.layout == Layout::Horizontal {
            w = self.element_w() as i32;
        }

        self._w = if !handle.scroll_enabled {
            w
        } else {
            w - self.scroll_w() // Exclude scrollbar if present
        };

        self._h = h;
        self.tooltip_text = "".into();
        self.tooltip_img = None;
        self.tab_names.clear();

        if self.theme.fill_window_bg {
            // self.painter.begin(true, Some(self.theme.window_bg_col));
        } else {
            // self.painter.begin(true, 0x00000000);
            self.painter.set_color(self.theme.window_bg_col);
            self.painter.fill_rect(
                self._x,
                self._y - handle.scroll_offset,
                handle.last_max_x,
                handle.last_max_y,
            );
        }

        handle.drag_enabled = drag;
        if drag {
            if self.input_started
                && self.get_input_in_rect(
                    self._window_x,
                    self._window_y,
                    self._window_w,
                    self.header_drag_h() as f32,
                    1.0,
                )
            {
                self.drag_handle = Some(handle.clone());
            } else if self.input_released {
                self.drag_handle = None;
            }

            // if handle == self.drag_handle {
            //     handle.redraws = 2;
            //     handle.drag_x += self.input_dx as i32;
            //     handle.drag_y += self.input_dy as i32;
            // }

            self._y += self.header_drag_h() as f32; // Header offset
            self.window_header_h += self.header_drag_h() as f32;
        }

        true
    }

    pub fn end_window(&mut self, bind_global_g: bool /* = true*/) {
        // FIXME:
        if let Some(ref mut handle) = self.current_window {
            // if handle.redraws > 0 || self.is_scrolling || self.is_typing {
            //     if self.scissor {
            //         self.scissor = false;
            //         // self.painter.disableScissor();
            //     }

            //     if self.tab_names != None {
            //         self.draw_tabs();
            //     }

            //     if handle.dragEnabled { // Draggable header
            //         self.painter.color = self.theme.separator_col;
            //         self.painter.fill_rect(0, 0, self._window_w, self.header_drag_h());
            //     }

            //     let wh = self._window_h - self.window_header_h; // Exclude header
            //     let fullHeight = self._y - handle.scroll_offset - self.window_header_h;

            //     if fullHeight < wh || handle.layout == Layout::Horizontal || !self.scroll_enabled {
            //         // Disable scrollbar
            //         handle.scrollEnabled = false;
            //         handle.scroll_offset = 0;
            //     } else { // Draw window scrollbar if necessary
            //         self.handle.scrollEnabled = true;
            //         if self.tab_scroll < 0 { // Restore tab
            //             self.handle.scroll_offset = self.tab_scroll;
            //             self.tab_scroll = 0;
            //         }
            //         let wy = self._window_y + self.window_header_h;
            //         let amountToScroll = fullHeight - wh;
            //         let amountScrolled = -self.handle.scroll_offset;
            //         let ratio = amountScrolled / amountToScroll;
            //         let barH = wh * (wh / fullHeight).abs();
            //         barH = barH.max(self.element_h());

            //         let totalScrollableArea = wh - barH;
            //         let e = amountToScroll / totalScrollableArea;
            //         let barY = totalScrollableArea * ratio + self.window_header_h;
            //         let barFocus = self.get_input_in_rect(self._window_x + self._window_w - self.scroll_w(), barY + self._window_y, self.scroll_w(), barH);

            //         if self.input_started && barFocus { // Start scrolling
            //             self.scroll_handle = self.handle;
            //             self.is_scrolling = true;
            //         }

            //         let scrollDelta: f32 = self.input_wheel_delta;
            //         if self.touch_controls && self.input_down && self.input_dy != 0 {
            //             self.is_scrolling = true;
            //             scrollDelta = -self.input_dy / 20;
            //         }
            //         if self.handle == self.scroll_handle { // Scroll
            //             self.scroll(self.input_dy * e, fullHeight);
            //         } else if scrollDelta != 0 && self.combo_selected_handle == None &&
            //             self.get_input_in_rect(self._window_x, wy, self._window_w, wh) { // Wheel
            //                 self.scroll(scrollDelta * self.element_h(), fullHeight);
            //         }

            //         // Stay in bounds
            //         if self.handle.scroll_offset > 0 {
            //             self.handle.scroll_offset = 0;
            //         } else if fullHeight + self.handle.scroll_offset < wh {
            //             self.handle.scroll_offset = wh - fullHeight;
            //         }

            //         self.painter.color = self.theme.window_bg_col; // Bg
            //         self.painter.fill_rect(_window_w - self.scroll_w(), wy, self.scroll_w(), wh);
            //         self.painter.color = self.theme.accent_col; // Bar
            //         let scrollbarFocus = self.get_input_in_rect(self._window_x + self._window_w - self.scroll_w(), wy, self.scroll_w(), wh);
            //         let barW = if (scrollbarFocus || handle == scrollHandle) {
            //             self.scroll_w() : self.scroll_w() / 3;
            //         self.painter.fill_rect(_window_w - barW - scrollAlign, barY, barW, barH);
            //     }

            //     self.handle.lastMaxX = _x;
            //     self.handle.lastMaxY = _y;
            //     if self.handle.layout == Layout::Vertical {
            //         self.handle.lastMaxX += _window_w;
            //     } else {
            //         self.handle.lastMaxY += _window_h;
            //     }

            //     self.handle.redraws--;

            //     self.painter.end();
            // }

            // self.window_ended = true;

            // // Draw window texture
            // if self.always_redraw_window || self.handle.redraws > -4 {
            //     if bindGlobalG {
            //         global_g.begin(false);
            //     }
            //     global_g.color = self.theme.window_tint_col;
            //     global_g.drawImage(self.handle.texture, self._window_x, self._window_y);
            //     if bindGlobalG {
            //         global_g.end();
            //     }

            //     if self.handle.redraws <= 0 {
            //         self.handle.redraws--;
            //     }
            // }
        }
    }

    fn scroll(&mut self, delta: f32, full_height: f32) {
        if let Some(ref mut current_window) = self.current_window {
            current_window.scroll_offset -= delta;
        }
    }

    pub fn tab(
        &mut self,
        handle: &Handle,
        text: &str,
        vertical: bool, /*= false*/
        color: i32,     /* = -1*/
    ) -> bool {
        if self.tab_names.len() == 0 {
            // First tab
            // self.tab_names = [];
            // self.tab_colors = [];
            // self.tab_handle = handle;
            self.tab_vertical = vertical;
            self._w -= if vertical {
                (self.element_offset() + self.element_w() - 1.0 * self.scale()) as i32
            } else {
                0 // Shrink window area by width of vertical tabs
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
                && self.get_input_in_rect(
                    self._window_x,
                    self._window_y,
                    self._window_w,
                    self.window_header_h,
                    1.0,
                )
            {
                self.input_x = -1.0;
                self.input_y = -1.0;
            }
            if vertical {
                self._x += self.window_header_w + 6.0;
                self._w -= 6;
            } else {
                self._y += self.window_header_h + 3.0;
            }
        }
        self.tab_names.push(text.into());
        self.tab_colors.push(color);

        handle.position == self.tab_names.len() as i32 - 1
    }

    fn draw_tabs(&mut self) {
        self.input_x = self.restore_x;
        self.input_y = self.restore_y;

        if let Some(ref current_window) = self.current_window {
            let tabX = 0.0;
            let tabY = 0.0;
            let tabHMin = (self.button_h() * 1.1) as i32;
            let header_h = if current_window.drag_enabled {
                self.header_drag_h()
            } else {
                0
            };
            let tabH = if self.theme.full_tabs && self.tab_vertical {
                ((self._window_h - header_h as f32) / self.tab_names.len() as f32) as i32
            } else {
                tabHMin
            };

            let origy = self._y;
            self._y = header_h as f32;
            if let Some(ref mut tab_handle) = self.tab_handle {
                tab_handle.changed = false;
            }

            // if self.is_ctrl_down && self.is_tab_down { // Next tab
            //     self.tab_handle.position += 1;
            //     if self.tab_handle.position >= self.tab_hames.len() {
            //         self.tab_handle.position = 0;
            //     }
            //     self.tab_handle.changed = true;
            //     self.is_tab_down = false;
            // }

            // if self.tab_handle.position >= self.tab_hames.len() {
            //     self.tab_handle.position = self.tab_hames.len() - 1;
            // }

            // self.painter.color = self.theme.separator_col; // Tab background

            // if self.tab_vertical {
            //     self.painter.fill_rect(0, self._y, self.element_w(), self._window_h);
            // } else {
            //     self.painter.fill_rect(0, self._y, self._window_w, self.button_offset_y + tabH + 2);
            // }

            // self.painter.color = self.theme.accent_col; // Underline tab buttons

            // if self.tab_vertical {
            //     self.painter.fill_rect(self.element_w(), self._y, 1, self._window_h);
            // } else {
            //     self.painter.fill_rect(self.button_offset_y, self._y + self.button_offset_y + tabH + 2, self._window_w - self.button_offset_y * 2, 1);
            // }

            // let basey = if self.tab_vertical {
            //     self._y
            // } else {
            //     self._y + 2
            // };

            // for i in 0..self.tab_hames.len() {
            //     self._x = tabX;
            //     self._y = basey + tabY;
            //     self._w = if self.tab_vertical {
            //         (self.element_w() - 1 * self.scale()) as i32
            //     } else  {
            //         if self.theme.full_tabs {
            //             (self._window_w / self.tab_hames.len()) as i32
            //         } else {
            //             (self.ops.font.width(self.font_size, self.tab_hames[i]) + self.button_offset_y * 2 + 18 * self.scale()) as i32
            //         }
            //     };

            //     let released = self.get_released(tabH);
            //     let pushed = self.get_pushed(tabH);
            //     let hover = self.get_hover(tabH);

            //     if released {
            //         let h = self.tab_handle.nest(self.tab_handle.position); // Restore tab scroll
            //         h.scroll_offset = self.current_window.scroll_offset;
            //         h = self.tab_handle.nest(i);
            //         self.tab_scroll = h.scroll_offset;
            //         tab_handle.position = i; // Set new tab
            //         current_window.redraws = 3;
            //         tab_handle.changed = true;
            //     }

            //     let selected = tab_handle.position == i;

            //     self.painter.color = if pushed || hover {
            //         self.theme.button_hover_col
            //     } else {
            //         if self.tab_colors[i] != -1 {
            //             self.tab_colors[i]
            //         } else {
            //             if selected {
            //                 self.theme.window_bg_col
            //             } else {
            //                 self.theme.separator_col;
            //             }
            //         }
            //     };

            //     if self.tab_vertical {
            //         tabY += tabH + 1;
            //     } else {
            //         tabX += _w + 1;
            //     }

            //     self.draw_rect(g, true, _x + self.button_offset_y, _y + self.button_offset_y, self._w, tabH);

            //     self.painter.color = if selected {
            //         self.theme.button_text_col
            //     } else {
            //         self.theme.label_col
            //     };

            //     self.draw_string(g, self.tab_hames[i], None, (tabH - tabHMin) / 2, self.theme.full_tabs ? Align::Center : Align::Left);

            //     if selected && !self.tab_vertical { // Hide underline for active tab
            //         self.painter.color = self.theme.window_bg_col;
            //         self.painter.fill_rect(self._x + self.button_offset_y + 1, self._y + self.button_offset_y + tabH, self._w - 1, 1);
            //     }
            // }
            // self._x = 0; // Restore positions
            // self._y = origy;

            // self._w = if !current_window.scrollEnabled {
            //     self._window_w
            // } else {
            //     self._window_w - self.scroll_w() as i32
            // }
        }
    }

    pub fn panel(
        &mut self,
        handle: &mut Handle,
        text: &str,
        is_tree: bool, /* = false*/
        filled: bool,  /* = true*/
        pack: bool,    /*= true*/
    ) -> bool {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);
            return handle.selected;
        }

        if self.get_released(-1.0) {
            handle.selected = !handle.selected;
            handle.changed = true;
            self.changed = true;
        }

        if filled {
            self.painter.set_color(self.theme.panel_bg_col);
            self.draw_rect(
                true,
                self._x,
                self._y,
                self._w as f32,
                self.element_h(),
                0.0,
            );
        }

        if is_tree {
            self.draw_tree(handle.selected);
        } else {
            self.draw_arrow(handle.selected);
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
        if pack && !handle.selected {
            self._y -= self.element_offset();
        }

        handle.selected
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

        let mut w = iw.min(self._w as f32);
        let mut x = self._x;

        let scroll = match self.current_window {
            Some(ref current_window) => current_window.scroll_enabled,
            None => false,
        };

        let r = if self.cur_ratio == -1 {
            1.0
        } else {
            self.get_ratio(self.ratios[self.cur_ratio as usize], 1.0)
        };

        if self.image_scroll_align {
            // Account for scrollbar size
            w = iw.min(self._w as f32 - self.button_offset_y * 2.0);
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

        let mut started = self.get_started(h);
        let mut down = self.get_pushed(h);
        let mut released = self.get_released(h);
        let mut hover = self.get_hover(h);

        if self.cur_ratio == -1 && (started || down || released || hover) {
            if self.input_x < self._window_x + self._x
                || self.input_x > self._window_x + self._x + w
            {
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
                    self._y + h_float,
                    w,
                    -h_float,
                )
            } else {
                self.painter.draw_scaled_subimage(
                    image, sx as f32, sy as f32, sw as f32, sh as f32, x, self._y, w, h_float,
                );
            }
        } else {
            if self.image_invert_y {
                self.painter
                    .draw_scaled_image(image, x, self._y + h_float, w, -h_float)
            } else {
                self.painter
                    .draw_scaled_image(image, x, self._y, w, h_float);
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

    pub fn text(
        &mut self,
        text: &str,
        align: Align, /* = Align::Left*/
        bg: Color,    /*= 0x00000000*/
    ) -> State {
        if text.contains("\n") {
            self.split_text(text, align, bg);
            return State::Idle;
        }

        let h = self.element_h().max(self.ops.font.height(self.font_size));
        if !self.is_visible(h) {
            self.end_element(Some(h + self.element_offset()));
            return State::Idle;
        }

        let started = self.get_started(h);
        let down = self.get_pushed(h);
        let released = self.get_released(h);
        let hover = self.get_hover(h);
        // if bg != 0x0000000 {
        //     self.painter.set_color(bg);
        //     self.painter.fill_rect(
        //         self._x + self.button_offset_y,
        //         self._y + self.button_offset_y,
        //         self._w as f32 - self.button_offset_y * 2.0,
        //         self.button_h(),
        //     );
        // }

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

    fn split_text(
        &mut self,
        lines: &str,
        align: Align, /*= Align::Left*/
        bg: Color,    /*= 0x00000000*/
    ) {
        for line in lines.split("\n") {
            self.text(line, align, bg);
        }
    }

    fn start_text_edit(&mut self, handle: &Handle) {
        self.is_typing = true;
        self.submit_text_handle = self.text_selected_handle.clone();
        self.text_to_submit = self.text_selected.clone();
        // self.text_selected_handle = handle;
        self.text_selected = handle.text.clone();
        // self.cursor_x = handle.text.len();
        if self.tab_pressed {
            self.tab_pressed = false;
            self.is_key_pressed = false; // Prevent text deselect after tab press
        } else if !self.highlight_on_select {
            // Set cursor to click location
            let x = self.input_x - (self._window_x + self._x + self.text_offset());
            self.cursor_x = 0;

            // while self.cursor_x < self.text_selected.len()
            //     && self
            //         .ops
            //         .font
            //         .width(self.font_size, self.text_selected.substr(0, self.cursor_x))
            //         < x
            // {
            //     self.cursor_x += 1;
            // }
        }

        // self.tab_pressed_handle = Some(handle);
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
            submit_text_handle.text = self.text_to_submit.clone();
            submit_text_handle.changed = true;
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
        let text = self.text_selected.clone();
        if self.is_key_pressed {
            // // Process input
            // if self.key == KeyCode::Left {
            //     // Move cursor
            //     if self.cursor_x > 0 {
            //         self.cursor_x -= 1;
            //     }
            // } else if self.key == KeyCode::Right {
            //     if self.cursor_x < text.len() {
            //         self.cursor_x += 1;
            //     }
            // } else if editable && self.key == KeyCode::Backspace {
            //     // Remove char
            //     if self.cursor_x > 0 && self.highlight_anchor == self.cursor_x {
            //         text =
            //             text.substr(0, self.cursor_x - 1) + text.substr(self.cursor_x, text.len());
            //         self.cursor_x -= 1;
            //     } else if self.highlight_anchor < self.cursor_x {
            //         text = text.substr(0, self.highlight_anchor)
            //             + text.substr(self.cursor_x, text.len());
            //         self.cursor_x = self.highlight_anchor;
            //     } else {
            //         text = text.substr(0, self.cursor_x)
            //             + text.substr(self.highlight_anchor, text.len());
            //     }
            // } else if editable && self.key == KeyCode::Delete {
            //     if self.highlight_anchor == self.cursor_x {
            //         text = text.substr(0, self.cursor_x) + text.substr(self.cursor_x + 1);
            //     } else if self.highlight_anchor < self.cursor_x {
            //         text = text.substr(0, self.highlight_anchor)
            //             + text.substr(self.cursor_x, text.len());
            //         self.cursor_x = self.highlight_anchor;
            //     } else {
            //         text = text.substr(0, self.cursor_x)
            //             + text.substr(self.highlight_anchor, text.len());
            //     }
            // } else if self.key == KeyCode::Return {
            //     // Deselect
            //     self.deselect_text();
            // } else if self.key == KeyCode::Escape {
            //     // Cancel
            //     self.text_selected = self.text_selected_handle.text;
            //     self.deselect_text();
            // } else if self.key == KeyCode::Tab && self.tab_switch_enabled && !self.is_ctrl_down {
            //     // Next field
            //     self.tab_pressed = true;
            //     self.deselect_text();
            //     self.key = None;
            // } else if self.key == KeyCode::Home {
            //     self.cursor_x = 0;
            // } else if self.key == KeyCode::End {
            //     self.cursor_x = text.len();
            // } else if self.is_ctrl_down && self.is_adown {
            //     // Select all
            //     self.cursor_x = text.len();
            //     self.highlight_anchor = 0;
            // } else if editable && // Write
            //     self.key != KeyCode::Shift &&
            //     self.key != KeyCode::CapsLock &&
            //     self.key != KeyCode::Control &&
            //     self.key != KeyCode::Meta &&
            //     self.key != KeyCode::Alt &&
            //     self.key != KeyCode::Up &&
            //     self.key != KeyCode::Down &&
            //     self.char != None &&
            //     self.char != "" &&
            //     self.char.charCodeAt(0) >= 32
            // {
            //     text =
            //         text.substr(0, self.highlight_anchor) + self.char + text.substr(self.cursor_x);
            //     self.cursor_x = if self.cursor_x + 1 > text.len() {
            //         text.len()
            //     } else {
            //         self.cursor_x + 1
            //     };

            //     // if self.dynamic_glyph_load && self.char.charCodeAt(0) > 126 && Graphics.fontGlyphs.indexOf(self.char.charCodeAt(0)) == -1 {
            //     // 	Graphics.fontGlyphs.push(self.char.charCodeAt(0));
            //     // 	Graphics.fontGlyphs = Graphics.fontGlyphs.copy(); // Trigger atlas update
            //     // }
            // }

            // let selecting = self.is_shift_down
            //     && (self.key == KeyCode::Left
            //         || self.key == KeyCode::Right
            //         || self.key == KeyCode::Shift);
            // if !selecting && !self.is_ctrl_down {
            //     self.highlight_anchor = self.cursor_x;
            // }
        }

        if self.text_to_paste != "" {
            // // Process cut copy paste
            // text = text.substr(0, self.highlight_anchor)
            //     + self.text_to_paste
            //     + text.substr(self.cursor_x);
            // self.cursor_x += self.text_to_paste.len();
            // self.highlight_anchor = self.cursor_x;
            // self.text_to_paste = "";
            // self.is_paste = false;
        }

        if self.highlight_anchor == self.cursor_x {
            self.text_to_copy = text.clone(); // Copy
        } else if self.highlight_anchor < self.cursor_x {
            // self.text_to_copy = text.substring(self.highlight_anchor, self.cursor_x);
        } else {
            // self.text_to_copy = text.substring(self.cursor_x, self.highlight_anchor);
        }

        if self.is_cut {
            // Cut
            if self.highlight_anchor == self.cursor_x {
                // text = "";
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
            // let mut istart = self.cursor_x;
            // let mut iend = self.highlight_anchor;

            // if self.highlight_anchor < self.cursor_x {
            //     istart = self.highlight_anchor;
            //     iend = self.cursor_x;
            // }

            // let hlstr = text.substr(istart, iend - istart);
            // let hlstrw = self.ops.font.width(self.font_size, hlstr);
            // let startoff = self.ops.font.width(self.font_size, text.substr(0, istart));
            // let hlStart = if align == Align::Left {
            //     self._x + startoff + off
            // } else {
            //     self._x + self._w - hlstrw - off
            // };

            // if align == Align::Right {
            //     hlStart -= self
            //         .ops
            //         .font
            //         .width(self.font_size, text.substr(iend, text.len()));
            // }

            // self.painter.set_color(self.theme.accent_select_col);
            // self.painter
            //     .fill_rect(hlStart, self._y + self.button_offset_y * 1.5, hlstrw, cursorHeight);
        }

        // Flash cursor
        // let time = Scheduler.time();
        // if self.is_key_down || time % (self.flash_speed() * 2.0) < self.flash_speed() {
        //     let str = if align == Align::Left {
        //         text.substr(0, self.cursor_x)
        //     } else {
        //         text.substring(self.cursor_x, text.len())
        //     };
        //     let strw = self.ops.font.width(self.font_size, str);
        //     let cursor_x = if align == Align::Left {
        //         self._x + strw + off
        //     } else {
        //         self._x + self._w - strw - off
        //     };
        //     self.painter.set_color(self.theme.text_col); // Cursor
        //     self.painter.fill_rect(
        //         cursor_x,
        //         self._y + self.button_offset_y * 1.5,
        //         1 * self.scale(),
        //         cursorHeight,
        //     );
        // }

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
            return handle.text.clone();
        }

        let hover = self.get_hover(-1.0);
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
            self._x + self.button_offset_y,
            self._y + self.button_offset_y,
            self._w as f32 - self.button_offset_y * 2.0,
            self.button_h(),
            0.0,
        );

        let startEdit = self.get_released(-1.0) || self.tab_pressed;

        // if self.text_selected_handle != handle && startEdit {
        //     self.start_text_edit(handle);
        // }
        // if self.text_selected_handle == handle {
        //     self.update_text_edit(align, editable);
        // }

        // if self.submit_text_handle == handle {
        //     self.submit_text_edit();
        // } else {
        //     handle.changed = false;
        // }

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

        // if self.text_selected_handle != handle {
        //     self.draw_string(handle.text.as_str(), None, 0.0, align, true);
        // } else {
        //     self.draw_string(self.text_selected.as_str(), None, 0.0, align, false);
        // }

        self.end_element(None);
        handle.text.clone()
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
            current_window.redraws = 2;
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

        let released = self.get_released(-1.0);
        let pushed = self.get_pushed(-1.0);
        let hover = self.get_hover(-1.0);

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
            self._x + self.button_offset_y,
            self._y + self.button_offset_y,
            self._w as f32 - self.button_offset_y * 2.0,
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

    pub fn check(&mut self, handle: &mut Handle, text: &str) -> bool {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);
            return handle.selected;
        }

        if self.get_released(-1.0) {
            handle.selected = !handle.selected;
            handle.changed = true;
            self.changed = true;
        } else {
            handle.changed = false;
        }

        let hover = self.get_hover(-1.0);

        // Check
        self.draw_check(handle.selected, hover);

        // Text
        self.painter.set_color(self.theme.text_col);
        self.draw_string(text, Some(self.title_offset_x), 0.0, Align::Left, true);

        self.end_element(None);

        handle.selected
    }

    pub fn radio(&mut self, handle: &mut Handle, position: i32, text: &str) -> bool {
        if !self.is_visible(self.element_h()) {
            self.end_element(None);
            return handle.position == position;
        }

        if position == 0 {
            handle.changed = false;
        }

        if self.get_released(-1.0) {
            handle.position = position;
            handle.changed = true;
            self.changed = true;
        }

        let hover = self.get_hover(-1.0);

        // Radio
        self.draw_radio(handle.position == position, hover);

        // Text
        self.painter.set_color(self.theme.text_col);
        self.draw_string(text, Some(self.title_offset_x), 0.0, Align::Left, true);

        self.end_element(None);

        handle.position == position
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
            return handle.position;
        }

        if self.get_released(-1.0) {
            if self.combo_selected_handle.is_none() {
                self.input_enabled = false;
                self.combo_selected_handle = Some(handle.clone());
                self.combo_selected_window = self.current_window.clone();
                self.combo_selected_align = align;
                self.combo_selected_texts = texts;
                self.combo_selected_label = label.into();
                self.combo_selected_x = (self._x + self._window_x) as i32;
                self.combo_selected_y = (self._y + self._window_y + self.element_h()) as i32;
                self.combo_selected_w = self._w as i32;

                // Adapt combo list width to combo item width
                // for t in texts {
                //     let w = self.ops.font.width(self.font_size, t) as i32 + 10;

                //     if self.combo_selected_w < w {
                //         self.combo_selected_w = w;
                //     }
                // }

                if self.combo_selected_w > self._w * 2 {
                    self.combo_selected_w = (self._w as f32 * 2.0) as i32;
                }

                if self.combo_selected_w > self._w {
                    self.combo_selected_w += self.text_offset() as i32;
                }
                self.combo_to_submit = handle.position;
            }
        }

        // if handle == self.submit_combo_handle {
        //     handle.position = self.combo_to_submit;
        //     self.submit_combo_handle = None;
        //     handle.changed = true;
        //     self.changed = true;
        // } else {
        //     handle.changed = false;
        // }

        let hover = self.get_hover(-1.0);
        if hover {
            // Bg
            self.painter.set_color(self.theme.accent_hover_col);
            self.draw_rect(
                self.theme.fill_accent_bg,
                self._x + self.button_offset_y,
                self._y + self.button_offset_y,
                self._w as f32 - self.button_offset_y * 2.0,
                self.button_h(),
                0.0,
            );
        } else {
            self.painter.set_color(self.theme.accent_col);
            self.draw_rect(
                self.theme.fill_accent_bg,
                self._x + self.button_offset_y,
                self._y + self.button_offset_y,
                self._w as f32 - self.button_offset_y * 2.0,
                self.button_h(),
                0.0,
            );
        }

        let x = self._x + self._w as f32 - self.arrow_offset_x - 8.0;
        let y = self._y + self.arrow_offset_y + 3.0;
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
                self._x -= 15.0;
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
                self._x += 15.0;
            }
        }

        if align == Align::Right {
            self._x -= 15.0;
        }

        self.painter.set_color(self.theme.text_col); // Value
                                                     // if handle.position < texts.len() as i32 {
                                                     //     self.draw_string( texts[handle.position as usize].as_str(), None, 0.0, align, true);
                                                     // }

        if align == Align::Right {
            self._x += 15.0;
        }

        self.end_element(None);

        handle.position
    }

    pub fn slider(
        &mut self,
        handle: &mut Handle,
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
            return handle.value;
        }

        if self.get_started(-1.0) {
            // self.scroll_handle = handle;
            self.is_scrolling = true;
            if self.touch_controls {
                self.slider_tooltip = true;
                self.slider_tooltip_x = self._x + self._window_x;
                self.slider_tooltip_y = self._y + self._window_y;
                self.slider_tooltip_w = self._w as f32;
            }
        }

        // handle.changed = false;
        // if handle == self.scroll_handle { // Scroll
        // 	let range = to - from;
        // 	let sliderX = self._x + self._window_x + self.button_offset_y;
        // 	let sliderW = self._w as f32 - self.button_offset_y * 2.0;
        // 	let step = range / sliderW;
        // 	let value = from + (self.input_x - sliderX) * step;
        // 	handle.value = (value * precision).round() / precision;

        // 	if handle.value < from {
        //         handle.value = from; // Stay in bounds
        //     } else if handle.value > to {
        //         handle.value = to;
        //     }

        // 	handle.changed = true;
        //     self.changed = true;
        // }

        let hover = self.get_hover(-1.0);
        self.draw_slider(handle.value, from, to, filled, hover); // Slider

        // Text edit
        let startEdit = (self.get_released(-1.0) || self.tab_pressed) && text_edit;
        if startEdit {
            // Mouse did not move
            // handle.text = handle.value + "";
            self.start_text_edit(handle);
            handle.changed = true;
            self.changed = true;
        }

        let lalign = if align == Align::Left {
            Align::Right
        } else {
            Align::Left
        };

        // if self.text_selected_handle == handle {
        // 	self.update_text_edit(lalign, true);
        // }

        // if self.submit_text_handle == handle {
        // 	self.submit_text_edit();
        // 	#if js
        // 	try {
        // 		handle.value = js.Lib.eval(handle.text);
        // 	}
        // 	catch(_) {}
        // 	#else
        // 	handle.value = Std.parseFloat(handle.text);
        // 	#end
        // 	handle.changed = true;
        //     self.changed = true;
        // }

        self.painter.set_color(self.theme.label_col); // Text
        self.draw_string(text, None, 0.0, align, true);

        if display_value {
            self.painter.set_color(self.theme.text_col); // Value

            // if self.text_selected_handle != handle {
            // 	self.draw_string(((handle.value * precision).round() / precision) + "", None, 0.0, lalign, true)
            // } else {
            // 	self.draw_string(self.text_selected.as_str(), None, 0.0, lalign, true);
            // }
        }

        self.end_element(None);
        handle.value
    }

    pub fn separator(&mut self, h: u32 /* = 4*/, fill: bool /* = true*/) {
        if !self.is_visible(self.element_h()) {
            self._y += h as f32 * self.scale();
            return;
        }

        if fill {
            self.painter.set_color(self.theme.separator_col);
            self.painter
                .fill_rect(self._x, self._y, self._w as f32, h as f32 * self.scale());
        }

        self._y += h as f32 * self.scale();
    }

    pub fn tooltip(&mut self, text: &str) {
        self.tooltip_text = text.into();
        self.tooltip_y = self._y + self._window_y;
    }

    pub fn tooltip_image(&mut self, image: Image, max_width: Option<i32> /*= None*/) {
        self.tooltip_img = Some(image);
        self.tooltip_img_max_width = max_width;
        self.tooltip_invert_y = self.image_invert_y;
        self.tooltip_y = self._y + self._window_y;
    }

    fn draw_arrow(&self, selected: bool) {
        let x = self._x + self.arrow_offset_x;
        let y = self._y + self.arrow_offset_y;

        self.painter.set_color(self.theme.text_col);

        if selected {
            self.painter.fill_triangle(
                x,
                y,
                x + self.arrow_size(),
                y,
                x + self.arrow_size() / 2.0,
                y + self.arrow_size(),
            );
        } else {
            self.painter.fill_triangle(
                x,
                y,
                x,
                y + self.arrow_size(),
                x + self.arrow_size(),
                y + self.arrow_size() / 2.0,
            );
        }
    }

    fn draw_tree(&self, selected: bool) {
        let sign_w = 7.0 * self.scale();
        let x = self._x + self.arrow_offset_x + 1.0;
        let y = self._y + self.arrow_offset_y + 1.0;

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
        let x = self._x + self.check_offset_x;
        let y = self._y + self.check_offset_y;

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

            let size = self.check_select_size() as i32;

            match self.check_select_image {
                Some(ref image) => {
                    self.painter.draw_scaled_image(
                        image,
                        x + self.check_select_offset_x,
                        y + self.check_select_offset_y,
                        size as f32,
                        size as f32,
                    );
                }
                None => {
                    println!("There no check_select_image")
                }
            }
        }
    }

    fn draw_radio(&self, selected: bool, hover: bool) {
        let x = self._x + self.radio_offset_x;
        let y = self._y + self.radio_offset_y;

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
        let x = self._x + self.button_offset_y;
        let y = self._y + self.button_offset_y;
        let w = self._w as f32 - self.button_offset_y * 2.0;

        self.painter.set_color(if hover {
            self.theme.accent_hover_col
        } else {
            self.theme.accent_col
        });

        self.draw_rect(self.theme.fill_accent_bg, x, y, w, self.button_h(), 0.0); // Bg

        self.painter.set_color(if hover {
            self.theme.accent_hover_col
        } else {
            self.theme.accent_col
        });

        let offset = (value - from) / (to - from);
        let barW = 8.0 * self.scale(); // Unfilled bar
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
        //     let arrowUp = self.is_key_pressed && self.key == (if unrollUp { KeyCode::Down} else { KeyCode::Up});
        //     let arrowDown = self.is_key_pressed && self.key == (if unrollUp { KeyCode::Up} else { KeyCode::Down});
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

        // let unrollRight = if self._x + self.combo_selected_w * 2
        //     < System.windowWidth() - self.window_border_right
        // {
        //     1
        // } else {
        //     -1
        // };

        // for i in 0..self.combo_selected_texts.len() {
        //     if unrollUp {
        //         self._y -= self.element_h() * 2.0;
        //     }

        //     self.theme.button_col = if i == self.combo_selected_handle.position {
        //         self.theme.accent_select_col
        //     } else {
        //         self.theme.separator_col
        //     };
        //     self.fill(
        //         0,
        //         0,
        //         self._w / self.scale(),
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
        //     if self._y + self.element_h() > System.windowHeight() - self.window_border_bottom
        //         || self._y - self.element_h() * 2.0 < self.window_border_top
        //     {
        //         self._x += (self.combo_selected_w * unrollRight) as f32; // Next column
        //         self._y = self.combo_selected_y as f32;
        //     }
        // }
        // self.theme.button_col = _BUTTON_COL;
        // self.theme.element_offset = _ELEMENT_OFFSET;

        // if !self.combo_selected_label.is_empty() {
        //     // Unroll down
        //     if unrollUp {
        //         self._y -= self.element_h() * 2.0;
        //         self.fill(
        //             0.0,
        //             0.0,
        //             self._w as f32 / self.scale(),
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
        //         self._y += self.element_h();
        //         self.fill(
        //             0,
        //             0,
        //             self._w / self.scale(),
        //             1 * self.scale(),
        //             self.theme.accent_select_col,
        //         ); // Separator
        //     } else {
        //         self.fill(
        //             0,
        //             0,
        //             self._w / self.scale(),
        //             self.element_h() / self.scale(),
        //             self.theme.separator_col,
        //         );
        //         self.fill(
        //             0,
        //             0,
        //             self._w / self.scale(),
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
        // 		self.tooltip_time = Scheduler.time();
        // 	}
        // 	if !self.tooltip_wait && Scheduler.time() - self.tooltip_time > self.tooltip_delay() {
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
        /*g: Graphics,*/ text: &str,
        x_offset: Option<f32>, /*= None*/
        y_offset: f32,         /*= 0*/
        align: Align,          /*= Align::Left*/
        truncation: bool,      /* = true*/
    ) {
        // let fullText = text;
        // if truncation {
        // 	while text.len() > 0 && self.ops.font.width(self.font_size, text) > self._w - 6 {
        // 		text = text.substr(0, text.len() - 1);
        // 	}

        // 	if text.len() < fullText.len() {
        // 		text += "..";
        // 		if self.is_hovered {
        //             self.tooltip(fullText);
        //         }
        // 	}
        // }

        // if x_offset.is_none() {
        //     x_offset = self.theme.text_offset;
        // }

        // x_offset *= self.scale();
        // self.painter.set_font(self.ops.font);
        // self.painter.set_font_size(self.font_size);

        // if align == Align::Center {
        //     x_offset = self._w / 2.0 - self.ops.font.width(self.font_size, text) / 2;
        // } else if align == Align::Right {
        //     x_offset = self._w - self.ops.font.width(self.font_size, text) - self.text_offset();
        // }

        // if !self.enabled {
        //     self.fade_color();
        // }
        // self.painter.pipeline = rtTextPipeline;
        // self.painter.draw_string(text, self._x + x_offset, self._y + self.font_offset_y + yOffset);
        // self.painter.pipeline = None;
    }

    fn end_element(&mut self, element_size: Option<f32> /* = None*/) {
        let element_size = match element_size {
            Some(val) => val,
            None => self.element_h() + self.element_offset(),
        };

        let condition = match self.current_window {
            Some(ref current_window) => {
                if current_window.layout == Layout::Vertical {
                    true
                } else {
                    false
                }
            }
            None => true,
        };

        if condition {
            if self.cur_ratio == -1
                || (self.ratios.len() > 0 && self.cur_ratio == self.ratios.len() as i32 - 1)
            {
                // New line
                self._y += element_size;

                if self.ratios.len() > 0 && self.cur_ratio == self.ratios.len() as i32 - 1 {
                    // Last row element
                    self.cur_ratio = -1;
                    self.ratios.clear();
                    self._x = self.x_before_split;
                    self._w = self.w_before_split;
                    self.highlight_full_row = false;
                }
            } else {
                // Row
                self.cur_ratio += 1;
                self._x += self._w as f32; // More row elements to place
                self._w = self.get_ratio(
                    self.ratios[self.cur_ratio as usize] as f32,
                    self.w_before_split as f32,
                ) as i32;
            }
        } else {
            // Horizontal
            self._x += self._w as f32 + self.element_offset();
        }
    }

    // Highlight all upcoming elements in the next row on a `mouse-over` event.
    #[inline]
    pub fn highlight_next_row(&mut self) {
        self.highlight_full_row = true;
    }

    #[inline]
    fn get_ratio(&self, ratio: f32, factor: f32) -> f32 {
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
        self.x_before_split = self._x;
        self.w_before_split = self._w;
        // self._w = getRatio(ratios[curRatio], self._w) as i32;
        unimplemented!()
    }

    pub fn indent(&mut self, both_sides: bool /* = true*/) {
        self._x += self.tab_w() as f32;
        self._w -= self.tab_w();
        if both_sides {
            self._w -= self.tab_w();
        }
    }

    pub fn unindent(&mut self, both_sides: bool /* = true*/) {
        self._x -= self.tab_w() as f32;
        self._w += self.tab_w();

        if both_sides {
            self._w += self.tab_w();
        }
    }

    fn fade_color(&self) {
        // self.painter.color = Color.fromFloats(self.painter.color.R, self.painter.color.G, self.painter.color.B, 0.25);
        unimplemented!()
    }

    pub fn fill(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // self.painter.color = color;
        if !self.enabled {
            self.fade_color();
        }
        // self.painter.fill_rect(_x + x * self.scale(), _y + y * self.scale() - 1, w * self.scale(), h * self.scale());
        // self.painter.color = 0xffffffff;
        unimplemented!()
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
        // self.painter.color = color;
        if !self.enabled {
            self.fade_color();
        }
        // self.painter.drawRect(_x + x * self.scale(), _y + y * self.scale(), w * self.scale(), h * self.scale(), strength);
        // self.painter.color = 0xffffffff;
        unimplemented!()
    }

    #[inline]
    fn draw_rect(
        &self,
        /*g: Graphics,*/ fill: bool,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        strength: f32, /* = 0.0*/
    ) {
        let strength = if strength == 0.0 { 1.0 } else { strength };

        if !self.enabled {
            self.fade_color();
        }
        // fill ? self.painter.fill_rect(x, y - 1, w, h + 1) : self.painter.drawRect(x, y, w, h, strength);
        unimplemented!()
    }

    fn is_visible(&self, elem_h: f32) -> bool {
        // if (current_window == None) return true;
        // return (_y + elem_h > windowHeaderH && _y < current_window.texture.height);
        unimplemented!()
    }

    fn get_released(&mut self, elem_h: f32 /*= -1.0*/) -> bool {
        // Input selection
        self.is_released = self.enabled
            && self.input_enabled
            && self.input_released
            && self.get_hover(elem_h)
            && self.get_initial_hover(elem_h);
        self.is_released
    }

    fn get_pushed(&mut self, elem_h: f32 /* = -1.0*/) -> bool {
        self.is_pushed = self.enabled
            && self.input_enabled
            && self.input_down
            && self.get_hover(elem_h)
            && self.get_initial_hover(elem_h);
        self.is_pushed
    }

    fn get_started(&mut self, elem_h: f32 /* = -1.0*/) -> bool {
        self.is_started =
            self.enabled && self.input_enabled && self.input_started && self.get_hover(elem_h);
        self.is_started
    }

    fn get_initial_hover(&self, elem_h: f32 /* = -1.0*/) -> bool {
        if self.scissor && self.input_y < self._window_y + self.window_header_h {
            return false;
        }

        let elem_h = if elem_h == -1.0 {
            self.element_h()
        } else {
            elem_h
        };

        // return enabled && inputEnabled &&
        // 	inputStartedX >= _window_x + _x && inputStartedX < (_window_x + _x + _w) &&
        // 	inputStartedY >= _window_y + _y && inputStartedY < (_window_y + _y + elem_h);
        unimplemented!()
    }

    fn get_hover(&mut self, elem_h: f32 /* = -1.0*/) -> bool {
        if self.scissor && self.input_y < self._window_y + self.window_header_h {
            return false;
        }

        let elem_h = if elem_h == -1.0 {
            self.element_h()
        } else {
            elem_h
        };

        // self.is_hovered = self.enabled && self.input_enabled &&
        // 	self.input_x >= self._window_x + (self.highlight_full_row ? 0 : _x) && self.input_x < (_window_x + _x + (highlight_full_row ? _window_w : _w)) &&
        // 	self.input_y >= self._window_y + self._y && self.input_y < (self._window_y + self._y + elem_h);
        // self.is_hovered
        unimplemented!()
    }

    fn get_input_in_rect(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        scale: f32, /* = 1.0*/
    ) -> bool {
        //  self.enabled && self.input_enabled &&
        // 	self.input_x >= x * scale && self.input_x < (x + w) * scale &&
        // 	self.input_y >= y * scale && self.input_y < (y + h) * scale;
        unimplemented!()
    }

    pub fn on_mouse_down(&mut self, button: i32, x: i32, y: i32) {
        // Input events
        if button == 0 {
            self.input_started = true;
        } else {
            self.input_started_r = true;
        }

        if button == 0 {
            self.input_down = true;
        } else {
            self.input_down_r = true;
        }

        // self.inputStartedTime = Scheduler.time();

        // #if (kha_android || kha_ios)
        // setInputPosition(x, y);
        // #end
        self.input_started_x = x as f32;
        self.input_started_y = y as f32;
    }

    pub fn on_mouse_up(&mut self, button: i32, x: i32, y: i32) {
        if self.is_scrolling {
            // Prevent action when scrolling is active
            self.is_scrolling = false;
            // self.scroll_handle = None;
            self.slider_tooltip = false;

            if x == self.input_started_x as i32 && y == self.input_started_y as i32 {
                // Mouse not moved
                if button == 0 {
                    self.input_released = true;
                } else {
                    self.input_released_r = true;
                }
            }
        } else {
            if button == 0 {
                self.input_released = true;
            } else {
                self.input_released_r = true;
            }
        }

        if button == 0 {
            self.input_down = false;
        } else {
            self.input_down_r = false;
        }

        // #if (kha_android || kha_ios)
        // setInputPosition(x, y);
        // #end
        self.deselect_text();

        if self.touch_hold {
            self.touch_hold = false;
            self.input_released = false;
            self.input_released_r = true;
        }
    }

    pub fn on_mouse_move(&self, x: i32, y: i32, movement_x: i32, movement_y: i32) {
        // #if (!kha_android && !kha_ios)
        // setInputPosition(x, y);
        // #end
    }

    pub fn on_mouse_wheel(&mut self, delta: i32) {
        self.input_wheel_delta = delta;
    }

    fn set_input_position(&mut self, x: i32, y: i32) {
        self.input_dx += x as f32 - self.input_x;
        self.input_dy += y as f32 - self.input_y;
        self.input_x = x as f32;
        self.input_y = y as f32;
    }

    pub fn on_key_down(&mut self, code: KeyCode) {
        // self.key = code;
        self.is_key_pressed = true;
        self.is_key_down = true;
        // keyRepeatTime = Scheduler.time() + 0.4;
        // switch code {
        // 	case KeyCode::Shift: isShiftDown = true;
        // 	case KeyCode::Control: is_ctrl_down = true;
        // 	#if kha_darwin
        // 	case KeyCode::Meta: is_ctrl_down = true;
        // 	#end
        // 	case KeyCode::Alt: isAltDown = true;
        // 	case KeyCode::Backspace: isBackspaceDown = true;
        // 	case KeyCode::Delete: isDeleteDown = true;
        // 	case KeyCode::Escape: isEscapeDown = true;
        // 	case KeyCode::Return: isReturnDown = true;
        // 	case KeyCode::Tab: is_tab_down = true;
        // 	case KeyCode::A: is_adown = true;
        // 	case KeyCode::Space: char = " ";
        // 	#if kha_android_rmb // Detect right mouse button on Android..
        // 	case KeyCode::Back: if (!inputDownR) onMouseDown(1, input_x as i32, inputY as i32);
        // 	#end
        // 	default:
        // }
    }

    pub fn on_key_up(&mut self, code: KeyCode) {
        self.is_key_down = false;
        // match code {
        // 	case KeyCode::Shift: isShiftDown = false;
        // 	case KeyCode::Control: is_ctrl_down = false;
        // 	#if kha_darwin
        // 	case KeyCode::Meta: is_ctrl_down = false;
        // 	#end
        // 	case KeyCode::Alt: isAltDown = false;
        // 	case KeyCode::Backspace: isBackspaceDown = false;
        // 	case KeyCode::Delete: isDeleteDown = false;
        // 	case KeyCode::Escape: isEscapeDown = false;
        // 	case KeyCode::Return: isReturnDown = false;
        // 	case KeyCode::Tab: is_tab_down = false;
        // 	case KeyCode::A: is_adown = false;
        // 	#if kha_android_rmb
        // 	case KeyCode::Back: onMouseUp(1, input_x as i32, inputY as i32);
        // 	#end
        // 	default:
        // }
    }

    pub fn on_key_press(&mut self, char: String) {
        self.char = char;
        self.is_key_pressed = true;
    }

    // #if (kha_android || kha_ios)
    // pub fn onTouchDown(index: i32, x: i32, y: i32) {
    // 	// Reset movement delta on touch start
    // 	if (index == 0) {
    // 		inputDX = 0;
    // 		inputDY = 0;
    // 		input_x = x;
    // 		inputY = y;
    // 	}
    // 	// Two fingers down - right mouse button
    // 	if (index == 1) {
    // 		onMouseUp(0, input_x as i32, inputY as i32);
    // 		onMouseDown(1, input_x as i32, inputY as i32);
    // 	}
    // }

    // pub fn onTouchUp(index: i32, x: i32, y: i32) {
    // 	if (index == 1) onMouseUp(1, input_x as i32, inputY as i32);
    // }

    // pub fn onTouchMove(index: i32, x: i32, y: i32) {
    // 	if (index == 0) setInputPosition(x, y);
    // }
    // #end

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
    pub fn font_size(&self) -> i32 {
        (self.theme.font_size as f32 * self.scale()) as i32
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

    pub fn resize(
        &mut self,
        handle: &mut Handle,
        w: i32,
        h: i32,
        kha_window_id: i32, /* = 0*/
    ) {
        handle.redraws = 2;
        if handle.texture.is_some() {
            // handle.texture.unload();
        }

        let w = if w < 1 { 1 } else { w };

        let h = if h < 1 { 1 } else { h };

        // handle.texture = Image.createRenderTarget(
        //     w,
        //     h,
        //     graphics4.TextureFormat.RGBA32,
        //     graphics4.DepthStencilFormat.NoDepthAndStencil,
        //     1,
        //     khaWindowId,
        // );
        // handle.texture.g2.imageScaleQuality = graphics2.ImageScaleQuality.High;
        // handle.texture.g2.mipmapScaleQuality = graphics2.ImageScaleQuality.High;
    }
}

#[derive(Default)]
pub struct HandleOptions {
    pub selected: bool,
    pub position: i32,
    pub value: f32,
    pub text: String,
    color: Color,
    pub layout: Layout,
}

#[derive(Clone)]
pub struct Handle {
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
    children: HashMap<i32, Handle>,
}

impl Default for Handle {
    fn default() -> Self {
        Self {
            selected: false,
            position: 0,
            color: Color::default(), // = Color.White,
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

impl Handle {
    pub fn new(ops: Option<HandleOptions> /* = None*/) {
        if ops.is_some() {
            // if ops.selected != None {
            // 	selected = ops.selected;
            // }

            // if ops.position != None {
            // 	position = ops.position;
            // }

            // if ops.value != None {
            // 	value = ops.value;
            // }

            // if ops.text != None {
            // 	text = ops.text;
            // }

            // if ops.color != None {
            // 	color = ops.color;
            // }

            // if ops.layout != None {
            // 	layout = ops.layout;
            // }
            unimplemented!()
        }
    }

    pub fn nest(i: i32, ops: HandleOptions /* = None*/) -> Handle {
        // if children == None {
        // 	children = [];
        // }
        // let c = children.get(i);
        // if (c == None) {
        // 	c = new Handle(ops);
        // 	children.set(i, c);
        // }
        // return c;
        unimplemented!()
    }

    pub fn unnest(i: i32) {
        // if children != None {
        // 	children.remove(i);
        // }
        unimplemented!()
    }

    // pub static let global = new Handle();
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

pub enum Usage {
    Dynamic,
    Static,
}
