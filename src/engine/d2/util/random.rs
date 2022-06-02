use crate::engine::d2::math::Math;

/// A seedable, portable random number generator. Fast and random enough for games.
/// [http://en.wikipedia.org/wiki/Linear_congruential_generator][1]
/// 
/// [1]: http://en.wikipedia.org/wiki/Linear_congruential_generator
pub struct Random {
    state: i32,
}

impl Random {
    pub fn new(seed: Option<i32>) -> Self {
        Self {
            state: if let Some(seed) = seed {
                seed
            } else {
                rand::random::<i32>()
            },
        }
    }

    /// Returns an integer between >= 0 and < INT_MAX
    pub fn next_int(&mut self) -> i32 {
        // These constants borrowed from glibc
        // Force float multiplication here to avoid overflow
        self.state = (1103515245.0 * self.state as f32 + 12345.0) as i32 % Math::INT_MAX;

        self.state
    }

    /// Returns a number >= 0 and < 1
    pub fn next_float(&self) -> f32 {
        rand::random::<f32>()
    }

    pub fn reset(&mut self, value: i32) {
        self.state = value;
    }
}
