/// Enemy and AI System
/// Defines enemy types, behaviors, and AI

use super::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Enemy AI behavior types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIBehavior {
    Passive,        // Doesn't attack unless provoked
    Aggressive,     // Attacks on sight
    Patrol,         // Patrols an area
    Guard,          // Guards a specific point
    Flee,           // Runs away from player
    Boss,           // Boss-specific behavior
}

/// Enemy type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyDef {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub base_stats: Stats,
    pub behavior: AIBehavior,
    pub exp_reward: i32,
    pub gold_reward: (i32, i32), // Min, max gold
    pub loot_table: Vec<(ItemId, f32)>, // Item ID, drop chance (0.0 to 1.0)
    pub aggro_range: f32,
    pub patrol_range: f32,
    pub move_speed: f32,
}

impl EnemyDef {
    pub fn new(id: u32, name: &str, level: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            base_stats: Stats::balanced(level),
            behavior: AIBehavior::Aggressive,
            exp_reward: 10 * level,
            gold_reward: (level * 2, level * 5),
            loot_table: Vec::new(),
            aggro_range: 200.0,
            patrol_range: 100.0,
            move_speed: 80.0,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_behavior(mut self, behavior: AIBehavior) -> Self {
        self.behavior = behavior;
        self
    }

    pub fn with_stats(mut self, stats: Stats) -> Self {
        self.base_stats = stats;
        self
    }

    pub fn with_loot(mut self, item_id: ItemId, chance: f32) -> Self {
        self.loot_table.push((item_id, chance));
        self
    }

    pub fn with_aggro_range(mut self, range: f32) -> Self {
        self.aggro_range = range;
        self
    }
}

/// Enemy instance
#[derive(Debug, Clone)]
pub struct Enemy {
    pub entity_id: EntityId,
    pub def_id: u32,
    pub stats: Stats,
    pub behavior: AIBehavior,
    pub state: EnemyState,
    pub spawn_position: Position,
    pub target_entity: Option<EntityId>,
    pub patrol_timer: f32,
    pub status_effects: Vec<TimedStatusEffect>,
}

impl Enemy {
    pub fn from_def(entity_id: EntityId, def: &EnemyDef, position: Position) -> Self {
        let mut stats = def.base_stats.clone();
        stats.restore_full();

        Self {
            entity_id,
            def_id: def.id,
            stats,
            behavior: def.behavior,
            state: EnemyState::Idle,
            spawn_position: position,
            target_entity: None,
            patrol_timer: 0.0,
            status_effects: Vec::new(),
        }
    }

    /// Update enemy AI and state
    pub fn update(&mut self, delta_time: f32, player_pos: &Position, def: &EnemyDef, my_pos: &Position) -> Option<EnemyAction> {
        // Update status effects
        self.update_status_effects(delta_time);

        // Skip AI if stunned or frozen
        if self.has_status_effect(StatusEffect::Stunned) || self.has_status_effect(StatusEffect::Frozen) {
            return None;
        }

        let distance_to_player = my_pos.distance_to(player_pos);

        match self.behavior {
            AIBehavior::Passive => {
                // Only attack if already in combat
                if self.state == EnemyState::Combat && distance_to_player < def.aggro_range * 1.5 {
                    Some(EnemyAction::Attack)
                } else {
                    self.state = EnemyState::Idle;
                    None
                }
            }
            AIBehavior::Aggressive => {
                if distance_to_player < def.aggro_range {
                    self.state = EnemyState::Combat;
                    if distance_to_player > 32.0 { // Attack range
                        Some(EnemyAction::MoveTowards(player_pos.as_vector2()))
                    } else {
                        Some(EnemyAction::Attack)
                    }
                } else {
                    self.state = EnemyState::Idle;
                    // Return to spawn if too far
                    let dist_from_spawn = my_pos.distance_to(&self.spawn_position);
                    if dist_from_spawn > 50.0 {
                        Some(EnemyAction::MoveTowards(self.spawn_position.as_vector2()))
                    } else {
                        None
                    }
                }
            }
            AIBehavior::Patrol => {
                self.patrol_timer -= delta_time;

                if distance_to_player < def.aggro_range {
                    self.state = EnemyState::Combat;
                    if distance_to_player > 32.0 {
                        Some(EnemyAction::MoveTowards(player_pos.as_vector2()))
                    } else {
                        Some(EnemyAction::Attack)
                    }
                } else {
                    self.state = EnemyState::Patrolling;
                    if self.patrol_timer <= 0.0 {
                        self.patrol_timer = 3.0;
                        // Pick random point near spawn
                        let mut rng = rand::thread_rng();
                        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                        let distance = rng.gen_range(0.0..def.patrol_range);
                        let target_x = self.spawn_position.x + angle.cos() * distance;
                        let target_y = self.spawn_position.y + angle.sin() * distance;
                        Some(EnemyAction::MoveTowards(raylib::prelude::Vector2::new(target_x, target_y)))
                    } else {
                        None
                    }
                }
            }
            AIBehavior::Guard => {
                if distance_to_player < def.aggro_range {
                    self.state = EnemyState::Combat;
                    if distance_to_player > 32.0 {
                        Some(EnemyAction::MoveTowards(player_pos.as_vector2()))
                    } else {
                        Some(EnemyAction::Attack)
                    }
                } else {
                    // Return to guard position
                    let dist_from_spawn = my_pos.distance_to(&self.spawn_position);
                    if dist_from_spawn > 10.0 {
                        Some(EnemyAction::MoveTowards(self.spawn_position.as_vector2()))
                    } else {
                        self.state = EnemyState::Idle;
                        None
                    }
                }
            }
            AIBehavior::Flee => {
                if distance_to_player < def.aggro_range {
                    self.state = EnemyState::Fleeing;
                    // Run away from player
                    let dx = my_pos.x - player_pos.x;
                    let dy = my_pos.y - player_pos.y;
                    let flee_x = my_pos.x + dx;
                    let flee_y = my_pos.y + dy;
                    Some(EnemyAction::MoveTowards(raylib::prelude::Vector2::new(flee_x, flee_y)))
                } else {
                    self.state = EnemyState::Idle;
                    None
                }
            }
            AIBehavior::Boss => {
                // Boss AI - more complex
                if distance_to_player < def.aggro_range * 2.0 {
                    self.state = EnemyState::Combat;

                    let health_percent = self.stats.health_percent();

                    // Change tactics based on health
                    if health_percent < 0.3 {
                        // Enraged at low health
                        if distance_to_player > 32.0 {
                            Some(EnemyAction::MoveTowards(player_pos.as_vector2()))
                        } else {
                            Some(EnemyAction::Attack)
                        }
                    } else if distance_to_player < 100.0 {
                        Some(EnemyAction::Attack)
                    } else {
                        Some(EnemyAction::MoveTowards(player_pos.as_vector2()))
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Update status effects
    fn update_status_effects(&mut self, delta_time: f32) {
        let mut effects_to_remove = Vec::new();

        for (i, effect) in self.status_effects.iter_mut().enumerate() {
            if effect.update(delta_time) {
                effects_to_remove.push(i);
            }

            if effect.should_tick(delta_time) {
                match effect.effect {
                    StatusEffect::Poisoned | StatusEffect::Burning => {
                        self.stats.take_damage(effect.power);
                    }
                    StatusEffect::Regenerating => {
                        self.stats.heal(effect.power);
                    }
                    _ => {}
                }
            }
        }

        for i in effects_to_remove.into_iter().rev() {
            self.status_effects.remove(i);
        }
    }

    /// Check if enemy has a status effect
    fn has_status_effect(&self, effect: StatusEffect) -> bool {
        self.status_effects.iter().any(|e| e.effect == effect)
    }

    /// Add a status effect
    pub fn add_status_effect(&mut self, effect: TimedStatusEffect) {
        self.status_effects.push(effect);
    }

    /// Take damage and return if still alive
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.stats.take_damage(damage)
    }

    /// Check if enemy is dead
    pub fn is_dead(&self) -> bool {
        self.stats.is_dead()
    }
}

/// Enemy state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyState {
    Idle,
    Patrolling,
    Combat,
    Fleeing,
}

/// Actions an enemy can take
#[derive(Debug, Clone)]
pub enum EnemyAction {
    MoveTowards(raylib::prelude::Vector2),
    Attack,
}

/// Enemy database
pub struct EnemyDatabase {
    enemies: std::collections::HashMap<u32, EnemyDef>,
}

impl EnemyDatabase {
    pub fn new() -> Self {
        Self {
            enemies: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, enemy: EnemyDef) {
        self.enemies.insert(enemy.id, enemy);
    }

    pub fn get(&self, id: u32) -> Option<&EnemyDef> {
        self.enemies.get(&id)
    }

    /// Create database with starter enemies
    pub fn with_starter_enemies() -> Self {
        let mut db = Self::new();

        // Slime - Easy enemy
        db.register(
            EnemyDef::new(1, "Slime", 1)
                .with_description("A bouncy slime creature.")
                .with_behavior(AIBehavior::Passive)
                .with_stats(Stats::new(5, 5, 5, 10, 5))
                .with_loot(ItemId(100), 0.3) // Health potion 30% drop
                .with_aggro_range(100.0),
        );

        // Goblin - Basic aggressive enemy
        db.register(
            EnemyDef::new(2, "Goblin", 2)
                .with_description("A small, aggressive goblin.")
                .with_behavior(AIBehavior::Aggressive)
                .with_stats(Stats::new(8, 10, 5, 8, 7))
                .with_loot(ItemId(100), 0.2)
                .with_loot(ItemId(1), 0.1), // Rusty sword 10% drop
        );

        // Skeleton Warrior - Patrolling enemy
        db.register(
            EnemyDef::new(3, "Skeleton Warrior", 3)
                .with_description("An undead warrior bound to patrol.")
                .with_behavior(AIBehavior::Patrol)
                .with_stats(Stats::new(12, 8, 5, 10, 5))
                .with_loot(ItemId(2), 0.15) // Iron sword 15% drop
                .with_loot(ItemId(10), 0.2), // Leather armor
        );

        // Dark Mage - Aggressive spellcaster
        db.register(
            EnemyDef::new(4, "Dark Mage", 4)
                .with_description("A mage corrupted by dark magic.")
                .with_behavior(AIBehavior::Aggressive)
                .with_stats(Stats::new(5, 8, 18, 8, 10))
                .with_loot(ItemId(3), 0.1) // Magic staff 10% drop
                .with_loot(ItemId(101), 0.3), // Mana potion
        );

        // Treasure Goblin - Fleeing enemy with good loot
        db.register(
            EnemyDef::new(5, "Treasure Goblin", 1)
                .with_description("A goblin carrying treasure! Don't let it escape!")
                .with_behavior(AIBehavior::Flee)
                .with_stats(Stats::new(3, 20, 3, 5, 30))
                .with_loot(ItemId(200), 1.0) // Always drops quest item
                .with_aggro_range(300.0),
        );

        // Boss: Ancient Guardian
        db.register(
            EnemyDef::new(100, "Ancient Guardian", 10)
                .with_description("A powerful guardian of ancient ruins.")
                .with_behavior(AIBehavior::Boss)
                .with_stats(Stats::new(25, 15, 15, 30, 10))
                .with_loot(ItemId(3), 0.8) // Magic staff 80% drop
                .with_loot(ItemId(11), 0.8) // Iron helmet
                .with_aggro_range(400.0),
        );

        db
    }
}

impl Default for EnemyDatabase {
    fn default() -> Self {
        Self::new()
    }
}
