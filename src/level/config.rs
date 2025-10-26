use serde::{Deserialize, Serialize};
use raylib::prelude::*;
use crate::ecs::components::{Transform as EcsTransform, Renderable, RenderShape, Velocity};

/// Level configuration that can be loaded from TOML/JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub name: String,
    pub description: Option<String>,
    pub camera: CameraConfig,
    pub entities: Vec<EntityConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub fov: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: [10.0, 10.0, 10.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fov: 45.0,
        }
    }
}

impl CameraConfig {
    pub fn to_camera3d(&self) -> Camera3D {
        Camera3D::perspective(
            Vector3::new(self.position[0], self.position[1], self.position[2]),
            Vector3::new(self.target[0], self.target[1], self.target[2]),
            Vector3::new(self.up[0], self.up[1], self.up[2]),
            self.fov,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityConfig {
    pub name: Option<String>,
    pub transform: TransformConfig,
    pub renderable: Option<RenderableConfig>,
    pub velocity: Option<VelocityConfig>,
    pub health: Option<f32>,
    pub entity_type: EntityType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformConfig {
    pub position: [f32; 3],
    #[serde(default)]
    pub rotation: [f32; 3],
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

impl TransformConfig {
    pub fn to_transform(&self) -> EcsTransform {
        EcsTransform {
            position: Vector3::new(self.position[0], self.position[1], self.position[2]),
            rotation: Vector3::new(self.rotation[0], self.rotation[1], self.rotation[2]),
            scale: Vector3::new(self.scale[0], self.scale[1], self.scale[2]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RenderableConfig {
    Cube {
        size: [f32; 3],
        color: [u8; 4],
    },
    Sphere {
        radius: f32,
        color: [u8; 4],
    },
    Cylinder {
        radius: f32,
        height: f32,
        color: [u8; 4],
    },
    Model {
        path: String,
    },
}

impl RenderableConfig {
    pub fn to_renderable(&self) -> Renderable {
        let shape = match self {
            RenderableConfig::Cube { size, color } => RenderShape::Cube {
                size: Vector3::new(size[0], size[1], size[2]),
                color: *color,
            },
            RenderableConfig::Sphere { radius, color } => RenderShape::Sphere {
                radius: *radius,
                color: *color,
            },
            RenderableConfig::Cylinder { radius, height, color } => RenderShape::Cylinder {
                radius: *radius,
                height: *height,
                color: *color,
            },
            RenderableConfig::Model { path } => RenderShape::Model {
                path: path.clone(),
            },
        };

        Renderable {
            shape,
            visible: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityConfig {
    #[serde(default)]
    pub linear: [f32; 3],
    #[serde(default)]
    pub angular: [f32; 3],
}

impl VelocityConfig {
    pub fn to_velocity(&self) -> Velocity {
        Velocity {
            linear: Vector3::new(self.linear[0], self.linear[1], self.linear[2]),
            angular: Vector3::new(self.angular[0], self.angular[1], self.angular[2]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Static,
    Player,
    Enemy,
    Prop,
}

impl Default for LevelConfig {
    fn default() -> Self {
        Self {
            name: "Default Level".to_string(),
            description: Some("A basic level".to_string()),
            camera: CameraConfig::default(),
            entities: vec![],
        }
    }
}

impl LevelConfig {
    /// Create a sample level for testing
    pub fn sample() -> Self {
        Self {
            name: "Sample Level".to_string(),
            description: Some("A sample level with some basic entities".to_string()),
            camera: CameraConfig {
                position: [15.0, 15.0, 15.0],
                target: [0.0, 0.0, 0.0],
                up: [0.0, 1.0, 0.0],
                fov: 45.0,
            },
            entities: vec![
                // Ground plane
                EntityConfig {
                    name: Some("Ground".to_string()),
                    transform: TransformConfig {
                        position: [0.0, -0.5, 0.0],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [20.0, 1.0, 20.0],
                    },
                    renderable: Some(RenderableConfig::Cube {
                        size: [1.0, 1.0, 1.0],
                        color: [100, 100, 100, 255],
                    }),
                    velocity: None,
                    health: None,
                    entity_type: EntityType::Static,
                },
                // Player
                EntityConfig {
                    name: Some("Player".to_string()),
                    transform: TransformConfig {
                        position: [0.0, 2.0, 0.0],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [1.0, 1.0, 1.0],
                    },
                    renderable: Some(RenderableConfig::Cube {
                        size: [1.0, 2.0, 1.0],
                        color: [0, 121, 241, 255], // BLUE
                    }),
                    velocity: Some(VelocityConfig {
                        linear: [0.0, 0.0, 0.0],
                        angular: [0.0, 0.0, 0.0],
                    }),
                    health: Some(100.0),
                    entity_type: EntityType::Player,
                },
                // Some cubes
                EntityConfig {
                    name: Some("Red Cube".to_string()),
                    transform: TransformConfig {
                        position: [3.0, 1.0, 0.0],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [1.0, 1.0, 1.0],
                    },
                    renderable: Some(RenderableConfig::Cube {
                        size: [2.0, 2.0, 2.0],
                        color: [230, 41, 55, 255], // RED
                    }),
                    velocity: None,
                    health: None,
                    entity_type: EntityType::Prop,
                },
                EntityConfig {
                    name: Some("Green Sphere".to_string()),
                    transform: TransformConfig {
                        position: [-3.0, 1.5, 0.0],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [1.0, 1.0, 1.0],
                    },
                    renderable: Some(RenderableConfig::Sphere {
                        radius: 1.5,
                        color: [0, 228, 48, 255], // GREEN
                    }),
                    velocity: Some(VelocityConfig {
                        linear: [0.0, 0.0, 0.0],
                        angular: [0.0, 1.0, 0.0],
                    }),
                    health: None,
                    entity_type: EntityType::Prop,
                },
                EntityConfig {
                    name: Some("Yellow Cylinder".to_string()),
                    transform: TransformConfig {
                        position: [0.0, 1.5, -5.0],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [1.0, 1.0, 1.0],
                    },
                    renderable: Some(RenderableConfig::Cylinder {
                        radius: 1.0,
                        height: 3.0,
                        color: [253, 249, 0, 255], // YELLOW
                    }),
                    velocity: None,
                    health: None,
                    entity_type: EntityType::Prop,
                },
            ],
        }
    }
}
