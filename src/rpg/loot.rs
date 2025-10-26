/// Loot and Treasure System
/// Chest spawning, loot generation, and reward systems

use super::*;
use rand::Rng;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

/// Types of treasure containers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChestType {
    Wooden,
    Iron,
    Golden,
    Legendary,
}

impl ChestType {
    /// Get the quality multiplier for loot
    pub fn quality_multiplier(&self) -> f32 {
        match self {
            ChestType::Wooden => 1.0,
            ChestType::Iron => 1.5,
            ChestType::Golden => 2.5,
            ChestType::Legendary => 5.0,
        }
    }

    /// Get base gold range
    pub fn gold_range(&self) -> (i32, i32) {
        match self {
            ChestType::Wooden => (5, 20),
            ChestType::Iron => (20, 50),
            ChestType::Golden => (100, 200),
            ChestType::Legendary => (500, 1000),
        }
    }

    /// Get number of items to drop
    pub fn item_count_range(&self) -> (u32, u32) {
        match self {
            ChestType::Wooden => (1, 2),
            ChestType::Iron => (2, 3),
            ChestType::Golden => (3, 5),
            ChestType::Legendary => (5, 10),
        }
    }

    /// Get color for rendering
    pub fn color(&self) -> Color {
        match self {
            ChestType::Wooden => Color::new(139, 69, 19, 255),
            ChestType::Iron => Color::GRAY,
            ChestType::Golden => Color::GOLD,
            ChestType::Legendary => Color::PURPLE,
        }
    }
}

/// Treasure chest instance
#[derive(Debug, Clone)]
pub struct Chest {
    pub entity_id: EntityId,
    pub chest_type: ChestType,
    pub position: Position,
    pub is_open: bool,
    pub contents: Vec<ItemStack>,
    pub gold: i32,
}

impl Chest {
    pub fn new(entity_id: EntityId, chest_type: ChestType, position: Position) -> Self {
        Self {
            entity_id,
            chest_type,
            position,
            is_open: false,
            contents: Vec::new(),
            gold: 0,
        }
    }

    /// Generate loot for this chest
    pub fn generate_loot(&mut self, _item_db: &ItemDatabase, player_level: i32) {
        let mut rng = rand::thread_rng();

        // Generate gold
        let (min_gold, max_gold) = self.chest_type.gold_range();
        self.gold = rng.gen_range(min_gold..=max_gold);

        // Generate items
        let (min_items, max_items) = self.chest_type.item_count_range();
        let item_count = rng.gen_range(min_items..=max_items);

        for _ in 0..item_count {
            if let Some(item_stack) = self.generate_random_item(_item_db, player_level, &mut rng) {
                self.contents.push(item_stack);
            }
        }
    }

    /// Generate a random item appropriate for player level
    fn generate_random_item(
        &self,
        _item_db: &ItemDatabase,
        _player_level: i32,
        rng: &mut impl Rng,
    ) -> Option<ItemStack> {
        // For starter implementation, just pick from available items
        // In a full game, you'd have a weighted loot table

        let quality = self.chest_type.quality_multiplier();

        // Determine rarity based on chest quality
        let rarity = if rng.r#gen::<f32>() < quality * 0.1 {
            Rarity::Legendary
        } else if rng.r#gen::<f32>() < quality * 0.2 {
            Rarity::Epic
        } else if rng.r#gen::<f32>() < quality * 0.35 {
            Rarity::Rare
        } else if rng.r#gen::<f32>() < quality * 0.5 {
            Rarity::Uncommon
        } else {
            Rarity::Common
        };

        // Pick a random item type
        let item_type = match rng.gen_range(0..5) {
            0 => ItemType::Weapon,
            1 => ItemType::Armor,
            2 => ItemType::Accessory,
            3 => ItemType::Consumable,
            _ => ItemType::Material,
        };

        // For demonstration, return health/mana potions for consumables
        // and some starter items for equipment
        match item_type {
            ItemType::Consumable => {
                let item_id = if rng.r#gen::<bool>() {
                    ItemId(100) // Health potion
                } else {
                    ItemId(101) // Mana potion
                };
                Some(ItemStack::new(item_id, rng.gen_range(1..=3)))
            }
            ItemType::Weapon => {
                // Pick weapon based on rarity
                let item_id = match rarity {
                    Rarity::Common | Rarity::Uncommon => ItemId(1), // Rusty sword
                    Rarity::Rare => ItemId(2),                      // Iron sword
                    Rarity::Epic | Rarity::Legendary => ItemId(3),  // Magic staff
                };
                Some(ItemStack::single(item_id))
            }
            ItemType::Armor => {
                // Pick armor
                let item_id = if rarity >= Rarity::Rare {
                    ItemId(11) // Iron helmet
                } else {
                    ItemId(10) // Leather armor
                };
                Some(ItemStack::single(item_id))
            }
            _ => {
                // Default to health potion
                Some(ItemStack::new(ItemId(100), 1))
            }
        }
    }

    /// Open the chest and return its contents
    pub fn open(&mut self) -> (Vec<ItemStack>, i32) {
        self.is_open = true;
        let contents = std::mem::take(&mut self.contents);
        let gold = self.gold;
        self.gold = 0;
        (contents, gold)
    }
}

/// Loot drop from defeated enemy
pub struct LootDrop {
    pub position: Position,
    pub items: Vec<ItemStack>,
    pub gold: i32,
    pub pickup_radius: f32,
    pub lifetime: f32, // Despawn after this many seconds
}

impl LootDrop {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            items: Vec::new(),
            gold: 0,
            pickup_radius: 32.0,
            lifetime: 60.0, // Despawn after 60 seconds
        }
    }

    /// Generate loot from a defeated enemy
    pub fn from_enemy(
        position: Position,
        enemy_def: &EnemyDef,
        item_db: &ItemDatabase,
    ) -> Self {
        let mut drop = Self::new(position);
        let mut rng = rand::thread_rng();

        // Generate gold
        let (min_gold, max_gold) = enemy_def.gold_reward;
        drop.gold = rng.gen_range(min_gold..=max_gold);

        // Roll for item drops
        for (item_id, chance) in &enemy_def.loot_table {
            if rng.r#gen::<f32>() < *chance {
                drop.items.push(ItemStack::single(*item_id));
            }
        }

        drop
    }

    /// Update lifetime and check if should despawn
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.lifetime -= delta_time;
        self.lifetime <= 0.0
    }

    /// Check if player is close enough to pick up
    pub fn can_pickup(&self, player_pos: &Position) -> bool {
        self.position.distance_to(player_pos) <= self.pickup_radius
    }

    /// Pick up all loot
    pub fn pickup(&mut self) -> (Vec<ItemStack>, i32) {
        let items = std::mem::take(&mut self.items);
        let gold = self.gold;
        self.gold = 0;
        (items, gold)
    }

    /// Check if loot has been picked up
    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.gold == 0
    }
}

/// Chest manager for the game
pub struct ChestManager {
    chests: Vec<Chest>,
    loot_drops: Vec<LootDrop>,
}

impl ChestManager {
    pub fn new() -> Self {
        Self {
            chests: Vec::new(),
            loot_drops: Vec::new(),
        }
    }

    /// Spawn a chest in the world
    pub fn spawn_chest(
        &mut self,
        entity_id: EntityId,
        chest_type: ChestType,
        position: Position,
        item_db: &ItemDatabase,
        player_level: i32,
    ) {
        let mut chest = Chest::new(entity_id, chest_type, position);
        chest.generate_loot(item_db, player_level);
        self.chests.push(chest);
    }

    /// Spawn a random chest
    pub fn spawn_random_chest(
        &mut self,
        entity_id: EntityId,
        position: Position,
        item_db: &ItemDatabase,
        player_level: i32,
    ) {
        let mut rng = rand::thread_rng();
        let chest_type = match rng.gen_range(0..100) {
            0..=60 => ChestType::Wooden,
            61..=85 => ChestType::Iron,
            86..=97 => ChestType::Golden,
            _ => ChestType::Legendary,
        };

        self.spawn_chest(entity_id, chest_type, position, item_db, player_level);
    }

    /// Try to open a chest at position
    pub fn try_open_chest(
        &mut self,
        player_pos: &Position,
        interaction_range: f32,
    ) -> Option<(Vec<ItemStack>, i32)> {
        for chest in &mut self.chests {
            if !chest.is_open && chest.position.distance_to(player_pos) <= interaction_range {
                return Some(chest.open());
            }
        }
        None
    }

    /// Spawn loot drop from enemy
    pub fn spawn_loot_drop(
        &mut self,
        position: Position,
        enemy_def: &EnemyDef,
        item_db: &ItemDatabase,
    ) {
        let loot = LootDrop::from_enemy(position, enemy_def, item_db);
        if !loot.is_empty() {
            self.loot_drops.push(loot);
        }
    }

    /// Update all loot drops
    pub fn update(&mut self, delta_time: f32) {
        // Update all loot drops and mark which should be removed
        let mut indices_to_remove = Vec::new();
        for (i, drop) in self.loot_drops.iter_mut().enumerate() {
            if drop.update(delta_time) {
                indices_to_remove.push(i);
            }
        }

        // Remove despawned loot (in reverse to maintain indices)
        for i in indices_to_remove.into_iter().rev() {
            self.loot_drops.remove(i);
        }
    }

    /// Try to pickup nearby loot
    pub fn try_pickup_loot(&mut self, player_pos: &Position) -> Vec<(Vec<ItemStack>, i32)> {
        let mut pickups = Vec::new();

        for drop in &mut self.loot_drops {
            if drop.can_pickup(player_pos) && !drop.is_empty() {
                pickups.push(drop.pickup());
            }
        }

        // Remove empty loot drops
        self.loot_drops.retain(|drop| !drop.is_empty());

        pickups
    }

    /// Get all chests
    pub fn get_chests(&self) -> &[Chest] {
        &self.chests
    }

    /// Get all loot drops
    pub fn get_loot_drops(&self) -> &[LootDrop] {
        &self.loot_drops
    }

    /// Clear all chests and loot (for level transitions)
    pub fn clear_all(&mut self) {
        self.chests.clear();
        self.loot_drops.clear();
    }
}

impl Default for ChestManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Procedural loot table for generating random drops
pub struct LootTable {
    pub entries: Vec<LootTableEntry>,
}

#[derive(Debug, Clone)]
pub struct LootTableEntry {
    pub item_id: ItemId,
    pub weight: f32,
    pub min_quantity: u32,
    pub max_quantity: u32,
    pub required_luck: i32, // Minimum luck stat to have chance at this item
}

impl LootTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(
        &mut self,
        item_id: ItemId,
        weight: f32,
        min_qty: u32,
        max_qty: u32,
        required_luck: i32,
    ) {
        self.entries.push(LootTableEntry {
            item_id,
            weight,
            min_quantity: min_qty,
            max_quantity: max_qty,
            required_luck,
        });
    }

    /// Roll for an item from this loot table
    pub fn roll(&self, player_luck: i32) -> Option<ItemStack> {
        let mut rng = rand::thread_rng();

        // Filter by luck requirement
        let available: Vec<&LootTableEntry> = self
            .entries
            .iter()
            .filter(|e| player_luck >= e.required_luck)
            .collect();

        if available.is_empty() {
            return None;
        }

        // Calculate total weight
        let total_weight: f32 = available.iter().map(|e| e.weight).sum();
        let mut roll = rng.r#gen::<f32>() * total_weight;

        // Select item
        for entry in available {
            roll -= entry.weight;
            if roll <= 0.0 {
                let quantity = rng.gen_range(entry.min_quantity..=entry.max_quantity);
                return Some(ItemStack::new(entry.item_id, quantity));
            }
        }

        None
    }
}

impl Default for LootTable {
    fn default() -> Self {
        Self::new()
    }
}
