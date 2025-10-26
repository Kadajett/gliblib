use std::collections::HashMap;
use super::components::*;
use super::examples::*;

pub type EntityId = usize;

/// Simple entity - just an ID with components
///
/// This struct holds optional components for each entity.
/// Not all entities need all components - only add what you need!
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,

    // Core components
    pub transform: Option<Transform>,
    pub renderable: Option<Renderable>,
    pub velocity: Option<Velocity>,
    pub health: Option<Health>,
    pub name: Option<Name>,
    pub camera: Option<Camera>,

    // Physics components
    pub rigidbody: Option<Rigidbody>,
    pub collider: Option<Collider>,

    // Tag components
    pub is_player: bool,
    pub is_enemy: bool,

    // Example components - Timing
    pub lifetime: Option<Lifetime>,
    pub cooldown: Option<Cooldown>,

    // Example components - Physics
    pub gravity: Option<Gravity>,
    pub bouncy: Option<Bouncy>,
    pub drag: Option<Drag>,

    // Example components - Combat
    pub projectile: Option<Projectile>,
    pub attack_ability: Option<AttackAbility>,

    // Example components - AI
    pub follow_target: Option<FollowTarget>,
    pub patrol_path: Option<PatrolPath>,

    // Example components - Visual Effects
    pub fade_out: Option<FadeOut>,
    pub auto_rotate: Option<AutoRotate>,

    // Example components - Tags
    pub collectible: Option<Collectible>,
    pub obstacle: Option<Obstacle>,
    pub damageable: Option<Damageable>,
    pub marked_for_death: Option<MarkedForDeath>,

    // Example components - Utility
    pub parent: Option<Parent>,
    pub child: Option<Child>,

    // Model component
    pub model: Option<Model>,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            transform: None,
            renderable: None,
            velocity: None,
            health: None,
            name: None,
            camera: None,
            rigidbody: None,
            collider: None,
            is_player: false,
            is_enemy: false,
            lifetime: None,
            cooldown: None,
            gravity: None,
            bouncy: None,
            drag: None,
            projectile: None,
            attack_ability: None,
            follow_target: None,
            patrol_path: None,
            fade_out: None,
            auto_rotate: None,
            collectible: None,
            obstacle: None,
            damageable: None,
            marked_for_death: None,
            parent: None,
            child: None,
            model: None,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.renderable = Some(renderable);
        self
    }

    pub fn with_velocity(mut self, velocity: Velocity) -> Self {
        self.velocity = Some(velocity);
        self
    }

    pub fn with_health(mut self, health: Health) -> Self {
        self.health = Some(health);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(Name(name));
        self
    }

    pub fn as_player(mut self) -> Self {
        self.is_player = true;
        self
    }

    pub fn as_enemy(mut self) -> Self {
        self.is_enemy = true;
        self
    }

    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn with_rigidbody(mut self, rigidbody: Rigidbody) -> Self {
        self.rigidbody = Some(rigidbody);
        self
    }

    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.collider = Some(collider);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.model = Some(model);
        self
    }
}

/// World holds all entities
pub struct World {
    entities: HashMap<EntityId, Entity>,
    next_id: EntityId,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.insert(id, Entity::new(id));
        id
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityId {
        let id = entity.id;
        self.entities.insert(id, entity);
        id
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.next_id = 0;
    }

    /// Builder-style entity creation
    pub fn spawn(&mut self) -> EntityBuilder {
        let id = self.next_id;
        self.next_id += 1;
        EntityBuilder {
            entity: Entity::new(id),
            world: self,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder pattern for creating entities
pub struct EntityBuilder<'a> {
    entity: Entity,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.entity.transform = Some(transform);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.entity.renderable = Some(renderable);
        self
    }

    pub fn with_velocity(mut self, velocity: Velocity) -> Self {
        self.entity.velocity = Some(velocity);
        self
    }

    pub fn with_health(mut self, health: Health) -> Self {
        self.entity.health = Some(health);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.entity.name = Some(Name(name));
        self
    }

    pub fn as_player(mut self) -> Self {
        self.entity.is_player = true;
        self
    }

    pub fn as_enemy(mut self) -> Self {
        self.entity.is_enemy = true;
        self
    }

    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.entity.camera = Some(camera);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.entity.model = Some(model);
        self
    }

    pub fn with_rigidbody(mut self, rigidbody: Rigidbody) -> Self {
        self.entity.rigidbody = Some(rigidbody);
        self
    }

    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.entity.collider = Some(collider);
        self
    }

    pub fn build(self) -> EntityId {
        let id = self.entity.id;
        self.world.entities.insert(id, self.entity);
        id
    }
}
