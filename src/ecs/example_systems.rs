//! Example systems demonstrating proper ECS patterns and best practices
//!
//! These systems show how to properly implement game logic in an ECS architecture.
//! Each system should have a single, focused responsibility.

use raylib::prelude::*;
use super::entity::{World, EntityId};
use super::components::*;
use super::examples::*;
use super::System;

// =============================================================================
// LIFETIME AND CLEANUP SYSTEMS
// =============================================================================

/// System that updates lifetime components and removes expired entities
///
/// This demonstrates:
/// - Two-pass iteration pattern (collect IDs, then modify)
/// - Proper entity removal
/// - Simple time-based logic
///
/// # Usage
/// ```
/// let mut lifetime_system = LifetimeSystem::new();
/// lifetime_system.update(&mut world, delta_time);
/// ```
pub struct LifetimeSystem {
    /// Reused vector to avoid allocations each frame
    entities_to_remove: Vec<EntityId>,
}

impl LifetimeSystem {
    pub fn new() -> Self {
        Self {
            entities_to_remove: Vec::new(),
        }
    }
}

impl Default for LifetimeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl System for LifetimeSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        // Clear from previous frame (reuse allocation)
        self.entities_to_remove.clear();

        // First pass: update lifetimes and collect expired entities
        for entity in world.entities_mut() {
            if let Some(lifetime) = &mut entity.lifetime {
                lifetime.remaining -= delta_time;

                if lifetime.is_expired() {
                    self.entities_to_remove.push(entity.id);
                }
            }
        }

        // Second pass: remove expired entities
        // We do this separately because we can't modify world while iterating
        for &id in &self.entities_to_remove {
            world.remove_entity(id);
        }
    }
}

/// System that removes entities marked for death
///
/// Useful for deferred deletion when you can't remove during iteration
pub struct DeathSystem;

impl System for DeathSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        let to_remove: Vec<EntityId> = world
            .entities()
            .filter(|e| e.marked_for_death.is_some())
            .map(|e| e.id)
            .collect();

        for id in to_remove {
            world.remove_entity(id);
        }
    }
}

// =============================================================================
// PHYSICS SYSTEMS
// =============================================================================

/// System that applies gravity to entities with velocity
///
/// This demonstrates:
/// - Simple component filtering
/// - Physics calculations
/// - Conditional logic based on component state
pub struct GravitySystem;

impl System for GravitySystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            // Only process entities that have both Velocity and Gravity
            if let (Some(velocity), Some(gravity)) = (&mut entity.velocity, &entity.gravity) {
                if gravity.enabled {
                    // Apply gravitational acceleration to downward velocity
                    velocity.linear.y -= gravity.force * delta_time;
                }
            }
        }
    }
}

/// System that applies drag/friction to moving entities
///
/// This demonstrates:
/// - Dampening/friction mechanics
/// - Component-based behavior modification
pub struct DragSystem;

impl System for DragSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            if let (Some(velocity), Some(drag)) = (&mut entity.velocity, &entity.drag) {
                // Apply exponential drag
                let linear_factor = (-drag.linear * delta_time).exp();
                let angular_factor = (-drag.angular * delta_time).exp();

                velocity.linear.x *= linear_factor;
                velocity.linear.y *= linear_factor;
                velocity.linear.z *= linear_factor;

                velocity.angular.x *= angular_factor;
                velocity.angular.y *= angular_factor;
                velocity.angular.z *= angular_factor;
            }
        }
    }
}

// =============================================================================
// VISUAL EFFECT SYSTEMS
// =============================================================================

/// System that auto-rotates entities
///
/// This demonstrates:
/// - Simple visual effects
/// - Direct transform manipulation
pub struct AutoRotateSystem;

impl System for AutoRotateSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            if let (Some(transform), Some(auto_rotate)) =
                (&mut entity.transform, &entity.auto_rotate) {
                transform.rotation.x += auto_rotate.speed.x * delta_time;
                transform.rotation.y += auto_rotate.speed.y * delta_time;
                transform.rotation.z += auto_rotate.speed.z * delta_time;
            }
        }
    }
}

/// System that fades out entities based on their lifetime
///
/// This demonstrates:
/// - Multi-component dependencies
/// - Visual effects tied to other systems
/// - Color manipulation
pub struct FadeOutSystem;

impl System for FadeOutSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        for entity in world.entities_mut() {
            // Requires both FadeOut and Lifetime components
            if let (Some(fade), Some(lifetime), Some(renderable)) =
                (&entity.fade_out, &entity.lifetime, &mut entity.renderable)
            {
                // Calculate alpha based on remaining lifetime percentage
                let alpha = (lifetime.percentage() * fade.initial_alpha * 255.0) as u8;

                // Apply alpha to renderable color
                match &mut renderable.shape {
                    RenderShape::Cube { color, .. } => color[3] = alpha,
                    RenderShape::Sphere { color, .. } => color[3] = alpha,
                    RenderShape::Cylinder { color, .. } => color[3] = alpha,
                    _ => {}
                }
            }
        }
    }
}

// =============================================================================
// COOLDOWN SYSTEMS
// =============================================================================

/// System that updates all cooldown components
///
/// This is a generic system that updates any cooldown, regardless of purpose.
/// This demonstrates the power of component composition.
pub struct CooldownSystem;

impl System for CooldownSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            // Update attack cooldown if present
            if let Some(attack) = &mut entity.attack_ability {
                attack.cooldown.remaining = (attack.cooldown.remaining - delta_time).max(0.0);
            }

            // Can easily extend to other cooldowns by adding similar checks
            // This shows how components can be independently managed
        }
    }
}

// =============================================================================
// AI SYSTEMS
// =============================================================================

/// System that makes entities follow their targets
///
/// This demonstrates:
/// - Entity-to-entity relationships
/// - Movement AI
/// - Distance calculations
/// - Safe handling of missing entities
pub struct FollowTargetSystem;

impl System for FollowTargetSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        // Collect data about targets first (can't have two mutable borrows)
        let mut follower_data = Vec::new();

        for entity in world.entities() {
            if let (Some(transform), Some(follow)) = (&entity.transform, &entity.follow_target) {
                // Get target position
                if let Some(target) = world.get_entity(follow.target_id) {
                    if let Some(target_transform) = &target.transform {
                        follower_data.push((
                            entity.id,
                            transform.position,
                            target_transform.position,
                            follow.follow_distance,
                            follow.speed,
                        ));
                    }
                }
            }
        }

        // Now update velocities based on collected data
        for (id, pos, target_pos, distance, speed) in follower_data {
            let direction = Vector3 {
                x: target_pos.x - pos.x,
                y: target_pos.y - pos.y,
                z: target_pos.z - pos.z,
            };

            let dist = (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z).sqrt();

            // Only move if further than desired distance
            if dist > distance {
                let normalized = Vector3 {
                    x: direction.x / dist,
                    y: direction.y / dist,
                    z: direction.z / dist,
                };

                if let Some(entity) = world.get_entity_mut(id) {
                    if entity.velocity.is_none() {
                        entity.velocity = Some(Velocity::default());
                    }

                    if let Some(velocity) = &mut entity.velocity {
                        velocity.linear.x = normalized.x * speed;
                        velocity.linear.y = normalized.y * speed;
                        velocity.linear.z = normalized.z * speed;
                    }
                }
            } else {
                // Stop moving when close enough
                if let Some(entity) = world.get_entity_mut(id) {
                    if let Some(velocity) = &mut entity.velocity {
                        velocity.linear = Vector3::zero();
                    }
                }
            }
        }
    }
}

/// System for patrol path AI
///
/// This demonstrates:
/// - Waypoint navigation
/// - State-based AI
/// - Vec component handling
pub struct PatrolSystem;

impl System for PatrolSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            if let (Some(transform), Some(patrol), Some(velocity)) =
                (&entity.transform, &mut entity.patrol_path, &mut entity.velocity)
            {
                if let Some(waypoint) = patrol.current() {
                    let direction = Vector3 {
                        x: waypoint.x - transform.position.x,
                        y: waypoint.y - transform.position.y,
                        z: waypoint.z - transform.position.z,
                    };

                    let dist = (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z).sqrt();

                    // Reached waypoint?
                    if dist < 0.5 {
                        patrol.next();
                    } else {
                        // Move toward waypoint
                        let normalized = Vector3 {
                            x: direction.x / dist,
                            y: direction.y / dist,
                            z: direction.z / dist,
                        };

                        velocity.linear.x = normalized.x * patrol.speed;
                        velocity.linear.y = normalized.y * patrol.speed;
                        velocity.linear.z = normalized.z * patrol.speed;
                    }
                }
            }
        }
    }
}

// =============================================================================
// COMBAT SYSTEMS
// =============================================================================

/// System that handles projectile collisions
///
/// This demonstrates:
/// - Collision detection
/// - Entity interaction
/// - Component-based damage
/// - Deferred entity removal
///
/// Note: This is a simplified example. A real game would use a proper
/// collision detection system with spatial partitioning.
pub struct ProjectileCollisionSystem;

impl System for ProjectileCollisionSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        let mut collisions = Vec::new();

        // Find projectiles
        let projectiles: Vec<(EntityId, Vector3, f32, Option<EntityId>)> = world
            .entities()
            .filter_map(|e| {
                if let (Some(transform), Some(projectile)) = (&e.transform, &e.projectile) {
                    if !projectile.has_hit {
                        return Some((e.id, transform.position, projectile.damage, projectile.owner_id));
                    }
                }
                None
            })
            .collect();

        // Find damageable entities
        let damageables: Vec<(EntityId, Vector3, f32)> = world
            .entities()
            .filter_map(|e| {
                if e.damageable.is_some() {
                    if let (Some(transform), Some(renderable)) = (&e.transform, &e.renderable) {
                        // Get approximate radius from renderable
                        let radius = match &renderable.shape {
                            RenderShape::Cube { size, .. } => size.x.max(size.y).max(size.z) / 2.0,
                            RenderShape::Sphere { radius, .. } => *radius,
                            RenderShape::Cylinder { radius, .. } => *radius,
                            _ => 1.0,
                        };
                        return Some((e.id, transform.position, radius));
                    }
                }
                None
            })
            .collect();

        // Check collisions
        for (proj_id, proj_pos, damage, owner_id) in projectiles {
            for (target_id, target_pos, target_radius) in &damageables {
                // Don't hit owner
                if let Some(owner) = owner_id {
                    if owner == *target_id {
                        continue;
                    }
                }

                let dx = proj_pos.x - target_pos.x;
                let dy = proj_pos.y - target_pos.y;
                let dz = proj_pos.z - target_pos.z;
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();

                if dist < *target_radius {
                    collisions.push((proj_id, *target_id, damage));
                }
            }
        }

        // Apply collisions
        for (proj_id, target_id, damage) in collisions {
            // Mark projectile as hit and for removal
            if let Some(proj) = world.get_entity_mut(proj_id) {
                if let Some(projectile) = &mut proj.projectile {
                    projectile.has_hit = true;
                }
                proj.marked_for_death = Some(MarkedForDeath);
            }

            // Damage target
            if let Some(target) = world.get_entity_mut(target_id) {
                if let Some(health) = &mut target.health {
                    health.current = (health.current - damage).max(0.0);

                    // Mark for death if health depleted
                    if !health.is_alive() {
                        target.marked_for_death = Some(MarkedForDeath);
                    }
                }
            }
        }
    }
}

// =============================================================================
// UTILITY SYSTEMS
// =============================================================================

/// Example of a state machine update system
///
/// This demonstrates:
/// - Enum-based state machines
/// - State transition logic
/// - Time tracking per state
///
/// Note: This is a template - you'd implement actual state logic for your game
pub struct StateMachineSystem;

impl System for StateMachineSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            // This is where you'd update your state machines
            // Example with a generic state machine component
            // You would implement this for your specific state types

            // Update time in state
            // state_machine.time_in_state += delta_time;

            // Check for state transitions based on conditions
            // match state_machine.current_state {
            //     State::Idle => {
            //         if some_condition {
            //             state_machine.transition_to(State::Moving);
            //         }
            //     }
            //     ...
            // }
        }
    }
}
