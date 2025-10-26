/// Skill and Ability System
/// Defines player skills, abilities, and their effects

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique skill identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillId(pub u32);

/// Skill types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillType {
    Active,  // Must be manually activated
    Passive, // Always active
}

/// Target type for skills
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TargetType {
    SelfTarget,
    SingleEnemy,
    AllEnemies,
    AreaOfEffect(f32), // Radius
}

/// Skill effect types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillEffect {
    Damage {
        physical: i32,
        magical: i32,
    },
    Heal(i32),
    ApplyStatusEffect {
        effect: StatusEffect,
        duration: f32,
        power: i32,
    },
    BuffStats {
        mods: StatModifiers,
        duration: f32,
    },
    Teleport {
        range: f32,
    },
}

/// Skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    pub id: SkillId,
    pub name: String,
    pub description: String,
    pub skill_type: SkillType,
    pub target_type: TargetType,
    pub mana_cost: i32,
    pub cooldown: f32,
    pub required_level: i32,
    pub effects: Vec<SkillEffect>,
}

impl SkillDef {
    pub fn new(id: u32, name: &str, skill_type: SkillType) -> Self {
        Self {
            id: SkillId(id),
            name: name.to_string(),
            description: String::new(),
            skill_type,
            target_type: TargetType::SelfTarget,
            mana_cost: 0,
            cooldown: 0.0,
            required_level: 1,
            effects: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_target(mut self, target: TargetType) -> Self {
        self.target_type = target;
        self
    }

    pub fn with_mana_cost(mut self, cost: i32) -> Self {
        self.mana_cost = cost;
        self
    }

    pub fn with_cooldown(mut self, cooldown: f32) -> Self {
        self.cooldown = cooldown;
        self
    }

    pub fn with_required_level(mut self, level: i32) -> Self {
        self.required_level = level;
        self
    }

    pub fn with_effect(mut self, effect: SkillEffect) -> Self {
        self.effects.push(effect);
        self
    }
}

/// Skill instance tracking cooldowns
#[derive(Debug, Clone)]
pub struct SkillInstance {
    pub skill_id: SkillId,
    pub current_cooldown: f32,
}

impl SkillInstance {
    pub fn new(skill_id: SkillId) -> Self {
        Self {
            skill_id,
            current_cooldown: 0.0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.current_cooldown <= 0.0
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.current_cooldown > 0.0 {
            self.current_cooldown -= delta_time;
        }
    }

    pub fn trigger(&mut self, cooldown: f32) {
        self.current_cooldown = cooldown;
    }
}

/// Skill database
pub struct SkillDatabase {
    skills: HashMap<SkillId, SkillDef>,
}

impl SkillDatabase {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    pub fn register(&mut self, skill: SkillDef) {
        self.skills.insert(skill.id, skill);
    }

    pub fn get(&self, id: SkillId) -> Option<&SkillDef> {
        self.skills.get(&id)
    }

    /// Create database with starter skills
    pub fn with_starter_skills() -> Self {
        let mut db = Self::new();

        // Warrior skills
        db.register(
            SkillDef::new(1, "Power Strike", SkillType::Active)
                .with_description("A powerful melee attack dealing 200% physical damage.")
                .with_target(TargetType::SingleEnemy)
                .with_mana_cost(10)
                .with_cooldown(3.0)
                .with_effect(SkillEffect::Damage {
                    physical: 20,
                    magical: 0,
                }),
        );

        db.register(
            SkillDef::new(2, "Whirlwind", SkillType::Active)
                .with_description("Spin attack hitting all nearby enemies.")
                .with_target(TargetType::AreaOfEffect(100.0))
                .with_mana_cost(20)
                .with_cooldown(8.0)
                .with_required_level(3)
                .with_effect(SkillEffect::Damage {
                    physical: 15,
                    magical: 0,
                }),
        );

        // Mage skills
        db.register(
            SkillDef::new(10, "Fireball", SkillType::Active)
                .with_description("Launch a ball of fire at an enemy.")
                .with_target(TargetType::SingleEnemy)
                .with_mana_cost(15)
                .with_cooldown(2.0)
                .with_effect(SkillEffect::Damage {
                    physical: 0,
                    magical: 30,
                })
                .with_effect(SkillEffect::ApplyStatusEffect {
                    effect: StatusEffect::Burning,
                    duration: 5.0,
                    power: 5,
                }),
        );

        db.register(
            SkillDef::new(11, "Ice Lance", SkillType::Active)
                .with_description("Freeze an enemy in place.")
                .with_target(TargetType::SingleEnemy)
                .with_mana_cost(20)
                .with_cooldown(10.0)
                .with_required_level(4)
                .with_effect(SkillEffect::Damage {
                    physical: 0,
                    magical: 25,
                })
                .with_effect(SkillEffect::ApplyStatusEffect {
                    effect: StatusEffect::Frozen,
                    duration: 3.0,
                    power: 0,
                }),
        );

        db.register(
            SkillDef::new(12, "Meteor Storm", SkillType::Active)
                .with_description("Call down meteors on all enemies.")
                .with_target(TargetType::AllEnemies)
                .with_mana_cost(40)
                .with_cooldown(15.0)
                .with_required_level(7)
                .with_effect(SkillEffect::Damage {
                    physical: 0,
                    magical: 50,
                }),
        );

        // Rogue skills
        db.register(
            SkillDef::new(20, "Backstab", SkillType::Active)
                .with_description("Deal massive damage from behind.")
                .with_target(TargetType::SingleEnemy)
                .with_mana_cost(15)
                .with_cooldown(5.0)
                .with_effect(SkillEffect::Damage {
                    physical: 40,
                    magical: 0,
                }),
        );

        db.register(
            SkillDef::new(21, "Shadow Step", SkillType::Active)
                .with_description("Teleport a short distance.")
                .with_target(TargetType::SelfTarget)
                .with_mana_cost(10)
                .with_cooldown(6.0)
                .with_required_level(3)
                .with_effect(SkillEffect::Teleport { range: 150.0 }),
        );

        db.register(
            SkillDef::new(22, "Poison Strike", SkillType::Active)
                .with_description("Attack that poisons the target.")
                .with_target(TargetType::SingleEnemy)
                .with_mana_cost(12)
                .with_cooldown(4.0)
                .with_required_level(5)
                .with_effect(SkillEffect::Damage {
                    physical: 15,
                    magical: 0,
                })
                .with_effect(SkillEffect::ApplyStatusEffect {
                    effect: StatusEffect::Poisoned,
                    duration: 10.0,
                    power: 3,
                }),
        );

        // Universal skills
        db.register(
            SkillDef::new(100, "Heal", SkillType::Active)
                .with_description("Restore health.")
                .with_target(TargetType::SelfTarget)
                .with_mana_cost(20)
                .with_cooldown(10.0)
                .with_effect(SkillEffect::Heal(50)),
        );

        // Passive skills
        db.register(
            SkillDef::new(200, "Swift Feet", SkillType::Passive)
                .with_description("Permanently increase movement speed.")
                .with_required_level(2)
                .with_effect(SkillEffect::BuffStats {
                    mods: StatModifiers {
                        move_speed: 20.0,
                        ..Default::default()
                    },
                    duration: 0.0, // Permanent
                }),
        );

        db.register(
            SkillDef::new(201, "Iron Skin", SkillType::Passive)
                .with_description("Permanently increase defense.")
                .with_required_level(3)
                .with_effect(SkillEffect::BuffStats {
                    mods: StatModifiers {
                        defense: 5,
                        ..Default::default()
                    },
                    duration: 0.0,
                }),
        );

        db.register(
            SkillDef::new(202, "Critical Mind", SkillType::Passive)
                .with_description("Permanently increase critical hit chance.")
                .with_required_level(4)
                .with_effect(SkillEffect::BuffStats {
                    mods: StatModifiers {
                        crit_chance: 0.1,
                        ..Default::default()
                    },
                    duration: 0.0,
                }),
        );

        db
    }
}

impl Default for SkillDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Player's skill manager
pub struct PlayerSkills {
    learned_skills: HashMap<SkillId, SkillInstance>,
}

impl PlayerSkills {
    pub fn new() -> Self {
        Self {
            learned_skills: HashMap::new(),
        }
    }

    /// Learn a new skill
    pub fn learn_skill(&mut self, skill_id: SkillId) {
        self.learned_skills
            .insert(skill_id, SkillInstance::new(skill_id));
    }

    /// Check if a skill is learned
    pub fn has_skill(&self, skill_id: SkillId) -> bool {
        self.learned_skills.contains_key(&skill_id)
    }

    /// Check if a skill is ready to use
    pub fn is_skill_ready(&self, skill_id: SkillId) -> bool {
        self.learned_skills
            .get(&skill_id)
            .map(|s| s.is_ready())
            .unwrap_or(false)
    }

    /// Use a skill (trigger cooldown)
    pub fn use_skill(&mut self, skill_id: SkillId, skill_def: &SkillDef) -> bool {
        if let Some(instance) = self.learned_skills.get_mut(&skill_id) {
            if instance.is_ready() {
                instance.trigger(skill_def.cooldown);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Update all skill cooldowns
    pub fn update(&mut self, delta_time: f32) {
        for instance in self.learned_skills.values_mut() {
            instance.update(delta_time);
        }
    }

    /// Get all learned skills
    pub fn get_all_skills(&self) -> Vec<SkillId> {
        self.learned_skills.keys().copied().collect()
    }
}

impl Default for PlayerSkills {
    fn default() -> Self {
        Self::new()
    }
}
