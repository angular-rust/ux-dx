use crate::ext::{engine::Engine, error::GameResult, event::Event};

pub trait Game {
    fn update(&mut self, engine: &mut Engine) -> GameResult;

    fn render(&mut self, engine: &mut Engine) -> GameResult;

    fn event(&mut self, _engine: &mut Engine, _event: Event) -> GameResult<bool> {
        Ok(false)
    }
}
