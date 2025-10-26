use raylib::prelude::*;
use std::collections::HashMap;
use super::entity::World;
use super::components::{Transform, RenderShape};

/// System trait - all systems implement this
pub trait System {
    fn update(&mut self, world: &mut World, delta_time: f32);
}

/// Cache for loaded models and textures to avoid reloading
pub struct ModelCache {
    models: HashMap<String, raylib::ffi::Model>,
    textures: HashMap<String, raylib::ffi::Texture2D>,
}

impl ModelCache {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn get_model(&mut self, path: &str) -> Option<raylib::ffi::Model> {
        if !self.models.contains_key(path) {
            // Try to load the model
            unsafe {
                let model = raylib::ffi::LoadModel(path.as_ptr() as *const i8);
                if model.meshes.is_null() {
                    return None; // Failed to load
                }
                self.models.insert(path.to_string(), model);
            }
        }
        self.models.get(path).copied()
    }

    pub fn get_texture(&mut self, path: &str) -> Option<raylib::ffi::Texture2D> {
        if !self.textures.contains_key(path) {
            // Try to load the texture
            unsafe {
                let texture = raylib::ffi::LoadTexture(path.as_ptr() as *const i8);
                if texture.id == 0 {
                    return None; // Failed to load
                }
                self.textures.insert(path.to_string(), texture);
            }
        }
        self.textures.get(path).copied()
    }

    pub fn cleanup(&mut self) {
        // Unload all models and textures
        for (_, model) in self.models.drain() {
            unsafe {
                raylib::ffi::UnloadModel(model);
            }
        }
        for (_, texture) in self.textures.drain() {
            unsafe {
                raylib::ffi::UnloadTexture(texture);
            }
        }
    }
}

impl Drop for ModelCache {
    fn drop(&mut self) {
        self.cleanup();
    }
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
pub struct RenderSystem {
    model_cache: ModelCache,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            model_cache: ModelCache::new(),
        }
    }

    pub fn render(&mut self, world: &World, d: &mut RaylibMode3D<RaylibDrawHandle>) {
        for entity in world.entities() {
            if let (Some(transform), Some(renderable)) = (&entity.transform, &entity.renderable) {
                if !renderable.visible {
                    continue;
                }

                // Helper function to draw rotated shapes
                let draw_with_rotation = |d: &mut RaylibMode3D<RaylibDrawHandle>,
                                          transform: &Transform,
                                          draw_fn: &dyn Fn(&mut RaylibMode3D<RaylibDrawHandle>)| {
                    // For now, if there's any rotation, we'll draw using a different approach
                    // Check if we have any rotation
                    let has_rotation = transform.rotation.x.abs() > 0.001
                                    || transform.rotation.y.abs() > 0.001
                                    || transform.rotation.z.abs() > 0.001;

                    if has_rotation {
                        // Use unsafe FFI to access the lower-level rlPushMatrix/rlPopMatrix
                        // This is necessary because raylib-rs doesn't expose these methods
                        unsafe {
                            raylib::ffi::rlPushMatrix();
                            raylib::ffi::rlTranslatef(transform.position.x, transform.position.y, transform.position.z);
                            raylib::ffi::rlRotatef(transform.rotation.x.to_degrees(), 1.0, 0.0, 0.0);
                            raylib::ffi::rlRotatef(transform.rotation.y.to_degrees(), 0.0, 1.0, 0.0);
                            raylib::ffi::rlRotatef(transform.rotation.z.to_degrees(), 0.0, 0.0, 1.0);
                            raylib::ffi::rlScalef(transform.scale.x, transform.scale.y, transform.scale.z);
                        }
                        draw_fn(d);
                        unsafe {
                            raylib::ffi::rlPopMatrix();
                        }
                    } else {
                        // No rotation, draw normally
                        draw_fn(d);
                    }
                };

                match &renderable.shape {
                    RenderShape::Cube { size, color } => {
                        let c = Color::new(color[0], color[1], color[2], color[3]);
                        let has_rotation = transform.rotation.x.abs() > 0.001
                                        || transform.rotation.y.abs() > 0.001
                                        || transform.rotation.z.abs() > 0.001;

                        if has_rotation {
                            draw_with_rotation(d, transform, &|d| {
                                d.draw_cube_v(Vector3::zero(), *size, c);
                                d.draw_cube_wires_v(Vector3::zero(), *size, Color::BLACK);
                            });
                        } else {
                            d.draw_cube_v(transform.position, *size, c);
                            d.draw_cube_wires_v(transform.position, *size, Color::BLACK);
                        }
                    }
                    RenderShape::Sphere { radius, color } => {
                        let c = Color::new(color[0], color[1], color[2], color[3]);
                        let has_rotation = transform.rotation.x.abs() > 0.001
                                        || transform.rotation.y.abs() > 0.001
                                        || transform.rotation.z.abs() > 0.001;

                        if has_rotation {
                            draw_with_rotation(d, transform, &|d| {
                                d.draw_sphere(Vector3::zero(), *radius, c);
                                d.draw_sphere_wires(Vector3::zero(), *radius, 16, 16, Color::BLACK);
                            });
                        } else {
                            d.draw_sphere(transform.position, *radius, c);
                            d.draw_sphere_wires(transform.position, *radius, 16, 16, Color::BLACK);
                        }
                    }
                    RenderShape::Cylinder { radius, height, color } => {
                        let c = Color::new(color[0], color[1], color[2], color[3]);
                        let has_rotation = transform.rotation.x.abs() > 0.001
                                        || transform.rotation.y.abs() > 0.001
                                        || transform.rotation.z.abs() > 0.001;

                        if has_rotation {
                            draw_with_rotation(d, transform, &|d| {
                                d.draw_cylinder(Vector3::zero(), *radius, *radius, *height, 16, c);
                                d.draw_cylinder_wires(Vector3::zero(), *radius, *radius, *height, 16, Color::BLACK);
                            });
                        } else {
                            d.draw_cylinder(transform.position, *radius, *radius, *height, 16, c);
                            d.draw_cylinder_wires(transform.position, *radius, *radius, *height, 16, Color::BLACK);
                        }
                    }
                    RenderShape::Model { path: _ } => {
                        // TODO: Implement model loading and rendering
                        // For now, just draw a placeholder cube
                        d.draw_cube_v(transform.position, Vector3::one(), Color::MAGENTA);
                    }
                }
            }

            // Handle model component rendering
            if let (Some(transform), Some(model)) = (&entity.transform, &entity.model) {
                // Load model and texture separately to avoid borrowing conflicts
                let raylib_model = self.model_cache.get_model(&model.model_path);
                let texture = if let Some(texture_path) = &model.texture_path {
                    self.model_cache.get_texture(texture_path)
                } else {
                    None
                };

                if let Some(raylib_model) = raylib_model {
                    // Apply texture if specified
                    if let Some(texture) = texture {
                        unsafe {
                            // Set the diffuse texture for the first material
                            if !raylib_model.materials.is_null() && raylib_model.materialCount > 0 {
                                let material = &mut *raylib_model.materials;
                                if !material.maps.is_null() {
                                    let maps = &mut *material.maps;
                                    (*maps).texture = texture;
                                }
                            }
                        }
                    }

                    // Draw the model with transform
                    unsafe {
                        raylib::ffi::rlPushMatrix();
                        raylib::ffi::rlTranslatef(transform.position.x, transform.position.y, transform.position.z);
                        raylib::ffi::rlRotatef(transform.rotation.x.to_degrees(), 1.0, 0.0, 0.0);
                        raylib::ffi::rlRotatef(transform.rotation.y.to_degrees(), 0.0, 1.0, 0.0);
                        raylib::ffi::rlRotatef(transform.rotation.z.to_degrees(), 0.0, 0.0, 1.0);
                        raylib::ffi::rlScalef(transform.scale.x * model.scale, transform.scale.y * model.scale, transform.scale.z * model.scale);
                        
                        raylib::ffi::DrawModel(raylib_model, Vector3::zero().into(), model.scale, model.tint.into());
                        
                        raylib::ffi::rlPopMatrix();
                    }
                } else {
                    // Fallback: draw a placeholder cube if model fails to load
                    d.draw_cube_v(transform.position, Vector3::one(), Color::MAGENTA);
                }
            }
        }
    }
}

/// First-person camera control system
pub struct FirstPersonCameraSystem;

impl FirstPersonCameraSystem {
    pub fn update(&self, world: &mut World, rl: &RaylibHandle) {
        for entity in world.entities_mut() {
            if let Some(camera) = &mut entity.camera {
                // Mouse look controls
                let mouse_delta = rl.get_mouse_delta();

                camera.yaw += mouse_delta.x * camera.mouse_sensitivity;
                camera.pitch -= mouse_delta.y * camera.mouse_sensitivity;

                // Clamp pitch to avoid gimbal lock
                camera.pitch = camera.pitch.clamp(-89.0, 89.0);

                // Normalize yaw to 0-360 range
                if camera.yaw > 360.0 {
                    camera.yaw -= 360.0;
                } else if camera.yaw < 0.0 {
                    camera.yaw += 360.0;
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

                    // Get camera orientation if entity has a camera
                    let (forward, right) = if let Some(camera) = &entity.camera {
                        let yaw_rad = camera.yaw.to_radians();
                        let forward = Vector3::new(
                            yaw_rad.cos(),
                            0.0,  // Don't move up/down with pitch
                            yaw_rad.sin(),
                        );
                        let right = Vector3::new(
                            (yaw_rad + std::f32::consts::FRAC_PI_2).cos(),
                            0.0,
                            (yaw_rad + std::f32::consts::FRAC_PI_2).sin(),
                        );
                        (forward, right)
                    } else {
                        // Default forward/right if no camera
                        (Vector3::new(0.0, 0.0, -1.0), Vector3::new(1.0, 0.0, 0.0))
                    };

                    // WASD movement relative to camera direction
                    if rl.is_key_down(KeyboardKey::KEY_W) {
                        velocity.linear = velocity.linear + forward * speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_S) {
                        velocity.linear = velocity.linear - forward * speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_A) {
                        velocity.linear = velocity.linear - right * speed;
                    }
                    if rl.is_key_down(KeyboardKey::KEY_D) {
                        velocity.linear = velocity.linear + right * speed;
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
