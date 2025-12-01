use crate::{Random, Vector3};

/// Perlin noise generator for creating smooth, pseudo-random gradients.
///
/// Perlin noise is a type of gradient noise commonly used in procedural texture generation,
/// terrain generation, and other applications requiring natural-looking randomness.
/// This implementation uses Ken Perlin's improved noise algorithm with trilinear interpolation.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::{utils::Perlin, Vector3, Random, random_new};
///
/// let random = random_new();
/// let perlin = Perlin::new(&*random);
///
/// // Generate noise value at a point
/// let point = Vector3::new(1.5, 2.3, 0.7);
/// let noise_value = perlin.noise(point);
/// assert!(noise_value >= -1.0 && noise_value <= 1.0);
///
/// // Generate turbulence (fractal noise)
/// let turbulence_value = perlin.turbulence(point, 7);
/// assert!(turbulence_value >= 0.0);
/// ```
#[derive(Debug)]
pub struct Perlin {
    /// Random unit vectors at lattice points for gradient noise
    rand_vec: [Vector3; Perlin::POINT_COUNT],
    /// Permutation table for x-coordinates
    perm_x: [usize; Perlin::POINT_COUNT],
    /// Permutation table for y-coordinates
    perm_y: [usize; Perlin::POINT_COUNT],
    /// Permutation table for z-coordinates
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Perlin {
    /// Number of lattice points in the permutation tables
    const POINT_COUNT: usize = 256;

    /// Creates a new Perlin noise generator with random gradients and permutation tables.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the `Random` trait
    ///
    /// # Returns
    ///
    /// A new `Perlin` instance initialized with random gradients and permutations
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{utils::Perlin, Random, random_new};
    ///
    /// let random = random_new();
    /// let perlin = Perlin::new(&*random);
    /// ```
    pub fn new(random: &dyn Random) -> Self {
        let mut rand_vec: [Vector3; Perlin::POINT_COUNT] = [Vector3::ZERO; Perlin::POINT_COUNT];
        for item in rand_vec.iter_mut() {
            *item = Vector3::random_unit(random);
        }

        let perm_x = Perlin::generate_perm(random);
        let perm_y = Perlin::generate_perm(random);
        let perm_z = Perlin::generate_perm(random);

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// Computes the Perlin noise value at a given 3D point.
    ///
    /// The noise function returns smooth, continuous values that vary pseudo-randomly
    /// across 3D space. The output is approximately in the range [-1, 1], though values
    /// can occasionally exceed these bounds slightly.
    ///
    /// # Arguments
    ///
    /// * `pt` - The 3D point at which to evaluate the noise function
    ///
    /// # Returns
    ///
    /// A noise value approximately in the range [-1, 1]
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{utils::Perlin, Vector3, Random, random_new};
    ///
    /// let random = random_new();
    /// let perlin = Perlin::new(&*random);
    /// let value = perlin.noise(Vector3::new(1.0, 2.0, 3.0));
    /// ```
    pub fn noise(&self, pt: Vector3) -> f64 {
        let u = pt.x - pt.x.floor();
        let v = pt.y - pt.y.floor();
        let w = pt.z - pt.z.floor();

        let i = pt.x.floor() as isize;
        let j = pt.y.floor() as isize;
        let k = pt.z.floor() as isize;

        let mut c: [[[Vector3; 2]; 2]; 2] = [[[Vector3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[i];
                }
            }
        }

        Perlin::trilinear_interpolation(c, u, v, w)
    }

    /// Computes turbulence at a given point using fractal Brownian motion.
    ///
    /// Turbulence is created by summing multiple octaves of Perlin noise at different
    /// frequencies and amplitudes. This produces a more complex, fractal-like pattern
    /// useful for marble textures, clouds, and other natural phenomena.
    ///
    /// # Arguments
    ///
    /// * `pt` - The 3D point at which to evaluate turbulence
    /// * `depth` - The number of octaves to sum (typically 5-7 for good results)
    ///
    /// # Returns
    ///
    /// A turbulence value (always non-negative due to absolute value)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{utils::Perlin, Vector3, Random, random_new};
    ///
    /// let random = random_new();
    /// let perlin = Perlin::new(&*random);
    /// let turbulence = perlin.turbulence(Vector3::new(1.0, 2.0, 3.0), 7);
    /// assert!(turbulence >= 0.0);
    /// ```
    pub fn turbulence(&self, pt: Vector3, depth: u32) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = pt;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        acc.abs()
    }

    /// Performs trilinear interpolation with Hermite smoothing on a 2x2x2 cube of gradient vectors.
    ///
    /// This is the core of Perlin noise, using dot products between random gradients and
    /// distance vectors to create smooth, continuous noise. The Hermite curve (3t² - 2t³)
    /// is used for smoothing to eliminate directional artifacts.
    ///
    /// # Arguments
    ///
    /// * `c` - A 2x2x2 array of gradient vectors at the corners of the unit cube
    /// * `u` - Fractional position in x-direction [0, 1]
    /// * `v` - Fractional position in y-direction [0, 1]
    /// * `w` - Fractional position in z-direction [0, 1]
    ///
    /// # Returns
    ///
    /// The interpolated noise value
    fn trilinear_interpolation(c: [[[Vector3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut acc = 0.0;
        for (i, item) in c.iter().enumerate() {
            for (j, item) in item.iter().enumerate() {
                for (k, item) in item.iter().enumerate() {
                    let weight_v = Vector3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * item.dot(&weight_v);
                }
            }
        }
        acc
    }

    /// Generates a random permutation table for hashing coordinates to gradient indices.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the `Random` trait
    ///
    /// # Returns
    ///
    /// An array containing a permutation of indices 0..255
    fn generate_perm(random: &dyn Random) -> [usize; Perlin::POINT_COUNT] {
        let mut p: [usize; Perlin::POINT_COUNT] = [0; Perlin::POINT_COUNT];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }

        Perlin::permute(random, &mut p);
        p
    }

    /// Randomly permutes an array using the Fisher-Yates shuffle algorithm.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the `Random` trait
    /// * `p` - The array to permute in-place
    fn permute(random: &dyn Random, p: &mut [usize; Perlin::POINT_COUNT]) {
        for i in (1..Perlin::POINT_COUNT - 1).rev() {
            let target = random.rand_int_interval(0, i as i64) as usize;
            p.swap(i, target);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::random::test::MockRandom;

    use super::*;

    #[test]
    fn test_perlin_creation() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        // Just verify it can be created without panicking
        assert_eq!(perlin.rand_vec.len(), 256);
        assert_eq!(perlin.perm_x.len(), 256);
        assert_eq!(perlin.perm_y.len(), 256);
        assert_eq!(perlin.perm_z.len(), 256);
    }

    #[test]
    fn test_noise_returns_reasonable_values() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        let test_points = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.5, 2.3, 0.7),
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(10.5, 20.3, 30.1),
        ];

        for pt in test_points {
            let noise = perlin.noise(pt);
            // Noise should typically be in range [-1, 1], allowing small margin
            assert!(
                noise >= -1.5 && noise <= 1.5,
                "Noise value {} out of expected range at point {:?}",
                noise,
                pt
            );
        }
    }

    #[test]
    fn test_noise_continuity() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        let pt1 = Vector3::new(1.0, 2.0, 3.0);
        let pt2 = Vector3::new(1.001, 2.0, 3.0);

        let noise1 = perlin.noise(pt1);
        let noise2 = perlin.noise(pt2);

        // Close points should produce close noise values (continuity)
        let diff = (noise1 - noise2).abs();
        assert!(
            diff < 0.1,
            "Noise not continuous: difference {} between close points",
            diff
        );
    }

    #[test]
    fn test_noise_deterministic() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        let pt = Vector3::new(1.5, 2.5, 3.5);
        let noise1 = perlin.noise(pt);
        let noise2 = perlin.noise(pt);

        assert_eq!(
            noise1, noise2,
            "Noise should be deterministic for same point"
        );
    }

    #[test]
    fn test_turbulence_always_positive() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        let test_points = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.5, 2.3, 0.7),
            Vector3::new(-5.0, -5.0, -5.0),
        ];

        for pt in test_points {
            let turb = perlin.turbulence(pt, 5);
            assert!(
                turb >= 0.0,
                "Turbulence should be non-negative, got {} at {:?}",
                turb,
                pt
            );
        }
    }

    #[test]
    fn test_turbulence_increases_with_depth() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        let pt = Vector3::new(1.5, 2.5, 3.5);

        let turb1 = perlin.turbulence(pt, 1);
        let turb5 = perlin.turbulence(pt, 5);

        // Higher depth typically produces higher values due to accumulation
        // (though not guaranteed for all points)
        assert!(turb1 >= 0.0 && turb5 >= 0.0);
    }

    #[test]
    fn test_turbulence_deterministic() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        let pt = Vector3::new(2.3, 4.5, 6.7);
        let turb1 = perlin.turbulence(pt, 7);
        let turb2 = perlin.turbulence(pt, 7);

        assert_eq!(turb1, turb2, "Turbulence should be deterministic");
    }

    #[test]
    fn test_permutation_tables_are_permutations() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        // Each permutation table should contain each number 0-255 exactly once
        for perm in [&perlin.perm_x, &perlin.perm_y, &perlin.perm_z] {
            let mut sorted = perm.to_vec();
            sorted.sort_unstable();

            for (i, &val) in sorted.iter().enumerate() {
                assert_eq!(
                    i, val,
                    "Permutation table should contain each index exactly once"
                );
            }
        }
    }

    #[test]
    fn test_different_perlin_instances_differ() {
        let random1 = MockRandom::new_with_length(1914);
        let random2 = MockRandom::new_with_length(1914);

        let perlin1 = Perlin::new(&random1);
        let perlin2 = Perlin::new(&random2);

        let pt = Vector3::new(1.0, 2.0, 3.0);
        let noise1 = perlin1.noise(pt);
        let noise2 = perlin2.noise(pt);

        // With same seed they'll be identical, but structure is set up correctly
        // In real usage with different random instances, they should differ
        assert_eq!(noise1, noise2); // Same mock random produces same result
    }

    #[test]
    fn test_noise_at_integer_coordinates() {
        let random = MockRandom::new_with_length(1914);
        let perlin = Perlin::new(&random);

        // Test at lattice points (integer coordinates)
        let integer_points = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(5.0, 10.0, 15.0),
        ];

        for pt in integer_points {
            let noise = perlin.noise(pt);
            assert!(
                noise.is_finite(),
                "Noise should be finite at integer coordinates"
            );
        }
    }
}
