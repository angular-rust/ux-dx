use iced_graphics::layer;
use iced_native::Rectangle;

use crate::platform::gles::{core30::gl, enums::*};

use super::{program, Transformation};

const MAX_INSTANCES: usize = 100_000;

#[derive(Debug)]
pub struct Pipeline {
    program: u32,
    vertex_array: u32,
    instances: u32,
    transform_location: i32,
    scale_location: i32,
    screen_height_location: i32,
    current_transform: Transformation,
    current_scale: f32,
    current_target_height: u32,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        let program = program::create(&[
            (GL_VERTEX_SHADER, include_str!("shader/quad.vert")),
            (GL_FRAGMENT_SHADER, include_str!("shader/quad.frag")),
        ]);

        let transform_location = gl::get_uniform_location(program, "u_Transform");

        let scale_location = gl::get_uniform_location(program, "u_Scale");

        let screen_height_location = gl::get_uniform_location(program, "u_ScreenHeight");

        gl::use_program(program);

        let matrix: [f32; 16] = Transformation::identity().into();
        gl::uniform_matrix4fv(transform_location, false, &matrix);

        gl::uniform1f(scale_location, 1.0);
        gl::uniform1f(screen_height_location, 0.0);

        gl::use_program(0);

        let (vertex_array, instances) = create_instance_buffer(MAX_INSTANCES);

        Pipeline {
            program,
            vertex_array,
            instances,
            transform_location,
            scale_location,
            screen_height_location,
            current_transform: Transformation::identity(),
            current_scale: 1.0,
            current_target_height: 0,
        }
    }

    pub fn draw(
        &mut self,
        target_height: u32,
        instances: &[layer::Quad],
        transformation: Transformation,
        scale: f32,
        bounds: Rectangle<u32>,
    ) {
        gl::enable(GL_SCISSOR_TEST);
        gl::scissor(
            bounds.x as i32,
            (target_height - (bounds.y + bounds.height)) as i32,
            bounds.width as i32,
            bounds.height as i32,
        );

        gl::use_program(self.program);
        gl::bind_vertex_array(self.vertex_array);
        gl::bind_buffer(GL_ARRAY_BUFFER, self.instances);

        if transformation != self.current_transform {
            let matrix: [f32; 16] = transformation.into();
            gl::uniform_matrix4fv(self.transform_location, false, &matrix);

            self.current_transform = transformation;
        }

        if scale != self.current_scale {
            gl::uniform1f(self.scale_location, scale);

            self.current_scale = scale;
        }

        if target_height != self.current_target_height {
            gl::uniform1f(self.screen_height_location, target_height as f32);

            self.current_target_height = target_height;
        }

        let mut i = 0;
        let total = instances.len();

        while i < total {
            let end = (i + MAX_INSTANCES).min(total);
            let amount = end - i;

            gl::buffer_sub_data(
                GL_ARRAY_BUFFER,
                0,
                &instances[i..end], /*bytemuck::cast_slice(&instances[i..end])*/
            );

            gl::draw_arrays_instanced(GL_TRIANGLE_STRIP, 0, 4, amount as i32);

            i += MAX_INSTANCES;
        }

        gl::bind_vertex_array(0);
        gl::use_program(0);
        gl::disable(GL_SCISSOR_TEST);
    }
}

fn create_instance_buffer(size: usize) -> (u32, u32) {
    let vertex_array = gl::gen_vertex_array();
    let buffer = gl::gen_buffer();

    gl::bind_vertex_array(vertex_array);
    gl::bind_buffer(GL_ARRAY_BUFFER, buffer);
    gl::buffer_data_size(
        GL_ARRAY_BUFFER,
        size * std::mem::size_of::<layer::Quad>(),
        GL_DYNAMIC_DRAW,
    );

    let stride = std::mem::size_of::<layer::Quad>() as i32;

    gl::enable_vertex_attrib_array(0);
    gl::vertex_attrib_pointer_offset(0, 2, GL_FLOAT, false, stride, 0);
    gl::vertex_attrib_divisor(0, 1);

    gl::enable_vertex_attrib_array(1);
    gl::vertex_attrib_pointer_offset(1, 2, GL_FLOAT, false, stride, 4 * 2);
    gl::vertex_attrib_divisor(1, 1);

    gl::enable_vertex_attrib_array(2);
    gl::vertex_attrib_pointer_offset(2, 4, GL_FLOAT, false, stride, 4 * (2 + 2));
    gl::vertex_attrib_divisor(2, 1);

    gl::enable_vertex_attrib_array(3);
    gl::vertex_attrib_pointer_offset(3, 4, GL_FLOAT, false, stride, 4 * (2 + 2 + 4));
    gl::vertex_attrib_divisor(3, 1);

    gl::enable_vertex_attrib_array(4);
    gl::vertex_attrib_pointer_offset(4, 1, GL_FLOAT, false, stride, 4 * (2 + 2 + 4 + 4));
    gl::vertex_attrib_divisor(4, 1);

    gl::enable_vertex_attrib_array(5);
    gl::vertex_attrib_pointer_offset(5, 1, GL_FLOAT, false, stride, 4 * (2 + 2 + 4 + 4 + 1));
    gl::vertex_attrib_divisor(5, 1);

    gl::bind_vertex_array(0);
    gl::bind_buffer(GL_ARRAY_BUFFER, 0);

    (vertex_array, buffer)
}
