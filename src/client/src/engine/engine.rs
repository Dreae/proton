use glium::glutin::Event;
use engine::window::Window;
use std::process::exit;
use proton_shared::tier0::GameEngine;

pub struct Engine {
  pub window: Window,
}

impl Engine {
    pub fn new() -> Engine {
      let window = Window::new();
      Engine {
        window: window,
      }
    }

    fn dispatch_event(&self, ev: Event) {
      match ev {
          Event::Closed => exit(0),
          _ => (),
      }
    }
}

impl GameEngine for Engine {
  fn on_start(&mut self) {
    self.window.load_shaders();
  }

  fn on_game_frame(&self, delta_t: f64) {
    for ev in self.window.display.poll_events() {
        self.dispatch_event(ev);
    }

    self.window.render_frame();
  }
}