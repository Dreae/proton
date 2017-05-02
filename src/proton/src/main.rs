extern crate proton_client;

use proton_client::engine::Engine;

fn main() {
    let engine = Engine::new();
    engine.start();
}
