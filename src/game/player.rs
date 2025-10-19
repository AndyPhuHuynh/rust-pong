use std::collections::HashSet;

use winit::keyboard::KeyCode;

use crate::graphics::{self, traits::Draw};

#[derive(Default)]
pub struct Player {
    pub program: u32,
    pub vao: u32,

    offset: (f32, f32),
    half_width: f32,
    half_height: f32,

    pub pos_location: u32,
    pub offset_location: u32
}

pub fn bind_player() -> Player {
    let mut player = Player::default();
    player.half_width = 0.25;
    player.half_height = 0.25;

    let vertices: [f32; 8] = [
         player.half_width,  player.half_height, // top right
         player.half_width, -player.half_height, // bottom right
        -player.half_width,  player.half_height, // top left
        -player.half_width, -player.half_height  // bottom left
    ]; 
    let stride = 2 * std::mem::size_of_val(&vertices[0]) as i32;

    let indices: [u32; 6] = [
        0, 1, 3,
        0, 2, 3
    ];

    unsafe {
        let vs = graphics::shaders::compile_shader(graphics::src::VS, gl::VERTEX_SHADER).expect("Unable to compile vertex shader");
        let fs = graphics::shaders::compile_shader(graphics::src::FS, gl::FRAGMENT_SHADER).expect("Unable to compile fragment shader");
        player.program = graphics::shaders::link_program(vs, fs).expect("Unable to link program");
        gl::UseProgram(player.program);
        player.pos_location = graphics::shaders::get_attrib_location(player.program, graphics::src::VS_ATTR_POS).unwrap();
        player.offset_location = graphics::shaders::get_uniform_location(player.program, graphics::src::UNIFORM_OFFSET).unwrap();
        player.offset = (0.25, 0.25);

        gl::GenVertexArrays(1, &mut player.vao);
        gl::BindVertexArray(player.vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as isize,
            indices.as_ptr() as * const _,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(player.pos_location, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(player.pos_location);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
    player
}

impl Draw for Player {
    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::Uniform2f(self.offset_location as i32, self.offset.0, self.offset.1);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

const Y_VELOCITY: f32 = 0.01;

impl Player {
    fn move_up(&mut self) {
        self.offset.1 = (self.offset.1 + Y_VELOCITY).min(1.0 - self.half_height);
    }

    fn move_down(&mut self) {
        self.offset.1 = (self.offset.1 - Y_VELOCITY).max(-1.0 + self.half_height);
    }

    pub fn update(&mut self, keys_pressed: &HashSet<KeyCode>) {
        if keys_pressed.contains(&KeyCode::ArrowUp) {
            self.move_up()
        }
        if keys_pressed.contains(&KeyCode::ArrowDown) {
            self.move_down();
        }
    }
}