use glium::backend::glutin_backend::GlutinFacade;
use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
  position: [f32; 3],
}
implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
  normal: [f32; 3],
}
implement_vertex!(Normal, normal);

pub struct Model {
  vertices: glium::VertexBuffer<Vertex>,
  normals: glium::VertexBuffer<Normal>,
  indices: glium::IndexBuffer<u16>,
}

impl Model {
  pub fn new(facade: &GlutinFacade, vertices: &[Vertex], normals: &[Normal], indices: &[u16]) -> Model {
    Model {
      vertices: glium::VertexBuffer::new(facade, vertices).unwrap(),
      normals: glium::VertexBuffer::new(facade, normals).unwrap(),
      indices: glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, indices).unwrap(),
    }
  }
}