use engine::{Model, load_model, Engine};
use proton_shared::tier0::{BaseEntity, EngineShared};

#[derive(Entity)]
pub struct ModelEntity {
  model: Model,
  base_entity: BaseEntity,
}

impl ModelEntity {
  pub fn new(model: &str, engine: &mut EngineShared<Engine>) -> ModelEntity {
    let mut ent = ModelEntity {
      model: load_model(model),
      base_entity: BaseEntity::new(),
    };

    ent.cache_model(&mut engine.engine_impl);

    ent
  }

  pub fn cache_model(&mut self, engine: &mut Engine) {
    self.model.cache(&mut engine.window.display);
  }

  pub fn on_spawn_post(&mut self) {
    
  }
}