use crate::foundation::time;

/// Just a simnple renderable trait
pub trait Renderable {
    fn render(&self, t: time::Time);
}