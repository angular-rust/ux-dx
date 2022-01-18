use super::Behavior;

#[derive(Default, Clone, Copy, Debug)]
pub struct Jitter {
    pub base: f32,
    pub strength: f32,
}

impl Jitter {
    pub fn new(base: f32, strength: f32) -> Self {
        Self { base, strength }
    }
}

impl Behavior for Jitter {
    fn update(&self, dt: f32) -> f32 {
        self.base + 2.0 * rand::random::<f32>() * self.strength - self.strength
    }

    fn is_complete(&self) -> bool {
        false
    }
}
