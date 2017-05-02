struct BaseEntity {
  pos: [f32; 3],
  model: String,
}

impl BaseEntity {
  set_pos(&mut self, new_pos: [f32; 3]) {
    self.pos = new_pos;
  }
}