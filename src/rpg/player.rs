/// Player Character System
/// Manages the player's stats, inventory, equipment, and progression

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Player character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub entity_id: Option<EntityId>,
    pub name: String,
    pub stats: Stats,
    pub level: Level,
    pub inventory: Inventory,
    pub equipment: Equipment,
    pub skills: Vec<SkillId>,
    pub status_effects: Vec<TimedStatusEffect>,
    pub quest_flags: HashMap<String, bool>,
    pub current_checkpoint: String,
}

impl Player {
    /// Create a new player with a given class
    pub fn new(name: &str, class: PlayerClass) -> Self {
        let stats = match class {
            PlayerClass::Warrior => Stats::warrior(1),
            PlayerClass::Mage => Stats::mage(1),
            PlayerClass::Rogue => Stats::rogue(1),
            PlayerClass::Balanced => Stats::balanced(1),
        };

        let mut stats = stats;
        stats.restore_full();

        Self {
            entity_id: None,
            name: name.to_string(),
            stats,
            level: Level::new(),
            inventory: Inventory::new(20), // 20 inventory slots
            equipment: Equipment::new(),
            skills: Vec::new(),
            status_effects: Vec::new(),
            quest_flags: HashMap::new(),
            current_checkpoint: "start".to_string(),
        }
    }

    /// Update player state (status effects, etc.)
    pub fn update(&mut self, delta_time: f32) {
        // Update status effects
        let mut effects_to_remove = Vec::new();

        for (i, effect) in self.status_effects.iter_mut().enumerate() {
            if effect.update(delta_time) {
                effects_to_remove.push(i);
            }

            // Apply ticking effects
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

        // Remove expired effects (in reverse to maintain indices)
        for i in effects_to_remove.into_iter().rev() {
            self.status_effects.remove(i);
        }
    }

    /// Add experience and handle level ups
    pub fn add_exp(&mut self, exp: i32) -> Option<i32> {
        if let Some(new_level) = self.level.add_exp(exp) {
            // Level up! Grant stat increases
            self.on_level_up(new_level);
            Some(new_level)
        } else {
            None
        }
    }

    /// Handle level up - increase stats
    fn on_level_up(&mut self, new_level: i32) {
        // Grant 5 stat points per level (distributed based on class tendencies)
        // For now, just give balanced increases
        self.stats.strength += 1;
        self.stats.dexterity += 1;
        self.stats.intelligence += 1;
        self.stats.vitality += 1;
        self.stats.luck += 1;

        self.stats.recalculate();
        self.stats.restore_full(); // Fully heal on level up

        println!("Level up! Now level {}", new_level);
    }

    /// Equip an item from inventory
    pub fn equip_item(
        &mut self,
        inventory_slot: usize,
        item_db: &ItemDatabase,
    ) -> Result<(), String> {
        // Get item ID first
        let item_id = self
            .inventory
            .get_item(inventory_slot)
            .ok_or("No item in that slot")?
            .item_id;

        let item_def = item_db
            .get(item_id)
            .ok_or("Item not found in database")?;

        // Check level requirement
        if self.level.current_level < item_def.required_level {
            return Err(format!(
                "Level {} required to equip {}",
                item_def.required_level, item_def.name
            ));
        }

        let equip_slot = item_def.equip_slot.ok_or("Item is not equippable")?;

        // If something is already equipped, unequip it first
        if let Some(old_item_id) = self.equipment.get_equipped(equip_slot) {
            if let Some(old_item_def) = item_db.get(old_item_id) {
                self.inventory.add_item(old_item_def, 1);
            }
        }

        // Remove from inventory and equip
        self.inventory.remove_item(inventory_slot, 1);
        self.equipment.equip(equip_slot, item_id);

        // Recalculate stats with new equipment
        self.recalculate_stats_with_equipment(item_db);

        Ok(())
    }

    /// Unequip an item and put it back in inventory
    pub fn unequip_item(
        &mut self,
        slot: EquipSlot,
        item_db: &ItemDatabase,
    ) -> Result<(), String> {
        let item_id = self
            .equipment
            .get_equipped(slot)
            .ok_or("No item equipped in that slot")?;

        let item_def = item_db.get(item_id).ok_or("Item not found in database")?;

        if !self.inventory.has_space() {
            return Err("Inventory is full".to_string());
        }

        self.equipment.unequip(slot);
        self.inventory.add_item(item_def, 1);

        // Recalculate stats without this equipment
        self.recalculate_stats_with_equipment(item_db);

        Ok(())
    }

    /// Recalculate stats including equipment bonuses
    fn recalculate_stats_with_equipment(&mut self, item_db: &ItemDatabase) {
        // First recalculate base stats
        self.stats.recalculate();

        // Then apply equipment bonuses
        for (_slot, item_id) in self.equipment.all_equipped() {
            if let Some(item_def) = item_db.get(item_id) {
                self.apply_stat_modifiers(&item_def.stat_mods);
            }
        }
    }

    /// Apply stat modifiers from equipment
    fn apply_stat_modifiers(&mut self, mods: &StatModifiers) {
        self.stats.strength += mods.strength;
        self.stats.dexterity += mods.dexterity;
        self.stats.intelligence += mods.intelligence;
        self.stats.vitality += mods.vitality;
        self.stats.luck += mods.luck;
        self.stats.max_health += mods.max_health;
        self.stats.max_mana += mods.max_mana;
        self.stats.physical_damage += mods.physical_damage;
        self.stats.magic_damage += mods.magic_damage;
        self.stats.defense += mods.defense;
        self.stats.dodge_chance += mods.dodge_chance;
        self.stats.crit_chance += mods.crit_chance;
        self.stats.move_speed += mods.move_speed;
    }

    /// Use a consumable item
    pub fn use_consumable(
        &mut self,
        inventory_slot: usize,
        item_db: &ItemDatabase,
    ) -> Result<String, String> {
        // Get item ID and validate first
        let item_id = self
            .inventory
            .get_item(inventory_slot)
            .ok_or("No item in that slot")?
            .item_id;

        let item_def = item_db
            .get(item_id)
            .ok_or("Item not found in database")?;

        if item_def.item_type != ItemType::Consumable {
            return Err("Item is not consumable".to_string());
        }

        // Apply consumable effects based on item ID
        let result = match item_def.id.0 {
            100 => {
                // Health Potion
                let healed = self.stats.heal(50);
                format!("Restored {} HP", healed)
            }
            101 => {
                // Mana Potion
                let restored = self.stats.restore_mana(30);
                format!("Restored {} MP", restored)
            }
            _ => "Used item".to_string(),
        };

        // Remove one from stack
        self.inventory.remove_item(inventory_slot, 1);

        Ok(result)
    }

    /// Add a status effect
    pub fn add_status_effect(&mut self, effect: TimedStatusEffect) {
        self.status_effects.push(effect);
    }

    /// Check if player has a status effect
    pub fn has_status_effect(&self, effect: StatusEffect) -> bool {
        self.status_effects.iter().any(|e| e.effect == effect)
    }

    /// Set a quest flag
    pub fn set_quest_flag(&mut self, flag: &str, value: bool) {
        self.quest_flags.insert(flag.to_string(), value);
    }

    /// Check a quest flag
    pub fn has_quest_flag(&self, flag: &str) -> bool {
        self.quest_flags.get(flag).copied().unwrap_or(false)
    }

    /// Save checkpoint
    pub fn save_checkpoint(&mut self, checkpoint_name: &str) {
        self.current_checkpoint = checkpoint_name.to_string();
    }
}

/// Player class archetypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerClass {
    Warrior,
    Mage,
    Rogue,
    Balanced,
}

impl PlayerClass {
    pub fn name(&self) -> &str {
        match self {
            PlayerClass::Warrior => "Warrior",
            PlayerClass::Mage => "Mage",
            PlayerClass::Rogue => "Rogue",
            PlayerClass::Balanced => "Adventurer",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            PlayerClass::Warrior => "High strength and vitality. Excels in melee combat.",
            PlayerClass::Mage => "High intelligence. Master of magic and spells.",
            PlayerClass::Rogue => "High dexterity and luck. Quick and deadly.",
            PlayerClass::Balanced => "Balanced stats. Jack of all trades.",
        }
    }
}
