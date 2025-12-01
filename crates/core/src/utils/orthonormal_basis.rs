use crate::Vector3;

/// An orthonormal basis constructed from a normal vector.
///
/// An orthonormal basis is a set of three mutually perpendicular unit vectors
/// that form a coordinate system. This is useful for transforming vectors between
/// different coordinate spaces, such as converting from world space to tangent space.
///
/// The basis is constructed using the Gram-Schmidt process, ensuring:
/// - All three vectors (u, v, w) are unit length
/// - All three vectors are mutually perpendicular
/// - The vectors form a right-handed coordinate system
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::{utils::OrthonormalBasis, Vector3};
///
/// let normal = Vector3::new(0.0, 1.0, 0.0);
/// let basis = OrthonormalBasis::new(normal);
///
/// // Transform a vector to local space
/// let world_vec = Vector3::new(1.0, 0.0, 0.0);
/// let local_vec = basis.transform_to_local(world_vec);
/// ```
pub struct OrthonormalBasis {
    /// The first basis vector
    pub u: Vector3,
    /// The second basis vector
    pub v: Vector3,
    /// The third basis vector (normal direction)
    pub w: Vector3,
}

impl OrthonormalBasis {
    /// Constructs a new orthonormal basis from a normal vector.
    ///
    /// The normal vector is normalized to become the `w` basis vector. The `u` and `v`
    /// vectors are then constructed to be perpendicular to `w` and to each other using
    /// the Gram-Schmidt process.
    ///
    /// The algorithm chooses an initial reference vector that is not parallel to the
    /// normal: (0, 1, 0) if the normal is mostly aligned with the x-axis, otherwise (1, 0, 0).
    ///
    /// # Arguments
    ///
    /// * `normal` - The normal vector to build the basis from. Does not need to be normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{utils::OrthonormalBasis, Vector3};
    ///
    /// // Create basis from upward-pointing normal
    /// let basis = OrthonormalBasis::new(Vector3::new(0.0, 1.0, 0.0));
    /// ```
    pub fn new(normal: Vector3) -> Self {
        let w = normal.unit();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).unit();
        let u = w.cross(&v);

        Self { u, v, w }
    }

    /// Transforms a vector from basis coordinates to local space.
    ///
    /// Given a vector in the basis coordinate system (where the basis vectors are the axes),
    /// this function returns the equivalent vector in local/world space.
    ///
    /// The transformation is computed as: `v.x * u + v.y * v + v.z * w`
    ///
    /// # Arguments
    ///
    /// * `v` - A vector in basis coordinates
    ///
    /// # Returns
    ///
    /// The vector transformed to local space
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{utils::OrthonormalBasis, Vector3};
    ///
    /// let basis = OrthonormalBasis::new(Vector3::new(0.0, 1.0, 0.0));
    /// let basis_vec = Vector3::new(1.0, 0.0, 0.0); // Unit vector along basis u-axis
    /// let local_vec = basis.transform_to_local(basis_vec);
    /// // local_vec is now aligned with the u basis vector in world space
    /// ```
    pub fn transform_to_local(&self, v: Vector3) -> Vector3 {
        (v.x * self.u) + (v.y * self.v) + (v.z * self.w)
    }
}

#[cfg(test)]
mod tests {
    use assert_eq_float::assert_eq_float;

    use super::*;

    #[test]
    fn test_basis_vectors_are_orthogonal() {
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let basis = OrthonormalBasis::new(normal);

        // Check all pairs are perpendicular (dot product = 0)
        assert_eq_float!(basis.u.dot(&basis.v), 0.0);
        assert_eq_float!(basis.u.dot(&basis.w), 0.0);
        assert_eq_float!(basis.v.dot(&basis.w), 0.0);
    }

    #[test]
    fn test_basis_vectors_are_unit_length() {
        let normal = Vector3::new(1.0, 2.0, 3.0);
        let basis = OrthonormalBasis::new(normal);

        // Check all vectors are unit length
        assert_eq_float!(basis.u.length(), 1.0);
        assert_eq_float!(basis.v.length(), 1.0);
        assert_eq_float!(basis.w.length(), 1.0);
    }

    #[test]
    fn test_w_aligned_with_normal() {
        let normal = Vector3::new(1.0, 2.0, 3.0);
        let basis = OrthonormalBasis::new(normal);

        // w should be the normalized version of the input normal
        let expected_w = normal.unit();
        assert_eq!(basis.w, expected_w);
    }

    #[test]
    fn test_right_handed_coordinate_system() {
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let basis = OrthonormalBasis::new(normal);

        // The construction does: v = w × a, then u = w × v
        // This means: v × u = w (not u × v = w)
        let cross = basis.v.cross(&basis.u);
        assert_eq!(cross, basis.w);
    }

    #[test]
    fn test_basis_with_x_aligned_normal() {
        // When normal is close to x-axis, algorithm should use (0,1,0) as reference
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let basis = OrthonormalBasis::new(normal);

        assert_eq_float!(basis.u.dot(&basis.v), 0.0);
        assert_eq_float!(basis.u.dot(&basis.w), 0.0);
        assert_eq_float!(basis.v.dot(&basis.w), 0.0);
    }

    #[test]
    fn test_basis_with_y_aligned_normal() {
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let basis = OrthonormalBasis::new(normal);

        assert_eq_float!(basis.u.length(), 1.0);
        assert_eq_float!(basis.v.length(), 1.0);
        assert_eq_float!(basis.w.length(), 1.0);
    }

    #[test]
    fn test_basis_with_z_aligned_normal() {
        let normal = Vector3::new(0.0, 0.0, 1.0);
        let basis = OrthonormalBasis::new(normal);

        assert_eq_float!(basis.u.dot(&basis.v), 0.0);
        assert_eq_float!(basis.u.dot(&basis.w), 0.0);
        assert_eq_float!(basis.v.dot(&basis.w), 0.0);
    }

    #[test]
    fn test_transform_to_local_basis_axes() {
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let basis = OrthonormalBasis::new(normal);

        // Transform basis coordinate axes
        let local_u = basis.transform_to_local(Vector3::new(1.0, 0.0, 0.0));
        let local_v = basis.transform_to_local(Vector3::new(0.0, 1.0, 0.0));
        let local_w = basis.transform_to_local(Vector3::new(0.0, 0.0, 1.0));

        // Should map to the basis vectors themselves
        assert_eq!(local_u, basis.u);
        assert_eq!(local_v, basis.v);
        assert_eq!(local_w, basis.w);
    }

    #[test]
    fn test_transform_to_local_arbitrary_vector() {
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let basis = OrthonormalBasis::new(normal);

        let basis_vec = Vector3::new(2.0, 3.0, 4.0);
        let local = basis.transform_to_local(basis_vec);

        // Result should be linear combination of basis vectors
        let expected = (basis.u * 2.0) + (basis.v * 3.0) + (basis.w * 4.0);
        assert_eq!(local, expected);
    }

    #[test]
    fn test_transform_preserves_length() {
        let normal = Vector3::new(1.0, 1.0, 1.0);
        let basis = OrthonormalBasis::new(normal);

        let basis_vec = Vector3::new(3.0, 4.0, 0.0);
        let local = basis.transform_to_local(basis_vec);

        // Orthonormal transformation preserves length
        assert_eq_float!(basis_vec.length(), local.length());
    }

    #[test]
    fn test_basis_consistency_with_unnormalized_input() {
        // Basis should work the same regardless of input normal length
        let normal1 = Vector3::new(1.0, 2.0, 3.0);
        let normal2 = Vector3::new(2.0, 4.0, 6.0); // Same direction, different length

        let basis1 = OrthonormalBasis::new(normal1);
        let basis2 = OrthonormalBasis::new(normal2);

        // Both should produce the same w vector
        assert_eq!(basis1.w, basis2.w);
    }
}
