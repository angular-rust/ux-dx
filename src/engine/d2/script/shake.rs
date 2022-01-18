use std::{cell::RefCell, rc::Rc};

use crate::engine::d2::{animation::Jitter, display::Sprite, Entity, EntityManager};

use super::Action;

struct ShakeProps {
    elapsed: f32,
    strength_x: f32,
    strength_y: f32,
    duration: f32,
}
/// Shakes an entity's sprite by jittering its X and Y for a set duration.
pub struct Shake {
    props: RefCell<ShakeProps>,
}

impl Shake {
    pub fn new(actor: Entity, strength_x: f32, strength_y: f32, seconds: f32) -> Self {
        let sprite = EntityManager::<Sprite>::get(&actor).unwrap_or_default();

        let jitter_x = Rc::new(Jitter::new(sprite.x.get(), strength_x));
        let jitter_y = Rc::new(Jitter::new(sprite.y.get(), strength_y));

        sprite.x.set_behavior(Some(jitter_x));
        sprite.y.set_behavior(Some(jitter_y));

        Self {
            props: RefCell::new(ShakeProps {
                strength_x,
                strength_y,
                duration: seconds,
                elapsed: 0.0,
            }),
        }
    }
}

impl Action for Shake {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(actor) {
            let mut props = self.props.borrow_mut();
            props.elapsed += dt;
            if props.elapsed >= props.duration {
                let overtime = props.elapsed - props.duration;

                sprite.x.update(dt); // Fake an update to account for this frame
                sprite.y.update(dt); // Fake an update to account for this frame

                props.elapsed = 0.0;
                return dt - overtime;
            }
        }

        -1.0
    }
}
