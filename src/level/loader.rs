use std::fs;
use std::path::Path;
use super::config::*;
use crate::ecs::World;
use crate::ecs::components::Health;

pub struct LevelLoader;

impl LevelLoader {
    /// Load level from TOML file
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> Result<LevelConfig, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse TOML: {}", e))
    }

    /// Load level from JSON file
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> Result<LevelConfig, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Save level to TOML file
    pub fn save_to_toml<P: AsRef<Path>>(level: &LevelConfig, path: P) -> Result<(), String> {
        let contents = toml::to_string_pretty(level)
            .map_err(|e| format!("Failed to serialize TOML: {}", e))?;

        fs::write(path, contents)
            .map_err(|e| format!("Failed to write file: {}", e))
    }

    /// Save level to JSON file
    pub fn save_to_json<P: AsRef<Path>>(level: &LevelConfig, path: P) -> Result<(), String> {
        let contents = serde_json::to_string_pretty(level)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        fs::write(path, contents)
            .map_err(|e| format!("Failed to write file: {}", e))
    }

    /// Spawn entities from level config into the world
    pub fn spawn_entities(level: &LevelConfig, world: &mut World) {
        for entity_config in &level.entities {
            let mut builder = world.spawn()
                .with_transform(entity_config.transform.to_transform());

            if let Some(name) = &entity_config.name {
                builder = builder.with_name(name.clone());
            }

            if let Some(renderable_config) = &entity_config.renderable {
                builder = builder.with_renderable(renderable_config.to_renderable());
            }

            if let Some(velocity_config) = &entity_config.velocity {
                builder = builder.with_velocity(velocity_config.to_velocity());
            }

            if let Some(health) = entity_config.health {
                builder = builder.with_health(Health::new(health));
            }

            match entity_config.entity_type {
                EntityType::Player => builder = builder.as_player(),
                EntityType::Enemy => builder = builder.as_enemy(),
                _ => {}
            }

            builder.build();
        }
    }
}

