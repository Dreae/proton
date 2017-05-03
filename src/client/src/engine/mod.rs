mod window;
mod engine;
mod model;

pub use self::window::Window;
pub use self::engine::Engine;
pub use self::model::Model;

use proton_shared::teir0::EngineShared;

pub fn create_engine() -> EngineShared<Engine> {
  EngineShared::new(Engine::new())
}