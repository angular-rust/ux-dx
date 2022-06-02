use crate::engine::d2::{
    display::Sprite, subsystem::RendererSystem, util::BitSets, Component, Entity, EntityManager,
    SpeedAdjuster, System,
};

use super::Tickable;

/// Updates all components and renders.
pub struct MainLoop {
    tickables: Vec<Box<dyn Tickable>>,
}

impl MainLoop {
    pub fn new() -> Self {
        Self {
            tickables: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if dt <= 0.0 {
            // This can happen on platforms that don't have monotonic timestamps and are prone to
            // system clock adjustment
            // log::warn!("Zero or negative time elapsed since the last frame! [rt: {}]", dt);
            return;
        }

        let dt = if dt > 1.0 {
            // Clamp deltaTime to a reasonable limit. Games tend not to cope well with huge
            // deltaTimes. Platforms should skip the next frame after unpausing to prevent sending
            // huge deltaTimes, but not all environments support detecting an unpause
            1.0
        } else {
            dt
        };

        // First update any tickables, folding away Nones
        let mut idx = 0;
        while idx < self.tickables.len() {
            if let Some(t) = self.tickables.get(idx) {
                if t.update(dt) {
                    self.tickables.remove(idx);
                } else {
                    idx += 1;
                }
            }
        }

        System::volume().update(dt);

        Self::update_entity(System::root(), dt);
    }

    pub fn render(&self, renderer: impl RendererSystem) {
        let g = renderer.graphics();
        renderer.will_render();
        unimplemented!()
        // Sprite::render(System::root(), renderer.graphics());
        // renderer.did_render();
    }

    pub fn add_tickable(&mut self, t: Box<dyn Tickable>) {
        self.tickables.push(t);
    }

    pub fn remove_tickable(&self, t: Box<dyn Tickable>) {
        unimplemented!()
        // if let Some(idx) = self.tickables.iter().position(|&item| {item == t}) {
        //     // Actual removals only happen in update()
        //     self.tickables[idx] = None;
        // }
    }

    // static
    fn update_entity(entity: Entity, dt: f32) {
        // Handle update speed adjustment
        if let Some(ref mut speed) = EntityManager::<SpeedAdjuster>::get(&entity) {
            speed.real_dt = dt;

            let dt = dt * speed.scale.get();
            if dt <= 0.0 {
                // This entity is paused, avoid descending into children. But do update the speed
                // adjuster (so it can still be animated)
                speed.on_update(dt);
                return;
            }
        }

        log::warn!("Should deal with components update logic");
        // // Update components
        // let p = entity.first_component;
        // while p.is_some() {
        //     let next = p.next;
        //     if !p.flags.contains(Component::STARTED) {
        //         p.flags = p.flags.add(Component::STARTED);
        //         p.on_start();
        //     }
        //     p.on_update(dt);
        //     p = next;
        // }

        // Update children
        let mut child = entity.first_child;
        while let Some(id) = child {
            let entity = id.entity();
            let next = entity.next;
            Self::update_entity(entity, dt);
            child = next;
        }
    }
}
