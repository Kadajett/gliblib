mod ecs;
mod level;

use raylib::prelude::*;
use ecs::*;
use level::*;

fn main() {
    // Initialize window
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("3D ECS Game - glibLib")
        .build();

    rl.set_target_fps(60);

    // Create ECS world
    let mut world = World::new();

    // Load sample level (you can also load from file)
    let level = LevelConfig::sample();

    // Optionally save the sample level to see the format
    if let Err(e) = LevelLoader::save_to_toml(&level, "levels/sample.toml") {
        println!("Warning: Could not save sample level to TOML: {}", e);
    }
    if let Err(e) = LevelLoader::save_to_json(&level, "levels/sample.json") {
        println!("Warning: Could not save sample level to JSON: {}", e);
    }

    // Spawn entities from level
    LevelLoader::spawn_entities(&level, &mut world);

    // Initialize camera from level config
    let mut camera = level.camera.to_camera3d();

    // Create systems
    let mut movement_system = MovementSystem;
    let player_input_system = PlayerInputSystem;
    let render_system = RenderSystem;

    println!("Controls:");
    println!("  WASD - Move player horizontally");
    println!("  Space - Move up");
    println!("  Left Shift - Move down");
    println!("  Mouse - Look around (when right button held)");
    println!("  ESC - Exit");

    // Game loop
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        // Update camera with mouse
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        // Update systems
        player_input_system.update(&mut world, &rl);
        movement_system.update(&mut world, delta_time);

        // Render
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        {
            let mut d3 = d.begin_mode3D(camera);

            // Render all entities
            render_system.render(&world, &mut d3);

            // Draw grid
            d3.draw_grid(20, 1.0);
        }

        // Draw UI
        d.draw_fps(10, 10);
        d.draw_text(&format!("Entities: {}", world.entities().count()), 10, 30, 20, Color::DARKGRAY);
        d.draw_text("Right-click + drag to look around", 10, 50, 20, Color::DARKGRAY);
    }
}
