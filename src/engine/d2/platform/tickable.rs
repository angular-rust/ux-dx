/// An object that exists outside of the entity hierarchy, but still needs to be updated each frame.
/// This is an implementation detail, nothing outside of d2::platform should implement self.
pub trait Tickable {
    /// @param dt The elapsed delta-time in seconds.
    /// @returns True if this Tickable should no longer be updated.
    fn update(&self, dt: f32) -> bool;
}
