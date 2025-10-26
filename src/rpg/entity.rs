/// Core Entity System
/// Provides a simple entity ID system for tracking game objects

use std::collections::HashMap;
use raylib::prelude::*;

/// Unique identifier for entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new(id: u64) -> Self {
        EntityId(id)
    }
}

/// Entity generator for creating unique IDs
pub struct EntityIdGenerator {
    next_id: u64,
}

impl EntityIdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn generate(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        id
    }
}

impl Default for EntityIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Position component - where an entity is in the world
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl From<Vector2> for Position {
    fn from(v: Vector2) -> Self {
        Position::new(v.x, v.y)
    }
}

/// Velocity component - how an entity moves
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn as_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

/// Sprite component - visual representation
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_id: Option<u32>,
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub source_rect: Option<Rectangle>, // For sprite sheets
}

impl Sprite {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            texture_id: None,
            color,
            width,
            height,
            source_rect: None,
        }
    }

    pub fn with_texture(texture_id: u32, width: f32, height: f32) -> Self {
        Self {
            texture_id: Some(texture_id),
            color: Color::WHITE,
            width,
            height,
            source_rect: None,
        }
    }
}

/// Simple component storage system
pub struct World {
    id_generator: EntityIdGenerator,
    positions: HashMap<EntityId, Position>,
    velocities: HashMap<EntityId, Velocity>,
    sprites: HashMap<EntityId, Sprite>,
}

impl World {
    pub fn new() -> Self {
        Self {
            id_generator: EntityIdGenerator::new(),
            positions: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
        }
    }

    /// Create a new entity and return its ID
    pub fn create_entity(&mut self) -> EntityId {
        self.id_generator.generate()
    }

    /// Add a position component to an entity
    pub fn add_position(&mut self, entity: EntityId, position: Position) {
        self.positions.insert(entity, position);
    }

    /// Add a velocity component to an entity
    pub fn add_velocity(&mut self, entity: EntityId, velocity: Velocity) {
        self.velocities.insert(entity, velocity);
    }

    /// Add a sprite component to an entity
    pub fn add_sprite(&mut self, entity: EntityId, sprite: Sprite) {
        self.sprites.insert(entity, sprite);
    }

    /// Get position component
    pub fn get_position(&self, entity: EntityId) -> Option<&Position> {
        self.positions.get(&entity)
    }

    /// Get mutable position component
    pub fn get_position_mut(&mut self, entity: EntityId) -> Option<&mut Position> {
        self.positions.get_mut(&entity)
    }

    /// Get velocity component
    pub fn get_velocity(&self, entity: EntityId) -> Option<&Velocity> {
        self.velocities.get(&entity)
    }

    /// Get mutable velocity component
    pub fn get_velocity_mut(&mut self, entity: EntityId) -> Option<&mut Velocity> {
        self.velocities.get_mut(&entity)
    }

    /// Get sprite component
    pub fn get_sprite(&self, entity: EntityId) -> Option<&Sprite> {
        self.sprites.get(&entity)
    }

    /// Get mutable sprite component
    pub fn get_sprite_mut(&mut self, entity: EntityId) -> Option<&mut Sprite> {
        self.sprites.get_mut(&entity)
    }

    /// Remove an entity and all its components
    pub fn remove_entity(&mut self, entity: EntityId) {
        self.positions.remove(&entity);
        self.velocities.remove(&entity);
        self.sprites.remove(&entity);
    }

    /// Update all entities with velocity - move them by their velocity
    pub fn update_movement(&mut self, delta_time: f32) {
        for (entity_id, velocity) in &self.velocities {
            if let Some(position) = self.positions.get_mut(entity_id) {
                position.x += velocity.x * delta_time;
                position.y += velocity.y * delta_time;
            }
        }
    }

    /// Draw all entities with sprites
    pub fn draw_entities(&self, d: &mut RaylibDrawHandle) {
        for (entity_id, sprite) in &self.sprites {
            if let Some(position) = self.positions.get(entity_id) {
                // Simple rectangle rendering for now
                // TODO: Support texture rendering when texture system is integrated
                d.draw_rectangle(
                    position.x as i32,
                    position.y as i32,
                    sprite.width as i32,
                    sprite.height as i32,
                    sprite.color,
                );
            }
        }
    }

    /// Get all entities with position and sprite (for rendering)
    pub fn entities_with_sprite(&self) -> Vec<(EntityId, &Position, &Sprite)> {
        self.sprites
            .keys()
            .filter_map(|id| {
                self.positions
                    .get(id)
                    .and_then(|pos| self.sprites.get(id).map(|spr| (*id, pos, spr)))
            })
            .collect()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
