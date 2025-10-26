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

/// Model component for 3D models with textures
#[derive(Debug, Clone)]
pub struct Model {
    pub model_path: String,
    pub texture_path: Option<String>,
    pub tint: Color,
    pub scale: f32,
}

impl Model {
    pub fn new(model_path: String) -> Self {
        Self {
            model_path,
            texture_path: None,
            tint: Color::WHITE,
            scale: 1.0,
        }
    }

    pub fn with_texture(mut self, texture_path: String) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = tint;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
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

/// Rigidbody component for physics simulation
#[derive(Debug, Clone, Copy)]
pub struct Rigidbody {
    /// Mass of the object (0.0 = infinite mass/static)
    pub mass: f32,
    /// Whether gravity affects this object
    pub use_gravity: bool,
    /// Velocity from physics (separate from manual velocity)
    pub velocity: Vector3,
    /// Accumulated forces to apply this frame
    pub force: Vector3,
    /// Drag coefficient (air resistance) - 0.0 = no drag, higher = more drag
    pub drag: f32,
    /// Is the object currently grounded/touching floor
    pub is_grounded: bool,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            mass: 1.0,
            use_gravity: true,
            velocity: Vector3::zero(),
            force: Vector3::zero(),
            drag: 0.01,
            is_grounded: false,
        }
    }
}

impl Rigidbody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            ..Default::default()
        }
    }

    pub fn kinematic() -> Self {
        Self {
            mass: 0.0,
            use_gravity: false,
            ..Default::default()
        }
    }

    pub fn with_gravity(mut self, use_gravity: bool) -> Self {
        self.use_gravity = use_gravity;
        self
    }

    pub fn with_drag(mut self, drag: f32) -> Self {
        self.drag = drag;
        self
    }

    pub fn add_force(&mut self, force: Vector3) {
        self.force = self.force + force;
    }

    pub fn is_static(&self) -> bool {
        self.mass == 0.0
    }
}

/// Collider shapes for collision detection
#[derive(Debug, Clone, Copy)]
pub enum ColliderShape {
    Box { size: Vector3 },
    Sphere { radius: f32 },
    Capsule { radius: f32, height: f32 },
}

/// Collider component for collision detection
#[derive(Debug, Clone, Copy)]
pub struct Collider {
    pub shape: ColliderShape,
    /// Is this a trigger (no physics response, just detection)
    pub is_trigger: bool,
    /// Restitution/bounciness (0.0 = no bounce, 1.0 = perfect bounce)
    pub restitution: f32,
    /// Friction coefficient (0.0 = no friction, 1.0 = high friction)
    pub friction: f32,
}

impl Collider {
    pub fn box_collider(size: Vector3) -> Self {
        Self {
            shape: ColliderShape::Box { size },
            is_trigger: false,
            restitution: 0.0,
            friction: 0.5,
        }
    }

    pub fn sphere_collider(radius: f32) -> Self {
        Self {
            shape: ColliderShape::Sphere { radius },
            is_trigger: false,
            restitution: 0.0,
            friction: 0.5,
        }
    }

    pub fn capsule_collider(radius: f32, height: f32) -> Self {
        Self {
            shape: ColliderShape::Capsule { radius, height },
            is_trigger: false,
            restitution: 0.0,
            friction: 0.5,
        }
    }

    pub fn as_trigger(mut self) -> Self {
        self.is_trigger = true;
        self
    }

    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution = restitution;
        self
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }
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
