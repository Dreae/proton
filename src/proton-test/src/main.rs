extern crate proton_client;

use proton_client::engine;
use proton_client::engine::ModelEntity;

fn main() {
    let mut engine = engine::create_engine();
    let mut ent = ModelEntity::new("models/teapot.obj");
    engine.start();
}
