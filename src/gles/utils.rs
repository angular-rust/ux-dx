use super::{core20::gl, enums::*, GLenum};
use std::{fs::File, io::Read, path::PathBuf, str};

pub fn shader_from_source(source: &str, kind: GLenum) -> Result<u32, String> {
    let id = gl::create_shader(kind);

    gl::shader_source(id, source.as_bytes());
    gl::compile_shader(id);

    let success = gl::get_shaderiv(id, GL_COMPILE_STATUS);

    if success == 0 {
        let len = gl::get_shaderiv(id, GL_INFO_LOG_LENGTH);

        return match gl::get_shader_info_log(id, len) {
            Some(message) => Err(message),
            None => Ok(id),
        };
    }

    Ok(id)
}

pub fn shader_from_file<T>(path: T, kind: GLenum) -> Result<u32, String>
where
    T: Into<PathBuf>,
{
    let path: PathBuf = path.into();
    let mut shader_file = File::open(path.as_path())
        .unwrap_or_else(|_| panic!("Failed to open {:?}", path.as_path()));

    let mut shader_source = String::new();

    shader_file
        .read_to_string(&mut shader_source)
        .expect("Failed to read shader");

    shader_from_source(shader_source.as_str(), kind)
}

pub fn program_from_shaders(vert_shader: u32, frag_shader: u32) -> Result<u32, String> {
    let program_id = gl::create_program();

    gl::attach_shader(program_id, frag_shader);
    gl::attach_shader(program_id, vert_shader);

    // Bind "Position" attribute to attribute 0.
    // Only nedded to reassign layout semantics of shader attributes.
    // But Opengl do it automaticaly.
    //
    // gl::bind_attrib_location(program_id, 0, "Position");

    gl::link_program(program_id);

    // error handling here
    let success = gl::get_programiv(program_id, GL_LINK_STATUS);

    if success == 0 {
        let len = gl::get_programiv(program_id, GL_INFO_LOG_LENGTH);

        return match gl::get_program_info_log(program_id, len) {
            Some(message) => Err(message),
            None => Ok(program_id),
        };
    }

    gl::detach_shader(program_id, vert_shader);
    gl::detach_shader(program_id, frag_shader);

    Ok(program_id)
}
