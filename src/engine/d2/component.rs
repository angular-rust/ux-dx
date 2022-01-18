use crate::engine::d2::util::Disposable;

use super::EntityID;

/// Components are bits of data and logic that can be added to entities.
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Component {
    /// The entity this component is attached to, or None
    pub owner: Option<EntityID>,

    // The owner's next component, for iteration
    // pub next: Box<Option<Component>>,

    // /// The component's name, generated based on its class. Components with the same name replace
    // /// eachother when added to an entity.
    // pub name: String,
    pub flags: u32,
}

impl Component {
    // Component flags
    pub const STARTED: u32 = 1 << 0;
    pub const NEXT_FLAG: u32 = 1 << 1; // Must be last!
}

pub trait ComponentExt {
    /// Called after this component has been added to an entity.
    fn on_added(&self) {}

    /// Called just before this component has been removed from its entity.
    fn on_removed(&self) {}

    /// Called just before this component's first update after being added. This is the best place to
    /// put initialization logic that requires accessing other components/entities, since it waits
    /// until the rest of the entity hierarchy is accessible.
    ///
    /// Note that on_start may be delayed until the next frame after adding a component, depending on
    /// where in the update step it was added.
    fn on_start(&self) {}

    /// Called just before this component will be removed from its entity, if on_start was previously
    /// called.
    fn on_stop(&self) {}

    /// Called when this component receives a game update.
    /// @param dt The time elapsed since the last frame, in seconds.
    fn on_update(&self, dt: f32) {}

    fn name(&self) -> Option<String> {
        // Subclasses will automagically implement this
        None
    }
}

impl ComponentExt for Component {}

impl Disposable for Component {
    /// Removes this component from its owning entity.
    fn dispose(&self) {
        if let Some(ref owner) = self.owner {
            todo!("shold get entity from storage");
            // owner.remove(self);
        }
    }
}
