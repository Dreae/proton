extern crate proton_client;
extern crate proton_shared;

use proton_client::engine;
use proton_client::engine::ModelEntity;
use proton_shared::tier0::Entity;

fn main() {
    let mut engine = engine::create_engine();
    let mut ent = ModelEntity::new("models/teapot.obj", &engine.engine_impl.window.display);
    engine.start();
}
