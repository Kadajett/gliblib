/// Character Stats System
/// Defines stats for players and enemies

use serde::{Deserialize, Serialize};

/// Core character statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    // Core attributes
    pub strength: i32,     // Physical damage and carrying capacity
    pub dexterity: i32,    // Attack speed and dodge chance
    pub intelligence: i32, // Magic damage and mana
    pub vitality: i32,     // Health and defense
    pub luck: i32,         // Critical chance and loot quality

    // Derived stats (calculated from attributes)
    pub max_health: i32,
    pub current_health: i32,
    pub max_mana: i32,
    pub current_mana: i32,
    pub physical_damage: i32,
    pub magic_damage: i32,
    pub defense: i32,
    pub dodge_chance: f32,  // 0.0 to 1.0
    pub crit_chance: f32,   // 0.0 to 1.0
    pub crit_multiplier: f32,
    pub move_speed: f32,
}

impl Stats {
    /// Create base stats (level 1)
    pub fn new(strength: i32, dexterity: i32, intelligence: i32, vitality: i32, luck: i32) -> Self {
        let mut stats = Self {
            strength,
            dexterity,
            intelligence,
            vitality,
            luck,
            max_health: 0,
            current_health: 0,
            max_mana: 0,
            current_mana: 0,
            physical_damage: 0,
            magic_damage: 0,
            defense: 0,
            dodge_chance: 0.0,
            crit_chance: 0.0,
            crit_multiplier: 1.5,
            move_speed: 100.0,
        };
        stats.recalculate();
        stats
    }

    /// Create balanced stats
    pub fn balanced(level: i32) -> Self {
        let base = 10 + level;
        Self::new(base, base, base, base, base)
    }

    /// Create warrior-focused stats
    pub fn warrior(level: i32) -> Self {
        let base = 10 + level;
        Self::new(base * 2, base, base / 2, base + 5, base)
    }

    /// Create mage-focused stats
    pub fn mage(level: i32) -> Self {
        let base = 10 + level;
        Self::new(base / 2, base, base * 2, base, base)
    }

    /// Create rogue-focused stats
    pub fn rogue(level: i32) -> Self {
        let base = 10 + level;
        Self::new(base, base * 2, base, base, base + 5)
    }

    /// Recalculate all derived stats based on attributes
    pub fn recalculate(&mut self) {
        // Health: 10 HP per vitality
        self.max_health = 50 + (self.vitality * 10);

        // Mana: 5 MP per intelligence
        self.max_mana = 20 + (self.intelligence * 5);

        // Physical damage: 2 damage per strength
        self.physical_damage = self.strength * 2;

        // Magic damage: 3 damage per intelligence
        self.magic_damage = self.intelligence * 3;

        // Defense: 1 defense per vitality
        self.defense = self.vitality;

        // Dodge chance: 1% per dexterity (max 75%)
        self.dodge_chance = (self.dexterity as f32 * 0.01).min(0.75);

        // Crit chance: 2% per luck (max 50%)
        self.crit_chance = (self.luck as f32 * 0.02).min(0.50);

        // Move speed: 100 + (0.5 per dexterity)
        self.move_speed = 100.0 + (self.dexterity as f32 * 0.5);

        // Cap current health/mana at max if needed
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }
        if self.current_mana > self.max_mana {
            self.current_mana = self.max_mana;
        }
    }

    /// Restore to full health and mana
    pub fn restore_full(&mut self) {
        self.current_health = self.max_health;
        self.current_mana = self.max_mana;
    }

    /// Take damage (returns true if still alive)
    pub fn take_damage(&mut self, damage: i32) -> bool {
        let actual_damage = (damage - self.defense).max(1); // Always deal at least 1 damage
        self.current_health = (self.current_health - actual_damage).max(0);
        self.current_health > 0
    }

    /// Heal health (returns amount actually healed)
    pub fn heal(&mut self, amount: i32) -> i32 {
        let old_health = self.current_health;
        self.current_health = (self.current_health + amount).min(self.max_health);
        self.current_health - old_health
    }

    /// Use mana (returns true if enough mana available)
    pub fn use_mana(&mut self, amount: i32) -> bool {
        if self.current_mana >= amount {
            self.current_mana -= amount;
            true
        } else {
            false
        }
    }

    /// Restore mana (returns amount actually restored)
    pub fn restore_mana(&mut self, amount: i32) -> i32 {
        let old_mana = self.current_mana;
        self.current_mana = (self.current_mana + amount).min(self.max_mana);
        self.current_mana - old_mana
    }

    /// Check if alive
    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }

    /// Check if dead
    pub fn is_dead(&self) -> bool {
        self.current_health <= 0
    }

    /// Get health percentage (0.0 to 1.0)
    pub fn health_percent(&self) -> f32 {
        if self.max_health == 0 {
            0.0
        } else {
            self.current_health as f32 / self.max_health as f32
        }
    }

    /// Get mana percentage (0.0 to 1.0)
    pub fn mana_percent(&self) -> f32 {
        if self.max_mana == 0 {
            0.0
        } else {
            self.current_mana as f32 / self.max_mana as f32
        }
    }
}

/// Character level and experience system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub current_level: i32,
    pub current_exp: i32,
    pub exp_to_next_level: i32,
}

impl Level {
    pub fn new() -> Self {
        Self {
            current_level: 1,
            current_exp: 0,
            exp_to_next_level: Self::calculate_exp_needed(1),
        }
    }

    /// Calculate XP needed for next level (exponential scaling)
    fn calculate_exp_needed(level: i32) -> i32 {
        100 + (level * level * 50)
    }

    /// Add experience and check for level up
    /// Returns Some(new_level) if leveled up, None otherwise
    pub fn add_exp(&mut self, exp: i32) -> Option<i32> {
        self.current_exp += exp;

        if self.current_exp >= self.exp_to_next_level {
            self.current_exp -= self.exp_to_next_level;
            self.current_level += 1;
            self.exp_to_next_level = Self::calculate_exp_needed(self.current_level);
            Some(self.current_level)
        } else {
            None
        }
    }

    /// Get experience progress percentage (0.0 to 1.0)
    pub fn exp_percent(&self) -> f32 {
        self.current_exp as f32 / self.exp_to_next_level as f32
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}

/// Status effects that can be applied to characters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffect {
    Poisoned,
    Burning,
    Frozen,
    Stunned,
    Blessed,
    Cursed,
    Hasted,
    Slowed,
    Invulnerable,
    Regenerating,
}

/// Timed status effect with duration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedStatusEffect {
    pub effect: StatusEffect,
    pub duration: f32,     // Seconds remaining
    pub tick_interval: f32, // How often the effect triggers
    pub tick_timer: f32,   // Timer for next tick
    pub power: i32,        // Effect strength (e.g., damage per tick)
}

impl TimedStatusEffect {
    pub fn new(effect: StatusEffect, duration: f32, power: i32) -> Self {
        let tick_interval = match effect {
            StatusEffect::Poisoned | StatusEffect::Burning | StatusEffect::Regenerating => 1.0,
            _ => 0.0,
        };

        Self {
            effect,
            duration,
            tick_interval,
            tick_timer: tick_interval,
            power,
        }
    }

    /// Update the effect timer, returns true if effect should be removed
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.duration -= delta_time;
        self.duration <= 0.0
    }

    /// Check if effect should tick, returns true and resets timer if so
    pub fn should_tick(&mut self, delta_time: f32) -> bool {
        if self.tick_interval > 0.0 {
            self.tick_timer -= delta_time;
            if self.tick_timer <= 0.0 {
                self.tick_timer = self.tick_interval;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_calculation() {
        let mut stats = Stats::balanced(1);
        assert_eq!(stats.max_health, 50 + (11 * 10));
        assert!(stats.is_alive());

        stats.take_damage(50);
        assert!(stats.is_alive());

        stats.take_damage(1000);
        assert!(stats.is_dead());
    }

    #[test]
    fn test_level_up() {
        let mut level = Level::new();
        assert_eq!(level.current_level, 1);

        let result = level.add_exp(150);
        assert_eq!(result, Some(2));
        assert_eq!(level.current_level, 2);
    }
}
