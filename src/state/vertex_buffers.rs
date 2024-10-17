use glium::{glutin::surface::WindowSurface, Display, VertexBuffer};
use std::collections::HashMap;

use crate::vertex::Vertex;

pub struct VertexBuffers {
    buffers: HashMap<VertexBufferName, VertexBuffer<Vertex>>,
}

impl VertexBuffers {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let mut buffers = HashMap::new();

        buffers.insert(VertexBufferName::Hex, hex_buffer(display));

        Self { buffers }
    }

    pub fn get(&self, name: &VertexBufferName) -> &VertexBuffer<Vertex> {
        self.buffers.get(name).unwrap()
    }
}

fn hex_buffer(display: &Display<WindowSurface>) -> VertexBuffer<Vertex> {
    let hexagon = [
        Vertex {
            position: [0.0, 1.0],
            tex_coords: [0.5, 1.055],
        },
        Vertex {
            position: [-8.0 / 9.0, 0.5],
            tex_coords: [-0.055, 0.7775],
        },
        Vertex {
            position: [8.0 / 9.0, 0.5],
            tex_coords: [1.055, 0.7775],
        },
        Vertex {
            position: [-8.0 / 9.0, -0.5],
            tex_coords: [-0.055, 0.2225],
        },
        Vertex {
            position: [8.0 / 9.0, -0.5],
            tex_coords: [1.055, 0.2225],
        },
        Vertex {
            position: [0.0, -1.0],
            tex_coords: [0.5, -0.055],
        },
    ];
    glium::VertexBuffer::new(display, &hexagon).unwrap()
}

#[derive(Eq, Hash, PartialEq)]
pub enum VertexBufferName {
    Hex,
}
