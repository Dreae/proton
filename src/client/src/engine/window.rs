use glium;
use cgmath;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;

use engine::shaders::*;
use engine::{ModelEntity, Vertex};

pub struct Window {
  pub display: GlutinFacade,
  geometry_program: Option<glium::Program>,
}

impl Window {
    pub fn new() -> Window {
      let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .with_depth_buffer(24)
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
      frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
      
      let ent = ModelEntity::new("models/nanosuit.obj", &self.display);
      ent.draw(&mut DrawTarget::with_frame(self.geometry_program.as_ref().unwrap(), &mut frame));

      frame.finish().unwrap();
    }
}

pub struct DrawTarget<'a> {
  active_shaders: &'a glium::Program,
  frame: Option<&'a mut glium::Frame>,
  frame_buffer: Option<&'a mut glium::framebuffer::MultiOutputFrameBuffer<'a>>,
}

impl <'a> DrawTarget<'a> {
  pub fn with_frame(active_shaders: &'a glium::Program, frame: &'a mut glium::Frame) -> DrawTarget<'a> {
    DrawTarget {
      active_shaders: active_shaders,
      frame: Some(frame),
      frame_buffer: None,
    }
  }
  
  pub fn draw(&mut self, vertex_buffer: &glium::VertexBuffer<Vertex>, index_buffer: &glium::IndexBuffer<u32>, transform: &cgmath::Matrix4<f32>) {
    let uniforms = uniform! {
      model_pos: Into::<[[f32; 4]; 4]>::into(*transform),
      u_light: [-1.0, 0.4, -0.9f32],
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    if let Some(ref mut frame) = self.frame {
      frame.draw(vertex_buffer, index_buffer, self.active_shaders, &uniforms, &params).unwrap();
    } else if let Some(ref mut frame_buffer) = self.frame_buffer {
      frame_buffer.draw(vertex_buffer, index_buffer, self.active_shaders, &uniforms, &params).unwrap();
    }

  }
}

pub trait Drawable {
  fn draw(&self, surface: &mut DrawTarget);
}