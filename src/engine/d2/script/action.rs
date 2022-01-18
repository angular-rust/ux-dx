use crate::engine::d2::Entity;

/// Represents a unit of execution that is called over time.
pub trait Action {
    /// Called when the acting entity has been updated.
    ///  *
    /// @param dt The time elapsed since the last frame, in seconds.
    /// @param actor The entity of the Script that this action was added to.
    /// @returns The amount of time in seconds spent this frame to finish the action, which may be
    ///   less than dt. Or -1 if the action is not yet finished.
    fn update(&self, dt: f32, actor: &mut Entity) -> f32;
}
