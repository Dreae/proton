use glium;
use cgmath;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;

use engine::shaders::*;
use engine::ModelEntity;
use proton_shared::tier0::{Drawable, DrawTarget, Vertex, ModelLoader};

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

      let mut model_loader = ModelLoader::new();
      
      let ent = ModelEntity::new("models/nanosuit.obj", &self.display, &mut model_loader);
      ent.draw(&mut DrawTarget::with_frame(self.geometry_program.as_ref().unwrap(), &mut frame));

      frame.finish().unwrap();
    }
}