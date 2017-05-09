mod window;
mod engine;
mod model_entity;
mod shaders;

pub use self::window::Window;
pub use self::engine::ClientEngine;
pub use self::model_entity::ModelEntity;

use proton_shared::tier0::EngineShared;

pub fn create_engine() -> EngineShared<ClientEngine> {
  EngineShared::new(ClientEngine::new())
}