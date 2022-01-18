use std::cell::RefCell;

use crate::engine::d2::Entity;

use super::Action;

struct DelayProps {
    duration: f32,
    elapsed: f32,
}
/// An action that simply waits for a certain amount of time to pass before finishing.
pub struct Delay {
    props: RefCell<DelayProps>,
}

impl Delay {
    pub fn new(seconds: f32) -> Self {
        Self {
            props: RefCell::new(DelayProps {
                duration: seconds,
                elapsed: 0.0,
            }),
        }
    }
}

impl Action for Delay {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        let mut props = self.props.borrow_mut();
        props.elapsed += dt;
        if props.elapsed >= props.duration {
            let overtime = props.elapsed - props.duration;
            props.elapsed = 0.0;

            dt - overtime
        } else {
            -1.0
        }
    }
}
