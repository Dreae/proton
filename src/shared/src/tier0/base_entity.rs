pub struct BaseEntity {
  pos: [f32; 3],
}

impl BaseEntity {
  pub fn set_pos(&mut self, new_pos: [f32; 3]) {
    self.pos = new_pos;
  }
}

impl BaseEntity {
  pub fn new() -> BaseEntity {
    BaseEntity {
      pos: [0.0, 0.0, 0.0],
    }
  }
}

pub trait Entity {
    fn set_pos(&mut self, pos: [f32; 3]);
}
