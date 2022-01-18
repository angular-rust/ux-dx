use vg::ErrorKind;

use crate::platform::gles::{core30::gl, enums::*};

const GLSL_VERSION: &str = "#version 100";

pub(crate) struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(src: &str, kind: u32) -> Result<Self, ErrorKind> {
        let id = gl::create_shader(kind);

        // Compile
        gl::shader_source(id, src.as_bytes());
        gl::compile_shader(id);

        // Validate

        // Validate
        let success = gl::get_shaderiv(id, GL_COMPILE_STATUS);
        if success == 0 {
            let len = gl::get_shaderiv(id, GL_INFO_LOG_LENGTH);

            let error = match gl::get_shader_info_log(id, len) {
                Some(message) => message,
                None => "".into(),
            };

            let name = match kind {
                GL_VERTEX_SHADER => "Vertex stage",
                GL_FRAGMENT_SHADER => "Fragment stage",
                _ => "Shader stage",
            };

            return Err(ErrorKind::ShaderCompileError(format!(
                "{}: {}",
                name, error
            )));
        }

        Ok(Shader { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::delete_shader(self.id);
    }
}

pub(crate) struct Program {
    id: u32,
}

impl Program {
    pub fn new(shaders: &[Shader], attrib_locations: &[&str]) -> Result<Self, ErrorKind> {
        let program = Self {
            id: gl::create_program(),
        };

        // Attach stages
        for shader in shaders {
            gl::attach_shader(program.id, shader.id());
        }

        for (i, loc) in attrib_locations.iter().enumerate() {
            gl::bind_attrib_location(program.id, i as u32, *loc);
        }

        gl::link_program(program.id);

        // Check for error
        let success = gl::get_programiv(program.id, GL_LINK_STATUS);

        if success == 0 {
            let len = gl::get_programiv(program.id, GL_INFO_LOG_LENGTH);

            let error = match gl::get_program_info_log(program.id, len) {
                Some(message) => message,
                None => "".into(),
            };

            return Err(ErrorKind::ShaderLinkError(error));
        }

        // Detach stages
        for shader in shaders {
            gl::detach_shader(program.id, shader.id());
        }

        Ok(program)
    }

    pub(crate) fn bind(&self) {
        gl::use_program(self.id);
    }

    pub(crate) fn unbind(&self) {
        gl::use_program(0);
    }

    fn uniform_location(&self, name: &str) -> Result<i32, ErrorKind> {
        Ok(gl::get_uniform_location(self.id, name))
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        gl::delete_program(self.id);
    }
}

pub struct MainProgram {
    program: Program,
    loc_viewsize: i32,
    loc_tex: i32,
    loc_glyphtex: i32,
    loc_frag: i32,
}

impl MainProgram {
    pub(crate) fn new(antialias: bool) -> Result<Self, ErrorKind> {
        let shader_defs = if antialias { "#define EDGE_AA 1" } else { "" };
        let vert_shader_src = format!(
            "{}\n{}\n{}",
            GLSL_VERSION,
            shader_defs,
            include_str!("main-vs.glsl")
        );
        let frag_shader_src = format!(
            "{}\n{}\n{}",
            GLSL_VERSION,
            shader_defs,
            include_str!("main-fs.glsl")
        );

        let vert_shader = Shader::new(&vert_shader_src, GL_VERTEX_SHADER)?;
        let frag_shader = Shader::new(&frag_shader_src, GL_FRAGMENT_SHADER)?;

        let program = Program::new(&[vert_shader, frag_shader], &["vertex", "tcoord"])?;

        let loc_viewsize = program.uniform_location("viewSize")?;
        let loc_tex = program.uniform_location("tex")?;
        let loc_glyphtex = program.uniform_location("glyphtex")?;
        let loc_frag = program.uniform_location("frag")?;

        Ok(Self {
            program,
            loc_viewsize,
            loc_tex,
            loc_glyphtex,
            loc_frag,
        })
    }

    pub(crate) fn set_tex(&self, tex: i32) {
        gl::uniform1i(self.loc_tex, tex);
    }

    pub(crate) fn set_glyphtex(&self, tex: i32) {
        gl::uniform1i(self.loc_glyphtex, tex);
    }

    pub(crate) fn set_view(&self, view: [f32; 2]) {
        gl::uniform2fv(self.loc_viewsize, &view);
    }

    pub(crate) fn set_config(&self, config: &[f32]) {
        gl::uniform4fv(self.loc_frag, config);
    }

    pub(crate) fn bind(&self) {
        self.program.bind();
    }

    pub(crate) fn unbind(&self) {
        self.program.unbind();
    }
}
