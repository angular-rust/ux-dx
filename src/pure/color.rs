use std::boxed::Box as Box_;
use std::mem;

// * SECTION:cogl-color
// * @short_description: A generic color definition
// *
// * #Color is a simple structure holding the definition of a color such
// * that it can be efficiently used by GL
// * Color:
// * @red: amount of red
// * @green: amount of green
// * @blue: amount of green
// * @alpha: alpha
// *
// * A structure for holding a color definition. The contents of
// * the Color structure are private and should never by accessed
// * directly.
// *
// * Since: 1.0
#[derive(Debug, PartialOrd, Ord)] // Hash
pub struct Color {
    // /*< private >*/
    // uint8_t COGL_PRIVATE (red);
    // uint8_t COGL_PRIVATE (green);
    // uint8_t COGL_PRIVATE (blue);

    // uint8_t COGL_PRIVATE (alpha);

    // /* padding in case we want to change to floats at
    // * some point */
    // uint32_t COGL_PRIVATE (padding0);
    // uint32_t COGL_PRIVATE (padding1);
    // uint32_t COGL_PRIVATE (padding2);
}

impl Color {
    /// Creates a new (empty) color
    ///
    /// # Returns
    ///
    /// a newly-allocated `Color`. Use `Color::free`
    ///  to free the allocated resources
    pub fn new() -> Color {
        unimplemented!()
    }

    /// Retrieves the alpha channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the alpha channel of the passed color
    pub fn get_alpha(&self) -> f32 {
        // return  ((float) color->alpha / 255.0);
        unimplemented!()
    }

    /// Retrieves the alpha channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the alpha channel of the passed color
    pub fn get_alpha_byte(&self) -> u8 {
        // return color->alpha;
        unimplemented!()
    }

    /// Retrieves the alpha channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the alpha channel of the passed color
    pub fn get_alpha_float(&self) -> f32 {
        // return (float) color->alpha / 255.0;
        unimplemented!()
    }

    /// Retrieves the blue channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the blue channel of the passed color
    pub fn get_blue(&self) -> f32 {
        // return  ((float) color->blue / 255.0);
        unimplemented!()
    }

    /// Retrieves the blue channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the blue channel of the passed color
    pub fn get_blue_byte(&self) -> u8 {
        // return color->blue;
        unimplemented!()
    }

    /// Retrieves the blue channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the blue channel of the passed color
    pub fn get_blue_float(&self) -> f32 {
        // return (float) color->blue / 255.0;
        unimplemented!()
    }

    /// Retrieves the green channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the green channel of the passed color
    pub fn get_green(&self) -> f32 {
        // return  ((float) color->green / 255.0);
        unimplemented!()
    }

    /// Retrieves the green channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the green channel of the passed color
    pub fn get_green_byte(&self) -> u8 {
        // return color->green;
        unimplemented!()
    }

    /// Retrieves the green channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the green channel of the passed color
    pub fn get_green_float(&self) -> f32 {
        // return (float) color->green / 255.0;
        unimplemented!()
    }

    /// Retrieves the red channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the red channel of the passed color
    pub fn get_red(&self) -> f32 {
        // return  ((float) color->red / 255.0);
        unimplemented!()
    }

    /// Retrieves the red channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the red channel of the passed color
    pub fn get_red_byte(&self) -> u8 {
        // return color->red;
        unimplemented!()
    }

    /// Retrieves the red channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the red channel of the passed color
    pub fn get_red_float(&self) -> f32 {
        // return (float) color->red / 255.0;
        unimplemented!()
    }

    /// Sets the values of the passed channels into a `Color`
    /// ## `red`
    /// value of the red channel, between 0 and 1.0
    /// ## `green`
    /// value of the green channel, between 0 and 1.0
    /// ## `blue`
    /// value of the blue channel, between 0 and 1.0
    /// ## `alpha`
    /// value of the alpha channel, between 0 and 1.0
    pub fn init_from_4f(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        // _COGL_RETURN_IF_FAIL (color != NULL);

        // color->red   =  (red * 255);
        // color->green =  (green * 255);
        // color->blue  =  (blue * 255);
        // color->alpha =  (alpha * 255);
        unimplemented!()
    }

    /// Sets the values of the passed channels into a `Color`
    /// ## `color_array`
    /// a pointer to an array of 4 float color components
    pub fn init_from_4fv(&mut self, color_array: &[f32]) {
        // _COGL_RETURN_IF_FAIL (color != NULL);

        // color->red   =  (color_array[0] * 255);
        // color->green =  (color_array[1] * 255);
        // color->blue  =  (color_array[2] * 255);
        // color->alpha =  (color_array[3] * 255);
        unimplemented!()
    }

    /// Sets the values of the passed channels into a `Color`.
    /// ## `red`
    /// value of the red channel, between 0 and 255
    /// ## `green`
    /// value of the green channel, between 0 and 255
    /// ## `blue`
    /// value of the blue channel, between 0 and 255
    /// ## `alpha`
    /// value of the alpha channel, between 0 and 255
    pub fn init_from_4ub(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        // _COGL_RETURN_IF_FAIL (color != NULL);

        // color->red   = red;
        // color->green = green;
        // color->blue  = blue;
        // color->alpha = alpha;
        unimplemented!()
    }

    /// Converts a non-premultiplied color to a pre-multiplied color. For
    /// example, semi-transparent red is (1.0, 0, 0, 0.5) when non-premultiplied
    /// and (0.5, 0, 0, 0.5) when premultiplied.
    pub fn premultiply(&mut self) {
        // color->red = (color->red * color->alpha + 128) / 255;
        // color->green = (color->green * color->alpha + 128) / 255;
        // color->blue = (color->blue * color->alpha + 128) / 255;
        unimplemented!()
    }

    /// Sets the alpha channel of `self` to `alpha`.
    /// ## `alpha`
    /// a float value between 0.0f and 1.0f
    pub fn set_alpha(&mut self, alpha: f32) {
        // color->alpha = alpha * 255.0;
        unimplemented!()
    }

    /// Sets the alpha channel of `self` to `alpha`.
    /// ## `alpha`
    /// a byte value between 0 and 255
    pub fn set_alpha_byte(&mut self, alpha: u8) {
        // color->alpha = alpha;
        unimplemented!()
    }

    /// Sets the alpha channel of `self` to `alpha`.
    /// ## `alpha`
    /// a float value between 0.0f and 1.0f
    pub fn set_alpha_float(&mut self, alpha: f32) {
        // color->alpha = alpha * 255.0;
        unimplemented!()
    }

    /// Sets the blue channel of `self` to `blue`.
    /// ## `blue`
    /// a float value between 0.0f and 1.0f
    pub fn set_blue(&mut self, blue: f32) {
        // color->blue = blue * 255.0;
        unimplemented!()
    }

    /// Sets the blue channel of `self` to `blue`.
    /// ## `blue`
    /// a byte value between 0 and 255
    pub fn set_blue_byte(&mut self, blue: u8) {
        // color->blue = blue;
        unimplemented!()
    }

    /// Sets the blue channel of `self` to `blue`.
    /// ## `blue`
    /// a float value between 0.0f and 1.0f
    pub fn set_blue_float(&mut self, blue: f32) {
        // color->blue = blue * 255.0;
        unimplemented!()
    }

    /// Sets the green channel of `self` to `green`.
    /// ## `green`
    /// a float value between 0.0f and 1.0f
    pub fn set_green(&mut self, green: f32) {
        // color->green = green * 255.0;
        unimplemented!()
    }

    /// Sets the green channel of `self` to `green`.
    /// ## `green`
    /// a byte value between 0 and 255
    pub fn set_green_byte(&mut self, green: u8) {
        // color->green = green;
        unimplemented!()
    }

    /// Sets the green channel of `self` to `green`.
    /// ## `green`
    /// a float value between 0.0f and 1.0f
    pub fn set_green_float(&mut self, green: f32) {
        // color->green = green * 255.0;
        unimplemented!()
    }

    /// Sets the red channel of `self` to `red`.
    /// ## `red`
    /// a float value between 0.0f and 1.0f
    pub fn set_red(&mut self, red: f32) {
        // color->red = red * 255.0;
        unimplemented!()
    }

    /// Sets the red channel of `self` to `red`.
    /// ## `red`
    /// a byte value between 0 and 255
    pub fn set_red_byte(&mut self, red: u8) {
        // color->red = red;
        unimplemented!()
    }

    /// Sets the red channel of `self` to `red`.
    /// ## `red`
    /// a float value between 0.0f and 1.0f
    pub fn set_red_float(&mut self, red: f32) {
        // color->red = red * 255.0;
        unimplemented!()
    }

    /// Converts `self` to the HLS format.
    ///
    /// The `hue` value is in the 0 .. 360 range. The `luminance` and
    /// `saturation` values are in the 0 .. 1 range.
    /// ## `hue`
    /// return location for the hue value or `None`
    /// ## `saturation`
    /// return location for the saturation value or `None`
    /// ## `luminance`
    /// return location for the luminance value or `None`
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        // float red, green, blue;
        // float min, max, delta;
        // float h, l, s;

        // red   = color->red / 255.0;
        // green = color->green / 255.0;
        // blue  = color->blue / 255.0;

        // if (red > green)
        //     {
        //     if (red > blue)
        //     max = red;
        //     else
        //     max = blue;

        //     if (green < blue)
        //     min = green;
        //     else
        //     min = blue;
        //     }
        // else
        //     {
        //     if (green > blue)
        //     max = green;
        //     else
        //     max = blue;

        //     if (red < blue)
        //     min = red;
        //     else
        //     min = blue;
        //     }

        // l = (max + min) / 2;
        // s = 0;
        // h = 0;

        // if (max != min)
        //     {
        //     if (l <= 0.5)
        //     s = (max - min) / (max + min);
        //     else
        //     s = (max - min) / (2.0 - max - min);

        //     delta = max - min;

        //     if (red == max)
        //     h = (green - blue) / delta;
        //     else if (green == max)
        //     h = 2.0 + (blue - red) / delta;
        //     else if (blue == max)
        //     h = 4.0 + (red - green) / delta;

        //     h *= 60;

        //     if (h < 0)
        //     h += 360.0;
        //     }

        // if (hue)
        //     *hue = h;

        // if (luminance)
        //     *luminance = l;

        // if (saturation)
        //     *saturation = s;
        unimplemented!()
    }

    /// Converts a pre-multiplied color to a non-premultiplied color. For
    /// example, semi-transparent red is (0.5, 0, 0, 0.5) when premultiplied
    /// and (1.0, 0, 0, 0.5) when non-premultiplied.
    pub fn unpremultiply(&mut self) {
        // if (color->alpha != 0) {
        //     color->red = (color->red * 255) / color->alpha;
        //     color->green = (color->green * 255) / color->alpha;
        //     color->blue = (color->blue * 255) / color->alpha;
        // }
        unimplemented!()
    }

    fn equal(v1: &Self, v2: &Self) -> bool {
        // const uint32_t *c1 = v1, *c2 = v2;

        // _COGL_RETURN_VAL_IF_FAIL (v1 != NULL, FALSE);
        // _COGL_RETURN_VAL_IF_FAIL (v2 != NULL, FALSE);
      
        // /* XXX: We don't compare the padding */
        // return *c1 == *c2 ? TRUE : FALSE;
        unimplemented!()
    }

    /// Converts a color expressed in HLS (hue, luminance and saturation)
    /// values into a `Color`.
    /// ## `color`
    /// return location for a `Color`
    /// ## `hue`
    /// hue value, in the 0 .. 360 range
    /// ## `saturation`
    /// saturation value, in the 0 .. 1 range
    /// ## `luminance`
    /// luminance value, in the 0 .. 1 range
    pub fn init_from_hsl(hue: f32, saturation: f32, luminance: f32) -> Color {
        // float tmp1, tmp2;
        // float tmp3[3];
        // float clr[3];
        // int   i;

        // hue /= 360.0;

        // if (saturation == 0)
        //     {
        //     color_init_from_4f (color, luminance, luminance, luminance, 1.0f);
        //     return;
        //     }

        // if (luminance <= 0.5)
        //     tmp2 = luminance * (1.0 + saturation);
        // else
        //     tmp2 = luminance + saturation - (luminance * saturation);

        // tmp1 = 2.0 * luminance - tmp2;

        // tmp3[0] = hue + 1.0 / 3.0;
        // tmp3[1] = hue;
        // tmp3[2] = hue - 1.0 / 3.0;

        // for (i = 0; i < 3; i++)
        //     {
        //     if (tmp3[i] < 0)
        //         tmp3[i] += 1.0;

        //     if (tmp3[i] > 1)
        //         tmp3[i] -= 1.0;

        //     if (6.0 * tmp3[i] < 1.0)
        //         clr[i] = tmp1 + (tmp2 - tmp1) * tmp3[i] * 6.0;
        //     else if (2.0 * tmp3[i] < 1.0)
        //         clr[i] = tmp2;
        //     else if (3.0 * tmp3[i] < 2.0)
        //         clr[i] = (tmp1 + (tmp2 - tmp1) * ((2.0 / 3.0) - tmp3[i]) * 6.0);
        //     else
        //         clr[i] = tmp1;
        //     }

        // color_init_from_4f (color, clr[0], clr[1], clr[2], 1.0f);
        unimplemented!()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Color {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Color::equal(self, other)
    }
}

impl Eq for Color {}
