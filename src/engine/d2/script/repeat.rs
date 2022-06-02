use std::cell::RefCell;

use crate::engine::d2::Entity;

use super::Action;

struct RepeatProps {
    action: Box<dyn Action>,
    count: i32,
    remaining: i32,
}
/// An action that repeats another action until it finishes a certain number of times.
pub struct Repeat {
    props: RefCell<RepeatProps>,
}

impl Repeat {
    /// @param count The number of times to repeat the action, or -1 to repeat forever.
    //  count: i32 = -1
    pub fn new(action: Box<dyn Action>, count: i32) -> Self {
        Self {
            props: RefCell::new(RepeatProps {
                action,
                count,
                remaining: count,
            }),
        }
    }
}

impl Action for Repeat {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        let mut props = self.props.borrow_mut();
        if props.count == 0 {
            // Handle the special case of a 0-count Repeat
            return 0.0;
        }

        let spent = props.action.update(dt, actor);
        props.remaining -= 1;
        if props.count > 0 && spent >= 0.0 && props.remaining == 0 {
            props.remaining = props.count; // Reset state in case this Action is reused
            return spent;
        }

        // Keep repeating
        -1.0
    }
}
