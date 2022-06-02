use crate::engine::d2::Entity;

use super::Action;

/// An action that calls a given fn once and immediately completes.
pub struct CallFunction {
    func: Box<dyn Fn()>,
}

impl CallFunction {
    /// @param fn The fn to call when this action is run.
    pub fn new(func: Box<dyn Fn()>) -> Self {
        Self { func }
    }
}

impl Action for CallFunction {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        (self.func)();
        0.0
    }
}
