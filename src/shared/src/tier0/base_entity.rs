pub struct BaseEntity {
  pos: [f32; 3],
  scale: [f32; 3],
}

impl BaseEntity {
  pub fn set_pos(&mut self, new_pos: [f32; 3]) {
    self.pos = new_pos;
  }

  pub fn set_scale(&mut self, scale: [f32; 3]) {
    self.scale = scale;
  }
}

impl BaseEntity {
  pub fn new() -> BaseEntity {
    BaseEntity {
      pos: [0.0, 0.0, 0.0],
      scale: [1.0, 1.0, 1.0],
    }
  }
}

pub trait Entity {
    fn set_pos(&mut self, pos: [f32; 3]);
    fn set_scale(&mut self, scale: [f32; 3]);
    fn _on_spawn_post(&mut self);
}
