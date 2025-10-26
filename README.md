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

## ECS Components

- `Transform` - Position, rotation, scale
- `Renderable` - Visual representation (Cube, Sphere, Cylinder, Model)
- `Velocity` - Linear and angular velocity
- `Health` - Health points
- `Name` - Entity label

## Systems

- `MovementSystem` - Applies velocity to transform
- `RenderSystem` - Draws all renderable entities
- `PlayerInputSystem` - Handles player input

## License

MIT
