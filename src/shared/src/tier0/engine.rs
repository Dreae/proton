use time;

pub struct EngineShared<T: GameEngine> {
  pub engine_impl: T,
  last_frame_t: f64,
  pub delta_t: f64,
}

pub trait GameEngine {
  fn on_game_frame(&self, delta_t: f64);
  fn on_start(&mut self);
}

impl <T: GameEngine> EngineShared<T> {
  pub fn new(engine: T) -> EngineShared<T> {
    EngineShared {
      engine_impl: engine,
      last_frame_t: 0.0,
      delta_t: 0.0,
    }
  }

  pub fn start(&mut self) {
    self.engine_impl.on_start();

    loop {
      let t = time::precise_time_s();
      self.delta_t = self.last_frame_t - t;
      self.last_frame_t = t;
      
      self.game_frame();
    }
  }

  pub fn game_frame(&self) {
    self.engine_impl.on_game_frame(self.delta_t);
  }
}