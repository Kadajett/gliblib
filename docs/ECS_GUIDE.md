# ECS System Guide

## Overview

This ECS (Entity Component System) is a simple, data-oriented architecture for game development. Unlike traditional object-oriented approaches, ECS separates **data** (Components) from **behavior** (Systems), operating on **entities** which are simply IDs with associated components.

## Core Concepts

### Entities

An **Entity** is just an ID with a collection of components. Entities don't have behavior themselves - they're just containers.

```rust
// Create an entity
let entity_id = world.spawn()
    .with_transform(Transform::new(Vector3::new(0.0, 0.0, 0.0)))
    .with_renderable(Renderable::cube(Vector3::one(), Color::RED))
    .build();
```

### Components

**Components** are pure data structures. They should contain no logic, only state.

**Good Component Design:**
```rust
// ✓ GOOD - Pure data
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub linear: Vector3,
    pub angular: Vector3,
}

// ✓ GOOD - Simple state
#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
```

**Bad Component Design:**
```rust
// ✗ BAD - Contains logic
pub struct Enemy {
    pub health: f32,
}

impl Enemy {
    pub fn update(&mut self, delta: f32) {  // Don't do this!
        // Logic belongs in systems, not components
    }
}
```

### Systems

**Systems** contain the logic that operates on entities with specific components. Systems should be focused on a single responsibility.

```rust
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        // Iterate over all entities with both Transform and Velocity
        for entity in world.entities_mut() {
            if let (Some(transform), Some(velocity)) =
                (&mut entity.transform, &entity.velocity) {
                // Apply velocity to position
                transform.position.x += velocity.linear.x * delta_time;
                transform.position.y += velocity.linear.y * delta_time;
                transform.position.z += velocity.linear.z * delta_time;
            }
        }
    }
}
```

## Component Design Patterns

### 1. Tag Components

Tags are zero-size components used for marking entities.

```rust
#[derive(Debug, Clone, Copy)]
pub struct Player;

#[derive(Debug, Clone, Copy)]
pub struct Enemy;

#[derive(Debug, Clone, Copy)]
pub struct Collectible;
```

**Usage:**
```rust
let player_id = world.spawn()
    .with_transform(Transform::default())
    .as_player()  // Marks entity with Player tag
    .build();

// In systems, check for tags
for entity in world.entities() {
    if entity.is_player {
        // Player-specific logic
    }
}
```

### 2. State Components

Components that hold state should be simple and composable.

```rust
#[derive(Debug, Clone, Copy)]
pub struct Lifetime {
    pub remaining: f32,
    pub initial: f32,
}

impl Lifetime {
    pub fn new(duration: f32) -> Self {
        Self {
            remaining: duration,
            initial: duration,
        }
    }

    pub fn percentage(&self) -> f32 {
        self.remaining / self.initial
    }
}
```

### 3. Behavior-Defining Components

Some components define how systems should interact with entities.

```rust
#[derive(Debug, Clone)]
pub struct Gravity {
    pub force: f32,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Bouncy {
    pub restitution: f32,  // How much velocity is preserved on bounce
}
```

## System Design Patterns

### 1. Single Responsibility Systems

Each system should do one thing well.

```rust
// ✓ GOOD - Focused system
pub struct GravitySystem;

impl System for GravitySystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        for entity in world.entities_mut() {
            if let (Some(velocity), Some(gravity)) =
                (&mut entity.velocity, &entity.gravity) {
                if gravity.enabled {
                    velocity.linear.y -= gravity.force * delta_time;
                }
            }
        }
    }
}

// ✗ BAD - Doing too much
pub struct MegaSystem; // Updates gravity, movement, collision, rendering...
```

### 2. System Ordering

Systems should be ordered based on their dependencies:

```rust
// In your game loop:
input_system.update(&mut world, &rl);      // 1. Handle input
ai_system.update(&mut world, delta_time);   // 2. AI decisions
physics_system.update(&mut world, delta_time); // 3. Physics
movement_system.update(&mut world, delta_time); // 4. Apply movement
collision_system.update(&mut world, delta_time); // 5. Resolve collisions
lifetime_system.update(&mut world, delta_time);  // 6. Update lifetimes
// Rendering happens separately
```

### 3. Query Patterns

Different ways to access entities and components:

```rust
// Pattern 1: Iterate all entities, filter by components
for entity in world.entities() {
    if let Some(transform) = &entity.transform {
        // Do something with transform
    }
}

// Pattern 2: Multiple component requirements
for entity in world.entities_mut() {
    if let (Some(transform), Some(velocity), Some(health)) =
        (&mut entity.transform, &entity.velocity, &entity.health) {
        // Entity has all three components
    }
}

// Pattern 3: Check tags
for entity in world.entities() {
    if entity.is_enemy && entity.health.is_some() {
        // Enemy with health
    }
}
```

## Complete Example: Projectile System

Here's a complete example showing components, systems, and usage:

### Components

```rust
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub damage: f32,
    pub owner_id: Option<EntityId>,
}

#[derive(Debug, Clone, Copy)]
pub struct Lifetime {
    pub remaining: f32,
}

#[derive(Debug, Clone)]
pub struct OnHit {
    pub effects: Vec<HitEffect>,
}

#[derive(Debug, Clone)]
pub enum HitEffect {
    Damage(f32),
    Knockback(Vector3),
    Explode { radius: f32, force: f32 },
}
```

### System

```rust
pub struct ProjectileSystem;

impl System for ProjectileSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        let mut entities_to_remove = Vec::new();

        for entity in world.entities_mut() {
            if let Some(lifetime) = &mut entity.lifetime {
                lifetime.remaining -= delta_time;

                if lifetime.remaining <= 0.0 {
                    entities_to_remove.push(entity.id);
                }
            }
        }

        // Remove expired entities
        for id in entities_to_remove {
            world.remove_entity(id);
        }
    }
}
```

### Usage

```rust
// Spawn a projectile
fn spawn_projectile(world: &mut World, position: Vector3, direction: Vector3) {
    world.spawn()
        .with_transform(Transform::new(position))
        .with_velocity(Velocity {
            linear: direction * 20.0,
            angular: Vector3::zero(),
        })
        .with_renderable(Renderable::sphere(0.2, Color::YELLOW))
        .with_projectile(Projectile {
            damage: 10.0,
            owner_id: None,
        })
        .with_lifetime(Lifetime { remaining: 5.0 })
        .build();
}
```

## Best Practices

### DO:
- ✓ Keep components as simple data structures
- ✓ Put all logic in systems
- ✓ Use composition over inheritance (combine multiple simple components)
- ✓ Make systems single-purpose
- ✓ Use tag components for categorization
- ✓ Think about system execution order

### DON'T:
- ✗ Add methods to components that modify state
- ✗ Store references between entities in components (use IDs instead)
- ✗ Create god-components with too much data
- ✗ Create god-systems that do everything
- ✗ Mutate components in render systems

## Common Patterns

### Parent-Child Relationships

```rust
#[derive(Debug, Clone)]
pub struct Parent {
    pub children: Vec<EntityId>,
}

#[derive(Debug, Clone, Copy)]
pub struct Child {
    pub parent: EntityId,
}
```

### State Machines

```rust
#[derive(Debug, Clone, Copy)]
pub enum EnemyState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
}

#[derive(Debug, Clone, Copy)]
pub struct StateMachine {
    pub current_state: EnemyState,
    pub time_in_state: f32,
}
```

### Timers and Cooldowns

```rust
#[derive(Debug, Clone, Copy)]
pub struct Cooldown {
    pub remaining: f32,
    pub duration: f32,
}

impl Cooldown {
    pub fn is_ready(&self) -> bool {
        self.remaining <= 0.0
    }

    pub fn reset(&mut self) {
        self.remaining = self.duration;
    }
}
```

## Performance Tips

1. **Component Layout**: Most accessed components should be checked first
2. **Early Exit**: Use `continue` when entity doesn't match requirements
3. **Batch Operations**: Collect IDs first, then modify in a separate pass
4. **Avoid Allocations**: Reuse vectors for temporary storage

```rust
// Good: Reuse allocation
let mut to_remove = Vec::new();
loop {
    to_remove.clear();
    // ... collect entities to remove
    for id in &to_remove {
        world.remove_entity(*id);
    }
}
```
