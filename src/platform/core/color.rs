// use std::mem;

// // @short_description: A generic color definition
// //
// // #Color is a simple structure holding the definition of a color such
// // that it can be efficiently used by GL
// // Color:
// // @red: amount of red
// // @green: amount of green
// // @blue: amount of green
// // @alpha: alpha
// //
// // A structure for holding a color definition. The contents of
// // the Color structure are private and should never by accessed
// // directly.
// //
// // Since: 1.0
// #[derive(Default, Debug, PartialOrd, Ord, Clone, Copy)] // Hash
// pub struct Color {
//     // < private >
//     pub red: u8,
//     pub green: u8,
//     pub blue: u8,

//     pub alpha: u8,
//     // padding in case we want to change to floats at some point

//     // uint32_t PRIVATE (padding0);
//     // uint32_t PRIVATE (padding1);
//     // uint32_t PRIVATE (padding2);
// }

// impl Color {
//     /// Creates a new (empty) color
//     ///
//     /// # Returns
//     ///
//     /// a newly-allocated `Color`. Use `Color::free`
//     ///  to free the allocated resources
//     pub fn new() -> Color {
//         Default::default()
//     }

//     /// Creates a new color from channels.
//     /// ## `red`
//     /// value of the red channel, between 0 and 1.0
//     /// ## `green`
//     /// value of the green channel, between 0 and 1.0
//     /// ## `blue`
//     /// value of the blue channel, between 0 and 1.0
//     /// ## `alpha`
//     /// value of the alpha channel, between 0 and 1.0
//     pub fn from_4f(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
//         Self {
//             red: (red * 255.0) as u8,
//             green: (green * 255.0) as u8,
//             blue: (blue * 255.0) as u8,
//             alpha: (alpha * 255.0) as u8,
//         }
//     }

//     /// Creates a new color from `color_array`.
//     /// ## `color_array`
//     /// a pointer to an array of 4 float color components
//     pub fn from_4fv(color_array: &[f32; 4]) -> Color {
//         Self {
//             red: (color_array[0] * 255.0) as u8,
//             green: (color_array[1] * 255.0) as u8,
//             blue: (color_array[2] * 255.0) as u8,
//             alpha: (color_array[3] * 255.0) as u8,
//         }
//     }

//     /// Creates a new color from channels.
//     /// ## `red`
//     /// value of the red channel, between 0 and 255
//     /// ## `green`
//     /// value of the green channel, between 0 and 255
//     /// ## `blue`
//     /// value of the blue channel, between 0 and 255
//     /// ## `alpha`
//     /// value of the alpha channel, between 0 and 255
//     pub fn from_4ub(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
//         Self {
//             red,
//             green,
//             blue,
//             alpha,
//         }
//     }

//     /// Converts a color expressed in HLS (hue, luminance and saturation)
//     /// values into a `Color`.
//     /// ## `color`
//     /// return location for a `Color`
//     /// ## `hue`
//     /// hue value, in the 0 .. 360 range
//     /// ## `saturation`
//     /// saturation value, in the 0 .. 1 range
//     /// ## `luminance`
//     /// luminance value, in the 0 .. 1 range
//     pub fn from_hsl(hue: f32, saturation: f32, luminance: f32) -> Color {
//         if saturation == 0.0 {
//             return Self::from_4f(luminance, luminance, luminance, 1.0);
//         }

//         let tmp2 = if luminance <= 0.5 {
//             luminance * (1.0 + saturation)
//         } else {
//             luminance + saturation - (luminance * saturation)
//         };

//         let tmp1 = 2.0 * luminance - tmp2;

//         let hue = hue % 360.0;

//         let mut tmp3: [f32; 3] = [0.0; 3];
//         tmp3[0] = hue + 1.0 / 3.0;
//         tmp3[1] = hue;
//         tmp3[2] = hue - 1.0 / 3.0;

//         let mut clr: [f32; 3] = [0.0; 3];
//         for i in 0..3 {
//             if tmp3[i] < 0.0 {
//                 tmp3[i] += 1.0;
//             }

//             if tmp3[i] > 1.0 {
//                 tmp3[i] -= 1.0;
//             }

//             if 6.0 * tmp3[i] < 1.0 {
//                 clr[i] = tmp1 + (tmp2 - tmp1) * tmp3[i] * 6.0;
//             } else if 2.0 * tmp3[i] < 1.0 {
//                 clr[i] = tmp2;
//             } else if 3.0 * tmp3[i] < 2.0 {
//                 clr[i] = tmp1 + (tmp2 - tmp1) * ((2.0 / 3.0) - tmp3[i]) * 6.0;
//             } else {
//                 clr[i] = tmp1;
//             }
//         }

//         Self::from_4f(clr[0], clr[1], clr[2], 1.0)
//     }
    
//     /// Retrieves the alpha channel of `self` as a fixed point
//     /// value between 0 and 1.0.
//     ///
//     /// # Returns
//     ///
//     /// the alpha channel of the passed color
//     pub fn alpha(&self) -> f32 {
//         self.alpha as f32 / 255.0
//     }

//     /// Retrieves the alpha channel of `self` as a byte value
//     /// between 0 and 255
//     ///
//     /// # Returns
//     ///
//     /// the alpha channel of the passed color
//     pub fn alpha_byte(&self) -> u8 {
//         self.alpha
//     }

//     /// Retrieves the alpha channel of `self` as a floating point
//     /// value between 0.0 and 1.0
//     ///
//     /// # Returns
//     ///
//     /// the alpha channel of the passed color
//     #[deprecated(note="Use get_alpha()")]
//     pub fn alpha_float(&self) -> f32 {
//         self.alpha as f32 / 255.0
//     }

//     /// Retrieves the blue channel of `self` as a fixed point
//     /// value between 0 and 1.0.
//     ///
//     /// # Returns
//     ///
//     /// the blue channel of the passed color
//     pub fn blue(&self) -> f32 {
//         self.blue as f32 / 255.0
//     }

//     /// Retrieves the blue channel of `self` as a byte value
//     /// between 0 and 255
//     ///
//     /// # Returns
//     ///
//     /// the blue channel of the passed color
//     pub fn blue_byte(&self) -> u8 {
//         self.blue
//     }

//     /// Retrieves the blue channel of `self` as a floating point
//     /// value between 0.0 and 1.0
//     ///
//     /// # Returns
//     ///
//     /// the blue channel of the passed color
//     #[deprecated(note="Use get_blue()")]
//     pub fn blue_float(&self) -> f32 {
//         self.blue as f32 / 255.0
//     }

//     /// Retrieves the green channel of `self` as a fixed point
//     /// value between 0 and 1.0.
//     ///
//     /// # Returns
//     ///
//     /// the green channel of the passed color
//     pub fn green(&self) -> f32 {
//         self.green as f32 / 255.0
//     }

//     /// Retrieves the green channel of `self` as a byte value
//     /// between 0 and 255
//     ///
//     /// # Returns
//     ///
//     /// the green channel of the passed color
//     pub fn green_byte(&self) -> u8 {
//         self.green
//     }

//     /// Retrieves the green channel of `self` as a floating point
//     /// value between 0.0 and 1.0
//     ///
//     /// # Returns
//     ///
//     /// the green channel of the passed color
//     #[deprecated(note="Use get_green()")]
//     pub fn green_float(&self) -> f32 {
//         self.green as f32 / 255.0
//     }

//     /// Retrieves the red channel of `self` as a fixed point
//     /// value between 0 and 1.0.
//     ///
//     /// # Returns
//     ///
//     /// the red channel of the passed color
//     pub fn red(&self) -> f32 {
//         self.red as f32 / 255.0
//     }

//     /// Retrieves the red channel of `self` as a byte value
//     /// between 0 and 255
//     ///
//     /// # Returns
//     ///
//     /// the red channel of the passed color
//     pub fn red_byte(&self) -> u8 {
//         self.red
//     }

//     /// Retrieves the red channel of `self` as a floating point
//     /// value between 0.0 and 1.0
//     ///
//     /// # Returns
//     ///
//     /// the red channel of the passed color
//     #[deprecated(note="Use get_red()")]
//     pub fn red_float(&self) -> f32 {
//         self.red as f32 / 255.0
//     }

//     /// Sets the values of the passed channels into a `Color`
//     /// ## `red`
//     /// value of the red channel, between 0 and 1.0
//     /// ## `green`
//     /// value of the green channel, between 0 and 1.0
//     /// ## `blue`
//     /// value of the blue channel, between 0 and 1.0
//     /// ## `alpha`
//     /// value of the alpha channel, between 0 and 1.0
//     #[deprecated(note="Use from_4f()")]
//     pub fn init_from_4f(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
//         self.red = (red * 255.0) as u8;
//         self.green = (green * 255.0) as u8;
//         self.blue = (blue * 255.0) as u8;
//         self.alpha = (alpha * 255.0) as u8;
//     }

//     /// Sets the values of the passed channels into a `Color`
//     /// ## `color_array`
//     /// a pointer to an array of 4 float color components
//     #[deprecated(note="Use from_4fv()")]
//     pub fn init_from_4fv(&mut self, color_array: &[f32; 4]) {
//         self.red = (color_array[0] * 255.0) as u8;
//         self.green = (color_array[1] * 255.0) as u8;
//         self.blue = (color_array[2] * 255.0) as u8;
//         self.alpha = (color_array[3] * 255.0) as u8;
//     }

//     /// Sets the values of the passed channels into a `Color`.
//     /// ## `red`
//     /// value of the red channel, between 0 and 255
//     /// ## `green`
//     /// value of the green channel, between 0 and 255
//     /// ## `blue`
//     /// value of the blue channel, between 0 and 255
//     /// ## `alpha`
//     /// value of the alpha channel, between 0 and 255
//     #[deprecated(note="Use from_4ub()")]
//     pub fn init_from_4ub(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
//         self.red = red;
//         self.green = green;
//         self.blue = blue;
//         self.alpha = alpha;
//     }

//     /// Converts a non-premultiplied color to a pre-multiplied color. For
//     /// example, semi-transparent red is (1.0, 0, 0, 0.5) when non-premultiplied
//     /// and (0.5, 0, 0, 0.5) when premultiplied.
//     pub fn premultiply(&mut self) {
//         self.red = ((self.red * self.alpha + 128) as f32 / 255.0) as u8;
//         self.green = ((self.green * self.alpha + 128) as f32 / 255.0) as u8;
//         self.blue = ((self.blue * self.alpha + 128) as f32 / 255.0) as u8;
//     }

//     /// Sets the alpha channel of `self` to `alpha`.
//     /// ## `alpha`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_alpha(&mut self, alpha: f32) {
//         self.alpha = (alpha * 255.0) as u8;
//     }

//     /// Sets the alpha channel of `self` to `alpha`.
//     /// ## `alpha`
//     /// a byte value between 0 and 255
//     pub fn set_alpha_byte(&mut self, alpha: u8) {
//         self.alpha = alpha;
//     }

//     /// Sets the alpha channel of `self` to `alpha`.
//     /// ## `alpha`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_alpha_float(&mut self, alpha: f32) {
//         self.alpha = (alpha * 255.0) as u8;
//     }

//     /// Sets the blue channel of `self` to `blue`.
//     /// ## `blue`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_blue(&mut self, blue: f32) {
//         self.blue = (blue * 255.0) as u8;
//     }

//     /// Sets the blue channel of `self` to `blue`.
//     /// ## `blue`
//     /// a byte value between 0 and 255
//     pub fn set_blue_byte(&mut self, blue: u8) {
//         self.blue = blue;
//     }

//     /// Sets the blue channel of `self` to `blue`.
//     /// ## `blue`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_blue_float(&mut self, blue: f32) {
//         self.blue = (blue * 255.0) as u8;
//     }

//     /// Sets the green channel of `self` to `green`.
//     /// ## `green`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_green(&mut self, green: f32) {
//         self.green = (green * 255.0) as u8;
//     }

//     /// Sets the green channel of `self` to `green`.
//     /// ## `green`
//     /// a byte value between 0 and 255
//     pub fn set_green_byte(&mut self, green: u8) {
//         self.green = green;
//     }

//     /// Sets the green channel of `self` to `green`.
//     /// ## `green`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_green_float(&mut self, green: f32) {
//         self.green = (green * 255.0) as u8;
//     }

//     /// Sets the red channel of `self` to `red`.
//     /// ## `red`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_red(&mut self, red: f32) {
//         self.red = (red * 255.0) as u8;
//     }

//     /// Sets the red channel of `self` to `red`.
//     /// ## `red`
//     /// a byte value between 0 and 255
//     pub fn set_red_byte(&mut self, red: u8) {
//         self.red = red;
//     }

//     /// Sets the red channel of `self` to `red`.
//     /// ## `red`
//     /// a float value between 0.0f and 1.0f
//     pub fn set_red_float(&mut self, red: f32) {
//         self.red = (red * 255.0) as u8;
//     }

//     /// Converts `self` to the HLS format.
//     ///
//     /// The `hue` value is in the 0 .. 360 range. The `luminance` and
//     /// `saturation` values are in the 0 .. 1 range.
//     /// ## `hue`
//     /// return location for the hue value or `None`
//     /// ## `saturation`
//     /// return location for the saturation value or `None`
//     /// ## `luminance`
//     /// return location for the luminance value or `None`
//     pub fn to_hsl(&self) -> (f32, f32, f32) {
//         let min: f32;
//         let max: f32;

//         let red = self.red as f32 / 255.0;
//         let green = self.green as f32 / 255.0;
//         let blue = self.blue as f32 / 255.0;

//         if red > green {
//             if red > blue {
//                 max = red;
//             } else {
//                 max = blue;
//             }

//             if green < blue {
//                 min = green;
//             } else {
//                 min = blue;
//             }
//         } else {
//             if green > blue {
//                 max = green;
//             } else {
//                 max = blue;
//             }

//             if red < blue {
//                 min = red;
//             } else {
//                 min = blue;
//             }
//         }

//         let l: f32 = (max + min) / 2.0;
//         let mut s: f32 = 0.0;
//         let mut h: f32 = 0.0;

//         let error_margin = std::f32::EPSILON;

//         if (max - min).abs() > error_margin {
//             if l <= 0.5 {
//                 s = (max - min) / (max + min);
//             } else {
//                 s = (max - min) / (2.0 - max - min);
//             }

//             let delta = max - min;

//             if (red - max).abs() < error_margin {
//                 h = (green - blue) / delta;
//             } else if (green - max).abs() < error_margin {
//                 h = 2.0 + (blue - red) / delta;
//             } else if (blue - max).abs() < error_margin {
//                 h = 4.0 + (red - green) / delta;
//             }

//             h *= 60.0;

//             if h < 0.0 {
//                 h += 360.0;
//             }
//         }

//         (h, s, l)
//     }

//     /// Converts a pre-multiplied color to a non-premultiplied color. For
//     /// example, semi-transparent red is (0.5, 0, 0, 0.5) when premultiplied
//     /// and (1.0, 0, 0, 0.5) when non-premultiplied.
//     pub fn unpremultiply(&mut self) {
//         if self.alpha != 0 {
//             self.red = ((self.red * 255) as f32 / self.alpha as f32) as u8;
//             self.green = ((self.green * 255) as f32 / self.alpha as f32) as u8;
//             self.blue = ((self.blue * 255) as f32 / self.alpha as f32) as u8;
//         }
//     }

//     fn equal(v1: &Self, v2: &Self) -> bool {
//         v1.red == v2.red && v1.green == v2.green && v1.blue == v2.blue && v1.alpha == v2.alpha
//     }

//     /// Converts a color expressed in HLS (hue, luminance and saturation)
//     /// values into a `Color`.
//     /// ## `color`
//     /// return location for a `Color`
//     /// ## `hue`
//     /// hue value, in the 0 .. 360 range
//     /// ## `saturation`
//     /// saturation value, in the 0 .. 1 range
//     /// ## `luminance`
//     /// luminance value, in the 0 .. 1 range
//     #[deprecated(note="Use from_hsl()")]
//     pub fn init_from_hsl(hue: f32, saturation: f32, luminance: f32) -> Color {
//         if saturation == 0.0 {
//             return Self::from_4f(luminance, luminance, luminance, 1.0);
//         }

//         let tmp2 = if luminance <= 0.5 {
//             luminance * (1.0 + saturation)
//         } else {
//             luminance + saturation - (luminance * saturation)
//         };

//         let tmp1 = 2.0 * luminance - tmp2;

//         let hue = hue % 360.0;

//         let mut tmp3: [f32; 3] = [0.0; 3];
//         tmp3[0] = hue + 1.0 / 3.0;
//         tmp3[1] = hue;
//         tmp3[2] = hue - 1.0 / 3.0;

//         let mut clr: [f32; 3] = [0.0; 3];
//         for i in 0..3 {
//             if tmp3[i] < 0.0 {
//                 tmp3[i] += 1.0;
//             }

//             if tmp3[i] > 1.0 {
//                 tmp3[i] -= 1.0;
//             }

//             if 6.0 * tmp3[i] < 1.0 {
//                 clr[i] = tmp1 + (tmp2 - tmp1) * tmp3[i] * 6.0;
//             } else if 2.0 * tmp3[i] < 1.0 {
//                 clr[i] = tmp2;
//             } else if 3.0 * tmp3[i] < 2.0 {
//                 clr[i] = tmp1 + (tmp2 - tmp1) * ((2.0 / 3.0) - tmp3[i]) * 6.0;
//             } else {
//                 clr[i] = tmp1;
//             }
//         }

        
//         Self::from_4f(clr[0], clr[1], clr[2], 1.0)
//     }
// }

// impl PartialEq for Color {
//     #[inline]
//     fn eq(&self, other: &Self) -> bool {
//         Color::equal(self, other)
//     }
// }

// impl Eq for Color {}
