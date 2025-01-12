extern crate redis;

use redis::Commands;
use std::thread;
use std::time::Duration;

fn main() {
    // Wait for the Redis server to start (if needed)
    thread::sleep(Duration::from_secs(2)); // Allow time for Redis to initialize

    // Connect to the Redis server
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("Invalid Redis URL");
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    // Example Redis operation: Set and Get a key
    let _: () = con.set("my_key", 42).expect("Failed to set key");
    let result: i32 = con.get("my_key").expect("Failed to get key");

    println!("The value of 'my_key' is: {}", result);

    // Optionally, you can interact more with Redis here
}
