use crate::platform::gles::{core30::gl, enums::*};

pub fn create(shader_sources: &[(u32, &str)]) -> u32 {
    let program = gl::create_program();

    let mut shaders = Vec::with_capacity(shader_sources.len());

    for (shader_type, shader_source) in shader_sources.iter() {
        let shader = gl::create_shader(*shader_type);

        gl::shader_source(shader, shader_source.as_bytes());
        gl::compile_shader(shader);

        if gl::get_shaderiv(shader, GL_COMPILE_STATUS) == 0 {
            let len = gl::get_shaderiv(shader, GL_INFO_LOG_LENGTH);

            match gl::get_shader_info_log(shader, len) {
                Some(message) => panic!("failed to compile shader: {}: {}", *shader_source, message),
                None => panic!("failed to compile shader: {}", *shader_source),
            }
        }

        gl::attach_shader(program, shader);

        shaders.push(shader);
    }

    gl::link_program(program);
    if gl::get_programiv(program, GL_LINK_STATUS) == 0 {
        let len = gl::get_programiv(program, GL_INFO_LOG_LENGTH);

        match gl::get_program_info_log(program, len) {
            Some(message) => panic!("failed to link program: {}", message),
            None => panic!("failed to link program"),
        }
    }

    for shader in shaders {
        gl::detach_shader(program, shader);
        gl::delete_shader(shader);
    }

    program
}
