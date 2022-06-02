use crate::platform::gles::{core30::*, enums::*};

use super::ShaderGL;

#[derive(Default, Clone, Copy, Debug)]
pub struct DrawTextureGL {
    pub inner: ShaderGL,
    a_pos: i32,
    a_uv: i32,
    a_alpha: i32,

    u_texture: i32, //UniformLocation,
}

impl DrawTextureGL {
    pub fn new() -> Self {
        let mut instance = Self {
            inner: ShaderGL::new(
                // Vertex shader
                r#"attribute highp vec2 a_pos;
                attribute mediump vec2 a_uv;
                attribute lowp float a_alpha;
                varying mediump vec2 v_uv;
                varying lowp float v_alpha;
                void main (void) {
                    v_uv = a_uv;
                    v_alpha = a_alpha;
                    gl_Position = vec4(a_pos, 0, 1);
                }"#,
                // Fragment shader
                r#"varying mediump vec2 v_uv;
                varying lowp float v_alpha;
                uniform lowp sampler2D u_texture;
                void main (void) {
                    gl_FragColor = texture2D(u_texture, v_uv) * v_alpha;
                }"#,
            ),
            ..Default::default()
        };

        instance.a_pos = instance.inner.attrib_location("a_pos");
        instance.a_uv = instance.inner.attrib_location("a_uv");
        instance.a_alpha = instance.inner.attrib_location("a_alpha");

        instance.u_texture = instance.inner.uniform_location("u_texture");
        instance.set_texture(0);

        instance
    }

    pub fn set_texture(&self, unit: i32) {
        gl::uniform1i(self.u_texture, unit);
    }

    // override
    pub fn prepare(&self) {
        gl::enable_vertex_attrib_array(self.a_pos as u32);
        gl::enable_vertex_attrib_array(self.a_uv as u32);
        gl::enable_vertex_attrib_array(self.a_alpha as u32);

        const BYTES_PER_ELEMENT: u32 = 4;
        let stride = 5 * BYTES_PER_ELEMENT as i32;
        gl::vertex_attrib_pointer_offset(
            self.a_pos as u32,
            2,
            GL_FLOAT,
            false,
            stride,
            0,
        );
        gl::vertex_attrib_pointer_offset(
            self.a_uv as u32,
            2,
            GL_FLOAT,
            false,
            stride,
            2 * BYTES_PER_ELEMENT,
        );
        gl::vertex_attrib_pointer_offset(
            self.a_alpha as u32,
            1,
            GL_FLOAT,
            false,
            stride,
            4 * BYTES_PER_ELEMENT,
        );
    }
}

impl AsRef<ShaderGL> for DrawTextureGL {
    fn as_ref(&self) -> &ShaderGL {
        &self.inner
    }
}
