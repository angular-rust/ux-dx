use regex::Regex;
use std::{collections::HashMap, fmt, rc::Rc};

use crate::engine::d2::{
    asset::{AssetPack, File},
    math::{Math, Rectangle},
    util::StringExtensions,
};

use super::{Graphics, Texture};

///
/// A bitmap font, created in any tool that exports the BMFont format, such as the original BMFont
/// editor, Hiero, or Glyph Designer.
///
#[derive(Default, Clone)]
pub struct Font {
    /// The name that was used to load this font.
    pub name: String,

    /// The size of this font, in pixels.
    pub size: f32,

    /// The vertical distance between each line of text in this font, in pixels.
    pub line_height: f32,

    /// A special glyph to handle the newline character, which is not included in most fonts
    /// static NEWLINE = Glyph::new('\n'.to_digit(10));
    pack: Option<Rc<dyn AssetPack>>,
    file: Option<Rc<dyn File>>,
    glyphs: HashMap<char, Glyph>,

    /// Used to track live-reloading updates. A signal listener can't be used here, because we can't
    /// guarantee it'll be properly disposed
    #[cfg(debug_assertions)]
    last_reload_count: usize,
}

impl Font {
    /// Parses a font using files in an asset pack.
    /// @param name The path to the font within the asset pack, excluding the .fnt suffix.
    pub fn new(pack: Rc<impl AssetPack + 'static>, name: String) -> Self {
        let file = pack.file(format!("{}.fnt", name), true);

        #[cfg(debug_assertions)]
        let last_reload_count = match file {
            Some(ref file) => file.reload_count(),
            None => 0,
        };

        let mut instance = Self {
            name,
            pack: Some(pack),
            file,
            glyphs: HashMap::new(),
            line_height: 0.0,
            size: 0.0,

            #[cfg(debug_assertions)]
            last_reload_count,
        };

        instance.reload();

        instance
    }

    /// Disposes the source .fnt File used to create this Font. This can free up some memory, if you
    /// don't intend to recreate this Font later from the same AssetPack.
    ///
    /// @returns This instance, for chaining.
    pub fn dispose_files(&mut self) -> &Self {
        if let Some(ref mut file) = self.file {
            file.dispose();
        }

        self
    }

    /// Splits text into multiple lines that fit into a given width when displayed using this font.
    pub fn split_lines(&self, text: String, max_width: f32) -> Vec<String> {
        let glyphs = self.glyphs(text);
        let mut line = String::new();
        let mut lines: Vec<String> = Vec::new();
        let mut x = 0;
        let mut idx = 0;
        let ll = glyphs.len() as i32;
        let mut last_space_idx = -1;

        while idx < ll {
            let glyph = &glyphs[idx as usize];
            if x + glyph.width > max_width as i32 {
                // Ran off the edge, add a line
                x = 0;

                if let Some(space) = line.rfind(" ") {
                    // Backtrack to the beginning of the last word
                    lines.push(line[..space].to_string());
                    idx = last_space_idx + 1;
                } else {
                    lines.push(line.clone());
                }
                line.clear();
            } else {
                if glyph.ch == ' ' {
                    last_space_idx = idx;
                }
                line.push(glyph.ch);
                x += glyph.x_advance;
                idx += 1;
                if idx != ll {
                    let next_glyph = &glyphs[idx as usize];
                    x += glyph.kerning(next_glyph.ch);
                }
            }
        }
        lines.push(line);

        lines
    }

    /// Get the list of Glyphs that make up a string. Characters without glyphs in this font will be
    /// missing from the list.
    pub fn glyphs(&self, text: String) -> Vec<Glyph> {
        let mut list: Vec<Glyph> = Vec::new();
        for ch in text.chars() {
            if let Some(glyph) = self.glyphs.get(&ch) {
                list.push(glyph.clone());
            } else {
                // log::warn!("Requested a missing character from [font: {}, charCode: {}]"
                //     self.name,
                //     charCode
                // );
            }
        }

        list
    }

    // ?align :TextAlign, wrap_width: f32 = 0, letter_spacing: f32 = 0, line_spacing: f32 = 0
    pub fn layout_text(
        &self,
        text: String,
        align: TextAlign,
        wrap_width: f32,
        letter_spacing: f32,
        line_spacing: f32,
    ) -> TextLayout {
        TextLayout::new(Rc::new(self.clone()), text, align, wrap_width, letter_spacing, line_spacing)
    }

    /// Get the Glyph for a given character code.
    #[inline]
    pub fn glyph(&self, ch: char) -> Option<&Glyph> {
        self.glyphs.get(&ch)
    }

    #[cfg(debug_assertions)]
    pub fn check_reload(&mut self) -> usize {
        // If the .fnt file was reloaded since the last check, reload the font

        let reload_count = if let Some(ref file) = self.file {
            file.reload_count()
        } else {
            0
        };

        if self.last_reload_count != reload_count {
            self.last_reload_count = reload_count;
            self.reload();
        }

        reload_count
    }

    fn reload(&mut self) {
        self.glyphs.clear();
        // self._glyphs.set(NEWLINE.charCode, NEWLINE); //DV

        let keyword_pattern = Regex::new(r#"~/([A-Za-z]+)(.*)/"#).unwrap();
        let pair_pattern = Regex::new(r#"~/([A-Za-z]+)=("[^"]*"|[^\s]+)/"#).unwrap();

        // let parser = ConfigParser::new(self.file.unwrap().to_string());
        let pages: HashMap<usize, Rc<dyn Texture>> = HashMap::new();

        // The basename of the font's path, where we'll find the textures

        let base_path = if let Some(idx) = self.name.rfind("/") {
            self.name[..idx + 1].to_string()
        } else {
            String::new()
        };

        let config = self.file.as_ref().unwrap().to_string();
        let keywords = keyword_pattern.captures(config.as_str()).unwrap();

        // // BMFont spec: http://www.angelcode.com/products/bmfont/doc/file_format.html
        // for (keyword, rest) in keywords.get(1).iter().zip(keywords.get(2).iter()) {
        //     match keyword.as_str() {
        //         "info" => {
        //             if let Some(pairs) = pair_pattern.captures(rest.as_str()) {
        //                 for pair in pairs.iter() {
        //                     match pair.key.as_str() {
        //                         "size" => {
        //                             self.size = pair.get_float();
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //         "common" => {
        //             for pair in parser.pairs() {
        //                 match pair.key.as_str() {
        //                     "lineHeight" => {
        //                         self.line_height = pair.get_float();
        //                     }
        //                 }
        //             }
        //         }
        //         "page" => {
        //             let page_id: usize = 0;
        //             let file: Option<String> = None;
        //             for pair in parser.pairs() {
        //                 match pair.key.as_str() {
        //                     "id" => {
        //                         page_id = pair.get_int() as usize;
        //                     }
        //                     "file" => {
        //                         file = pair.get_string();
        //                     }
        //                 }
        //             }
        //             let path = format!("{}{}", base_path, file.unwrap_or_default().remove_file_extension());
        //             if let Some(texture) = self.pack.get_texture(path, true) {
        //                 pages.insert(page_id, texture.clone());
        //             }
        //         }
        //         "char" => {
        //             let mut glyph = None;
        //             for pair in parser.pairs() {
        //                 match pair.key.as_str() {
        //                     "id" => {
        //                         let ch = pair.get_int() as u32;
        //                         glyph = Some(Glyph::new(char::from_u32(ch).unwrap()));
        //                     }
        //                     "x" => {
        //                         if let Some(glyph) = glyph {
        //                             glyph.x = pair.get_int();
        //                         }
        //                     }
        //                     "y" => {
        //                         if let Some(glyph) = glyph {
        //                             glyph.y = pair.get_int();
        //                         }
        //                     }
        //                     "width" => {
        //                         if let Some(glyph) = glyph {
        //                             glyph.width = pair.get_int();
        //                         }
        //                     }
        //                     "height" => {
        //                         if let Some(glyph) = glyph {
        //                             glyph.height = pair.get_int();
        //                         }
        //                     }
        //                     "page" => {
        //                         if let Some(glyph) = glyph {
        //                             let page_id = pair.get_int() as usize;
        //                             if let Some(page) = pages.get(&page_id) {
        //                                 glyph.page = Some(page.clone());
        //                             }
        //                         }
        //                     }
        //                     "xoffset" => {
        //                         if let Some(glyph) = glyph {
        //                             glyph.x_offset = pair.get_int();
        //                         }
        //                     }
        //                     "yoffset" => {
        //                         if let Some(glyph) = glyph {
        //                             glyph.y_offset = pair.get_int();
        //                         }
        //                     }
        //                     "xadvance" => {
        //                         if let Some(glyph) = glyph {
        //                             glyph.x_advance = pair.get_int();
        //                         }
        //                     }
        //                 }
        //             }

        //             if let Some(ref glyph) = glyph {
        //                 self.glyphs.insert(glyph.ch, *glyph);
        //             }
        //         }
        //         "kerning" => {
        //             let mut first: Option<&Glyph> = None;
        //             let second = 0;
        //             let amount = 0;
        //             for ref pair in parser.pairs() {
        //                 match pair.key.as_str() {
        //                     "first" => {
        //                         first = self.glyphs.get(&char::from_u32(pair.get_int() as u32).unwrap());
        //                     }
        //                     "second" => {
        //                         second = pair.get_int() as u32;
        //                     }
        //                     "amount" => {
        //                         amount = pair.get_int();
        //                     }
        //                 }
        //             }
        //             if let Some(glyph) = first {
        //                 if amount != 0 {
        //                     glyph.set_kerning(char::from_u32(second).unwrap(), amount);
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}

impl PartialEq for Font {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size && self.line_height == other.line_height
    }
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Font")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}

/// Represents a single glyph in a bitmap font.
#[derive(Default, Clone, Debug)]
pub struct Glyph {
    /// This glyph's ASCII character code.
    pub ch: char,

    // Location and dimensions of this glyph on the sprite sheet
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    /// The atlas that contains this glyph.
    pub page: Option<Rc<dyn Texture>>,

    pub x_offset: i32,
    pub y_offset: i32,

    pub x_advance: i32,

    kernings: HashMap<char, i32>,
}

impl Glyph {
    fn new(ch: char) -> Self {
        Self {
            ch,
            kernings: HashMap::new(),
            height: 0,
            width: 0,
            x: 0,
            y: 0,
            x_advance: 0,
            x_offset: 0,
            y_offset: 0,
            page: None,
        }
    }

    /// Draws this glyph to a Graphics surface.
    pub fn draw(&self, gfx: &Box<dyn Graphics>, dest_x: f32, dest_y: f32) {
        // Avoid drawing whitespace
        if let Some(ref page) = self.page {
            if self.width > 0 {
                gfx.draw_sub_texture(
                    page,
                    dest_x + self.x_offset as f32,
                    dest_y + self.y_offset as f32,
                    self.x as f32,
                    self.y as f32,
                    self.width as f32,
                    self.height as f32,
                );
            }
        }
    }

    pub fn kerning(&self, ch: char) -> i32 {
        self.kernings.get(&ch).map(|c| *c).unwrap_or_default()
    }

    fn set_kerning(&mut self, ch: char, amount: i32) {
        self.kernings.insert(ch, amount);
    }
}

impl PartialEq for Glyph {
    fn eq(&self, other: &Self) -> bool {
        self.ch == other.ch
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]

pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Left
    }
}

/// Measures and lays out a block of text, handling word wrapping, alignment and newline characters.
#[derive(Default, Clone, Debug)]
pub struct TextLayout {
    /// The bounding box that contains this text.
    pub bounds: Rectangle,

    /// The number of lines in this text.
    pub lines: usize,

    font: Option<Rc<Font>>,
    glyphs: Vec<Glyph>,
    offsets: Vec<f32>,
    line_offset: f32,
}

impl TextLayout {
    fn new(
        font: Rc<Font>,
        text: String,
        align: TextAlign,
        wrap_width: f32,
        letter_spacing: f32,
        line_spacing: f32,
    ) -> Self {
        let mut instance = Self {
            font: Some(font.clone()),
            glyphs: Vec::new(),
            offsets: Vec::new(),
            line_offset: font.line_height.round() + line_spacing,
            bounds: Rectangle::default(),
            lines: 0,
        };

        let mut line_widths = Vec::new();

        for charcode in text.chars() {
            if let Some(glyph) = font.glyph(charcode) {
                instance.glyphs.push(glyph.clone());
            } else {
                // log::warn!("Requested a missing character from font [font: {}, charCode: {}]", font.name, charCode);
            }
        }

        let mut last_space_idx: i32 = -1;
        let mut line_width: f32 = 0.0;
        let mut line_height: f32 = 0.0;
        let new_line = font.glyph('\n').unwrap();

        let mut idx = 0;
        while idx < instance.glyphs.len() {
            let glyph = &instance.glyphs[idx];
            instance.offsets[idx] = line_width.round();

            let wordwrap = wrap_width > 0.0 && line_width + glyph.width as f32 > wrap_width;
            if wordwrap || glyph == new_line {
                // Wrap using the last word divider
                if wordwrap {
                    if last_space_idx >= 0 {
                        instance.glyphs[last_space_idx as usize] = new_line.clone();
                        line_width = instance.offsets[last_space_idx as usize];
                        idx = last_space_idx as usize;
                    } else {
                        instance.glyphs.insert(idx, new_line.clone());
                    }
                }
                last_space_idx = -1;

                line_height = instance.line_offset;
                {
                    // add_line
                    instance.bounds.width = instance.bounds.width.max(line_width);
                    instance.bounds.height += line_height;

                    line_widths[instance.lines] = line_width;
                    line_width = 0.0;
                    line_height = 0.0;
                    instance.lines += 1;
                }
            } else {
                if glyph.ch == ' ' {
                    last_space_idx = idx as i32;
                }
                line_width += glyph.x_advance as f32 + letter_spacing;
                line_height = line_height.max((glyph.height + glyph.y_offset) as f32);

                // Handle kerning with the next glyph
                if idx + 1 < instance.glyphs.len() {
                    let next_glyph = &instance.glyphs[idx + 1];
                    line_width += glyph.kerning(next_glyph.ch) as f32;
                }
            }

            idx += 1;
        }

        // Handle the remaining lineWidth/Height
        {
            // add_line
            instance.bounds.width = instance.bounds.width.max(line_width);
            instance.bounds.height += line_height;

            line_widths[instance.lines] = line_width;
            // line_width = 0.0;
            // line_height = 0.0;
            instance.lines += 1;
        }

        let mut line_y = 0.0;
        let mut align_offset = Self::align_offset(align, line_widths[0], wrap_width);

        let mut top = Math::FLOAT_MAX;
        let mut bottom = Math::FLOAT_MIN;

        // Pack bounds
        let mut line = 0;
        let mut idx = 0;
        let ll = instance.glyphs.len();
        while idx < ll {
            let glyph = &instance.glyphs[idx];

            if glyph.ch == '\n' {
                line_y += instance.line_offset;
                line += 1;
                align_offset = Self::align_offset(align, line_widths[line], wrap_width);
            }
            instance.offsets[idx] += align_offset;

            let glyph_y = line_y + glyph.y_offset as f32;
            top = top.min(glyph_y);
            bottom = bottom.max(glyph_y + glyph.height as f32);

            idx += 1;
        }

        instance.bounds.x = Self::align_offset(align, instance.bounds.width, wrap_width);
        instance.bounds.y = top;
        instance.bounds.height = bottom - top;

        instance
    }

    /// Draws this text to a Graphics.
    pub fn draw(&self, gfx: &Box<dyn Graphics>) {
        let mut y = 0.0;
        let mut idx = 0;
        let ll = self.glyphs.len();

        while idx < ll {
            let glyph = &self.glyphs[idx];
            if glyph.ch == '\n' {
                y += self.line_offset;
            } else {
                let x = self.offsets[idx];
                glyph.draw(gfx, x, y);
            }
            idx += 1;
        }
    }

    // static
    fn align_offset(align: TextAlign, linewidth: f32, total_width: f32) -> f32 {
        match align {
            TextAlign::Left => 0.0,
            TextAlign::Right => total_width - linewidth,
            TextAlign::Center => ((total_width - linewidth) / 2.0).round(),
        }
    }
}

pub struct ConfigPair {
    pub key: String,
    value: String,
}

impl ConfigPair {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn int(&self) -> i32 {
        self.value.parse::<i32>().unwrap_or(0)
    }

    pub fn float(&self) -> f32 {
        self.value.parse::<f32>().unwrap_or(0.0)
    }

    pub fn string(&self) -> Option<String> {
        match self.value.chars().nth(0) {
            Some(first) => {
                if first != '"' {
                    return None;
                }
            }
            None => return None,
        }

        Some(self.value[1..self.value.len() - 1].to_string())
    }
}
