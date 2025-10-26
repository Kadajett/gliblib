use raylib::prelude::*;

/// Position component for 3D entities
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }
}

impl Transform {
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn with_scale(mut self, scale: Vector3) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_rotation(mut self, rotation: Vector3) -> Self {
        self.rotation = rotation;
        self
    }
}

/// Render component - what to draw
#[derive(Debug, Clone)]
pub enum RenderShape {
    Cube { size: Vector3, color: [u8; 4] },
    Sphere { radius: f32, color: [u8; 4] },
    Cylinder { radius: f32, height: f32, color: [u8; 4] },
    Model { path: String },
}

#[derive(Debug, Clone)]
pub struct Renderable {
    pub shape: RenderShape,
    pub visible: bool,
}

impl Renderable {
    pub fn cube(size: Vector3, color: Color) -> Self {
        Self {
            shape: RenderShape::Cube {
                size,
                color: [color.r, color.g, color.b, color.a],
            },
            visible: true,
        }
    }

    pub fn sphere(radius: f32, color: Color) -> Self {
        Self {
            shape: RenderShape::Sphere {
                radius,
                color: [color.r, color.g, color.b, color.a],
            },
            visible: true,
        }
    }
}

/// Velocity component for moving entities
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub linear: Vector3,
    pub angular: Vector3,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear: Vector3::zero(),
            angular: Vector3::zero(),
        }
    }
}

/// Tag for player entity
#[derive(Debug, Clone, Copy)]
pub struct Player;

/// Tag for enemy entities
#[derive(Debug, Clone, Copy)]
pub struct Enemy;

/// Health component
#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}

/// Name/label component
#[derive(Debug, Clone)]
pub struct Name(pub String);
