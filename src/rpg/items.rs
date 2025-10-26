/// Item and Equipment System
/// Defines items, equipment, and inventory management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of items in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    QuestItem,
    Material,
}

/// Item rarity affects stats and drop rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    /// Get stat multiplier for this rarity
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Rarity::Common => 1.0,
            Rarity::Uncommon => 1.25,
            Rarity::Rare => 1.5,
            Rarity::Epic => 2.0,
            Rarity::Legendary => 3.0,
        }
    }

    /// Get color for UI display
    pub fn color(&self) -> raylib::prelude::Color {
        match self {
            Rarity::Common => raylib::prelude::Color::LIGHTGRAY,
            Rarity::Uncommon => raylib::prelude::Color::GREEN,
            Rarity::Rare => raylib::prelude::Color::BLUE,
            Rarity::Epic => raylib::prelude::Color::PURPLE,
            Rarity::Legendary => raylib::prelude::Color::ORANGE,
        }
    }
}

/// Equipment slot types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipSlot {
    Weapon,
    Helmet,
    Chest,
    Legs,
    Boots,
    Gloves,
    Ring1,
    Ring2,
    Amulet,
}

/// Stat modifiers that items can provide
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct StatModifiers {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub vitality: i32,
    pub luck: i32,
    pub max_health: i32,
    pub max_mana: i32,
    pub physical_damage: i32,
    pub magic_damage: i32,
    pub defense: i32,
    pub dodge_chance: f32,
    pub crit_chance: f32,
    pub move_speed: f32,
}

impl StatModifiers {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create modifiers with scaled stats based on rarity
    pub fn scaled(base_stats: StatModifiers, rarity: Rarity) -> Self {
        let mult = rarity.stat_multiplier();
        StatModifiers {
            strength: (base_stats.strength as f32 * mult) as i32,
            dexterity: (base_stats.dexterity as f32 * mult) as i32,
            intelligence: (base_stats.intelligence as f32 * mult) as i32,
            vitality: (base_stats.vitality as f32 * mult) as i32,
            luck: (base_stats.luck as f32 * mult) as i32,
            max_health: (base_stats.max_health as f32 * mult) as i32,
            max_mana: (base_stats.max_mana as f32 * mult) as i32,
            physical_damage: (base_stats.physical_damage as f32 * mult) as i32,
            magic_damage: (base_stats.magic_damage as f32 * mult) as i32,
            defense: (base_stats.defense as f32 * mult) as i32,
            dodge_chance: base_stats.dodge_chance * mult,
            crit_chance: base_stats.crit_chance * mult,
            move_speed: base_stats.move_speed * mult,
        }
    }
}

/// A unique item ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub u32);

/// Item definition - the template for an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDef {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub max_stack: u32,
    pub sell_value: i32,
    pub equip_slot: Option<EquipSlot>,
    pub stat_mods: StatModifiers,
    pub required_level: i32,
}

impl ItemDef {
    pub fn new(id: u32, name: &str, item_type: ItemType) -> Self {
        Self {
            id: ItemId(id),
            name: name.to_string(),
            description: String::new(),
            item_type,
            rarity: Rarity::Common,
            max_stack: if matches!(item_type, ItemType::Consumable | ItemType::Material) {
                99
            } else {
                1
            },
            sell_value: 1,
            equip_slot: None,
            stat_mods: StatModifiers::new(),
            required_level: 1,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_rarity(mut self, rarity: Rarity) -> Self {
        self.rarity = rarity;
        self
    }

    pub fn with_equip_slot(mut self, slot: EquipSlot) -> Self {
        self.equip_slot = Some(slot);
        self
    }

    pub fn with_stat_mods(mut self, mods: StatModifiers) -> Self {
        self.stat_mods = mods;
        self
    }

    pub fn with_value(mut self, value: i32) -> Self {
        self.sell_value = value;
        self
    }

    pub fn with_required_level(mut self, level: i32) -> Self {
        self.required_level = level;
        self
    }
}

/// An instance of an item in someone's inventory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item_id: ItemId,
    pub quantity: u32,
}

impl ItemStack {
    pub fn new(item_id: ItemId, quantity: u32) -> Self {
        Self { item_id, quantity }
    }

    pub fn single(item_id: ItemId) -> Self {
        Self::new(item_id, 1)
    }
}

/// Inventory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub max_slots: usize,
    pub items: Vec<Option<ItemStack>>,
    pub gold: i32,
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        Self {
            max_slots,
            items: vec![None; max_slots],
            gold: 0,
        }
    }

    /// Try to add an item to inventory
    /// Returns the number of items that couldn't be added (0 if all added successfully)
    pub fn add_item(&mut self, item_def: &ItemDef, mut quantity: u32) -> u32 {
        // First try to stack with existing items
        for slot in &mut self.items {
            if let Some(stack) = slot {
                if stack.item_id == item_def.id {
                    let space_available = item_def.max_stack - stack.quantity;
                    let amount_to_add = quantity.min(space_available);
                    stack.quantity += amount_to_add;
                    quantity -= amount_to_add;

                    if quantity == 0 {
                        return 0;
                    }
                }
            }
        }

        // Then try to add to empty slots
        while quantity > 0 {
            if let Some(empty_slot) = self.items.iter_mut().find(|slot| slot.is_none()) {
                let amount_to_add = quantity.min(item_def.max_stack);
                *empty_slot = Some(ItemStack::new(item_def.id, amount_to_add));
                quantity -= amount_to_add;
            } else {
                // No more empty slots
                break;
            }
        }

        quantity // Return how many couldn't be added
    }

    /// Remove an item from inventory by slot index
    pub fn remove_item(&mut self, slot: usize, quantity: u32) -> Option<ItemStack> {
        if slot >= self.items.len() {
            return None;
        }

        if let Some(stack) = &mut self.items[slot] {
            if stack.quantity <= quantity {
                // Remove entire stack
                self.items[slot].take()
            } else {
                // Partial removal
                stack.quantity -= quantity;
                Some(ItemStack::new(stack.item_id, quantity))
            }
        } else {
            None
        }
    }

    /// Get item at slot
    pub fn get_item(&self, slot: usize) -> Option<&ItemStack> {
        self.items.get(slot).and_then(|s| s.as_ref())
    }

    /// Count total quantity of a specific item
    pub fn count_item(&self, item_id: ItemId) -> u32 {
        self.items
            .iter()
            .filter_map(|slot| slot.as_ref())
            .filter(|stack| stack.item_id == item_id)
            .map(|stack| stack.quantity)
            .sum()
    }

    /// Check if inventory has space for at least one more item
    pub fn has_space(&self) -> bool {
        self.items.iter().any(|slot| slot.is_none())
    }

    /// Add gold
    pub fn add_gold(&mut self, amount: i32) {
        self.gold = (self.gold + amount).max(0);
    }

    /// Try to spend gold, returns true if successful
    pub fn spend_gold(&mut self, amount: i32) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }
}

/// Equipment loadout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub slots: HashMap<EquipSlot, ItemId>,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    /// Equip an item to a slot
    pub fn equip(&mut self, slot: EquipSlot, item_id: ItemId) -> Option<ItemId> {
        self.slots.insert(slot, item_id)
    }

    /// Unequip an item from a slot
    pub fn unequip(&mut self, slot: EquipSlot) -> Option<ItemId> {
        self.slots.remove(&slot)
    }

    /// Get equipped item in a slot
    pub fn get_equipped(&self, slot: EquipSlot) -> Option<ItemId> {
        self.slots.get(&slot).copied()
    }

    /// Get all equipped items
    pub fn all_equipped(&self) -> Vec<(EquipSlot, ItemId)> {
        self.slots.iter().map(|(k, v)| (*k, *v)).collect()
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self::new()
    }
}

/// Item database - stores all item definitions
pub struct ItemDatabase {
    items: HashMap<ItemId, ItemDef>,
}

impl ItemDatabase {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Register an item definition
    pub fn register(&mut self, item: ItemDef) {
        self.items.insert(item.id, item);
    }

    /// Get item definition by ID
    pub fn get(&self, id: ItemId) -> Option<&ItemDef> {
        self.items.get(&id)
    }

    /// Create a database with some starter items
    pub fn with_starter_items() -> Self {
        let mut db = Self::new();

        // Weapons
        db.register(
            ItemDef::new(1, "Rusty Sword", ItemType::Weapon)
                .with_description("An old, rusty sword. Better than nothing.")
                .with_equip_slot(EquipSlot::Weapon)
                .with_stat_mods(StatModifiers {
                    physical_damage: 5,
                    ..Default::default()
                })
                .with_value(10),
        );

        db.register(
            ItemDef::new(2, "Iron Sword", ItemType::Weapon)
                .with_description("A sturdy iron sword.")
                .with_rarity(Rarity::Uncommon)
                .with_equip_slot(EquipSlot::Weapon)
                .with_stat_mods(StatModifiers {
                    physical_damage: 15,
                    strength: 2,
                    ..Default::default()
                })
                .with_value(50)
                .with_required_level(3),
        );

        db.register(
            ItemDef::new(3, "Magic Staff", ItemType::Weapon)
                .with_description("A staff imbued with magical energy.")
                .with_rarity(Rarity::Rare)
                .with_equip_slot(EquipSlot::Weapon)
                .with_stat_mods(StatModifiers {
                    magic_damage: 25,
                    intelligence: 5,
                    max_mana: 20,
                    ..Default::default()
                })
                .with_value(150)
                .with_required_level(5),
        );

        // Armor
        db.register(
            ItemDef::new(10, "Leather Armor", ItemType::Armor)
                .with_description("Basic leather armor.")
                .with_equip_slot(EquipSlot::Chest)
                .with_stat_mods(StatModifiers {
                    defense: 3,
                    max_health: 10,
                    ..Default::default()
                })
                .with_value(20),
        );

        db.register(
            ItemDef::new(11, "Iron Helmet", ItemType::Armor)
                .with_description("A sturdy iron helmet.")
                .with_rarity(Rarity::Uncommon)
                .with_equip_slot(EquipSlot::Helmet)
                .with_stat_mods(StatModifiers {
                    defense: 5,
                    vitality: 2,
                    ..Default::default()
                })
                .with_value(40),
        );

        // Consumables
        db.register(
            ItemDef::new(100, "Health Potion", ItemType::Consumable)
                .with_description("Restores 50 HP.")
                .with_value(10),
        );

        db.register(
            ItemDef::new(101, "Mana Potion", ItemType::Consumable)
                .with_description("Restores 30 MP.")
                .with_value(10),
        );

        // Quest Items
        db.register(
            ItemDef::new(200, "Mysterious Key", ItemType::QuestItem)
                .with_description("An old key. What does it unlock?")
                .with_rarity(Rarity::Rare)
                .with_value(0),
        );

        db
    }
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self::new()
    }
}
