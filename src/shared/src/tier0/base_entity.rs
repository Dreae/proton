use cgmath::Point3;

pub struct BaseEntity {
  pos: Point3<f32>,
  scale: Point3<f32>,
}

impl BaseEntity {
  pub fn set_pos(&mut self, new_pos: [f32; 3]) {
    self.pos = From::from(new_pos);
  }

  pub fn get_pos(&self) -> [f32; 3] {
    self.pos.into()
  }

  pub fn set_scale(&mut self, scale: [f32; 3]) {
    self.scale = From::from(scale);
  }

  pub fn get_scale(&self) -> [f32; 3] {
    self.scale.into()
  }
}

impl BaseEntity {
  pub fn new() -> BaseEntity {
    BaseEntity {
      pos: Point3::new(0.0, 0.0, 0.0),
      scale: Point3::new(1.0, 1.0, 1.0),
    }
  }
}

pub trait Entity {
    fn set_pos(&mut self, pos: [f32; 3]);
    fn get_pos(&self) -> [f32; 3];

    fn set_scale(&mut self, scale: [f32; 3]);
    fn get_scale(&self) -> [f32; 3];

    fn _on_spawn_post(&mut self);
}
