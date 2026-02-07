use chrono::Utc;
use gridhammer_core::sim::engine::Engine;

fn main() {
    let start = Utc::now();
    let seed = 42; // TODO: Parse from args
    let evse_count = 1; // TODO: Parse from args
    let mut engine = Engine::new(start, seed, evse_count);

    println!("Starting GridHammer Core...");
    println!("Clock start: {}", engine.clock().now());

    // Minimal run loop
    for i in 1..=5 {
        engine.tick();
        println!("Tick {}: {}", i, engine.clock().now());
    }
}
