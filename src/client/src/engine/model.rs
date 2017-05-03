use glium::backend::glutin_backend::GlutinFacade;
use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
  position: [f32; 3],
  normal: [f32; 3],
  texcoord: [f32; 2],
}
implement_vertex!(Vertex, position, normal, texcoord);

pub struct Mesh {
  vertices: Vec<Vertex>,
  indices: Vec<u16>,
}

pub struct Model {
  meshes: Vec<Mesh>,
}

impl Model {
  pub fn new(meshes: Vec<Mesh>) -> Model {
    Model {
      meshes: meshes,
    }
  }
}