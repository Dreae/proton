extern crate proton_client;
extern crate proton_shared;

use proton_client::engine;
use proton_client::engine::ModelEntity;
use proton_shared::tier0::Entity;

fn main() {
    let mut engine = engine::create_engine();
    engine.start();
}
