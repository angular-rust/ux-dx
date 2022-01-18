use crate::engine::d2::animation::AnimatedFloat;

use super::Component;

/// Adjusts the update speed of an entity (and its components and children). Can be used for slow
/// motion and fast forward effects.

#[derive(Default, Clone, Debug)]
pub struct SpeedAdjuster {
    pub inner: Component,
    /// The scale that time should pass for the owning entity and its children. Values lower than
    /// 1.0 will play slower than realtime, and values higher than 1.0 will play faster. When the
    /// scale is 0, the entity is basically paused.
    pub scale: AnimatedFloat,
    pub real_dt: f32,
}

impl SpeedAdjuster {
    // scale: f32 = 1
    pub fn new(scale: f32) -> Self {
        Self {
            inner: Default::default(),
            scale: AnimatedFloat::new(scale, None),
            real_dt: 0.0,
        }
    }

    // Note that this may be called by MainLoop before on_started!
    // override
    pub fn on_update(&mut self, dt: f32) {
        let mut dt = dt;
        // Ensure this component is immune to its own time scaling
        if self.real_dt > 0.0 {
            dt = self.real_dt;
            self.real_dt = 0.0;
        }

        self.scale.update(dt);
    }
}

impl AsRef<Component> for SpeedAdjuster {
    fn as_ref(&self) -> &Component {
        &self.inner
    }
}
