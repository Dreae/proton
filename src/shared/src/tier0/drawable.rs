use glium;
use cgmath;
use tier0::Vertex;
use glium::Surface;

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