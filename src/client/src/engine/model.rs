use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use glium;

use cgmath;
use engine::window::DrawTarget;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub texcoord: [f32; 2],
}
implement_vertex!(Vertex, position, normal, texcoord);

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
  cached: bool,
  index_buffer: Option<glium::IndexBuffer<u32>>,
  vertex_buffer: Option<glium::VertexBuffer<Vertex>>,
}

impl Mesh {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
    Mesh {
      indices: indices,
      vertices: vertices,
      cached: false,
      vertex_buffer: None,
      index_buffer: None,
    }
  }

  pub fn is_cached(&self) -> bool {
    self.cached
  }

  pub fn cache(&mut self, facade: &GlutinFacade) {
    if !self.cached {
      self.vertex_buffer = Some(glium::VertexBuffer::new(facade, &self.vertices).unwrap());
      self.index_buffer = Some(glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap());
    }

    self.cached = true;
  }

  pub fn draw(&self, surface: &mut DrawTarget, position: &[f32; 3], scale: &[f32; 3]) {
    if !self.is_cached() {
      panic!("FATAL: Tried to draw mesh that wasn't cached");
    }
    // TODO: Obviously this stuff can't live here.
    let pos = cgmath::Matrix4::from_translation((*position).into());
    let scale = cgmath::Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]);
    let transform = pos * scale;

    surface.draw(self.vertex_buffer.as_ref().unwrap(), self.index_buffer.as_ref().unwrap(), &transform);
  }
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

  pub fn cache(&mut self, facade: &GlutinFacade) {
    for mut mesh in &mut self.meshes {
      mesh.cache(facade);
    }
  }

  pub fn draw(&self, surface: &mut DrawTarget, position: &[f32; 3], scale: &[f32; 3]) {
    for mesh in &self.meshes {
      mesh.draw(surface, position, scale);
    }
  }
}