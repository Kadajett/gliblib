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

/// Camera component for first-person view
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Field of view in degrees
    pub fov: f32,
    /// Target offset from entity position (where the camera looks relative to position)
    pub target_offset: Vector3,
    /// Up vector (typically [0, 1, 0])
    pub up: Vector3,
    /// Yaw rotation in degrees (left/right)
    pub yaw: f32,
    /// Pitch rotation in degrees (up/down)
    pub pitch: f32,
    /// Mouse sensitivity for first-person controls
    pub mouse_sensitivity: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            fov: 60.0,
            target_offset: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            mouse_sensitivity: 0.1,
        }
    }
}

impl Camera {
    pub fn new(fov: f32) -> Self {
        Self {
            fov,
            ..Default::default()
        }
    }

    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.mouse_sensitivity = sensitivity;
        self
    }

    /// Convert to Raylib Camera3D using the entity's transform
    pub fn to_camera3d(&self, position: Vector3) -> Camera3D {
        // Calculate the forward direction from yaw and pitch
        let pitch_rad = self.pitch.to_radians();
        let yaw_rad = self.yaw.to_radians();

        let forward = Vector3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        );

        let target = position + forward;

        Camera3D::perspective(position, target, self.up, self.fov)
    }
}
