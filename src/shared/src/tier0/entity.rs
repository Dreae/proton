pub trait Entity {
    fn set_pos(&mut self, pos: [f32; 3]);
    fn get_pos(&self) -> [f32; 3];

    fn set_scale(&mut self, scale: [f32; 3]);
    fn get_scale(&self) -> [f32; 3];

    fn _on_spawn_post(&mut self);
}
