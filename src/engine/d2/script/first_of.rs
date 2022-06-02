use std::cell::RefCell;

use crate::engine::d2::Entity;

use super::Action;

struct FirstOfProps {
    running_actions: Vec<Box<dyn Action>>,
}
/// An action that manages a list of other actions, running them together in parallel until the
/// first of them finishes.
pub struct FirstOf {
    props: RefCell<FirstOfProps>,
}

impl FirstOf {
    pub fn new<A: Action>(actions: Option<Vec<Box<dyn Action>>>) -> Self {
        Self {
            props: RefCell::new(FirstOfProps {
                running_actions: actions.unwrap_or_default(),
            }),
        }
    }

    pub fn add(&self, action: Box<dyn Action>) {
        let mut props = self.props.borrow_mut();
        props.running_actions.push(action);
    }

    // pub fn remove(&self, action: Box<dyn Action>) -> bool {
    //     match self.running_actions.iter().position(|&item| item == action) {
    //         Some(idx) => {
    //             self.running_actions.remove(idx);
    //             true
    //         },
    //         None => false
    //     }
    // }

    pub fn remove_all(&self) {
        let mut props = self.props.borrow_mut();
        props.running_actions.clear();
    }
}

impl Action for FirstOf {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        let mut props = self.props.borrow_mut();
        for action in props.running_actions.iter_mut() {
            let spent = action.update(dt, actor);
            if spent >= 0.0 {
                return spent;
            }
        }

        -1.0
    }
}
