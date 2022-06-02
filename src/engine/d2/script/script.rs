use std::{fmt, rc::Rc};

use crate::engine::d2::{util::Disposable, Component};

use super::Action;

/// Manages a set of actions that are updated over time. Scripts simplify writing composable
/// animations.
pub struct Script {
    pub inner: Component,
    handles: Vec<Handle>,
}

impl Script {
    pub fn new() -> Self {
        Self {
            inner: Component::default(),
            handles: Vec::new(),
        }
    }

    /// Add an action to this Script.
    /// @returns A handle that can be disposed to stop the action.
    pub fn run(&mut self, action: Rc<dyn Action>) -> impl Disposable {
        let handle = Handle::new(action);
        self.handles.push(handle.clone());

        handle
    }

    /// Remove all actions from this Script.
    pub fn stop_all(&mut self) {
        self.handles.clear();
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        let mut idx = 0;
        while idx < self.handles.len() {
            if let Some(handle) = self.handles.get_mut(idx) {
                // removed mean action is none
                if let Some(ref mut action) = handle.action {
                    if let Some(owner) = self.inner.owner {
                        let entity = owner.entity();
                        if action.update(dt, &mut owner.entity()) >= 0.0 {
                            self.handles.remove(idx);
                        } else {
                            idx += 1;
                        }
                    } else {
                        idx += 1;
                    }
                } else {
                    idx += 1;
                }
            }
        }
    }
}

impl AsRef<Component> for Script {
    fn as_ref(&self) -> &Component {
        &self.inner
    }
}

#[derive(Default, Clone)]
pub struct Handle {
    pub action: Option<Rc<dyn Action>>,
}

impl Handle {
    pub fn new(action: Rc<dyn Action>) -> Self {
        Self {
            action: Some(action),
        }
    }
}

impl Disposable for Handle {
    fn dispose(&self) {
        // TODO: should deal with it
        // self.action = None;
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Handle")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
