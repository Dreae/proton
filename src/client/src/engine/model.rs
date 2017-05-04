use glium::backend::glutin_backend::GlutinFacade;
use glium;

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
    return self.cached;
  }

  pub fn cache(&mut self, facade: &GlutinFacade) {
    if !self.cached {
      self.vertex_buffer = Some(glium::VertexBuffer::new(facade, &self.vertices).unwrap());
      self.index_buffer = Some(glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap());
    }

    self.cached = true;
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

  pub fn cache(&mut self, facade: &mut GlutinFacade) {
    self.meshes.iter_mut().map(|mut mesh| {
      mesh.cache(facade);
    });
  }
}