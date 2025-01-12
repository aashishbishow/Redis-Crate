use std::process::Command;
use std::path::Path;

fn main() {
    // Path to the Redis server binary inside your project's binaries folder
    let redis_binary = Path::new("./binaries/redis-server");

    // Check if the Redis binary exists
    if !redis_binary.exists() {
        panic!("Redis server binary not found at {:?}", redis_binary.display());
    }

    // Print the path to the Redis binary
    println!("Found Redis binary at {:?}", redis_binary.display());

    // Start the Redis server (in the background)
    let _redis_process = Command::new(redis_binary)
        .arg("--port")
        .arg("6379") // Default Redis port
        .spawn()
        .expect("Failed to start Redis server");

    // Optionally, you can add a delay here to ensure Redis has time to start before continuing
}
