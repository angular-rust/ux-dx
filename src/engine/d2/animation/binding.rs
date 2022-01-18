use crate::engine::d2::util::Value;

use super::Behavior;

pub type BindingFunction = Box<dyn Fn(f32) -> f32>;

pub struct Binding {
    target: Value<f32>,
    func: Option<BindingFunction>,
}

impl Binding {
    pub fn new(target: Value<f32>, func: Option<BindingFunction>) -> Self {
        Self { target, func }
    }
}

impl Behavior for Binding {
    fn update(&self, dt: f32) -> f32 {
        let value = self.target.get();

        // TODO: Be lazy and only call _fn when the value is changed?
        if let Some(ref func) = self.func {
            func(*value)
        } else {
            *value
        }
    }

    fn is_complete(&self) -> bool {
        false
    }
}
