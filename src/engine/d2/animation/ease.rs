use std::{f32::consts::PI, marker::Sized, rc::Rc};

/// Receives and returns a number between 0..1.
pub type EaseFunction = Rc<dyn Fn(f32) -> f32>;

/// Easing functions that can be used to animate values. For a cheat sheet, see <http://easings.net>.
pub struct Ease {}

impl Ease {
    pub const PI_HALF: f32 = PI / 2.0;
    pub const PI2: f32 = PI * 2.0;
    pub const B1: f32 = 1.0 / 2.75;
    pub const B2: f32 = 2.0 / 2.75;
    pub const B3: f32 = 1.5 / 2.75;
    pub const B4: f32 = 2.5 / 2.75;
    pub const B5: f32 = 2.25 / 2.75;
    pub const B6: f32 = 2.625 / 2.75;
    pub const ELASTIC_AMPLITUDE: f32 = 1.0;
    pub const ELASTIC_PERIOD: f32 = 0.4;

    // Operation of in/out easers:
    //
    // in(t)
    //        return t;
    // out(t)
    //         return 1 - in(1 - t);
    // inOut(t)
    //         return (t <= .5) ? in(t * 2) / 2 : out(t * 2 - 1) / 2 + .5;

    /// Linear, no easing.
    pub fn linear(t: f32) -> f32 {
        t
    }

    /// Quadratic in.
    pub fn quad_in(t: f32) -> f32 {
        t * t
    }

    /// Quadratic out.
    pub fn quad_out(t: f32) -> f32 {
        t * (2.0 - t)
    }

    /// Quadratic in and out.
    pub fn quad_in_out(t: f32) -> f32 {
        if t <= 0.5 {
            t * t * 2.0
        } else {
            let t = t - 1.0;
            1.0 - t * t * 2.0
        }
    }

    /// Quadratic out and in
    pub fn quad_out_in(t: f32) -> f32 {
        if t < 0.5 {
            let t = t * 2.0;
            -0.5 * t * (t - 2.0)
        } else {
            let t = t * 2.0 - 1.0;
            0.5 * t * t + 0.5
        }
    }

    /// Cubic in.
    pub fn cube_in(t: f32) -> f32 {
        t * t * t
    }

    /// Cubic out.
    pub fn cube_out(t: f32) -> f32 {
        let t = t - 1.0;
        1.0 + t * t * t
    }

    /// Cubic in and out.
    pub fn cube_in_out(t: f32) -> f32 {
        if t <= 0.5 {
            t * t * t * 4.0
        } else {
            let t = t - 1.0;
            1.0 + t * t * t * 4.0
        }
    }

    /// Cubic out and in.
    pub fn cube_out_in(t: f32) -> f32 {
        let t = t * 2.0 - 1.0;
        0.5 * (t * t * t + 1.0)
    }

    /// Quartic in.
    pub fn quart_in(t: f32) -> f32 {
        t * t * t * t
    }

    /// Quartic out.
    pub fn quart_out(t: f32) -> f32 {
        let t = t - 1.0;
        1.0 - t * t * t * t
    }

    /// Quartic in and out.
    pub fn quart_in_out(t: f32) -> f32 {
        if t <= 0.5 {
            t * t * t * t * 8.0
        } else {
            let t = t * 2.0 - 2.0;
            (1.0 - t * t * t * t) / 2.0 + 0.5
        }
    }

    /// Quartic out and in
    pub fn quart_out_in(t: f32) -> f32 {
        if t < 0.5 {
            let t = t * 2.0 - 1.0;
            -0.5 * t * t * t * t + 0.5
        } else {
            let t = t * 2.0 - 1.0;
            0.5 * t * t * t * t + 0.5
        }
    }

    /// Quintic in.
    pub fn quint_in(t: f32) -> f32 {
        t * t * t * t * t
    }

    /// Quintic out.
    pub fn quint_out(t: f32) -> f32 {
        let t = t - 1.0;
        t * t * t * t * t + 1.0
    }

    /// Quintic in and out.
    pub fn quint_in_out(t: f32) -> f32 {
        let mut t = t * 2.0;
        if t < 1.0 {
            (t * t * t * t * t) / 2.0
        } else {
            t -= 2.0;
            (t * t * t * t * t + 2.0) / 2.0
        }
    }

    /// Quintic out and in.
    pub fn quint_out_in(t: f32) -> f32 {
        let t = t * 2.0 - 1.0;
        0.5 * (t * t * t * t * t + 1.0)
    }

    /// Sine in.
    pub fn sine_in(t: f32) -> f32 {
        1.0 - (Self::PI_HALF * t).cos()
    }

    /// Sine out.
    pub fn sine_out(t: f32) -> f32 {
        (Self::PI_HALF * t).sin()
    }

    /// Sine in and out.
    pub fn sine_in_out(t: f32) -> f32 {
        0.5 - (PI * t).cos() / 2.0
    }

    /// Sine out and in.
    pub fn sine_out_in(t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            if t < 0.5 {
                0.5 * ((t * 2.0) * Self::PI_HALF).sin()
            } else {
                -0.5 * ((t * 2.0 - 1.0) * Self::PI_HALF).cos() + 1.0
            }
        }
    }

    /// Bounce in.
    pub fn bounce_in(t: f32) -> f32 {
        let t = 1.0 - t;
        if t < Self::B1 {
            1.0 - 7.5625 * t * t
        } else if t < Self::B2 {
            1.0 - (7.5625 * (t - Self::B3) * (t - Self::B3) + 0.75)
        } else if t < Self::B4 {
            1.0 - (7.5625 * (t - Self::B5) * (t - Self::B5) + 0.9375)
        } else {
            1.0 - (7.5625 * (t - Self::B6) * (t - Self::B6) + 0.984375)
        }
    }

    /// Bounce out.
    pub fn bounce_out(t: f32) -> f32 {
        if t < Self::B1 {
            7.5625 * t * t
        } else if t < Self::B2 {
            7.5625 * (t - Self::B3) * (t - Self::B3) + 0.75
        } else if t < Self::B4 {
            7.5625 * (t - Self::B5) * (t - Self::B5) + 0.9375
        } else {
            7.5625 * (t - Self::B6) * (t - Self::B6) + 0.984375
        }
    }

    /// Bounce in and out.
    pub fn bounce_in_out(t: f32) -> f32 {
        if t < 0.5 {
            let t = 1.0 - t * 2.0;
            if t < Self::B1 {
                (1.0 - 7.5625 * t * t) / 2.0
            } else if t < Self::B2 {
                (1.0 - (7.5625 * (t - Self::B3) * (t - Self::B3) + 0.75)) / 2.0
            } else if t < Self::B4 {
                (1.0 - (7.5625 * (t - Self::B5) * (t - Self::B5) + 0.9375)) / 2.0
            } else {
                (1.0 - (7.5625 * (t - Self::B6) * (t - Self::B6) + 0.984375)) / 2.0
            }
        } else {
            let t = t * 2.0 - 1.0;
            if t < Self::B1 {
                (7.5625 * t * t) / 2.0 + 0.5
            } else if t < Self::B2 {
                (7.5625 * (t - Self::B3) * (t - Self::B3) + 0.75) / 2.0 + 0.5
            } else if t < Self::B4 {
                (7.5625 * (t - Self::B5) * (t - Self::B5) + 0.9375) / 2.0 + 0.5
            } else {
                (7.5625 * (t - Self::B6) * (t - Self::B6) + 0.984375) / 2.0 + 0.5
            }
        }
    }

    /// Circle in.
    pub fn circ_in(t: f32) -> f32 {
        1.0 - (1.0 - t * t).sqrt()
    }

    /// Circle out.
    pub fn circ_out(t: f32) -> f32 {
        let t = t - 1.0;
        (1.0 - t * t).sqrt()
    }

    /// Circle in and out.
    pub fn circ_in_out(t: f32) -> f32 {
        if t <= 0.5 {
            ((1.0 - t * t * 4.0).sqrt() - 1.0) / -2.0
        } else {
            ((1.0 - (t * 2.0 - 2.0) * (t * 2.0 - 2.0)).sqrt() + 1.0) / 2.0
        }
    }

    /// Circle out and in.
    pub fn circ_out_in(t: f32) -> f32 {
        if t < 0.5 {
            let t = t * 2.0 - 1.0;
            0.5 * (1.0 - t * t).sqrt()
        } else {
            let t = t * 2.0 - 1.0;
            -0.5 * (((1.0 - t * t).sqrt() - 1.0) - 1.0)
        }
    }

    /// Exponential in.
    pub fn expo_in(t: f32) -> f32 {
        2_f32.powf(10.0 * (t - 1.0))
    }

    /// Exponential out.
    pub fn expo_out(t: f32) -> f32 {
        -(2_f32.powf(-10.0 * t)) + 1.0
    }

    /// Exponential in and out.
    pub fn expo_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2_f32.powf(10.0 * (t * 2.0 - 1.0)) / 2.0
        } else {
            (-(2_f32.powf(-10.0 * (t * 2.0 - 1.0))) + 2.0) / 2.0
        }
    }

    /// Exponential out and in.
    pub fn expo_out_in(t: f32) -> f32 {
        if t < 0.5 {
            0.5 * (1.0 - 2_f32.powf(-20.0 * t))
        } else {
            if t == 0.5 {
                0.5
            } else {
                0.5 * (2_f32.powf(20.0 * (t - 1.0)) + 1.0)
            }
        }
    }

    /// Back in.
    pub fn back_in(t: f32) -> f32 {
        t * t * (2.70158 * t - 1.70158)
    }

    /// Back out.
    pub fn back_out(t: f32) -> f32 {
        let t = t - 1.0;
        1.0 - t * t * (-2.70158 * t - 1.70158)
    }

    /// Back in and out.
    pub fn back_in_out(t: f32) -> f32 {
        let mut t = t * 2.0;
        if t < 1.0 {
            t * t * (2.70158 * t - 1.70158) / 2.0
        } else {
            t -= 2.0;
            (1.0 - t * t * (-2.70158 * t - 1.70158)) / 2.0 + 0.5
        }
    }

    /// Elastic in.
    pub fn elastic_in(t: f32) -> f32 {
        let t = t - 1.0;
        -(Self::ELASTIC_AMPLITUDE
            * 2_f32.powf(10.0 * t)
            * ((t - (Self::ELASTIC_PERIOD / Self::PI2 * (1.0 / Self::ELASTIC_AMPLITUDE).asin()))
                * Self::PI2
                / Self::ELASTIC_PERIOD)
                .sin())
    }

    /// Elastic out.
    pub fn elastic_out(t: f32) -> f32 {
        Self::ELASTIC_AMPLITUDE
            * 2_f32.powf(-10.0 * t)
            * ((t - (Self::ELASTIC_PERIOD / Self::PI2 * (1.0 / Self::ELASTIC_AMPLITUDE).asin()))
                * Self::PI2
                / Self::ELASTIC_PERIOD)
                .sin()
            + 1.0
    }

    /// Elastic in and out.
    pub fn elastic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            let t = t - 0.5;
            -0.5 * (2_f32.powf(10.0 * t)
                * ((t - (Self::ELASTIC_PERIOD / 4.0)) * Self::PI2 / Self::ELASTIC_PERIOD).sin())
        } else {
            let t = t - 0.5;
            2_f32.powf(-10.0 * t)
                * ((t - (Self::ELASTIC_PERIOD / 4.0)) * Self::PI2 / Self::ELASTIC_PERIOD).sin()
                * 0.5
                + 1.0
        }
    }
}
