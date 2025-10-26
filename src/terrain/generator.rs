/// Terrain Generation and Rendering
/// Generates large terrain meshes using Perlin noise

use raylib::prelude::*;
use super::noise::*;

/// Terrain configuration
#[derive(Debug, Clone)]
pub struct TerrainConfig {
    pub width: usize,           // Number of vertices in X direction
    pub depth: usize,           // Number of vertices in Z direction
    pub cell_size: f32,         // Size of each grid cell
    pub height_scale: f32,      // Multiplier for height values
    pub octaves: u32,           // Number of noise octaves
    pub persistence: f32,       // Amplitude multiplier per octave
    pub lacunarity: f32,        // Frequency multiplier per octave
    pub noise_scale: f32,       // Overall noise frequency scale
    pub seed: u32,              // Random seed
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            width: 200,
            depth: 200,
            cell_size: 1.0,
            height_scale: 10.0,
            octaves: 6,
            persistence: 0.5,
            lacunarity: 2.0,
            noise_scale: 50.0,
            seed: 42,
        }
    }
}

/// Generated terrain data
pub struct Terrain {
    pub config: TerrainConfig,
    pub heightmap: Vec<Vec<f32>>,
    pub vertices: Vec<Vector3>,
    pub colors: Vec<Color>,
}

impl Terrain {
    /// Generate new terrain from configuration
    pub fn generate(config: TerrainConfig) -> Self {
        let noise = PerlinNoise::new(config.seed);

        // Generate heightmap
        let mut heightmap = vec![vec![0.0; config.depth]; config.width];
        for x in 0..config.width {
            for z in 0..config.depth {
                let nx = x as f32 / config.noise_scale;
                let nz = z as f32 / config.noise_scale;

                let height = noise.fractal_noise2d(
                    nx,
                    nz,
                    config.octaves,
                    config.persistence,
                    config.lacunarity,
                );

                // Normalize and scale
                heightmap[x][z] = height * config.height_scale;
            }
        }

        // Generate vertices and colors
        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        for x in 0..config.width {
            for z in 0..config.depth {
                let world_x = (x as f32 - config.width as f32 / 2.0) * config.cell_size;
                let world_z = (z as f32 - config.depth as f32 / 2.0) * config.cell_size;
                let world_y = heightmap[x][z];

                vertices.push(Vector3::new(world_x, world_y, world_z));

                // Color based on height
                let color = Self::height_to_color(world_y, config.height_scale);
                colors.push(color);
            }
        }

        Self {
            config,
            heightmap,
            vertices,
            colors,
        }
    }

    /// Convert height to color (terrain coloring)
    fn height_to_color(height: f32, max_height: f32) -> Color {
        let normalized = (height / max_height + 1.0) / 2.0; // 0.0 to 1.0

        if normalized < 0.3 {
            // Deep water
            Color::new(20, 50, 150, 255)
        } else if normalized < 0.4 {
            // Shallow water
            Color::new(50, 100, 200, 255)
        } else if normalized < 0.45 {
            // Beach/sand
            Color::new(210, 180, 140, 255)
        } else if normalized < 0.6 {
            // Grass
            Color::new(50, 150, 50, 255)
        } else if normalized < 0.75 {
            // Dark grass/dirt
            Color::new(100, 120, 60, 255)
        } else if normalized < 0.85 {
            // Rock
            Color::new(120, 120, 120, 255)
        } else {
            // Snow
            Color::new(240, 240, 250, 255)
        }
    }

    /// Get height at world position (bilinear interpolation)
    pub fn get_height_at(&self, world_x: f32, world_z: f32) -> f32 {
        // Convert world coordinates to grid coordinates
        let grid_x = (world_x / self.config.cell_size) + (self.config.width as f32 / 2.0);
        let grid_z = (world_z / self.config.cell_size) + (self.config.depth as f32 / 2.0);

        // Clamp to valid range
        if grid_x < 0.0
            || grid_z < 0.0
            || grid_x >= (self.config.width - 1) as f32
            || grid_z >= (self.config.depth - 1) as f32
        {
            return 0.0;
        }

        // Get integer and fractional parts
        let x0 = grid_x.floor() as usize;
        let z0 = grid_z.floor() as usize;
        let x1 = (x0 + 1).min(self.config.width - 1);
        let z1 = (z0 + 1).min(self.config.depth - 1);

        let fx = grid_x - x0 as f32;
        let fz = grid_z - z0 as f32;

        // Bilinear interpolation
        let h00 = self.heightmap[x0][z0];
        let h10 = self.heightmap[x1][z0];
        let h01 = self.heightmap[x0][z1];
        let h11 = self.heightmap[x1][z1];

        let h0 = h00 * (1.0 - fx) + h10 * fx;
        let h1 = h01 * (1.0 - fx) + h11 * fx;

        h0 * (1.0 - fz) + h1 * fz
    }

    /// Render terrain using grid of triangles
    pub fn render(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
        // Draw terrain as a grid of quads (2 triangles each)
        for x in 0..(self.config.width - 1) {
            for z in 0..(self.config.depth - 1) {
                let idx00 = x * self.config.depth + z;
                let idx10 = (x + 1) * self.config.depth + z;
                let idx01 = x * self.config.depth + (z + 1);
                let idx11 = (x + 1) * self.config.depth + (z + 1);

                let v00 = self.vertices[idx00];
                let v10 = self.vertices[idx10];
                let v01 = self.vertices[idx01];
                let v11 = self.vertices[idx11];

                let c00 = self.colors[idx00];
                let c10 = self.colors[idx10];
                let c01 = self.colors[idx01];
                let c11 = self.colors[idx11];

                // Average color for each triangle
                let color1 = Self::average_color(&[c00, c10, c01]);
                let color2 = Self::average_color(&[c10, c11, c01]);

                // First triangle
                d.draw_triangle3D(v00, v10, v01, color1);

                // Second triangle
                d.draw_triangle3D(v10, v11, v01, color2);
            }
        }
    }

    /// Render terrain with wireframe overlay
    pub fn render_wireframe(&self, d: &mut RaylibMode3D<RaylibDrawHandle>, color: Color) {
        for x in 0..(self.config.width - 1) {
            for z in 0..(self.config.depth - 1) {
                let idx00 = x * self.config.depth + z;
                let idx10 = (x + 1) * self.config.depth + z;
                let idx01 = x * self.config.depth + (z + 1);

                let v00 = self.vertices[idx00];
                let v10 = self.vertices[idx10];
                let v01 = self.vertices[idx01];

                // Draw grid lines
                d.draw_line_3D(v00, v10, color);
                d.draw_line_3D(v00, v01, color);
            }
        }
    }

    /// Average multiple colors
    fn average_color(colors: &[Color]) -> Color {
        let r: u32 = colors.iter().map(|c| c.r as u32).sum();
        let g: u32 = colors.iter().map(|c| c.g as u32).sum();
        let b: u32 = colors.iter().map(|c| c.b as u32).sum();
        let a: u32 = colors.iter().map(|c| c.a as u32).sum();

        let count = colors.len() as u32;
        Color::new(
            (r / count) as u8,
            (g / count) as u8,
            (b / count) as u8,
            (a / count) as u8,
        )
    }
}
