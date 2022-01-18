use std::rc::Rc;

use crate::engine::d2::{
    animation::AnimatedFloat,
    display::Font,
    util::{BitSets, Listener2},
};

use super::{Graphics, Sprite, TextAlign, TextLayout};

/// A sprite that displays a line of text using a bitmap font.
#[derive(Default, Clone, Debug)]
pub struct TextSprite {
    pub inner: Sprite,

    /// The maximum available width of this text before word wrapping to a new line. Defaults to 0
    /// (no word wrapping).
    pub wrap_width: AnimatedFloat,

    /// Additional horizontal space to apply between letters, in pixels. Defaults to 0. Positive
    /// values make text look "looser", negative values look "tighter".
    pub letter_spacing: AnimatedFloat,

    /// Additional vertical space to apply between lines, in pixels. Defaults to 0. Positive values
    /// make lines look "looser", negative values look "tighter".
    pub line_spacing: AnimatedFloat,

    /// The font used to display the text.
    pub(crate) font: Font,
    /// The text being displayed. Can contain contain newline characters (\n) for multiline text.
    pub(crate) text: Option<String>,
    // The horizontal text alignment, for multiline text. Left by default.
    pub(crate) align: TextAlign,

    pub(crate) layout: TextLayout,

    #[cfg(debug_assertions)]
    pub(crate) last_reload_count: Option<usize>,
}

impl TextSprite {
    // Component flags
    const TEXT_DIRTY: u32 = Sprite::NEXT_FLAG << 0;
    const NEXT_FLAG: u32 = Sprite::NEXT_FLAG << 1; // Must be last!

    pub fn new(font: Font, text: Option<String>) -> Self {
        let inner = Sprite::new();

        let dirty_text: Option<Listener2<f32, f32>> = Some(Rc::new(|_, _| {
            // TODO: recursive hell
            // self.inner.inner.flags = flags.add(Self::TEXT_DIRTY);
        }));

        let mut instance = Self {
            inner,
            font,
            text,
            align: TextAlign::Left,
            wrap_width: AnimatedFloat::new(0.0, dirty_text.clone()),
            letter_spacing: AnimatedFloat::new(0.0, dirty_text.clone()),
            line_spacing: AnimatedFloat::new(0.0, dirty_text.clone()),
            layout: TextLayout::default(),
            #[cfg(debug_assertions)]
            last_reload_count: None,
        };

        instance.inner.component.flags = instance.inner.component.flags.add(Self::TEXT_DIRTY);

        instance
    }

    // override
    pub fn draw(&mut self, gfx: &Box<dyn Graphics>) {
        self.update_layout();

        #[cfg(feature = "debug_text")]
        {
            // Draw the bounding boxes for debugging
            g.fill_rect(0xff0000, 0, 0, getNaturalWidth(), getNaturalHeight());
            g.fill_rect(0x00ff00, layout.bounds.x, layout.bounds.y, layout.bounds.width, layout.bounds.height);
        }

        self.layout.draw(gfx);
    }

    // override
    pub fn natural_width(&mut self) -> f32 {
        self.update_layout();
        if self.wrap_width.get() > 0.0 {
            self.wrap_width.get()
        } else {
            self.layout.bounds.width
        }
    }

    // override
    pub fn natural_height(&mut self) -> f32 {
        self.update_layout();
        let padded_height = self.layout.lines as f32 * (self.font.line_height + self.line_spacing.get());
        let bounds_height = self.layout.bounds.height;

        padded_height.max(bounds_height)
    }

    // override
    pub fn contains_local(&mut self, local_x: f32, local_y: f32) -> bool {
        self.update_layout();

        self.layout.bounds.contains(local_x, local_y)
    }

    /// Chainable convenience method to set the wrap width.
    /// @returns This instance, for chaining.
    pub fn set_wrap_width(&mut self, wrap_width: f32) -> &Self {
        self.wrap_width.set(wrap_width);

        self
    }

    /// Chainable convenience method to set the letter spacing.
    /// @returns This instance, for chaining.
    pub fn set_letter_spacing(&mut self, letter_spacing: f32) -> &Self {
        self.letter_spacing.set(letter_spacing);

        self
    }

    /// Chainable convenience method to set the line spacing.
    /// @returns This instance, for chaining.
    pub fn set_line_spacing(&mut self, line_spacing: f32) -> &Self {
        self.line_spacing.set(line_spacing);

        self
    }

    #[inline]
    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn set_text(&mut self, text: Option<String>) {
        if text != self.text {
            self.text = text;
            self.inner.component.flags = self.inner.component.flags.add(Self::TEXT_DIRTY);
        }
    }

    #[inline]
    pub fn font(&self) -> Font {
        self.font.clone()
    }

    pub fn set_font(&mut self, font: Font) {
        if font != self.font {
            self.font = font;
            self.inner.component.flags = self.inner.component.flags.add(Self::TEXT_DIRTY);
        }
    }

    #[inline]
    pub fn align(&self) -> TextAlign {
        self.align
    }

    // /// Chainable convenience method to set the text alignment.
    // /// @returns This instance, for chaining.
    // pub fn set_align(&self, align: TextAlign) -> &Self {
    //     self.align = align;

    //     self
    // }

    pub fn set_align(&mut self, align: TextAlign) {
        if align != self.align {
            self.align = align;
            self.inner.component.flags = self.inner.component.flags.add(Self::TEXT_DIRTY);
        }
    }

    fn update_layout(&mut self) {
        #[cfg(debug_assertions)]
        {
            let reload_count = self.font.check_reload();
            if self
                .last_reload_count
                .map(|val| val != reload_count)
                .unwrap_or_default()
            {
                self.last_reload_count = Some(reload_count);
                self.inner.component.flags = self.inner.component.flags.add(Self::TEXT_DIRTY);
            }
        }

        // Recreate the layout if necessary
        if self.inner.component.flags.contains(Self::TEXT_DIRTY) {
            self.inner.component.flags = self.inner.component.flags.remove(Self::TEXT_DIRTY);
            self.layout = self.font.layout_text(
                self.text.clone().unwrap_or_default(),
                self.align,
                self.wrap_width.get(),
                self.letter_spacing.get(),
                self.line_spacing.get(),
            );
        }
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        self.inner.on_update(dt);
        self.wrap_width.update(dt);
        self.letter_spacing.update(dt);
        self.line_spacing.update(dt);
    }
}

impl AsRef<Sprite> for TextSprite {
    fn as_ref(&self) -> &Sprite {
        &self.inner
    }
}
