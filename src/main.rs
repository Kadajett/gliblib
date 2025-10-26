mod ecs;
mod level;

use raylib::prelude::*;
use level::*;

// Import ECS types explicitly to avoid conflicts with Raylib types
use ecs::entity::World;
use ecs::components::{Transform as EcsTransform, Camera as EcsCamera};
use ecs::systems::*;

fn main() {
    // Initialize window
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("3D ECS Game - glibLib")
        .build();

    rl.set_target_fps(60);
    rl.disable_cursor(); // Lock cursor for first-person controls

    // Create ECS world
    let mut world = World::new();

    // Load sample level (you can also load from file)
    let level = LevelLoader::load_from_json("levels/sample.json").unwrap();

    // Spawn entities from level
    LevelLoader::spawn_entities(&level, &mut world);

    // Create a camera entity attached to the player if player exists,
    // otherwise create a standalone camera entity
    let camera_entity_id = if let Some(player) = world.entities_mut().find(|e| e.is_player) {
        // Add camera to existing player
        player.camera = Some(EcsCamera::default());
        player.id
    } else {
        // Create standalone camera entity at the level's camera position
        world.spawn()
            .with_transform(EcsTransform::new(Vector3::new(
                level.camera.position[0],
                level.camera.position[1],
                level.camera.position[2],
            )))
            .with_camera(EcsCamera::new(level.camera.fov))
            .build()
    };

    // Create systems
    let mut movement_system = MovementSystem;
    let player_input_system = PlayerInputSystem;
    let first_person_camera_system = FirstPersonCameraSystem;
    let render_system = RenderSystem;

    println!("Controls:");
    println!("  WASD - Move forward/backward/strafe");
    println!("  Space - Move up");
    println!("  Left Shift - Move down");
    println!("  Mouse - Look around (first-person)");
    println!("  ESC - Exit");

    // Game loop
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        // Update systems
        first_person_camera_system.update(&mut world, &rl);
        player_input_system.update(&mut world, &rl);
        movement_system.update(&mut world, delta_time);

        // Get camera from entity
        let camera3d = if let Some(camera_entity) = world.get_entity(camera_entity_id) {
            if let (Some(transform), Some(camera)) = (&camera_entity.transform, &camera_entity.camera) {
                camera.to_camera3d(transform.position)
            } else {
                // Fallback camera if components are missing
                level.camera.to_camera3d()
            }
        } else {
            // Fallback camera if entity was removed
            level.camera.to_camera3d()
        };

        // Render
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        {
            let mut d3 = d.begin_mode3D(camera3d);

            // Render all entities
            render_system.render(&world, &mut d3);

            // Draw grid
            d3.draw_grid(20, 1.0);
        }

        // Draw UI
        d.draw_fps(10, 10);
        d.draw_text(&format!("Entities: {}", world.entities().count()), 10, 30, 20, Color::DARKGRAY);
        d.draw_text("Move mouse to look around", 10, 50, 20, Color::DARKGRAY);
    }
}
