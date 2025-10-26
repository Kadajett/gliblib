/// Perlin Noise Implementation
/// Based on Ken Perlin's improved noise algorithm

use std::f32::consts::PI;

/// Perlin noise generator
pub struct PerlinNoise {
    permutation: [u8; 512],
}

impl PerlinNoise {
    /// Create a new Perlin noise generator with a seed
    pub fn new(seed: u32) -> Self {
        let mut permutation = [0u8; 512];

        // Initialize permutation table with seed-based values
        let mut p = [0u8; 256];
        for i in 0..256 {
            p[i] = i as u8;
        }

        // Fisher-Yates shuffle with seed
        let mut rng_state = seed;
        for i in (1..256).rev() {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let j = (rng_state % (i as u32 + 1)) as usize;
            p.swap(i, j);
        }

        // Duplicate permutation to avoid wrapping
        for i in 0..256 {
            permutation[i] = p[i];
            permutation[256 + i] = p[i];
        }

        Self { permutation }
    }

    /// Generate 2D Perlin noise at given coordinates
    pub fn noise2d(&self, x: f32, y: f32) -> f32 {
        // Find unit grid cell containing point
        let xi = (x.floor() as i32 & 255) as usize;
        let yi = (y.floor() as i32 & 255) as usize;

        // Relative x, y within cell
        let xf = x - x.floor();
        let yf = y - y.floor();

        // Fade curves for x, y
        let u = Self::fade(xf);
        let v = Self::fade(yf);

        // Hash coordinates of 4 corners
        let aa = self.permutation[self.permutation[xi] as usize + yi] as usize;
        let ab = self.permutation[self.permutation[xi] as usize + yi + 1] as usize;
        let ba = self.permutation[self.permutation[xi + 1] as usize + yi] as usize;
        let bb = self.permutation[self.permutation[xi + 1] as usize + yi + 1] as usize;

        // Gradient vectors at corners
        let g1 = Self::grad2d(aa, xf, yf);
        let g2 = Self::grad2d(ba, xf - 1.0, yf);
        let g3 = Self::grad2d(ab, xf, yf - 1.0);
        let g4 = Self::grad2d(bb, xf - 1.0, yf - 1.0);

        // Bilinear interpolation
        let x1 = Self::lerp(g1, g2, u);
        let x2 = Self::lerp(g3, g4, u);
        let result = Self::lerp(x1, x2, v);

        result
    }

    /// Octave-based fractal noise (multiple frequencies)
    pub fn fractal_noise2d(
        &self,
        x: f32,
        y: f32,
        octaves: u32,
        persistence: f32,
        lacunarity: f32,
    ) -> f32 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            total += self.noise2d(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= lacunarity;
        }

        total / max_value
    }

    /// Fade function (6t^5 - 15t^4 + 10t^3)
    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Linear interpolation
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

    /// Compute gradient dot product
    fn grad2d(hash: usize, x: f32, y: f32) -> f32 {
        // Use hash to select gradient direction
        let h = hash & 3;
        match h {
            0 => x + y,
            1 => -x + y,
            2 => x - y,
            3 => -x - y,
            _ => 0.0,
        }
    }
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Utility function for generating height maps
pub fn generate_heightmap(
    width: usize,
    height: usize,
    scale: f32,
    octaves: u32,
    persistence: f32,
    lacunarity: f32,
    seed: u32,
) -> Vec<Vec<f32>> {
    let noise = PerlinNoise::new(seed);
    let mut heightmap = vec![vec![0.0; height]; width];

    for x in 0..width {
        for y in 0..height {
            let nx = x as f32 / scale;
            let ny = y as f32 / scale;

            let value = noise.fractal_noise2d(nx, ny, octaves, persistence, lacunarity);

            // Normalize to 0.0 - 1.0 range
            heightmap[x][y] = (value + 1.0) / 2.0;
        }
    }

    heightmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perlin_noise() {
        let noise = PerlinNoise::new(42);
        let value = noise.noise2d(1.5, 2.3);
        assert!(value >= -1.0 && value <= 1.0);
    }

    #[test]
    fn test_fractal_noise() {
        let noise = PerlinNoise::new(42);
        let value = noise.fractal_noise2d(1.5, 2.3, 4, 0.5, 2.0);
        assert!(value >= -1.0 && value <= 1.0);
    }
}
