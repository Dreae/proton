use engine::Model;
use engine::load_model;
use proton_shared::tier0::BaseEntity;

#[derive(Entity)]
pub struct ModelEntity {
  model: Model,
  base_entity: BaseEntity,
}

impl ModelEntity {
  pub fn new(model: &str) -> ModelEntity {
    ModelEntity {
      model: load_model(model),
      base_entity: BaseEntity::new(),
    }
  }
}