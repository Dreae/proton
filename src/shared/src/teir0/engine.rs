pub struct EngineShared<T: GameEngine> {
  engine: T,
}

pub trait GameEngine {
  fn on_game_frame(&self);
}

impl <T: GameEngine> EngineShared<T> {
  pub fn new(engine: T) -> EngineShared<T> {
    EngineShared {
      engine: engine,
    }
  }

  pub fn start(&self) {
    loop {
      self.game_frame();
    }
  }

  pub fn game_frame(&self) {
    self.engine.on_game_frame();
  }
}