use std::cmp::PartialEq;

pub trait Behavior {
    fn update(&self, dt: f32) -> f32;
    fn is_complete(&self) -> bool;
}
