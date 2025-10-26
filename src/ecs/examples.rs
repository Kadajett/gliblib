//! Example components demonstrating proper ECS patterns
//!
//! This module shows best practices for component design in our ECS.
//! Components should be pure data with no behavior - all logic belongs in systems.

use raylib::prelude::*;
use super::entity::EntityId;

// =============================================================================
// TAG COMPONENTS
// =============================================================================
// Tag components are zero-sized types used to categorize entities.
// They're efficient and great for filtering entities in systems.

/// Marks an entity as collectible (coins, powerups, etc.)
///
/// # Example
/// ```
/// world.spawn()
///     .with_transform(Transform::new(position))
///     .with_collectible(Collectible)
///     .build();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Collectible;

/// Marks an entity as an obstacle that blocks movement
#[derive(Debug, Clone, Copy)]
pub struct Obstacle;

/// Marks an entity as damageable by the player
#[derive(Debug, Clone, Copy)]
pub struct Damageable;

// =============================================================================
// LIFETIME AND TIMING COMPONENTS
// =============================================================================

/// Component for entities that should be removed after a certain time
///
/// Useful for:
/// - Projectiles with limited range
/// - Temporary visual effects
/// - Timed powerups
///
/// # Example
/// ```
/// // Create a particle that lasts 2 seconds
/// world.spawn()
///     .with_transform(Transform::new(position))
///     .with_renderable(Renderable::sphere(0.1, Color::RED))
///     .with_lifetime(Lifetime::new(2.0))
///     .build();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Lifetime {
    /// Time remaining in seconds
    pub remaining: f32,
    /// Initial lifetime (useful for effects that scale with lifetime)
    pub initial: f32,
}

impl Lifetime {
    /// Create a new lifetime component
    pub fn new(duration: f32) -> Self {
        Self {
            remaining: duration,
            initial: duration,
        }
    }

    /// Get the percentage of lifetime remaining (0.0 to 1.0)
    pub fn percentage(&self) -> f32 {
        if self.initial <= 0.0 {
            0.0
        } else {
            (self.remaining / self.initial).max(0.0).min(1.0)
        }
    }

    /// Check if the lifetime has expired
    pub fn is_expired(&self) -> bool {
        self.remaining <= 0.0
    }
}

/// Cooldown timer for abilities or actions
///
/// # Example
/// ```
/// // In a shooting system:
/// if let Some(cooldown) = &mut entity.shoot_cooldown {
///     if cooldown.is_ready() {
///         spawn_bullet(...);
///         cooldown.reset();
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Cooldown {
    /// Time remaining until ready
    pub remaining: f32,
    /// Total cooldown duration
    pub duration: f32,
}

impl Cooldown {
    /// Create a new cooldown (starts ready)
    pub fn new(duration: f32) -> Self {
        Self {
            remaining: 0.0,
            duration,
        }
    }

    /// Create a cooldown that starts on cooldown
    pub fn new_active(duration: f32) -> Self {
        Self {
            remaining: duration,
            duration,
        }
    }

    /// Check if the cooldown is ready
    pub fn is_ready(&self) -> bool {
        self.remaining <= 0.0
    }

    /// Reset the cooldown to full duration
    pub fn reset(&mut self) {
        self.remaining = self.duration;
    }

    /// Get the percentage of cooldown complete (0.0 = just used, 1.0 = ready)
    pub fn percentage(&self) -> f32 {
        if self.duration <= 0.0 {
            1.0
        } else {
            1.0 - (self.remaining / self.duration).max(0.0).min(1.0)
        }
    }
}

// =============================================================================
// PHYSICS COMPONENTS
// =============================================================================

/// Applies gravitational force to entities with velocity
///
/// # Example
/// ```
/// world.spawn()
///     .with_transform(Transform::new(position))
///     .with_velocity(Velocity::default())
///     .with_gravity(Gravity::new(9.8))
///     .build();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Gravity {
    /// Gravitational acceleration (units per second squared)
    pub force: f32,
    /// Whether gravity is currently enabled
    pub enabled: bool,
}

impl Gravity {
    /// Create gravity with standard Earth gravity (9.8)
    pub fn earth() -> Self {
        Self {
            force: 9.8,
            enabled: true,
        }
    }

    /// Create gravity with custom force
    pub fn new(force: f32) -> Self {
        Self {
            force,
            enabled: true,
        }
    }

    /// Create disabled gravity
    pub fn disabled() -> Self {
        Self {
            force: 0.0,
            enabled: false,
        }
    }
}

/// Makes an entity bounce when colliding with surfaces
#[derive(Debug, Clone, Copy)]
pub struct Bouncy {
    /// Restitution coefficient (0.0 = no bounce, 1.0 = perfect bounce)
    pub restitution: f32,
}

impl Bouncy {
    /// Create a bouncy component with given restitution
    pub fn new(restitution: f32) -> Self {
        Self {
            restitution: restitution.max(0.0).min(1.0),
        }
    }
}

/// Applies drag/friction to velocity over time
#[derive(Debug, Clone, Copy)]
pub struct Drag {
    /// Linear drag coefficient (higher = more drag)
    pub linear: f32,
    /// Angular drag coefficient
    pub angular: f32,
}

impl Drag {
    /// Create drag with the same coefficient for linear and angular
    pub fn uniform(coefficient: f32) -> Self {
        Self {
            linear: coefficient,
            angular: coefficient,
        }
    }

    /// Create drag with separate coefficients
    pub fn new(linear: f32, angular: f32) -> Self {
        Self { linear, angular }
    }
}

// =============================================================================
// COMBAT COMPONENTS
// =============================================================================

/// Marks an entity as a projectile
///
/// # Example
/// ```
/// // Spawn a bullet
/// world.spawn()
///     .with_transform(Transform::new(position))
///     .with_velocity(Velocity { linear: direction * 50.0, angular: Vector3::zero() })
///     .with_projectile(Projectile::new(25.0))
///     .with_lifetime(Lifetime::new(3.0))
///     .build();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    /// Damage dealt on hit
    pub damage: f32,
    /// Optional entity ID of who fired this projectile
    pub owner_id: Option<EntityId>,
    /// Whether this projectile has already hit something
    pub has_hit: bool,
}

impl Projectile {
    /// Create a new projectile with given damage
    pub fn new(damage: f32) -> Self {
        Self {
            damage,
            owner_id: None,
            has_hit: false,
        }
    }

    /// Create a projectile with an owner
    pub fn with_owner(damage: f32, owner_id: EntityId) -> Self {
        Self {
            damage,
            owner_id: Some(owner_id),
            has_hit: false,
        }
    }
}

/// Component for entities that can attack
#[derive(Debug, Clone, Copy)]
pub struct AttackAbility {
    /// Damage per attack
    pub damage: f32,
    /// Range of attack
    pub range: f32,
    /// Cooldown between attacks
    pub cooldown: Cooldown,
}

impl AttackAbility {
    /// Create a new attack ability
    pub fn new(damage: f32, range: f32, attack_speed: f32) -> Self {
        Self {
            damage,
            range,
            cooldown: Cooldown::new(1.0 / attack_speed),
        }
    }

    /// Check if this ability can attack now
    pub fn can_attack(&self) -> bool {
        self.cooldown.is_ready()
    }

    /// Perform an attack (resets cooldown)
    pub fn attack(&mut self) {
        self.cooldown.reset();
    }
}

// =============================================================================
// AI AND BEHAVIOR COMPONENTS
// =============================================================================

/// Simple state machine for AI
///
/// # Example
/// ```
/// // Enemy AI with states
/// #[derive(Debug, Clone, Copy)]
/// enum EnemyState {
///     Idle,
///     Patrol,
///     Chase,
///     Attack,
/// }
///
/// let enemy = world.spawn()
///     .with_transform(Transform::new(position))
///     .with_state_machine(StateMachine::new(EnemyState::Idle))
///     .as_enemy()
///     .build();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StateMachine<T> {
    /// Current state
    pub current_state: T,
    /// Time spent in current state
    pub time_in_state: f32,
}

impl<T> StateMachine<T> {
    /// Create a new state machine in the given state
    pub fn new(initial_state: T) -> Self {
        Self {
            current_state: initial_state,
            time_in_state: 0.0,
        }
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: T) {
        self.current_state = new_state;
        self.time_in_state = 0.0;
    }
}

/// AI that follows a target entity
#[derive(Debug, Clone, Copy)]
pub struct FollowTarget {
    /// Entity to follow
    pub target_id: EntityId,
    /// Desired distance from target
    pub follow_distance: f32,
    /// Movement speed when following
    pub speed: f32,
}

impl FollowTarget {
    /// Create a new follow target behavior
    pub fn new(target_id: EntityId, distance: f32, speed: f32) -> Self {
        Self {
            target_id,
            follow_distance: distance,
            speed,
        }
    }
}

/// AI that patrols between waypoints
#[derive(Debug, Clone)]
pub struct PatrolPath {
    /// Waypoints to patrol
    pub waypoints: Vec<Vector3>,
    /// Current waypoint index
    pub current_waypoint: usize,
    /// Whether to loop back to start
    pub looping: bool,
    /// Movement speed
    pub speed: f32,
}

impl PatrolPath {
    /// Create a new patrol path
    pub fn new(waypoints: Vec<Vector3>, speed: f32, looping: bool) -> Self {
        Self {
            waypoints,
            current_waypoint: 0,
            looping,
            speed,
        }
    }

    /// Get the current waypoint
    pub fn current(&self) -> Option<Vector3> {
        self.waypoints.get(self.current_waypoint).copied()
    }

    /// Advance to next waypoint
    pub fn next(&mut self) -> Option<Vector3> {
        if self.waypoints.is_empty() {
            return None;
        }

        self.current_waypoint += 1;

        if self.current_waypoint >= self.waypoints.len() {
            if self.looping {
                self.current_waypoint = 0;
            } else {
                self.current_waypoint = self.waypoints.len() - 1;
            }
        }

        self.current()
    }
}

// =============================================================================
// VISUAL EFFECTS COMPONENTS
// =============================================================================

/// Makes an entity fade out over time (requires Lifetime component)
///
/// The alpha value will decrease from 1.0 to 0.0 over the entity's lifetime.
#[derive(Debug, Clone, Copy)]
pub struct FadeOut {
    /// Initial alpha value
    pub initial_alpha: f32,
}

impl FadeOut {
    pub fn new() -> Self {
        Self { initial_alpha: 1.0 }
    }
}

impl Default for FadeOut {
    fn default() -> Self {
        Self::new()
    }
}

/// Makes an entity scale up or down over time
#[derive(Debug, Clone, Copy)]
pub struct ScaleOverTime {
    /// Scale change per second
    pub rate: Vector3,
    /// Minimum scale
    pub min_scale: Vector3,
    /// Maximum scale
    pub max_scale: Vector3,
}

/// Rotates an entity continuously
#[derive(Debug, Clone, Copy)]
pub struct AutoRotate {
    /// Rotation speed in radians per second for each axis
    pub speed: Vector3,
}

impl AutoRotate {
    /// Create rotation around Y axis (common for pickups)
    pub fn around_y(speed: f32) -> Self {
        Self {
            speed: Vector3::new(0.0, speed, 0.0),
        }
    }

    /// Create rotation with custom speeds for each axis
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            speed: Vector3::new(x, y, z),
        }
    }
}

// =============================================================================
// UTILITY COMPONENTS
// =============================================================================

/// Stores a color that can be used for tinting or effects
#[derive(Debug, Clone, Copy)]
pub struct Tint {
    pub color: Color,
}

impl Tint {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

/// Marks an entity to be removed at the end of the frame
///
/// Useful for deferred deletion when you can't remove during iteration
#[derive(Debug, Clone, Copy)]
pub struct MarkedForDeath;

/// Parent-child relationship for hierarchical transforms
#[derive(Debug, Clone)]
pub struct Parent {
    pub children: Vec<EntityId>,
}

impl Parent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child_id: EntityId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }
}

impl Default for Parent {
    fn default() -> Self {
        Self::new()
    }
}

/// Child component storing parent reference
#[derive(Debug, Clone, Copy)]
pub struct Child {
    pub parent_id: EntityId,
    /// Local transform relative to parent
    pub local_offset: Vector3,
}

impl Child {
    pub fn new(parent_id: EntityId) -> Self {
        Self {
            parent_id,
            local_offset: Vector3::zero(),
        }
    }

    pub fn with_offset(parent_id: EntityId, offset: Vector3) -> Self {
        Self {
            parent_id,
            local_offset: offset,
        }
    }
}
