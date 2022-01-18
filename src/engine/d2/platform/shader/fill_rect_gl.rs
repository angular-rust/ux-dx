use crate::{
    engine::d2::platform::shader::ShaderGL,
    platform::gles::{core30::*, enums::*},
};

#[derive(Default, Clone, Copy, Debug)]
pub struct FillRectGL {
    pub inner: ShaderGL,
    a_pos: i32,
    a_rgb: i32,
    a_alpha: i32,
}

impl FillRectGL {
    pub fn new() -> Self {
        let mut instance = Self {
            inner: ShaderGL::new(
                // Vertex shader
                r#"attribute highp vec2 a_pos;
                attribute lowp vec3 a_rgb;
                attribute lowp float a_alpha;
                varying lowp vec4 v_color;
                void main (void) {
                    v_color = vec4(a_rgb*a_alpha, a_alpha);
                    gl_Position = vec4(a_pos, 0, 1);
                }"#,
                // Fragment shader
                r#"varying lowp vec4 v_color;
                void main (void) {
                    gl_FragColor = v_color;
                }"#,
            ),
            ..Default::default()
        };

        instance.a_pos = instance.inner.attrib_location("a_pos");
        instance.a_rgb = instance.inner.attrib_location("a_rgb");
        instance.a_alpha = instance.inner.attrib_location("a_alpha");

        instance
    }

    // override
    pub fn prepare(&self) {
        gl::enable_vertex_attrib_array(self.a_pos as u32);
        gl::enable_vertex_attrib_array(self.a_rgb as u32);
        gl::enable_vertex_attrib_array(self.a_alpha as u32);

        const BYTES_PER_ELEMENT: u32 = 4;
        let stride = 6 * BYTES_PER_ELEMENT as i32;
        gl::vertex_attrib_pointer_offset(
            self.a_pos as u32,
            2,
            GL_FLOAT,
            false,
            stride,
            0 * BYTES_PER_ELEMENT,
        );
        gl::vertex_attrib_pointer_offset(
            self.a_rgb as u32,
            3,
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
            5 * BYTES_PER_ELEMENT,
        );
    }
}

impl AsRef<ShaderGL> for FillRectGL {
    fn as_ref(&self) -> &ShaderGL {
        &self.inner
    }
}
