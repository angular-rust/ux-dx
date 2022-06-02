use std::cell::RefCell;

use crate::engine::d2::Entity;

use super::Action;

struct ParallelProps {
    running_actions: Vec<Box<dyn Action>>,
    completed_actions: Vec<Box<dyn Action>>,
}

/// An action that manages a list of other actions, running them together in parallel until they all
/// finish.
pub struct Parallel {
    props: RefCell<ParallelProps>,
}

impl Parallel {
    pub fn new<A: Action>(actions: Option<Vec<Box<dyn Action>>>) -> Self {
        Self {
            props: RefCell::new(ParallelProps {
                completed_actions: Vec::new(),
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
    //         }
    //         None => false
    //     }
    // }

    pub fn remove_all(&mut self) {
        let mut props = self.props.borrow_mut();
        props.running_actions.clear();
        props.completed_actions.clear();
    }
}

impl Action for Parallel {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        let mut props = self.props.borrow_mut();
        let mut done = true;
        let mut max_spent = 0.0;
        for idx in 0..props.running_actions.len() {
            if let Some(action) = props.running_actions.get_mut(idx) {
                let spent = action.update(dt, actor);
                if spent >= 0.0 {
                    props.running_actions.remove(idx);
                    // self.completed_actions.push(action); // DV
                    if spent > max_spent {
                        max_spent = spent;
                    }
                } else {
                    // We can't possibly finish this frame, but continue ticking the rest of the
                    // actions anyways
                    done = false;
                }
            }
        }

        if done {
            // self.running_actions = self.completed_actions; // DV
            props.completed_actions.clear();

            max_spent
        } else {
            -1.0
        }
    }
}
