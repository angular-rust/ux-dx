use super::opengl;
use crate::ext::{
    engine::Engine,
    error::{GameError, GameResult},
};
use std::{path::Path, rc::Rc};

const DEFAULT_VERTEX_SHADER_SOURCE: &str = include_str!("shaders/default.vert");
const DEFAULT_FRAGMENT_SHADER_SOURCE: &str = include_str!("shaders/default.frag");

pub struct Program {
    program: Rc<opengl::Program>,
}

impl Program {
    pub fn new(
        vertex_shader_source: impl AsRef<str>,
        fragment_shader_source: impl AsRef<str>,
    ) -> GameResult<Self> {
        let program = opengl::Program::new(
            vertex_shader_source.as_ref(),
            fragment_shader_source.as_ref(),
        )
        .map_err(|error| GameError::InitError(error.into()))?;
        Ok(Self {
            program: Rc::new(program),
        })
    }

    pub fn load(
        engine: &mut Engine,
        vertex_shader_path: impl AsRef<Path>,
        fragment_shader_path: impl AsRef<Path>,
    ) -> GameResult<Self> {
        let vertex_shader_source = engine.filesystem().read_to_string(vertex_shader_path)?;
        let fragment_shader_source = engine.filesystem().read_to_string(fragment_shader_path)?;
        Self::new(&vertex_shader_source, &fragment_shader_source)
    }

    pub(crate) fn default() -> GameResult<Rc<opengl::Program>> {
        let program = super::opengl::Program::new(
            DEFAULT_VERTEX_SHADER_SOURCE,
            DEFAULT_FRAGMENT_SHADER_SOURCE,
        )
        .map_err(|error| GameError::InitError(error.into()))?;
        Ok(Rc::new(program))
    }

    pub(crate) fn program(&self) -> &Rc<opengl::Program> {
        &self.program
    }
}
