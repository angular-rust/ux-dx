#![allow(unused_mut)]
use winit::event::VirtualKeyCode;

use super::*;

pub struct Ext {}

impl Ext {
    #[deprecated]
    pub fn text_area(
        ui: &mut Ui,
        handle: &Handle,
        align: Align,   /* = Align.Left*/
        editable: bool, /*= true*/
    ) -> String {
        // TODO: think about Cow may be
        let mut lines: Vec<String> = match handle.props.read() {
            Ok(props) => props
                .text
                .replace("\t", "    ")
                .lines()
                .map(|line| line.to_owned())
                .collect(),
            Err(e) => panic!("RwLock poisoned"),
        };

        let selected = if let Some(text_selected_handle) = &ui.text_selected_handle {
            text_selected_handle == handle // Text being edited
        } else {
            false
        };

        let cursorStartX = ui.cursor_x;
        let keyPressed = selected && ui.is_key_pressed;
        ui.highlight_on_select = false;
        ui.tab_switch_enabled = false;
        ui.painter.set_color(ui.theme.separator_col);
        ui.draw_rect(
            true,
            ui.x + ui.button_offset_y,
            ui.y + ui.button_offset_y,
            ui.w - ui.button_offset_y * 2.0,
            lines.len() as f32 * ui.element_h() - ui.button_offset_y * 2.0,
            0.0,
        );

        match handle.props.write() {
            Ok(mut props) => {
                for (idx, line) in lines.iter_mut().enumerate() {
                    // Draw lines
                    if (!selected && ui.hover(-1.0))
                        || (selected && idx as i32 == props.position)
                    {
                        props.position = idx as i32; // Set active line
                        props.text = line.clone();
                        ui.text_input(handle, "", align, editable);
                        if let Some(key) = ui.key {
                            if keyPressed
                                && key != VirtualKeyCode::Return
                                && key != VirtualKeyCode::Escape
                            {
                                // Edit text
                                *line = ui.text_selected.clone();
                            }
                        }
                    } else {
                        ui.text(line, align, None);
                    }
                    ui.y -= ui.element_offset();
                }
                ui.y += ui.element_offset();

                if keyPressed {
                    if let Some(key) = ui.key {
                        // Move cursor vertically
                        if key == VirtualKeyCode::Down && props.position < lines.len() as i32 - 1 {
                            props.position += 1;
                        }

                        if key == VirtualKeyCode::Up && props.position > 0 {
                            props.position -= 1;
                        }

                        // New line
                        if editable && key == VirtualKeyCode::Return {
                            props.position += 1;
                            // lines.insert(
                            //     props.position,
                            //     lines[props.position - 1].substr(ui.cursor_x),
                            // );
                            // lines[props.position - 1] =
                            //     lines[props.position - 1].substr(0, ui.cursor_x);
                            ui.start_text_edit(handle);
                            ui.cursor_x = 0;
                            ui.highlight_anchor = 0;
                        }

                        // Delete line
                        if editable
                            && key == VirtualKeyCode::Back
                            && cursorStartX == 0
                            && props.position > 0
                        {
                            props.position -= 1;
                            // ui.cursor_x = lines[props.position].length;
                            // ui.highlight_anchor = ui.cursor_x;
                            // lines[props.position] += lines[props.position + 1];
                            // lines.splice(props.position + 1, 1);
                        }
                        // ui.text_selected = lines[props.position];
                    }
                }

                ui.highlight_on_select = true;
                ui.tab_switch_enabled = true;

                props.text = lines.join("\n");
                props.text.clone() // FIXME: HUGE OVERHEAD for every frame of draw cicle
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    // let _element_offset = 0;
    // let _BUTTON_COL = 0;

    pub fn begin_menu(ui: &Ui) {
        // _element_offset = ui.theme.element_offset;
        // _BUTTON_COL = ui.theme.button_col;
        // ui.theme.element_offset = 0;
        // ui.theme.button_col = ui.theme.separator_col;
        ui.painter.set_color(ui.theme.separator_col);
        ui.painter
            .fill_rect(0.0, 0.0, ui.window_w, Self::menubar_h(ui));
    }

    pub fn end_menu(ui: &Ui) {
        // ui.theme.element_offset = _element_offset;
        // ui.theme.button_col = _BUTTON_COL;
    }

    #[deprecated]
    pub fn menu_button(ui: &mut Ui, text: String) -> bool {
        if let Some((width, _)) = ui.painter.measure(text.as_str()) {
            ui.w = width + 25.0 * ui.scale();
            ui.button(text.as_str(), Align::Center, "")
        } else {
            false
        }
    }

    pub fn menubar_h(ui: &Ui) -> f32 {
        ui.button_h() * 1.1 + 2.0 + ui.button_offset_y
    }
}
