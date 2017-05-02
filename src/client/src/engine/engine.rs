use glium::Surface;
use glium::glutin::Event;
use engine::window::Window;
use std::process::exit;

pub struct Engine {
  window: Window,
}

impl Engine {
    pub fn new() -> Engine {
      let window = Window::new();
      Engine {
        window: window,
      }
    }

    pub fn start(&self) {
      loop {
        self.game_frame();
      }
    }

    fn game_frame(&self) {
      let mut frame = self.window.display.draw();
      frame.clear_color(0.0, 0.0, 1.0, 1.0);
      frame.finish().unwrap();

      for ev in self.window.display.poll_events() {
          self.dispatch_event(ev);
      }
    }

    fn dispatch_event(&self, ev: Event) {
      match ev {
          Event::Closed => exit(0),
          _ => (),
      }
    }
}