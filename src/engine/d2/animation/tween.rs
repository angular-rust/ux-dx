use std::{cell::RefCell, rc::Rc};

use crate::engine::d2::animation::Ease;

use super::{Behavior, EaseFunction};

struct TweenProps {
    elapsed: f32,
    from: f32,
    to: f32,
    duration: f32,
}

pub struct Tween {
    props: RefCell<TweenProps>,
    easing: EaseFunction,
}

impl Tween {
    pub fn new(from: f32, to: f32, seconds: f32, easing: Option<EaseFunction>) -> Self {
        let props = RefCell::new(TweenProps {
            from,
            to,
            elapsed: 0.0,
            duration: seconds,
        });

        Self {
            props,
            easing: easing.unwrap_or(Rc::new(Ease::linear)),
        }
    }

    pub fn elapsed(&self) -> f32 {
        self.props.borrow().elapsed
    }
}

impl Behavior for Tween {
    fn update(&self, dt: f32) -> f32 {
        let mut props = self.props.borrow_mut();
        props.elapsed += dt;

        if props.elapsed >= props.duration {
            props.to
        } else {
            props.from + (props.to - props.from) * (self.easing)(props.elapsed / props.duration)
        }
    }

    fn is_complete(&self) -> bool {
        let props = self.props.borrow();
        props.elapsed >= props.duration
    }
}
