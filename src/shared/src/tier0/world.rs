use tier0::{Entity, Spawnable};

pub struct World {
  free_ents: Vec<usize>,
  num_ents: usize,
  entities: [*mut Entity; 8096],
}

impl World {
  fn create_entity<T>(&mut self) -> Box<T> where T: Spawnable + Entity + 'static {
    let mut ent_box = Box::new(T::create());
    let ent = Box::into_raw(ent_box);
    if let Some(idx) = self.free_ents.pop() {
      self.entities[idx] = ent;
    } else {
      self.entities[self.num_ents] = ent;
      self.num_ents = self.num_ents + 1;
    }

    unsafe {
      Box::from_raw(ent)
    }
  }
}