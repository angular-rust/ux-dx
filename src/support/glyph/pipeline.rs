use crate::platform::gles::{core30::gl, enums::*};

use super::{
    ab_glyph::{point, Rect},
    Region,
};

mod cache;

use cache::Cache;

pub struct Pipeline {
    program: u32,
    vertex_array: u32,
    instances: u32,
    transform: i32,
    cache: Cache,
    current_instances: usize,
    supported_instances: usize,
    current_transform: [f32; 16],
}

impl Pipeline {
    pub fn new(cache_width: u32, cache_height: u32) -> Pipeline {
        let cache = Cache::new(cache_width, cache_height);

        let program = create_program(&[
            (GL_VERTEX_SHADER, include_str!("./shader/vertex.vert")),
            (GL_FRAGMENT_SHADER, include_str!("./shader/fragment.frag")),
        ]);

        let (vertex_array, instances) = create_instance_buffer(Instance::INITIAL_AMOUNT);

        let transform = gl::get_uniform_location(program, "transform");

        let sampler = gl::get_uniform_location(program, "font_sampler");

        gl::use_program(program);

        gl::uniform1i(sampler, 0);

        gl::uniform_matrix4fv(transform, false, &IDENTITY_MATRIX);

        gl::use_program(0);

        Pipeline {
            program,
            cache,
            vertex_array,
            instances,
            transform,
            current_instances: 0,
            supported_instances: Instance::INITIAL_AMOUNT,
            current_transform: IDENTITY_MATRIX,
        }
    }

    pub fn draw(&mut self, transform: [f32; 16], region: Option<Region>) {
        gl::use_program(self.program);

        if self.current_transform != transform {
            gl::uniform_matrix4fv(self.transform, false, &transform);

            self.current_transform = transform;
        }

        if let Some(region) = region {
            gl::enable(GL_SCISSOR_TEST);
            gl::scissor(
                region.x as i32,
                region.y as i32,
                region.width as i32,
                region.height as i32,
            );
        }

        gl::active_texture(GL_TEXTURE0);
        gl::bind_texture(GL_TEXTURE_2D, self.cache.texture);

        gl::bind_vertex_array(self.vertex_array);

        gl::draw_arrays_instanced(GL_TRIANGLE_STRIP, 0, 4, self.current_instances as i32);

        gl::bind_vertex_array(0);
        gl::bind_texture(GL_TEXTURE_2D, 0);
        gl::disable(GL_SCISSOR_TEST);
        gl::use_program(0);
    }

    pub fn update_cache(&mut self, offset: [u16; 2], size: [u16; 2], data: &[u8]) {
        self.cache.update(offset, size, data);
    }

    pub fn increase_cache_size(&mut self, width: u32, height: u32) {
        self.cache.destroy();

        self.cache = Cache::new(width, height);
    }

    pub fn upload(&mut self, instances: &[Instance]) {
        if instances.is_empty() {
            self.current_instances = 0;
            return;
        }

        if instances.len() > self.supported_instances {
            gl::delete_buffers(&[self.instances]);
            gl::delete_vertex_arrays(&[self.vertex_array]);

            let (new_vertex_array, new_instances) = create_instance_buffer(instances.len());

            self.vertex_array = new_vertex_array;
            self.instances = new_instances;
            self.supported_instances = instances.len();
        }

        gl::bind_buffer(GL_ARRAY_BUFFER, self.instances);
        gl::buffer_sub_data(
            GL_ARRAY_BUFFER,
            0,
            instances, /*bytemuck::cast_slice(instances)*/
        );
        gl::bind_buffer(GL_ARRAY_BUFFER, 0);

        self.current_instances = instances.len();
    }
}

// Helpers
#[cfg_attr(rustfmt, rustfmt_skip)]
const IDENTITY_MATRIX: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Instance {
    left_top: [f32; 3],
    right_bottom: [f32; 2],
    tex_left_top: [f32; 2],
    tex_right_bottom: [f32; 2],
    color: [f32; 4],
}

unsafe impl bytemuck::Zeroable for Instance {}
unsafe impl bytemuck::Pod for Instance {}

impl Instance {
    const INITIAL_AMOUNT: usize = 50_000;

    pub fn from_vertex(
        glyph_brush::GlyphVertex {
            mut tex_coords,
            pixel_coords,
            bounds,
            extra,
        }: glyph_brush::GlyphVertex,
    ) -> Instance {
        let gl_bounds = bounds;

        let mut gl_rect = Rect {
            min: point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
            max: point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
        };

        // handle overlapping bounds, modify uv_rect to preserve texture aspect
        if gl_rect.max.x > gl_bounds.max.x {
            let old_width = gl_rect.width();
            gl_rect.max.x = gl_bounds.max.x;
            tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
        }

        if gl_rect.min.x < gl_bounds.min.x {
            let old_width = gl_rect.width();
            gl_rect.min.x = gl_bounds.min.x;
            tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
        }

        if gl_rect.max.y > gl_bounds.max.y {
            let old_height = gl_rect.height();
            gl_rect.max.y = gl_bounds.max.y;
            tex_coords.max.y =
                tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
        }

        if gl_rect.min.y < gl_bounds.min.y {
            let old_height = gl_rect.height();
            gl_rect.min.y = gl_bounds.min.y;
            tex_coords.min.y =
                tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
        }

        Instance {
            left_top: [gl_rect.min.x, gl_rect.max.y, extra.z],
            right_bottom: [gl_rect.max.x, gl_rect.min.y],
            tex_left_top: [tex_coords.min.x, tex_coords.max.y],
            tex_right_bottom: [tex_coords.max.x, tex_coords.min.y],
            color: extra.color,
        }
    }
}

fn create_program(shader_sources: &[(u32, &str)]) -> u32 {
    let program = gl::create_program();

    let mut shaders = Vec::with_capacity(shader_sources.len());

    for (shader_type, shader_source) in shader_sources.iter() {
        let shader = gl::create_shader(*shader_type);

        gl::shader_source(shader, shader_source.as_bytes());
        gl::compile_shader(shader);

        if gl::get_shaderiv(shader, GL_COMPILE_STATUS) == 0 {
            let len = gl::get_shaderiv(shader, GL_INFO_LOG_LENGTH);

            match gl::get_shader_info_log(shader, len) {
                Some(message) => {
                    panic!("failed to compile shader: {}: {}", *shader_source, message)
                }
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

fn create_instance_buffer(size: usize) -> (u32, u32) {
    let vertex_array = gl::gen_vertex_array();
    let buffer = gl::gen_buffer();

    gl::bind_vertex_array(vertex_array);
    gl::bind_buffer(GL_ARRAY_BUFFER, buffer);
    gl::buffer_data_size(
        GL_ARRAY_BUFFER,
        size * std::mem::size_of::<Instance>(),
        GL_DYNAMIC_DRAW,
    );

    let stride = std::mem::size_of::<Instance>() as i32;

    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer_offset(0, 3, GL_FLOAT, false, stride, 0);
    gl::vertex_attrib_divisor(0, 1);

    gl::enable_vertex_attrib_array(1);
    gl::vertex_attrib_pointer_offset(1, 2, GL_FLOAT, false, stride, 4 * 3);
    gl::vertex_attrib_divisor(1, 1);

    gl::enable_vertex_attrib_array(2);
    gl::vertex_attrib_pointer_offset(2, 2, GL_FLOAT, false, stride, 4 * (3 + 2));
    gl::vertex_attrib_divisor(2, 1);

    gl::enable_vertex_attrib_array(3);
    gl::vertex_attrib_pointer_offset(3, 2, GL_FLOAT, false, stride, 4 * (3 + 2 + 2));
    gl::vertex_attrib_divisor(3, 1);

    gl::enable_vertex_attrib_array(4);
    gl::vertex_attrib_pointer_offset(4, 4, GL_FLOAT, false, stride, 4 * (3 + 2 + 2 + 2));
    gl::vertex_attrib_divisor(4, 1);

    gl::bind_vertex_array(0);
    gl::bind_buffer(GL_ARRAY_BUFFER, 0);

    (vertex_array, buffer)
}
