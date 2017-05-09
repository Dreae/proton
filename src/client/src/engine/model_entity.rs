use engine::{Model, load_model};
use proton_shared::tier0::BaseEntity;
use engine::window::{Drawable, DrawTarget};

use glium::backend::glutin_backend::GlutinFacade;
use glium;

#[derive(Entity)]
pub struct ModelEntity {
  model: Model,
  base_entity: BaseEntity,
}

impl ModelEntity {
  pub fn new(model: &str, facade: &GlutinFacade) -> ModelEntity {
    let mut ent = ModelEntity {
      model: load_model(model),
      base_entity: BaseEntity::new(),
    };

    ent.cache_model(facade);

    ent
  }

  pub fn cache_model(&mut self, facade: &GlutinFacade) {
    self.model.cache(facade);
  }

  pub fn on_spawn_post(&mut self) {
    
  }
}

impl Drawable for ModelEntity {
  fn draw(&self, surface: &mut DrawTarget) {
    self.model.draw(surface, &self.base_entity.get_pos(), &self.base_entity.get_scale());
  }
}