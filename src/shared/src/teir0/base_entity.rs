pub struct BaseEntity {
  pos: [f32; 3],
  model: String,
}

impl BaseEntity {
  pub fn set_pos(&mut self, new_pos: [f32; 3]) {
    self.pos = new_pos;
  }
}

pub trait Entity {

}