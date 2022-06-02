use crate::{
    engine::d2::util::StringExtensions,
    platform::gles::{core30::*, enums::*},
};

#[derive(Default, Clone, Copy, Debug)]
pub struct ShaderGL {
    program: u32, // Program,
}

impl ShaderGL {
    pub fn new(vert_source: &str, frag_source: &str) -> Self {
        // Prepend the required precision rigamarole
        let header = ["#ifdef GL_ES", "precision mediump float;", "#endif"].join("\n");
        let frag_source = format!("{}\n{}", header, frag_source);

        let program = gl::create_program();
        gl::attach_shader(
            program,
            Self::create_shader(GL_VERTEX_SHADER, vert_source.as_bytes()),
        );
        gl::attach_shader(
            program,
            Self::create_shader(GL_FRAGMENT_SHADER, frag_source.as_bytes()),
        );
        gl::link_program(program);
        gl::use_program(program);

        #[cfg(debug_assertions)]
        {
            let success = gl::get_programiv(program, GL_LINK_STATUS);

            if success == 0 {
                let len = gl::get_programiv(program, GL_INFO_LOG_LENGTH);

                match gl::get_program_info_log(program, len) {
                    Some(message) => log::error!("PROGRAM LINK ERROR: \n{}", message),
                    None => log::error!("PROGRAM LINK ERROR: without info logs"),
                }
            }
        }

        Self { program }
    }

    pub fn use_program(&self) {
        gl::use_program(self.program);
    }

    pub fn prepare(&self) {
        panic!("abstract");
    }

    pub fn attrib_location<S: Into<String>>(&self, name: S) -> i32 {
        let name: String = name.into();
        let loc = gl::get_attrib_location(self.program, name.as_str());
        assert!(loc >= 0, "Missing attribute [name: {}]", name);
        loc
    }

    // UniformLocation
    pub fn uniform_location<S: Into<String>>(&self, name: S) -> i32 {
        let name: String = name.into();
        let loc = gl::get_uniform_location(self.program, name.as_str());
        assert!(loc >= 0, "Missing uniform [name: {}]", name);
        loc
    }

    // static Shader
    pub fn create_shader(shader_type: u32, source: &[u8]) -> u32 {
        let shader = gl::create_shader(shader_type);
        gl::shader_source(shader, source);
        gl::compile_shader(shader);

        #[cfg(debug_assertions)]
        {
            let success = gl::get_shaderiv(shader, GL_COMPILE_STATUS);

            if success == 0 {
                let len = gl::get_shaderiv(shader, GL_INFO_LOG_LENGTH);
                let type_name = if shader_type == GL_VERTEX_SHADER {
                    "vertex"
                } else {
                    "fragment"
                };

                match gl::get_shader_info_log(shader, len) {
                    Some(message) => {
                        log::error!("Error compiling {} shader\n{}", type_name, message)
                    }
                    None => log::error!("Error compiling {} shader", type_name),
                };
            }
        }

        shader
    }
}
