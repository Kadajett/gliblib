use raylib::prelude::*;
use super::entity::World;
use super::components::{Transform, Rigidbody, Collider, ColliderShape};
use super::systems::System;

/// Physics system that applies gravity and integrates velocity
pub struct PhysicsSystem {
    pub gravity: Vector3,
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self {
            gravity: Vector3::new(0.0, -9.8, 0.0), // Standard gravity
        }
    }
}

impl PhysicsSystem {
    pub fn new(gravity: Vector3) -> Self {
        Self { gravity }
    }
}

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            if let (Some(transform), Some(rigidbody)) = (&mut entity.transform, &mut entity.rigidbody) {
                // Skip static objects
                if rigidbody.is_static() {
                    continue;
                }

                // Apply gravity
                if rigidbody.use_gravity {
                    rigidbody.add_force(self.gravity * rigidbody.mass);
                }

                // Calculate acceleration from force (F = ma, so a = F/m)
                let acceleration = if rigidbody.mass > 0.0 {
                    rigidbody.force / rigidbody.mass
                } else {
                    Vector3::zero()
                };

                // Update velocity with acceleration
                rigidbody.velocity = rigidbody.velocity + (acceleration * delta_time);

                // Apply drag (air resistance)
                let drag_force = rigidbody.velocity * -rigidbody.drag;
                rigidbody.velocity = rigidbody.velocity + (drag_force * delta_time);

                // Update position
                transform.position = transform.position + (rigidbody.velocity * delta_time);

                // Reset forces for next frame
                rigidbody.force = Vector3::zero();

                // Reset grounded state (will be set by collision system)
                rigidbody.is_grounded = false;
            }
        }
    }
}

/// Collision data structure
#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub entity_a: usize,
    pub entity_b: usize,
    pub normal: Vector3,
    pub penetration: f32,
}

/// Collision detection and resolution system
pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }

    /// Check collision between two axis-aligned bounding boxes
    fn check_aabb_collision(
        pos_a: Vector3,
        size_a: Vector3,
        pos_b: Vector3,
        size_b: Vector3,
    ) -> Option<(Vector3, f32)> {
        let half_a = size_a / 2.0;
        let half_b = size_b / 2.0;

        let delta = pos_b - pos_a;

        let overlap_x = half_a.x + half_b.x - delta.x.abs();
        let overlap_y = half_a.y + half_b.y - delta.y.abs();
        let overlap_z = half_a.z + half_b.z - delta.z.abs();

        if overlap_x > 0.0 && overlap_y > 0.0 && overlap_z > 0.0 {
            // Find the axis with minimum overlap (that's our collision normal)
            let mut normal = Vector3::zero();
            let mut penetration = overlap_x;

            if overlap_x < overlap_y && overlap_x < overlap_z {
                penetration = overlap_x;
                normal.x = if delta.x > 0.0 { 1.0 } else { -1.0 };
            } else if overlap_y < overlap_z {
                penetration = overlap_y;
                normal.y = if delta.y > 0.0 { 1.0 } else { -1.0 };
            } else {
                penetration = overlap_z;
                normal.z = if delta.z > 0.0 { 1.0 } else { -1.0 };
            }

            Some((normal, penetration))
        } else {
            None
        }
    }

    /// Check collision between two spheres
    fn check_sphere_collision(
        pos_a: Vector3,
        radius_a: f32,
        pos_b: Vector3,
        radius_b: f32,
    ) -> Option<(Vector3, f32)> {
        let delta = pos_b - pos_a;
        let distance = delta.length();
        let min_distance = radius_a + radius_b;

        if distance < min_distance && distance > 0.0 {
            let normal = delta / distance;
            let penetration = min_distance - distance;
            Some((normal, penetration))
        } else {
            None
        }
    }

    /// Check collision between sphere and box
    fn check_sphere_box_collision(
        sphere_pos: Vector3,
        radius: f32,
        box_pos: Vector3,
        box_size: Vector3,
    ) -> Option<(Vector3, f32)> {
        let half_size = box_size / 2.0;
        let min = box_pos - half_size;
        let max = box_pos + half_size;

        // Find closest point on box to sphere center
        let closest = Vector3::new(
            sphere_pos.x.clamp(min.x, max.x),
            sphere_pos.y.clamp(min.y, max.y),
            sphere_pos.z.clamp(min.z, max.z),
        );

        let delta = sphere_pos - closest;
        let distance = delta.length();

        if distance < radius && distance > 0.0 {
            let normal = delta / distance;
            let penetration = radius - distance;
            Some((normal, penetration))
        } else if distance == 0.0 {
            // Sphere center is inside box - push out via shortest axis
            let penetrations = Vector3::new(
                (half_size.x - (sphere_pos.x - box_pos.x).abs()).min(radius),
                (half_size.y - (sphere_pos.y - box_pos.y).abs()).min(radius),
                (half_size.z - (sphere_pos.z - box_pos.z).abs()).min(radius),
            );

            let mut normal = Vector3::zero();
            let mut penetration = penetrations.x;

            if penetrations.x < penetrations.y && penetrations.x < penetrations.z {
                normal.x = if sphere_pos.x > box_pos.x { 1.0 } else { -1.0 };
                penetration = penetrations.x;
            } else if penetrations.y < penetrations.z {
                normal.y = if sphere_pos.y > box_pos.y { 1.0 } else { -1.0 };
                penetration = penetrations.y;
            } else {
                normal.z = if sphere_pos.z > box_pos.z { 1.0 } else { -1.0 };
                penetration = penetrations.z;
            }

            Some((normal, penetration + radius))
        } else {
            None
        }
    }

    /// Resolve collision between two entities
    fn resolve_collision(
        transform_a: &mut Transform,
        rigidbody_a: &mut Rigidbody,
        collider_a: &Collider,
        transform_b: &mut Transform,
        rigidbody_b: &mut Rigidbody,
        collider_b: &Collider,
        normal: Vector3,
        penetration: f32,
    ) {
        // Don't resolve if either is a trigger
        if collider_a.is_trigger || collider_b.is_trigger {
            return;
        }

        let static_a = rigidbody_a.is_static();
        let static_b = rigidbody_b.is_static();

        // Position correction
        if !static_a && !static_b {
            let correction = normal * (penetration / 2.0);
            transform_a.position = transform_a.position - correction;
            transform_b.position = transform_b.position + correction;
        } else if !static_a {
            transform_a.position = transform_a.position - (normal * penetration);
        } else if !static_b {
            transform_b.position = transform_b.position + (normal * penetration);
        }

        // Velocity resolution using impulse method
        let relative_velocity = rigidbody_b.velocity - rigidbody_a.velocity;
        let velocity_along_normal = relative_velocity.x * normal.x
            + relative_velocity.y * normal.y
            + relative_velocity.z * normal.z;

        // Don't resolve if velocities are separating
        if velocity_along_normal > 0.0 {
            return;
        }

        // Calculate restitution (bounciness)
        let restitution = (collider_a.restitution + collider_b.restitution) / 2.0;

        // Calculate impulse scalar
        let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
        let impulse_scalar = if !static_a && !static_b {
            impulse_scalar / (1.0 / rigidbody_a.mass + 1.0 / rigidbody_b.mass)
        } else if !static_a {
            impulse_scalar / (1.0 / rigidbody_a.mass)
        } else if !static_b {
            impulse_scalar / (1.0 / rigidbody_b.mass)
        } else {
            0.0
        };

        let impulse = normal * impulse_scalar;

        // Apply impulse
        if !static_a {
            rigidbody_a.velocity = rigidbody_a.velocity - (impulse / rigidbody_a.mass);
        }
        if !static_b {
            rigidbody_b.velocity = rigidbody_b.velocity + (impulse / rigidbody_b.mass);
        }

        // Apply friction
        let friction = (collider_a.friction + collider_b.friction) / 2.0;
        let tangent = relative_velocity - (normal * velocity_along_normal);
        let tangent_length = tangent.length();

        if tangent_length > 0.001 {
            let tangent_normalized = tangent / tangent_length;
            let friction_impulse = tangent_normalized * (-friction * impulse_scalar.abs());

            if !static_a {
                rigidbody_a.velocity = rigidbody_a.velocity - (friction_impulse / rigidbody_a.mass);
            }
            if !static_b {
                rigidbody_b.velocity = rigidbody_b.velocity + (friction_impulse / rigidbody_b.mass);
            }
        }

        // Set grounded state if colliding from above
        if normal.y > 0.5 && !static_a {
            rigidbody_a.is_grounded = true;
        }
        if normal.y < -0.5 && !static_b {
            rigidbody_b.is_grounded = true;
        }
    }
}

impl System for CollisionSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        let mut collisions = Vec::new();

        // Collect all collisions
        let entities: Vec<_> = world.entities().collect();
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let entity_a = &entities[i];
                let entity_b = &entities[j];

                if let (Some(transform_a), Some(collider_a), Some(transform_b), Some(collider_b)) = (
                    &entity_a.transform,
                    &entity_a.collider,
                    &entity_b.transform,
                    &entity_b.collider,
                ) {
                    let collision_result = match (&collider_a.shape, &collider_b.shape) {
                        (ColliderShape::Box { size: size_a }, ColliderShape::Box { size: size_b }) => {
                            Self::check_aabb_collision(transform_a.position, *size_a, transform_b.position, *size_b)
                        }
                        (ColliderShape::Sphere { radius: radius_a }, ColliderShape::Sphere { radius: radius_b }) => {
                            Self::check_sphere_collision(transform_a.position, *radius_a, transform_b.position, *radius_b)
                        }
                        (ColliderShape::Sphere { radius }, ColliderShape::Box { size }) => {
                            Self::check_sphere_box_collision(transform_a.position, *radius, transform_b.position, *size)
                        }
                        (ColliderShape::Box { size }, ColliderShape::Sphere { radius }) => {
                            Self::check_sphere_box_collision(transform_b.position, *radius, transform_a.position, *size)
                                .map(|(normal, pen)| (-normal, pen))
                        }
                        // Capsule collisions simplified to sphere for now
                        (ColliderShape::Capsule { radius, .. }, ColliderShape::Sphere { radius: radius_b }) => {
                            Self::check_sphere_collision(transform_a.position, *radius, transform_b.position, *radius_b)
                        }
                        (ColliderShape::Sphere { radius }, ColliderShape::Capsule { radius: radius_b, .. }) => {
                            Self::check_sphere_collision(transform_a.position, *radius, transform_b.position, *radius_b)
                        }
                        (ColliderShape::Capsule { radius: radius_a, .. }, ColliderShape::Capsule { radius: radius_b, .. }) => {
                            Self::check_sphere_collision(transform_a.position, *radius_a, transform_b.position, *radius_b)
                        }
                        (ColliderShape::Capsule { radius, .. }, ColliderShape::Box { size }) => {
                            Self::check_sphere_box_collision(transform_a.position, *radius, transform_b.position, *size)
                        }
                        (ColliderShape::Box { size }, ColliderShape::Capsule { radius, .. }) => {
                            Self::check_sphere_box_collision(transform_b.position, *radius, transform_a.position, *size)
                                .map(|(normal, pen)| (-normal, pen))
                        }
                    };

                    if let Some((normal, penetration)) = collision_result {
                        collisions.push(Collision {
                            entity_a: entity_a.id,
                            entity_b: entity_b.id,
                            normal,
                            penetration,
                        });
                    }
                }
            }
        }

        // Resolve all collisions
        for collision in collisions {
            // Get mutable references to both entities
            let entity_ids: Vec<_> = world.entities().map(|e| e.id).collect();

            if let (Some(idx_a), Some(idx_b)) = (
                entity_ids.iter().position(|&id| id == collision.entity_a),
                entity_ids.iter().position(|&id| id == collision.entity_b),
            ) {
                // We need to temporarily extract the entities to resolve collision
                // This is safe because we know idx_a != idx_b
                let mut entities_vec: Vec<_> = world.entities_mut().collect();

                if idx_a < entities_vec.len() && idx_b < entities_vec.len() {
                    let (entity_a, entity_b) = if idx_a < idx_b {
                        let (left, right) = entities_vec.split_at_mut(idx_b);
                        (&mut left[idx_a], &mut right[0])
                    } else {
                        let (left, right) = entities_vec.split_at_mut(idx_a);
                        (&mut right[0], &mut left[idx_b])
                    };

                    if let (
                        Some(transform_a),
                        Some(rigidbody_a),
                        Some(collider_a),
                        Some(transform_b),
                        Some(rigidbody_b),
                        Some(collider_b),
                    ) = (
                        &mut entity_a.transform,
                        &mut entity_a.rigidbody,
                        &entity_a.collider,
                        &mut entity_b.transform,
                        &mut entity_b.rigidbody,
                        &entity_b.collider,
                    ) {
                        Self::resolve_collision(
                            transform_a,
                            rigidbody_a,
                            collider_a,
                            transform_b,
                            rigidbody_b,
                            collider_b,
                            collision.normal,
                            collision.penetration,
                        );
                    }
                }
            }
        }
    }
}
