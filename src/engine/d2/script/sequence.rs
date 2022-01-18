use std::cell::RefCell;

use crate::engine::d2::Entity;

use super::Action;

struct SequenceProps {
    running_actions: Vec<Box<dyn Action>>,
    idx: usize,
}

/// An action that manages a list of other actions, running them one-by-one sequentially until they
/// all finish.
pub struct Sequence {
    props: RefCell<SequenceProps>,
}

impl Sequence {
    pub fn new(actions: Option<Vec<Box<dyn Action>>>) -> Self {
        Self {
            props: RefCell::new(SequenceProps {
                idx: 0,
                running_actions: actions.unwrap_or_default(),
            }),
        }
    }

    pub fn add(&self, action: Box<dyn Action>) -> &Self {
        let mut props = self.props.borrow_mut();
        props.running_actions.push(action);

        self
    }

    // pub fn remove(&self, action: Box<dyn Action>) -> bool {
    //     let mut props = self.props.borrow_mut();
    //     match props.running_actions.iter().position(|&item| item == action) {
    //         Some(idx) => {
    //             props.running_actions.remove(idx);
    //             true
    //         },
    //         None => false
    //     }
    // }

    pub fn remove_all(&self) {
        let mut props = self.props.borrow_mut();
        props.idx = 0;
        props.running_actions.clear();
    }
}

impl Action for Sequence {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        let mut props = self.props.borrow_mut();
        // The total time taken by the actions updated this frame
        let mut total = 0.0;

        loop {
            let idx = props.idx;
            if let Some(action) = props.running_actions.get_mut(idx) {
                let spent = action.update(dt - total, actor);
                if spent >= 0.0 {
                    // This action completed, add it to the total time
                    total += spent;
                } else {
                    // This action didn't complete, so neither will this sequence
                    return -1.0;
                }
            }

            props.idx += 1;
            if props.idx >= props.running_actions.len() {
                // If this is the last action, reset to the starting position and finish
                props.idx = 0;
                break;
            } else if total > dt {
                // Otherwise, if there are still actions but not enough time to complete them
                return -1.0;
            }
        }

        total
    }
}
