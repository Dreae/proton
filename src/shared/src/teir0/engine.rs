use time;

pub struct EngineShared<T: GameEngine> {
  engine: T,
  last_frame_t: f64,
  pub delta_t: f64,
}

pub trait GameEngine {
  fn on_game_frame(&self, delta_t: f64);
}

impl <T: GameEngine> EngineShared<T> {
  pub fn new(engine: T) -> EngineShared<T> {
    EngineShared {
      engine: engine,
      last_frame_t: 0.0,
      delta_t: 0.0,
    }
  }

  pub fn start(&mut self) {
    loop {
      let t = time::precise_time_s();
      self.delta_t = self.last_frame_t - t;
      self.last_frame_t = t;
      
      self.game_frame();
    }
  }

  pub fn game_frame(&self) {
    self.engine.on_game_frame(self.delta_t);
  }
}