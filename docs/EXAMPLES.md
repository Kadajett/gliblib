# ECS Usage Examples

This document provides concrete examples of how to use the ECS system in your game.

## Table of Contents
1. [Creating Entities](#creating-entities)
2. [Using Systems](#using-systems)
3. [Common Patterns](#common-patterns)
4. [Complete Examples](#complete-examples)

## Creating Entities

### Basic Entity Creation

```rust
use ecs::*;

// Simple static object
let cube_id = world.spawn()
    .with_transform(Transform::new(Vector3::new(0.0, 0.0, 0.0)))
    .with_renderable(Renderable::cube(Vector3::one(), Color::RED))
    .build();

// Moving object with velocity
let moving_id = world.spawn()
    .with_transform(Transform::new(Vector3::new(5.0, 0.0, 0.0)))
    .with_renderable(Renderable::sphere(1.0, Color::BLUE))
    .with_velocity(Velocity {
        linear: Vector3::new(2.0, 0.0, 0.0),
        angular: Vector3::zero(),
    })
    .build();
```

### Player Entity

```rust
let player_id = world.spawn()
    .with_transform(Transform::new(Vector3::zero()))
    .with_renderable(Renderable::cube(Vector3::new(1.0, 2.0, 1.0), Color::GREEN))
    .with_velocity(Velocity::default())
    .with_health(Health::new(100.0))
    .with_name("Player".to_string())
    .as_player()
    .build();
```

### Projectile Entity

```rust
fn spawn_bullet(world: &mut World, position: Vector3, direction: Vector3) -> EntityId {
    world.spawn()
        .with_transform(Transform::new(position))
        .with_renderable(Renderable::sphere(0.2, Color::YELLOW))
        .with_velocity(Velocity {
            linear: direction.scale_by(20.0),  // 20 units/second
            angular: Vector3::zero(),
        })
        .with_projectile(Projectile::new(25.0))
        .with_lifetime(Lifetime::new(3.0))  // Disappears after 3 seconds
        .build()
}
```

### Enemy with AI

```rust
let enemy_id = world.spawn()
    .with_transform(Transform::new(Vector3::new(10.0, 0.0, 10.0)))
    .with_renderable(Renderable::cube(Vector3::one(), Color::RED))
    .with_velocity(Velocity::default())
    .with_health(Health::new(50.0))
    .with_follow_target(FollowTarget::new(player_id, 3.0, 2.0))
    .with_attack_ability(AttackAbility::new(10.0, 2.0, 1.0))  // 10 dmg, 2m range, 1 atk/sec
    .as_enemy()
    .with_damageable(Damageable)
    .build();
```

### Collectible Item

```rust
let coin_id = world.spawn()
    .with_transform(Transform::new(Vector3::new(5.0, 1.0, 5.0)))
    .with_renderable(Renderable::cylinder(0.5, 0.1, Color::GOLD))
    .with_auto_rotate(AutoRotate::around_y(2.0))  // Spin around Y axis
    .with_collectible(Collectible)
    .build();
```

### Temporary Visual Effect

```rust
fn spawn_explosion(world: &mut World, position: Vector3) {
    world.spawn()
        .with_transform(Transform::new(position))
        .with_renderable(Renderable::sphere(1.0, Color::ORANGE))
        .with_lifetime(Lifetime::new(0.5))
        .with_fade_out(FadeOut::new())
        .with_scale_over_time(ScaleOverTime {
            rate: Vector3::new(4.0, 4.0, 4.0),  // Grow quickly
            min_scale: Vector3::one(),
            max_scale: Vector3::new(3.0, 3.0, 3.0),
        })
        .build();
}
```

### Patrolling Guard

```rust
let waypoints = vec![
    Vector3::new(0.0, 0.0, 0.0),
    Vector3::new(10.0, 0.0, 0.0),
    Vector3::new(10.0, 0.0, 10.0),
    Vector3::new(0.0, 0.0, 10.0),
];

let guard_id = world.spawn()
    .with_transform(Transform::new(waypoints[0]))
    .with_renderable(Renderable::cube(Vector3::one(), Color::PURPLE))
    .with_velocity(Velocity::default())
    .with_patrol_path(PatrolPath::new(waypoints, 3.0, true))
    .build();
```

## Using Systems

### System Setup

```rust
// In your game initialization
let mut movement_system = MovementSystem;
let mut gravity_system = GravitySystem;
let mut lifetime_system = LifetimeSystem::new();
let mut auto_rotate_system = AutoRotateSystem;
let mut fade_out_system = FadeOutSystem;
let player_input_system = PlayerInputSystem;
let render_system = RenderSystem;

// In your game loop
loop {
    let delta_time = rl.get_frame_time();

    // Update systems in order
    player_input_system.update(&mut world, &rl);
    gravity_system.update(&mut world, delta_time);
    movement_system.update(&mut world, delta_time);
    auto_rotate_system.update(&mut world, delta_time);
    fade_out_system.update(&mut world, delta_time);
    lifetime_system.update(&mut world, delta_time);

    // Render
    // ...
}
```

### Custom System Example

```rust
/// System that makes collectibles bob up and down
pub struct BobbingSystem {
    time: f32,
}

impl BobbingSystem {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }
}

impl System for BobbingSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        self.time += delta_time;

        for entity in world.entities_mut() {
            if entity.collectible.is_some() {
                if let Some(transform) = &mut entity.transform {
                    // Bob up and down using sine wave
                    let original_y = transform.position.y;
                    transform.position.y = original_y + (self.time * 2.0).sin() * 0.3;
                }
            }
        }
    }
}
```

## Common Patterns

### Spawning Entities from Player Input

```rust
// In your input handling
if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
    // Find player
    for entity in world.entities() {
        if entity.is_player {
            if let Some(transform) = &entity.transform {
                // Spawn projectile in front of player
                let forward = Vector3::new(0.0, 0.0, -1.0);
                spawn_bullet(&mut world, transform.position, forward);
            }
        }
    }
}
```

### Despawning Entities When Health Reaches Zero

```rust
pub struct DeathOnZeroHealthSystem;

impl System for DeathOnZeroHealthSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        for entity in world.entities_mut() {
            if let Some(health) = &entity.health {
                if !health.is_alive() {
                    // Mark for removal
                    entity.marked_for_death = Some(MarkedForDeath);

                    // Optionally spawn death effect
                    if let Some(transform) = &entity.transform {
                        spawn_explosion(world, transform.position);
                    }
                }
            }
        }
    }
}
```

### Collision Detection Between Entity Types

```rust
pub struct CollectibleSystem;

impl System for CollectibleSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        let mut collected = Vec::new();

        // Find player position
        let player_pos = world.entities()
            .find(|e| e.is_player)
            .and_then(|e| e.transform.as_ref())
            .map(|t| t.position);

        if let Some(player_pos) = player_pos {
            // Check collectibles
            for entity in world.entities() {
                if entity.collectible.is_some() {
                    if let Some(transform) = &entity.transform {
                        let dist = (player_pos - transform.position).length();
                        if dist < 1.0 {  // Collection radius
                            collected.push(entity.id);
                        }
                    }
                }
            }
        }

        // Remove collected items
        for id in collected {
            world.remove_entity(id);
            // Update score, play sound, etc.
        }
    }
}
```

## Complete Examples

### Wave-Based Enemy Spawner

```rust
pub struct WaveSpawner {
    wave: u32,
    time_until_next_wave: f32,
    wave_duration: f32,
}

impl WaveSpawner {
    pub fn new() -> Self {
        Self {
            wave: 0,
            time_until_next_wave: 5.0,
            wave_duration: 10.0,
        }
    }

    pub fn spawn_wave(&mut self, world: &mut World, player_id: EntityId) {
        self.wave += 1;
        let enemy_count = 3 + self.wave * 2;

        for i in 0..enemy_count {
            let angle = (i as f32 / enemy_count as f32) * std::f32::consts::TAU;
            let distance = 15.0;
            let position = Vector3::new(
                angle.cos() * distance,
                0.0,
                angle.sin() * distance,
            );

            world.spawn()
                .with_transform(Transform::new(position))
                .with_renderable(Renderable::cube(Vector3::one(), Color::RED))
                .with_velocity(Velocity::default())
                .with_health(Health::new(20.0 * self.wave as f32))
                .with_follow_target(FollowTarget::new(player_id, 2.0, 3.0))
                .with_gravity(Gravity::earth())
                .as_enemy()
                .with_damageable(Damageable)
                .build();
        }
    }
}

impl System for WaveSpawner {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        self.time_until_next_wave -= delta_time;

        if self.time_until_next_wave <= 0.0 {
            // Find player
            if let Some(player) = world.entities().find(|e| e.is_player) {
                self.spawn_wave(world, player.id);
                self.time_until_next_wave = self.wave_duration;
            }
        }
    }
}
```

### Particle System

```rust
pub struct ParticleEmitter {
    position: Vector3,
    particles_per_second: f32,
    accumulator: f32,
}

impl ParticleEmitter {
    pub fn new(position: Vector3, rate: f32) -> Self {
        Self {
            position,
            particles_per_second: rate,
            accumulator: 0.0,
        }
    }

    pub fn emit_particle(world: &mut World, position: Vector3) {
        let random_vel = Vector3::new(
            (rand::random::<f32>() - 0.5) * 2.0,
            rand::random::<f32>() * 5.0,
            (rand::random::<f32>() - 0.5) * 2.0,
        );

        world.spawn()
            .with_transform(Transform::new(position))
            .with_renderable(Renderable::sphere(0.1, Color::ORANGE))
            .with_velocity(Velocity {
                linear: random_vel,
                angular: Vector3::zero(),
            })
            .with_gravity(Gravity::earth())
            .with_lifetime(Lifetime::new(2.0))
            .with_fade_out(FadeOut::new())
            .build();
    }
}

impl System for ParticleEmitter {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        self.accumulator += delta_time * self.particles_per_second;

        while self.accumulator >= 1.0 {
            Self::emit_particle(world, self.position);
            self.accumulator -= 1.0;
        }
    }
}
```

### Powerup System

```rust
#[derive(Debug, Clone)]
pub enum PowerupType {
    Health,
    Speed,
    Damage,
}

#[derive(Debug, Clone)]
pub struct Powerup {
    pub powerup_type: PowerupType,
    pub duration: Option<f32>,
}

pub struct PowerupSystem;

impl System for PowerupSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        let mut collected = Vec::new();

        // Find player
        let player_pos = world.entities()
            .find(|e| e.is_player)
            .and_then(|e| e.transform.as_ref())
            .map(|t| t.position);

        if let Some(player_pos) = player_pos {
            // Check powerups
            for entity in world.entities() {
                if let Some(powerup) = &entity.powerup {
                    if let Some(transform) = &entity.transform {
                        let dist = (player_pos - transform.position).length();
                        if dist < 1.0 {
                            collected.push((entity.id, powerup.clone()));
                        }
                    }
                }
            }
        }

        // Apply powerups
        for (id, powerup) in collected {
            // Remove powerup entity
            world.remove_entity(id);

            // Apply effect to player
            for entity in world.entities_mut() {
                if entity.is_player {
                    match powerup.powerup_type {
                        PowerupType::Health => {
                            if let Some(health) = &mut entity.health {
                                health.current = health.max;
                            }
                        }
                        PowerupType::Speed => {
                            // Increase speed (would need speed component)
                        }
                        PowerupType::Damage => {
                            // Increase damage (would need damage component)
                        }
                    }
                }
            }
        }
    }
}
```

## Tips and Best Practices

1. **Component Composition**: Combine simple components rather than creating complex ones
   ```rust
   // Good: Combine simple components
   world.spawn()
       .with_velocity(Velocity::default())
       .with_gravity(Gravity::earth())
       .with_drag(Drag::uniform(0.1))
       .build();
   ```

2. **System Ordering Matters**: Physics before rendering, input before AI
   ```rust
   input_system.update(...);
   ai_system.update(...);
   physics_system.update(...);
   movement_system.update(...);
   collision_system.update(...);
   // Then render
   ```

3. **Use Tags for Filtering**: They're efficient and clear
   ```rust
   for entity in world.entities() {
       if entity.is_enemy && entity.health.is_some() {
           // Process enemies with health
       }
   }
   ```

4. **Defer Deletions**: Collect IDs first, remove after iteration
   ```rust
   let to_remove: Vec<EntityId> = world.entities()
       .filter(|e| should_remove(e))
       .map(|e| e.id)
       .collect();

   for id in to_remove {
       world.remove_entity(id);
   }
   ```

5. **Reuse Allocations**: Keep vectors between frames
   ```rust
   pub struct MySystem {
       temp_buffer: Vec<EntityId>,  // Reused each frame
   }
   ```
