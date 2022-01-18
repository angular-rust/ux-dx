use cgmath::prelude::*;
use cgmath::Point2;
use std::rc::Rc;

use crate::engine::d2::{
    display::Sprite,
    input::{EventSource, PointerEvent},
    subsystem::PointerSystem,
    util::Signal1,
    EntityManager, System,
};

pub struct BasicPointer {
    pub down: Signal1<PointerEvent>,
    pub move_: Signal1<PointerEvent>,
    pub up: Signal1<PointerEvent>,

    // static
    shared_event: PointerEvent, // = PointerEvent::new();
    // static
    // scratch_point: Point2<f32>, // i32
    x: f32,
    y: f32,
    is_down: bool,
}

impl BasicPointer {
    // x: f32 = 0, y: f32 = 0, isDown :bool = false
    pub fn new(x: f32, y: f32, is_down: bool) -> Self {
        Self {
            down: Signal1::new(None),
            move_: Signal1::new(None),
            up: Signal1::new(None),
            x,
            y,
            is_down,
            // scratch_point: Point2::zero(),
            shared_event: PointerEvent::new(),
        }
    }

    /// Called by the platform to handle a down event.
    pub fn submit_down(&mut self, view_x: f32, view_y: f32, source: EventSource) {
        if self.is_down {
            return; // Ignore repeat down events
        }
        // Ensure a move event is sent first
        self.submit_move(view_x, view_y, source.clone());
        self.is_down = true;

        // Take a snapshot of the entire event bubbling chain
        let mut chain: Vec<Sprite> = Vec::new();
        let hit = Sprite::hit_test(System::root(), view_x, view_y);
        if let Some(hitsprite) = hit.clone() {
            let mut owner = hitsprite.component.owner;
            loop {
                match owner {
                    Some(item) => {
                        let entity = item.entity();
                        if let Some(sprite) = EntityManager::<Sprite>::get(&entity) {
                            chain.push(sprite.clone());
                        }
                        owner = entity.parent;
                    }
                    None => break,
                }
            }
        }
        // Finally, emit the event up the chain
        self.prepare(view_x, view_y, hit, Some(source));
        for ref mut sprite in chain {
            sprite.on_pointer_down(self.shared_event.clone());
            if self.shared_event.stopped {
                return;
            }
        }
        self.down.emit(self.shared_event.clone());
    }

    /// Called by the platform to handle a move event.
    pub fn submit_move(&mut self, view_x: f32, view_y: f32, source: EventSource) {
        if view_x == self.x && view_y == self.y {
            return; // Ignore repeated duplicate move events
        }

        // Take a snapshot of the entire event bubbling chain
        let mut chain = Vec::new();
        let hit = Sprite::hit_test(System::root(), view_x, view_y);
        if let Some(hitsprite) = hit.clone() {
            let mut owner = hitsprite.component.owner;
            loop {
                match owner {
                    Some(item) => {
                        let entity = item.entity();
                        if let Some(sprite) = EntityManager::<Sprite>::get(&entity) {
                            chain.push(sprite);
                        }
                        owner = entity.parent;
                    }
                    None => break,
                }
            }
        }

        // Finally, emit the event up the chain
        self.prepare(view_x, view_y, hit, Some(source));
        for ref mut sprite in chain {
            sprite.on_pointer_move(self.shared_event.clone());
            if self.shared_event.stopped {
                return;
            }
        }
        self.move_.emit(self.shared_event.clone());
    }

    /// Called by the platform to handle an up event.
    pub fn submit_up(&mut self, view_x: f32, view_y: f32, source: EventSource) {
        if !self.is_down {
            return; // Ignore repeat up events
        }
        // Ensure a move event is sent first
        self.submit_move(view_x, view_y, source.clone());
        self.is_down = false;

        // Take a snapshot of the entire event bubbling chain
        let mut chain = Vec::new();
        let hit = Sprite::hit_test(System::root(), view_x, view_y);
        if let Some(hitsprite) = hit.clone() {
            let mut owner = hitsprite.component.owner;
            loop {
                match owner {
                    Some(item) => {
                        let entity = item.entity();
                        if let Some(sprite) = EntityManager::<Sprite>::get(&entity) {
                            chain.push(sprite);
                        }
                        owner = entity.parent;
                    }
                    None => break,
                }
            }
        }

        // Finally, emit the event up the chain
        self.prepare(view_x, view_y, hit, Some(source.clone()));
        for ref mut sprite in chain {
            sprite.on_pointer_up(self.shared_event.clone());
            if self.shared_event.stopped {
                return;
            }
        }
        self.up.emit(self.shared_event.clone());
    }

    fn prepare(&mut self, view_x: f32, view_y: f32, hit: Option<Sprite>, source: Option<EventSource>) {
        self.x = view_x;
        self.y = view_y;
        self.shared_event
            .init(self.shared_event.id + 1, view_x, view_y, hit, source);
    }
}

impl PointerSystem for BasicPointer {
    fn down(&self) -> &Signal1<PointerEvent> {
        &self.down
    }

    fn move_signal(&self) -> &Signal1<PointerEvent> {
        &self.move_
    }

    fn up_signal(&self) -> &Signal1<PointerEvent> {
        &self.up
    }

    fn is_supported(&self) -> bool {
        true
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn is_down(&self) -> bool {
        self.is_down
    }
}
