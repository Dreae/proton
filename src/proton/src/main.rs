extern crate proton_client;

use proton_client::engine;

fn main() {
    let engine = engine::create_engine();
    engine.start();
}
