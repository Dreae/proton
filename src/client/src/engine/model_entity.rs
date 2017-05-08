use engine::{Model, load_model, ClientEngine};
use proton_shared::tier0::{BaseEntity, EngineShared};
use engine::window::Drawable;

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
  fn draw(&self, surface: &mut glium::Frame, active_shaders: &glium::Program) {
    self.model.draw(surface, active_shaders, self.base_entity.get_pos());
  }
}