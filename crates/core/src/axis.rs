/// Represents the three primary spatial axes in 3D space.
///
/// This enum is used to specify directional components along the X, Y, or Z axis.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Axis;
///
/// let axis = Axis::X;
/// println!("Selected axis: {:?}", axis);
///
/// // Iterate over all axes
/// for axis in Axis::iter() {
///     println!("{:?}", axis);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// The X-axis (horizontal, left-right)
    X,
    /// The Y-axis (vertical, up-down)
    Y,
    /// The Z-axis (depth, forward-backward)
    Z,
}

impl Axis {
    /// Returns an iterator over all three axes in order: X, Y, Z.
    ///
    /// This is useful when you need to perform operations on all axes,
    /// such as transforming coordinates or checking bounds in all directions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Axis;
    ///
    /// let axes: Vec<Axis> = Axis::iter().collect();
    /// assert_eq!(axes, vec![Axis::X, Axis::Y, Axis::Z]);
    /// ```
    ///
    /// # Performance
    ///
    /// This iterator is very efficient as it simply iterates over a static array.
    pub fn iter() -> impl Iterator<Item = Axis> {
        static AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
        AXES.iter().copied() // .copied() is used to iterate over values, not references
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_iter_count() {
        let count = Axis::iter().count();
        assert_eq!(count, 3, "There should be exactly 3 axes");
    }

    #[test]
    fn test_axis_iter_order() {
        let axes: Vec<Axis> = Axis::iter().collect();
        assert_eq!(axes, vec![Axis::X, Axis::Y, Axis::Z]);
    }

    #[test]
    fn test_axis_iter_values() {
        let mut iter = Axis::iter();
        assert_eq!(iter.next(), Some(Axis::X));
        assert_eq!(iter.next(), Some(Axis::Y));
        assert_eq!(iter.next(), Some(Axis::Z));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_axis_clone() {
        let axis = Axis::X;
        let cloned = axis.clone();
        assert_eq!(axis, cloned);
    }

    #[test]
    fn test_axis_copy() {
        let axis = Axis::Y;
        let copied = axis; // Copy trait allows this
        assert_eq!(axis, copied);
    }

    #[test]
    fn test_axis_debug() {
        assert_eq!(format!("{:?}", Axis::X), "X");
        assert_eq!(format!("{:?}", Axis::Y), "Y");
        assert_eq!(format!("{:?}", Axis::Z), "Z");
    }

    #[test]
    fn test_axis_equality() {
        assert_eq!(Axis::X, Axis::X);
        assert_eq!(Axis::Y, Axis::Y);
        assert_eq!(Axis::Z, Axis::Z);
        assert_ne!(Axis::X, Axis::Y);
        assert_ne!(Axis::Y, Axis::Z);
        assert_ne!(Axis::X, Axis::Z);
    }

    #[test]
    fn test_axis_iter_multiple_calls() {
        // Ensure iter() can be called multiple times
        let first: Vec<_> = Axis::iter().collect();
        let second: Vec<_> = Axis::iter().collect();
        assert_eq!(first, second);
    }

    #[test]
    fn test_axis_iter_independent() {
        // Ensure iterators are independent
        let mut iter1 = Axis::iter();
        let mut iter2 = Axis::iter();

        assert_eq!(iter1.next(), Some(Axis::X));
        assert_eq!(iter2.next(), Some(Axis::X));
        assert_eq!(iter1.next(), Some(Axis::Y));
        assert_eq!(iter2.next(), Some(Axis::Y));
    }
}
