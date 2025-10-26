/// RPG UI System
/// Renders health bars, inventory, equipment, and stats

use raylib::prelude::*;
use super::*;

/// UI panel positions
#[derive(Debug, Clone, Copy)]
pub enum UIPanelPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

/// RPG UI Renderer
pub struct RpgUI {
    pub show_inventory: bool,
    pub show_character_sheet: bool,
    pub show_status_effects: bool,
}

impl Default for RpgUI {
    fn default() -> Self {
        Self {
            show_inventory: false,
            show_character_sheet: false,
            show_status_effects: true,
        }
    }
}

impl RpgUI {
    pub fn new() -> Self {
        Self::default()
    }

    /// Draw health bar
    pub fn draw_health_bar(
        d: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        current: i32,
        max: i32,
        label: &str,
    ) {
        let percent = (current as f32 / max as f32).clamp(0.0, 1.0);
        let filled_width = (width as f32 * percent) as i32;

        // Background
        d.draw_rectangle(x, y, width, height, Color::DARKGRAY);

        // Health fill
        let health_color = if percent > 0.6 {
            Color::GREEN
        } else if percent > 0.3 {
            Color::YELLOW
        } else {
            Color::RED
        };
        d.draw_rectangle(x, y, filled_width, height, health_color);

        // Border
        d.draw_rectangle_lines(x, y, width, height, Color::BLACK);

        // Text
        let text = format!("{}: {}/{}", label, current, max);
        d.draw_text(&text, x + 5, y + (height / 2 - 8), 16, Color::WHITE);
    }

    /// Draw mana bar
    pub fn draw_mana_bar(
        d: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        current: i32,
        max: i32,
    ) {
        let percent = (current as f32 / max as f32).clamp(0.0, 1.0);
        let filled_width = (width as f32 * percent) as i32;

        // Background
        d.draw_rectangle(x, y, width, height, Color::DARKGRAY);

        // Mana fill
        d.draw_rectangle(x, y, filled_width, height, Color::SKYBLUE);

        // Border
        d.draw_rectangle_lines(x, y, width, height, Color::BLACK);

        // Text
        let text = format!("Mana: {}/{}", current, max);
        d.draw_text(&text, x + 5, y + (height / 2 - 8), 16, Color::WHITE);
    }

    /// Draw experience bar
    pub fn draw_exp_bar(
        d: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        current: i32,
        required: i32,
        level: i32,
    ) {
        let percent = (current as f32 / required as f32).clamp(0.0, 1.0);
        let filled_width = (width as f32 * percent) as i32;

        // Background
        d.draw_rectangle(x, y, width, height, Color::DARKGRAY);

        // EXP fill
        d.draw_rectangle(x, y, filled_width, height, Color::GOLD);

        // Border
        d.draw_rectangle_lines(x, y, width, height, Color::BLACK);

        // Text
        let text = format!("Level {} - EXP: {}/{}", level, current, required);
        d.draw_text(&text, x + 5, y + (height / 2 - 8), 16, Color::BLACK);
    }

    /// Draw player HUD (health, mana, exp)
    pub fn draw_player_hud(d: &mut RaylibDrawHandle, player: &Player) {
        let margin = 10;
        let bar_width = 300;
        let bar_height = 30;
        let spacing = 5;

        let mut y = margin;

        // Player name and level
        d.draw_text(
            &format!("{} - Level {}", player.name, player.level.current_level),
            margin,
            y,
            24,
            Color::WHITE,
        );
        y += 30;

        // Health bar
        Self::draw_health_bar(
            d,
            margin,
            y,
            bar_width,
            bar_height,
            player.stats.current_health,
            player.stats.max_health,
            "HP",
        );
        y += bar_height + spacing;

        // Mana bar
        Self::draw_mana_bar(
            d,
            margin,
            y,
            bar_width,
            bar_height,
            player.stats.current_mana,
            player.stats.max_mana,
        );
        y += bar_height + spacing;

        // Experience bar
        Self::draw_exp_bar(
            d,
            margin,
            y,
            bar_width,
            bar_height,
            player.level.current_exp,
            player.level.exp_to_next_level,
            player.level.current_level,
        );
    }

    /// Draw status effects
    pub fn draw_status_effects(d: &mut RaylibDrawHandle, player: &Player, x: i32, mut y: i32) {
        if player.status_effects.is_empty() {
            return;
        }

        d.draw_text("Status Effects:", x, y, 18, Color::WHITE);
        y += 22;

        for effect in &player.status_effects {
            let (color, icon) = match effect.effect {
                StatusEffect::Poisoned => (Color::DARKGREEN, "POI"),
                StatusEffect::Burning => (Color::RED, "BRN"),
                StatusEffect::Frozen => (Color::SKYBLUE, "FRZ"),
                StatusEffect::Stunned => (Color::YELLOW, "STN"),
                StatusEffect::Blessed => (Color::GOLD, "BLS"),
                StatusEffect::Cursed => (Color::PURPLE, "CRS"),
                StatusEffect::Hasted => (Color::ORANGE, "HST"),
                StatusEffect::Slowed => (Color::BLUE, "SLW"),
                StatusEffect::Regenerating => (Color::GREEN, "RGN"),
                StatusEffect::Invulnerable => (Color::WHITE, "INV"),
            };

            let text = format!("[{}] {:.1}s", icon, effect.duration);
            d.draw_text(&text, x, y, 16, color);
            y += 20;
        }
    }

    /// Draw inventory panel
    pub fn draw_inventory(d: &mut RaylibDrawHandle, inventory: &Inventory, db: &ItemDatabase) {
        let panel_x = 400;
        let panel_y = 100;
        let panel_width = 500;
        let panel_height = 500;

        // Panel background
        d.draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::new(40, 40, 40, 240));
        d.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, Color::WHITE);

        // Title
        d.draw_text("Inventory", panel_x + 10, panel_y + 10, 24, Color::WHITE);

        // Gold display
        let gold_text = format!("Gold: {}", inventory.gold);
        d.draw_text(&gold_text, panel_x + 350, panel_y + 15, 18, Color::GOLD);

        // Draw items grid
        let item_size = 60;
        let items_per_row = 6;
        let start_x = panel_x + 20;
        let start_y = panel_y + 50;

        for (idx, maybe_stack) in inventory.items.iter().enumerate() {
            let row = (idx / items_per_row) as i32;
            let col = (idx % items_per_row) as i32;
            let x = start_x + col * (item_size + 10);
            let y = start_y + row * (item_size + 10);

            if let Some(stack) = maybe_stack {
                if let Some(item) = db.get(stack.item_id) {
                    // Item slot background
                    d.draw_rectangle(x, y, item_size, item_size, Color::new(60, 60, 60, 255));
                    d.draw_rectangle_lines(x, y, item_size, item_size, item.rarity.color());

                    // Item name (abbreviated)
                    let name_short = if item.name.len() > 8 {
                        format!("{}...", &item.name[..6])
                    } else {
                        item.name.clone()
                    };
                    d.draw_text(&name_short, x + 5, y + 5, 12, Color::WHITE);

                    // Stack size if > 1
                    if stack.quantity > 1 {
                        let stack_text = format!("x{}", stack.quantity);
                        d.draw_text(&stack_text, x + item_size - 25, y + item_size - 18, 14, Color::YELLOW);
                    }

                    // Item type icon
                    let type_icon = match item.item_type {
                        ItemType::Weapon => "W",
                        ItemType::Armor => "A",
                        ItemType::Accessory => "R",
                        ItemType::Consumable => "C",
                        ItemType::QuestItem => "Q",
                        ItemType::Material => "M",
                    };
                    d.draw_text(type_icon, x + 5, y + item_size - 18, 14, Color::LIGHTGRAY);
                }
            } else {
                // Empty slot
                d.draw_rectangle(x, y, item_size, item_size, Color::new(30, 30, 30, 255));
                d.draw_rectangle_lines(x, y, item_size, item_size, Color::DARKGRAY);
            }
        }

        // Instructions
        d.draw_text("Press I to close", panel_x + 10, panel_y + panel_height - 30, 16, Color::LIGHTGRAY);
    }

    /// Draw character sheet
    pub fn draw_character_sheet(d: &mut RaylibDrawHandle, player: &Player) {
        let panel_x = 300;
        let panel_y = 80;
        let panel_width = 700;
        let panel_height = 560;

        // Panel background
        d.draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::new(40, 40, 40, 240));
        d.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, Color::WHITE);

        // Title
        d.draw_text(
            &format!("{} - Character Sheet", player.name),
            panel_x + 20,
            panel_y + 20,
            24,
            Color::WHITE,
        );

        let mut y = panel_y + 60;
        let left_col = panel_x + 30;
        let right_col = panel_x + 380;

        // Core attributes
        d.draw_text("Attributes", left_col, y, 20, Color::GOLD);
        y += 25;

        let attrs = [
            ("Strength", player.stats.strength),
            ("Dexterity", player.stats.dexterity),
            ("Intelligence", player.stats.intelligence),
            ("Vitality", player.stats.vitality),
            ("Luck", player.stats.luck),
        ];

        for (name, value) in attrs {
            d.draw_text(&format!("{}: {}", name, value), left_col, y, 18, Color::WHITE);
            y += 22;
        }

        // Combat stats
        y = panel_y + 60;
        d.draw_text("Combat Stats", right_col, y, 20, Color::GOLD);
        y += 25;

        let combat_stats = [
            format!("Physical DMG: {}", player.stats.physical_damage),
            format!("Magic DMG: {}", player.stats.magic_damage),
            format!("Defense: {}", player.stats.defense),
            format!("Dodge: {:.1}%", player.stats.dodge_chance * 100.0),
            format!("Crit Chance: {:.1}%", player.stats.crit_chance * 100.0),
            format!("Crit Multi: {:.1}x", player.stats.crit_multiplier),
            format!("Move Speed: {:.0}", player.stats.move_speed),
        ];

        for stat in combat_stats {
            d.draw_text(&stat, right_col, y, 18, Color::WHITE);
            y += 22;
        }

        // Equipment section
        y = panel_y + 280;
        d.draw_text("Equipment", left_col, y, 20, Color::GOLD);
        y += 25;

        let slot_names = [
            (EquipSlot::Weapon, "Weapon"),
            (EquipSlot::Helmet, "Helmet"),
            (EquipSlot::Chest, "Chest"),
            (EquipSlot::Legs, "Legs"),
            (EquipSlot::Boots, "Boots"),
            (EquipSlot::Gloves, "Gloves"),
            (EquipSlot::Ring1, "Ring 1"),
            (EquipSlot::Ring2, "Ring 2"),
            (EquipSlot::Amulet, "Amulet"),
        ];

        for (slot, slot_name) in slot_names {
            let item_text = if let Some(item_id) = player.equipment.get_equipped(slot) {
                format!("{}: #{}", slot_name, item_id.0)
            } else {
                format!("{}: [Empty]", slot_name)
            };
            d.draw_text(&item_text, left_col, y, 16, Color::LIGHTGRAY);
            y += 20;
        }

        // Instructions
        d.draw_text(
            "Press C to close",
            panel_x + 20,
            panel_y + panel_height - 30,
            16,
            Color::LIGHTGRAY,
        );
    }

    /// Draw quick stats overlay (minimal HUD)
    pub fn draw_quick_stats(d: &mut RaylibDrawHandle, player: &Player, x: i32, y: i32) {
        let mut current_y = y;

        // Compact stats display
        d.draw_text(
            &format!("STR:{} DEX:{} INT:{} VIT:{} LCK:{}",
                player.stats.strength,
                player.stats.dexterity,
                player.stats.intelligence,
                player.stats.vitality,
                player.stats.luck
            ),
            x,
            current_y,
            16,
            Color::LIGHTGRAY,
        );
        current_y += 20;

        d.draw_text(
            &format!("ATK:{} DEF:{} SPD:{:.0}",
                player.stats.physical_damage + player.stats.magic_damage,
                player.stats.defense,
                player.stats.move_speed
            ),
            x,
            current_y,
            16,
            Color::LIGHTGRAY,
        );
    }
}
