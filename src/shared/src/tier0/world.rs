use tier0::Entity;
use std::cell::Cell;

pub struct World<'a> {
  entities: [&'a mut Entity; 8096],
}