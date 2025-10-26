mod ecs;
mod level;
pub mod rpg;

use raylib::prelude::*;
use level::*;
use rpg::*;

// Import ECS types explicitly to avoid conflicts with Raylib types
use ecs::entity::World;
use ecs::components::{Transform as EcsTransform, Camera as EcsCamera};
use ecs::systems::*;
use ecs::physics::{PhysicsSystem, CollisionSystem};

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
    let mut physics_system = PhysicsSystem::default();
    let mut collision_system = CollisionSystem::new();
    let player_input_system = PlayerInputSystem;
    let first_person_camera_system = FirstPersonCameraSystem;
    let mut render_system = RenderSystem::new();

    // Create RPG player
    let mut rpg_player = Player::new("Hero", PlayerClass::Warrior);

    // Give player some starting items
    let mut item_db = ItemDatabase::with_starter_items();

    // Add some test items to inventory
    if let Some(item) = item_db.get(ItemId(2)) { // Iron Sword
        rpg_player.inventory.add_item(item, 1);
    }
    if let Some(item) = item_db.get(ItemId(4)) { // Health Potion
        rpg_player.inventory.add_item(item, 5);
    }

    // Add some gold
    rpg_player.inventory.add_gold(150);

    // Add some experience
    rpg_player.add_exp(250);

    // Add a status effect for demo
    rpg_player.status_effects.push(TimedStatusEffect::new(
        StatusEffect::Blessed,
        15.0,
        5,
    ));

    // Create RPG UI
    let mut rpg_ui = RpgUI::new();

    // Debug flags
    let mut show_bounding_boxes = false;

    println!("Controls:");
    println!("  WASD - Move forward/backward/strafe");
    println!("  Space - Move up");
    println!("  Left Shift - Move down");
    println!("  Mouse - Look around (first-person)");
    println!("  I - Toggle Inventory");
    println!("  C - Toggle Character Sheet");
    println!("  B - Toggle Bounding Boxes");
    println!("  ESC - Exit");

    // Game loop
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        // Update RPG player
        rpg_player.update(delta_time);

        // Handle UI input
        if rl.is_key_pressed(KeyboardKey::KEY_I) {
            rpg_ui.show_inventory = !rpg_ui.show_inventory;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            rpg_ui.show_character_sheet = !rpg_ui.show_character_sheet;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_B) {
            show_bounding_boxes = !show_bounding_boxes;
        }

        // Update systems
        first_person_camera_system.update(&mut world, &rl);
        player_input_system.update(&mut world, &rl);
        movement_system.update(&mut world, delta_time);
        physics_system.update(&mut world, delta_time);
        collision_system.update(&mut world, delta_time);

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
            render_system.render(&world, &mut d3, show_bounding_boxes);

            // Draw grid
            d3.draw_grid(20, 1.0);
        }

        // Draw UI
        d.draw_fps(10, 10);

        // Draw RPG HUD - always visible
        RpgUI::draw_player_hud(&mut d, &rpg_player);

        // Draw status effects in top right
        if rpg_ui.show_status_effects {
            RpgUI::draw_status_effects(&mut d, &rpg_player, 950, 10);
        }

        // Draw quick stats at bottom
        RpgUI::draw_quick_stats(&mut d, &rpg_player, 10, 650);

        // Draw inventory panel if toggled
        if rpg_ui.show_inventory {
            RpgUI::draw_inventory(&mut d, &rpg_player.inventory, &item_db);
        }

        // Draw character sheet if toggled
        if rpg_ui.show_character_sheet {
            RpgUI::draw_character_sheet(&mut d, &rpg_player);
        }

        // Help text
        d.draw_text("Press I for Inventory, C for Character", 10, 690, 16, Color::LIGHTGRAY);
    }
}
