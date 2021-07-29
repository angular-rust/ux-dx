use crate::gles::{core30::gl, enums::*};
// use std::rc::Rc;

// pub type ProgramId = <Context as HasContext>::Program;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Self, String> {
        let id = {
            let vertex_shader_id = gl::create_shader(GL_VERTEX_SHADER);
            gl::shader_source(vertex_shader_id, vertex_shader_source.as_bytes());
            gl::compile_shader(vertex_shader_id);

            if gl::get_shaderiv(vertex_shader_id, GL_COMPILE_STATUS) == 0 {
                let len = gl::get_shaderiv(vertex_shader_id, GL_INFO_LOG_LENGTH);
                return match gl::get_shader_info_log(vertex_shader_id, len) {
                    Some(message) => Err(message),
                    None => Err("Error compile vertex shader".into()),
                };
            }

            let fragment_shader_id = gl::create_shader(GL_FRAGMENT_SHADER);
            gl::shader_source(fragment_shader_id, fragment_shader_source.as_bytes());
            gl::compile_shader(fragment_shader_id);

            if gl::get_shaderiv(fragment_shader_id, GL_COMPILE_STATUS) == 0 {
                let len = gl::get_shaderiv(fragment_shader_id, GL_INFO_LOG_LENGTH);
                return match gl::get_shader_info_log(fragment_shader_id, len) {
                    Some(message) => Err(message),
                    None => Err("Error compile fragment shader".into()),
                };
            }

            let program_id = gl::create_program();

            gl::attach_shader(program_id, vertex_shader_id);
            gl::attach_shader(program_id, fragment_shader_id);

            gl::link_program(program_id);
            if gl::get_programiv(program_id, GL_LINK_STATUS) == 0 {
                let len = gl::get_programiv(program_id, GL_INFO_LOG_LENGTH);
                return match gl::get_program_info_log(program_id, len) {
                    Some(message) => Err(message),
                    None => Err("Error link program".into()),
                };
            }

            gl::delete_shader(vertex_shader_id);
            gl::delete_shader(fragment_shader_id);

            program_id
        };
        Ok(Self { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        gl::use_program(self.id);
    }

    pub fn unbind(&self) {
        gl::use_program(0);
    }

    pub fn set_uniform_matrix_4(&self, name: &str, mat4: &[f32; 16]) {
        let location = gl::get_uniform_location(self.id, name);
        gl::uniform_matrix4fv(location, false, mat4);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        gl::delete_program(self.id);
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
