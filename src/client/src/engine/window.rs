use glium;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;

pub struct Window {
  pub display: GlutinFacade,
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
      }
    }
}