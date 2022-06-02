use crate::engine::d2::{display::TextSprite, Component};

/// A component that uses its entity's TextSprite to display an FPS log.
pub struct FpsDisplay {
    pub inner: Component,
    fps_frames: usize,
    fps_time: f32,
}

impl FpsDisplay {
    pub fn new() -> Self {
        Self {
            inner: Component::default(),
            fps_frames: 0,
            fps_time: 0.0,
        }
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        self.fps_frames += 1;
        self.fps_time += dt;
        if self.fps_time > 1.0 {
            let fps = self.fps_frames as f32 / self.fps_time;
            let text = format!("FPS: {}", (fps * 100.0) as i32 / 100);

            todo!("should deal with it");
            // Use our owner's TextSprite if available, otherwise just log it
            // let sprite = EntityManager::<TextSprite>::get(&self.inner.owner);
            // if sprite.is_some() {
            //     sprite.set_text(text);
            // } else {
            //     log::info!("{}", text);
            // }

            // self.reset();
        }
    }

    fn reset(&mut self) {
        self.fps_time = 0.0;
        self.fps_frames = 0;
    }
}

impl AsRef<Component> for FpsDisplay {
    fn as_ref(&self) -> &Component {
        &self.inner
    }
}
