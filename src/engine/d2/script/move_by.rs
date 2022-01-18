use std::rc::Rc;

use crate::engine::d2::{
    animation::{Ease, EaseFunction, Tween},
    display::Sprite,
    Entity, EntityManager,
};

use super::Action;

/// An action that translates the owner's sprite by a certain amount.
pub struct MoveBy {
    tween_x: Rc<Tween>,
    tween_y: Rc<Tween>,

    x: f32,
    y: f32,
    seconds: f32,
}

impl MoveBy {
    pub fn new(
        actor: Entity,
        x: f32,
        y: f32,
        seconds: f32,
        easing_x: Option<EaseFunction>,
        easing_y: Option<EaseFunction>,
    ) -> Self {
        let sprite = EntityManager::<Sprite>::get(&actor).unwrap_or_default();

        let easing_x = easing_x.unwrap_or(Rc::new(Ease::linear));
        // let easing_y = easing_y.unwrap_or(easing_x); // TODO: DV this should be correct
        let easing_y = easing_y.unwrap_or(Rc::new(Ease::linear));

        let tween_x = Rc::new(Tween::new(sprite.x.get(), sprite.x.get() + x, seconds, Some(easing_x)));
        sprite.x.set_behavior(Some(tween_x.clone()));

        let tween_y = Rc::new(Tween::new(sprite.y.get(), sprite.y.get() + y, seconds, Some(easing_y)));
        sprite.y.set_behavior(Some(tween_y.clone()));

        Self {
            x,
            y,
            seconds,
            // easing_x,
            // easing_y,
            tween_x,
            tween_y,
        }
    }
}

impl Action for MoveBy {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(actor) {
            sprite.x.update(dt); // Fake an update to account for this frame
            sprite.y.update(dt); // Fake an update to account for this frame
            let overtime = self.tween_x.elapsed().max(self.tween_y.elapsed()) - self.seconds;
            if overtime > 0.0 {
                0_f32.max(dt - overtime)
            } else {
                0.0
            }
        } else {
            -1.0
        }
    }
}
