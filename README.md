# glibLib - 3D ECS Game Engine

A basic 3D game engine built with Raylib and Rust, featuring an Entity Component System (ECS) and level configuration system.

## Features

- **ECS Architecture**: Component-based entity system for flexible game object management
- **Level Config System**: Load levels from TOML or JSON files
- **3D Rendering**: Built on Raylib for simple 3D graphics
- **Player Control**: WASD movement with mouse look

## Controls

- `WASD` - Move player horizontally
- `Space` - Move up
- `Left Shift` - Move down
- `Right Mouse + Drag` - Look around
- `ESC` - Exit

## Project Structure

```
src/
├── ecs/
│   ├── components.rs   # Component definitions (Transform, Renderable, etc.)
│   ├── entity.rs       # Entity and World management
│   ├── systems.rs      # Systems (Movement, Render, Input)
│   └── mod.rs
├── level/
│   ├── config.rs       # Level configuration structures
│   ├── loader.rs       # Level loading/saving
│   └── mod.rs
└── main.rs             # Game entry point

levels/                 # Level configuration files
├── sample.toml
└── sample.json
```

## Building and Running

```bash
cargo run
```

## Creating Levels

Levels can be created in TOML or JSON format. Example:

```toml
name = "My Level"
description = "A custom level"

[camera]
position = [10.0, 10.0, 10.0]
target = [0.0, 0.0, 0.0]
up = [0.0, 1.0, 0.0]
fov = 45.0

[[entities]]
name = "Ground"
entity_type = "static"

[entities.transform]
position = [0.0, -0.5, 0.0]
scale = [20.0, 1.0, 20.0]

[entities.renderable]
type = "Cube"
size = [1.0, 1.0, 1.0]
color = [100, 100, 100, 255]
```

## ECS System

### Core Components

- `Transform` - Position, rotation, scale
- `Renderable` - Visual representation (Cube, Sphere, Cylinder, Model)
- `Velocity` - Linear and angular velocity
- `Health` - Health points
- `Name` - Entity label

### Example Components (src/ecs/examples.rs:1)

The engine includes many example components showing best practices:

**Timing & Lifecycle**:
- `Lifetime` - Auto-remove entities after duration
- `Cooldown` - Ability cooldown timers

**Physics**:
- `Gravity` - Gravitational force
- `Bouncy` - Bounce on collision
- `Drag` - Air resistance/friction

**Combat**:
- `Projectile` - Bullets, arrows, etc.
- `AttackAbility` - Melee/ranged attacks

**AI & Behavior**:
- `FollowTarget` - Chase an entity
- `PatrolPath` - Waypoint navigation
- `StateMachine` - AI state management

**Visual Effects**:
- `FadeOut` - Fade alpha over lifetime
- `AutoRotate` - Continuous rotation

**Tags**:
- `Collectible`, `Obstacle`, `Damageable`, `MarkedForDeath`

### Core Systems

- `MovementSystem` - Applies velocity to transform
- `RenderSystem` - Draws all renderable entities
- `PlayerInputSystem` - Handles player input

### Example Systems (src/ecs/example_systems.rs:1)

Complete, documented example systems demonstrating patterns:
- `LifetimeSystem` - Entity expiration
- `GravitySystem` - Apply gravity
- `DragSystem` - Apply air resistance
- `AutoRotateSystem` - Rotate entities
- `FadeOutSystem` - Fade effects
- `FollowTargetSystem` - AI following
- `PatrolSystem` - Waypoint navigation
- `ProjectileCollisionSystem` - Collision detection

## Documentation

- **[ECS Guide](docs/ECS_GUIDE.md)** - Comprehensive guide to the ECS architecture, best practices, and patterns
- **[Examples](docs/EXAMPLES.md)** - Concrete code examples for common game development tasks

## Learning the ECS

1. Read [docs/ECS_GUIDE.md](docs/ECS_GUIDE.md) to understand ECS concepts
2. Check [docs/EXAMPLES.md](docs/EXAMPLES.md) for practical examples
3. Look at `src/ecs/examples.rs` for component examples with documentation
4. Study `src/ecs/example_systems.rs` for system implementation patterns
5. Run the demo and explore the sample level

## Quick Start

```rust
use ecs::*;

// Create a world
let mut world = World::new();

// Spawn an entity
let enemy = world.spawn()
    .with_transform(Transform::new(Vector3::new(5.0, 0.0, 0.0)))
    .with_renderable(Renderable::cube(Vector3::one(), Color::RED))
    .with_velocity(Velocity::default())
    .with_health(Health::new(100.0))
    .with_gravity(Gravity::earth())
    .as_enemy()
    .build();

// Create and update systems
let mut gravity_system = GravitySystem;
let mut movement_system = MovementSystem;

// In game loop
gravity_system.update(&mut world, delta_time);
movement_system.update(&mut world, delta_time);
```

## License

MIT
