use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::foundation::colorspace::Color;

pub const DEFAULT_DARK_THEME: &'static str = "Default Dark";
pub const DEFAULT_LIGHT_THEME: &'static str = "Default Light";

#[derive(Default, Clone)]
pub struct Theme {
    pub name: String,
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LinkStyle {
    Line = 0,
    CubicBezier = 1,
}

impl Default for LinkStyle {
    fn default() -> Self {
        Self::Line
    }
}

#[derive(Default)]
struct ThemeRegistry {
    themes: Arc<RwLock<HashMap<String, Theme>>>,
}

impl ThemeRegistry {
    pub fn new() -> Self {
        let instance = Self::default();

        instance.register(DEFAULT_DARK_THEME, default_dark());
        instance.register(DEFAULT_LIGHT_THEME, default_light());

        instance
    }

    pub fn register<T: Into<String>>(&self, name: T, theme: Theme) {
        match self.themes.write() {
            Ok(mut themes) => {
                themes.insert(name.into(), theme);
            }
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn get<T: Into<String>>(&self, name: T) -> Option<Theme> {
        match self.themes.read() {
            Ok(themes) => themes.get(&name.into()).map(|theme| theme.clone()),
            Err(e) => panic!("RwLock poisoned"),
        }
    }

    pub fn global() -> &'static Self {
        static THEME_REGISTRY: OnceCell<ThemeRegistry> = OnceCell::new();
        THEME_REGISTRY.get_or_init(|| Self::new())
    }
}

pub struct Themes;

impl Themes {
    pub fn register<T: Into<String>>(name: T, theme: Theme) {
        ThemeRegistry::global().register(name, theme)
    }

    pub fn get<T: Into<String>>(name: T) -> Option<Theme> {
        ThemeRegistry::global().get(name)
    }
}

fn default_dark() -> Theme {
    Theme {
        name: DEFAULT_DARK_THEME.into(),
        window_bg_col: Color::rgb(0x33, 0x33, 0x33),
        window_tint_col: Color::rgb(0xff, 0xff, 0xff),
        accent_col: Color::rgb(0x44, 0x44, 0x44),
        accent_hover_col: Color::rgb(0x49, 0x49, 0x49),
        accent_select_col: Color::rgb(0x60, 0x60, 0x60),
        button_col: Color::rgb(0x46, 0x46, 0x46),
        button_text_col: Color::rgb(0xe8, 0xe7, 0xe5),
        button_hover_col: Color::rgb(0x49, 0x49, 0x49),
        button_pressed_col: Color::rgb(0x1b, 0x1b, 0x1b),
        text_col: Color::rgb(0xe8, 0xe7, 0xe5),
        label_col: Color::rgb(0xc8, 0xc8, 0xc8),
        separator_col: Color::rgb(0x27, 0x27, 0x27),
        highlight_col: Color::rgb(0x20, 0x5d, 0x9c),
        context_col: Color::rgb(0x22, 0x22, 0x22),
        panel_bg_col: Color::rgb(0x3b, 0x3b, 0x3b),

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
    }
}

// 2x scaled, for games
fn default_light() -> Theme {
    Theme {
        name: DEFAULT_LIGHT_THEME.into(),
        window_bg_col: Color::rgb(0xef, 0xef, 0xef),
        window_tint_col: Color::rgb(0x22, 0x22, 0x22),
        accent_col: Color::rgb(0xee, 0xee, 0xee),
        accent_hover_col: Color::rgb(0xbb, 0xbb, 0xbb),
        accent_select_col: Color::rgb(0xaa, 0xaa, 0xaa),
        button_col: Color::rgb(0xcc, 0xcc, 0xcc),
        button_text_col: Color::rgb(0x22, 0x22, 0x22),
        button_hover_col: Color::rgb(0xb3, 0xb3, 0xb3),
        button_pressed_col: Color::rgb(0xb1, 0xb1, 0xb1),
        text_col: Color::rgb(0x99, 0x99, 0x99),
        label_col: Color::rgb(0xaa, 0xaa, 0xaa),
        separator_col: Color::rgb(0x99, 0x99, 0x99),
        highlight_col: Color::rgb(0x20, 0x5d, 0x9c),
        context_col: Color::rgb(0xaa, 0xaa, 0xaa),
        panel_bg_col: Color::rgb(0xaa, 0xaa, 0xaa),

        font_size: 13 * 2,
        element_w: 100 * 2,
        element_h: 24 * 2,
        element_offset: 4 * 2,
        arrow_size: 5 * 2,
        button_h: 22 * 2,
        check_size: 15 * 2,
        check_select_size: 8 * 2,
        scroll_w: 6 * 2,
        text_offset: 8 * 2,
        tab_w: 12 * 2,
        fill_window_bg: false,
        fill_button_bg: true,
        fill_accent_bg: false,
        link_style: LinkStyle::Line,
        full_tabs: false,
    }
}
