use raylib::prelude::*;
use super::entity::World;
use super::components::*;

/// System trait - all systems implement this
pub trait System {
    fn update(&mut self, world: &mut World, delta_time: f32);
}

/// Movement system - applies velocity to transform
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            if let (Some(transform), Some(velocity)) = (&mut entity.transform, &entity.velocity) {
                transform.position.x += velocity.linear.x * delta_time;
                transform.position.y += velocity.linear.y * delta_time;
                transform.position.z += velocity.linear.z * delta_time;

                transform.rotation.x += velocity.angular.x * delta_time;
                transform.rotation.y += velocity.angular.y * delta_time;
                transform.rotation.z += velocity.angular.z * delta_time;
            }
        }
    }
}

/// Render system - draws all renderable entities
pub struct RenderSystem;

impl RenderSystem {
    pub fn render(&self, world: &World, d: &mut RaylibMode3D<RaylibDrawHandle>) {
        for entity in world.entities() {
            if let (Some(transform), Some(renderable)) = (&entity.transform, &entity.renderable) {
                if !renderable.visible {
                    continue;
                }

                match &renderable.shape {
                    RenderShape::Cube { size, color } => {
                        let c = Color::new(color[0], color[1], color[2], color[3]);
                        d.draw_cube_v(transform.position, *size, c);
                        d.draw_cube_wires_v(transform.position, *size, Color::BLACK);
                    }
                    RenderShape::Sphere { radius, color } => {
                        let c = Color::new(color[0], color[1], color[2], color[3]);
                        d.draw_sphere(transform.position, *radius, c);
                        d.draw_sphere_wires(transform.position, *radius, 16, 16, Color::BLACK);
                    }
                    RenderShape::Cylinder { radius, height, color } => {
                        let c = Color::new(color[0], color[1], color[2], color[3]);
                        d.draw_cylinder(transform.position, *radius, *radius, *height, 16, c);
                        d.draw_cylinder_wires(transform.position, *radius, *radius, *height, 16, Color::BLACK);
                    }
                    RenderShape::Model { path: _ } => {
                        // TODO: Implement model loading and rendering
                        // For now, just draw a placeholder cube
                        d.draw_cube_v(transform.position, Vector3::one(), Color::MAGENTA);
                    }
                }
            }
        }
    }
}

/// Simple player input system
pub struct PlayerInputSystem;

impl PlayerInputSystem {
    pub fn update(&self, world: &mut World, rl: &RaylibHandle) {
        let speed = 5.0;

        for entity in world.entities_mut() {
            if entity.is_player {
                if let Some(velocity) = &mut entity.velocity {
                    // Reset velocity
                    velocity.linear = Vector3::zero();

                    // WASD movement
                    if rl.is_key_down(KeyboardKey::KEY_W) {
                        velocity.linear.z = -speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_S) {
                        velocity.linear.z = speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_A) {
                        velocity.linear.x = -speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_D) {
                        velocity.linear.x = speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_SPACE) {
                        velocity.linear.y = speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                        velocity.linear.y = -speed;
                    }
                }
            }
        }
    }
}
