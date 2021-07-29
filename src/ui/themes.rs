use crate::Color;

pub static DARK_THEME: Theme = Theme {
    // NAME: "Default Dark".to_owned(),
    window_bg_col: Color {
        red: 0x33 as f32 / 255.,
        green: 0x33 as f32 / 255.,
        blue: 0x33 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    window_tint_col: Color {
        red: 0xff as f32 / 255.,
        green: 0xff as f32 / 255.,
        blue: 0xff as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    accent_col: Color {
        red: 0x44 as f32 / 255.,
        green: 0x44 as f32 / 255.,
        blue: 0x44 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    accent_hover_col: Color {
        red: 0x49 as f32 / 255.,
        green: 0x49 as f32 / 255.,
        blue: 0x49 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    accent_select_col: Color {
        red: 0x60 as f32 / 255.,
        green: 0x60 as f32 / 255.,
        blue: 0x60 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    button_col: Color {
        red: 0x46 as f32 / 255.,
        green: 0x46 as f32 / 255.,
        blue: 0x46 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    button_text_col: Color {
        red: 0xe8 as f32 / 255.,
        green: 0xe7 as f32 / 255.,
        blue: 0xe5 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    button_hover_col: Color {
        red: 0x49 as f32 / 255.,
        green: 0x49 as f32 / 255.,
        blue: 0x49 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    button_pressed_col: Color {
        red: 0x1b as f32 / 255.,
        green: 0x1b as f32 / 255.,
        blue: 0x1b as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    text_col: Color {
        red: 0xe8 as f32 / 255.,
        green: 0xe7 as f32 / 255.,
        blue: 0xe5 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    label_col: Color {
        red: 0xc8 as f32 / 255.,
        green: 0xc8 as f32 / 255.,
        blue: 0xc8 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    separator_col: Color {
        red: 0x27 as f32 / 255.,
        green: 0x27 as f32 / 255.,
        blue: 0x27 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    highlight_col: Color {
        red: 0x20 as f32 / 255.,
        green: 0x5d as f32 / 255.,
        blue: 0x9c as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    context_col: Color {
        red: 0x22 as f32 / 255.,
        green: 0x22 as f32 / 255.,
        blue: 0x22 as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },
    panel_bg_col: Color {
        red: 0x3b as f32 / 255.,
        green: 0x3b as f32 / 255.,
        blue: 0x3b as f32 / 255.,
        alpha: 0xff as f32 / 255.,
    },

    font_size: 13,
    element_w: 100,
    element_h: 24,
    element_offset: 4,
    arrow_size: 5,
    button_h: 22,
    check_size: 15,
    check_select_size: 8,
    scroll_w: 6,
    text_offset: 8,
    tab_w: 6,
    fill_window_bg: false,
    fill_button_bg: true,
    fill_accent_bg: false,
    link_style: LinkStyle::Line,
    full_tabs: false,
};

#[derive(Default, Copy, Clone)]
pub struct Theme {
    // pub NAME: String,
    pub window_bg_col: Color,
    pub window_tint_col: Color,
    pub accent_col: Color,
    pub accent_hover_col: Color,
    pub accent_select_col: Color,
    pub button_col: Color,
    pub button_text_col: Color,
    pub button_hover_col: Color,
    pub button_pressed_col: Color,
    pub text_col: Color,
    pub label_col: Color,
    pub separator_col: Color,
    pub highlight_col: Color,
    pub context_col: Color,
    pub panel_bg_col: Color,

    pub font_size: u32,
    pub element_w: u32,
    pub element_h: u32,
    pub element_offset: u32,
    pub arrow_size: u32,
    pub button_h: u32,
    pub check_size: u32,
    pub check_select_size: u32,
    pub scroll_w: u32,
    pub text_offset: u32,
    pub tab_w: u32, // Indentation
    pub fill_window_bg: bool,
    pub fill_button_bg: bool,
    pub fill_accent_bg: bool,
    pub link_style: LinkStyle,
    pub full_tabs: bool, // Make tabs take full window width
}

#[derive(Copy, Clone)]
pub enum LinkStyle {
    Line = 0,
    CubicBezier = 1,
}

impl Default for LinkStyle {
    fn default() -> Self {
        Self::Line
    }
}
