use serde::{Deserialize, Serialize};
use raylib::prelude::*;
use crate::ecs::components::{Transform as EcsTransform, Renderable, RenderShape, Velocity, Model};

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
    pub model: Option<ModelConfig>,
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
pub struct ModelConfig {
    pub model_path: String,
    pub texture_path: Option<String>,
    #[serde(default = "default_tint")]
    pub tint: [u8; 4],
    #[serde(default = "default_model_scale")]
    pub scale: f32,
}

fn default_tint() -> [u8; 4] {
    [255, 255, 255, 255]
}

fn default_model_scale() -> f32 {
    1.0
}

impl ModelConfig {
    pub fn to_model(&self) -> Model {
        let mut model = Model::new(self.model_path.clone());
        
        if let Some(texture_path) = &self.texture_path {
            model = model.with_texture(texture_path.clone());
        }
        
        model = model.with_tint(Color::new(self.tint[0], self.tint[1], self.tint[2], self.tint[3]));
        model = model.with_scale(self.scale);
        
        model
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

