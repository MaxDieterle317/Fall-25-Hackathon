// src/main.rs
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds essential Bevy plugins for rendering, input, etc.
        .add_systems(Startup, hello_world_system) // Registers a system to run once at startup
        .run(); // Starts the Bevy application
}

// A system is a Rust function that contains game logic
fn hello_world_system() {
    println!("Hello, Bevy!");
}
