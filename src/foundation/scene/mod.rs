//! Scene management - WIP

pub mod light;
use self::light::*;

pub mod model;
use self::model::*;

#[derive(Default)]
pub struct Scene {
    pub lights: LightManager,
    pub models: Vec<Model>,
}

impl Scene {
    // user can create as much scenes as it possible
    // becouse scene also is actor and can have transitions in director
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, model: Model) {
        self.models.push(model)
    }
}
