use crate::graphics;

#[derive(Default)]
pub struct Shape {
    pub program: u32,
    pub vao: u32,

    pub start_location: (f32, f32),
    pub offset: (f32, f32),
    pub velocity: (f32, f32),
    pub half_width: f32,
    pub half_height: f32,

    pub pos_location: u32,
    pub offset_location: u32
}

pub fn bind_shape(start_x: f32, start_y: f32, half_width: f32, half_height: f32) -> Shape {
    let mut shape = Shape::default();
    shape.start_location = (start_x, start_y);
    shape.half_width = half_width;
    shape.half_height = half_height;

    let vertices: [f32; 8] = [
        (shape.start_location.0 + shape.half_width), (shape.start_location.1 + shape.half_height), // top right
        (shape.start_location.0 + shape.half_width), (shape.start_location.1 - shape.half_height), // bottom right
        (shape.start_location.0 - shape.half_width), (shape.start_location.1 + shape.half_height), // top left
        (shape.start_location.0 - shape.half_width), (shape.start_location.1 - shape.half_height)  // bottom left
    ]; 
    let stride = 2 * std::mem::size_of_val(&vertices[0]) as i32;

    let indices: [u32; 6] = [
        0, 1, 3,
        0, 2, 3
    ];

    unsafe {
        let vs = graphics::shaders::compile_shader(graphics::src::VS, gl::VERTEX_SHADER).expect("Unable to compile vertex shader");
        let fs = graphics::shaders::compile_shader(graphics::src::FS, gl::FRAGMENT_SHADER).expect("Unable to compile fragment shader");
        shape.program = graphics::shaders::link_program(vs, fs).expect("Unable to link program");
        gl::UseProgram(shape.program);
        shape.pos_location = graphics::shaders::get_attrib_location(shape.program, graphics::src::VS_ATTR_POS).unwrap();
        shape.offset_location = graphics::shaders::get_uniform_location(shape.program, graphics::src::UNIFORM_OFFSET).unwrap();

        gl::GenVertexArrays(1, &mut shape.vao);
        gl::BindVertexArray(shape.vao);

        graphics::shaders::generate_and_bind_vbo(&vertices);
        graphics::shaders::generate_and_bind_ebo(&indices);

        gl::VertexAttribPointer(shape.pos_location, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(shape.pos_location);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
    shape
}

impl Shape {
    pub fn center_x(&self) -> f32 {
        self.start_location.0 + self.offset.0
    }

    pub fn center_y(&self) -> f32 {
        self.start_location.1 + self.offset.1
    }

    pub fn left_edge(&self) -> f32 {
        self.center_x() - self.half_width
    } 

    pub fn right_edge(&self) -> f32 {
        self.center_x() + self.half_width
    } 
    
    pub fn top_edge(&self) -> f32 {
        self.center_y() + self.half_height
    }

    pub fn bottom_edge(&self) -> f32 {
        self.center_y() - self.half_height
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);

            gl::Uniform2f(self.offset_location as i32, self.offset.0, self.offset.1);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}

pub enum Collision {
    None,
    Left,
    Right,
    Top,
    Bottom
}

pub fn in_collision(shape1: &Shape, shape2: &Shape) -> Collision {
    let dx = shape1.center_x() - shape2.center_x();
    let dy = shape1.center_y() - shape2.center_y();
    let overlap_x = (shape1.half_width + shape2.half_width) - dx.abs();
    let overlap_y = (shape1.half_height + shape2.half_height) - dy.abs();    

    if overlap_x < 0.0 || overlap_y < 0.0 {
        return Collision::None;
    }
    if overlap_x < overlap_y {
        if dx > 0.0 { Collision::Left } else { Collision::Right }
    } else {
        if dy > 0.0 { Collision::Bottom } else { Collision::Top }
    }
}