use image::{Rgb, ImageBuffer};
use bytemuck;

pub struct Engine {
    vertices: Vec<u8>,
    indices: Vec<u16>,
    vertex_size: u32,
    program_vertex: Box<dyn Fn(&[u8]) -> [f32; 4]>,
    program_color: Box<dyn Fn([f32; 4]) -> [f32; 4]>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            vertices: Vec::with_capacity(0),
            indices: Vec::with_capacity(0),
            vertex_size: 0,
            program_vertex: Box::new(|_| [0.0; 4]),
            program_color: Box::new(|_| [0.0; 4]),
        }
    }

    pub fn bind_programs<T>(&mut self, program_vertex: Box<dyn Fn(&T) -> [f32; 4]>, program_color: Box<dyn Fn([f32; 4]) -> [f32; 4]>) {

    }

    pub fn render(&self, from: u16, to: u16) -> ImageBuffer::<Rgb<u8>, Vec<u8>> {
        unimplemented!()
    }
}