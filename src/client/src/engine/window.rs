use glium;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;

use engine::shaders::*;
use engine::Vertex;

pub struct Window {
  pub display: GlutinFacade,
  geometry_program: Option<glium::Program>,
}

impl Window {
    pub fn new() -> Window {
      let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

      Window {
        display: display,
        geometry_program: None,
      }
    }

    pub fn load_shaders(&mut self) {
      self.geometry_program = Some(glium::Program::from_source(&self.display, GEOMETRY_VERTEX_SHADER, GEOMETRY_FRAGMENT_SHADER, None).unwrap());
    }

    pub fn render_frame(&self) {
      let mut frame = self.display.draw();
      frame.clear_color(0.0, 0.0, 1.0, 1.0);
      let vertex1 = Vertex {
        position: [-0.5, -0.5, 0.0],
        normal: [0.0, 0.0, 0.0],
        texcoord: [0.0, 0.0],
      };
      let vertex2 = Vertex {
        position: [0.0, 0.5, 0.0],
        normal: [0.0, 0.0, 0.0],
        texcoord: [0.0, 0.0],
      };
      let vertex3 = Vertex {
        position: [0.5, -0.25, 0.0],
        normal: [0.0, 0.0, 0.0],
        texcoord: [0.0, 0.0],
      };
      let shape = vec![vertex1, vertex2, vertex3];
      let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
      let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
      frame.draw(&vertex_buffer, &indices, &self.geometry_program.as_ref().unwrap(), &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

      frame.finish().unwrap();
    }
}