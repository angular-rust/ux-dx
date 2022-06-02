use crate::engine::d2::Entity;

/// A transition between two scenes.
#[derive(Default, Clone, Debug)]
pub struct Transition {
    pub from: Entity,
    pub to: Entity,
}

impl Transition {
    /// Called by the Director to start the transition.
    /// @param from The old scene being transitioned from.
    /// @param to The new scene being transitioned to.
    pub fn init(&mut self, from: Entity, to: Entity) {
        self.from = from;
        self.to = to;
    }

    /// Called by the Director to update the transition.
    /// @returns True if the transition is complete.
    pub fn update(&self, dt: f32) -> bool {
        // See subclasses
        true
    }

    /// Completes the transition. Note that the Director may call this at any time to fast-forward
    /// the transition.
    pub fn complete(&self) {
        // See subclasses
    }
}
