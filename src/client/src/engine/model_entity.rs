use proton_shared::tier0::Model;
use proton_shared::tier0::{BaseEntity, ModelLoader};
use proton_shared::tier0::{Drawable, DrawTarget};

use glium::backend::glutin_backend::GlutinFacade;
use glium;

#[derive(Entity)]
pub struct ModelEntity {
  model: *mut Model,
  base_entity: BaseEntity,
}

impl ModelEntity {
  pub fn new(model: &str, facade: &GlutinFacade, model_loader: &mut ModelLoader) -> ModelEntity {
    let mut ent = ModelEntity {
      model: model_loader.load_model(model),
      base_entity: BaseEntity::new(),
    };

    ent.cache_model(facade);

    ent
  }

  pub fn cache_model(&mut self, facade: &GlutinFacade) {
    let model = unsafe { &mut *self.model };
    model.cache(facade);
  }

  pub fn on_spawn_post(&mut self) {
    
  }
}

impl Drawable for ModelEntity {
  fn draw(&self, surface: &mut DrawTarget) {
    let model = unsafe { &mut *self.model };
    model.draw(surface, &self.base_entity.get_pos(), &self.base_entity.get_scale());
  }
}