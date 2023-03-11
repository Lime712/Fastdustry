use crate::arc_core::math::geom::interp::Interp;

pub trait Vector<T: Vector<T>> {
    /// Returns a copy of this vector.
    fn cpy(&self, ) -> T;

    /// Returns the euclidean length.
    fn len(&self, ) -> f32;

    /// This method is faster than `len(&self, )` because it avoids calculating a square root. It is useful for comparisons, but not for getting exact lengths, as the return value is the square of the actual length.
    /// Returns the squared euclidean length.
    fn len2(&self, ) -> f32;

    /// Limits the length of this vector, based on the desired maximum length.
    fn limit(&mut self, limit: f32) -> T;

    /// Limits the length of this vector, based on the desired maximum length squared.
    /// This method is slightly faster than `limit(&self, )`.
    fn limit2(&mut self, limit2: f32) -> T;

    /// Sets the length of this vector. Does nothing if this vector is zero.
    fn set_length(&mut self, len: f32) -> T;

    /// Sets the length of this vector, based on the square of the desired length. Does nothing if this vector is zero.
    /// This method is slightly faster than `set_length(&self, )`.
    fn set_length2(&mut self, len2: f32) -> T;

    /// Clamps this vector's length to given min and max values.
    fn clamp(&mut self, min: f32, max: f32) -> T;

    /// Sets this vector from the given vector.
    fn set(&mut self, v: T) -> T;

    /// Subtracts the given vector from this vector.
    fn sub(&mut self, v: T) -> T;

    /// Normalizes this vector. Does nothing if it is zero.
    fn nor(&mut self, ) -> T;

    /// Adds the given vector to this vector.
    fn add(&mut self, v: T) -> T;

    /// Returns the dot product between this and the other vector.
    fn dot(&self, v: T) -> f32;

    /// Scales this vector by a scalar.
    fn scl_f32(&mut self, scalar: f32) -> T;

    /// Scales this vector by another vector.
    fn scl_vec(&mut self, v: T) -> T;

    /// Inverse of `scl(&self, )`.
    fn div(&mut self, other: T) -> T;

    /// Returns the distance between this and the other vector.
    fn dst(&self, v: T) -> f32;

    /// This method is faster than `dst(&self, v)` because it avoids calculating a square root. It is useful for comparisons, but not for getting accurate distances, as the return value is the square of the actual distance.
    /// Returns the squared distance between this and the other vector.
    fn dst2(&self, v: T) -> f32;

    /// Linearly interpolates between this vector and the target vector by alpha which is in the range [0,1]. The result is stored in this vector.
    fn lerp(&mut self, target: T, alpha: f32) -> T;

    /// Interpolates between this vector and the given target vector by alpha (&self, within range [0,1]) using the given Interpolation method. the result is stored in this vector.
    fn interpolate(&mut self, target: T, alpha: f32, interpolator: Interp) -> T;

    /// Sets this vector to the unit vector with a random direction
    fn set_to_random_direction(&mut self, ) -> T;

    /// Returns true if this vector is a unit length vector
    fn is_unit(&self, ) -> bool;

    /// Returns true if this vector is a unit length vector within the given margin.
    fn is_unit_margin(&self, margin: f32) -> bool;

    /// Returns true if this vector is a zero vector
    fn is_zero(&self, ) -> bool;

    /// Returns true if the length of this vector is smaller than the given margin
    fn is_zero_margin(&self, margin: f32) -> bool;

    /// Returns true if this vector is in line with the other vector (&self, either in the same or the opposite direction)
    fn is_on_line(&self, other: T, epsilon: f32) -> bool;

    /// Returns true if this vector is in line with the other vector (&self, either in the same or the opposite direction)
    fn is_on_line2(&self, other: T) -> bool;

    /// Returns true if this vector is collinear with the other vector (&self, isOnLine(&self, other, epsilon) && hasSameDirection(&self, other)).
    fn is_collinear(&self, other: T, epsilon: f32) -> bool;

    /// Returns true if this vector is collinear with the other vector (&self, isOnLine(&self, other) && hasSameDirection(&self, other)).
    fn is_collinear2(&self, other: T) -> bool;

    /// Returns true if this vector is opposite collinear with the other vector (&self, isOnLine(&self, other, epsilon) && hasOppositeDirection(&self, other)).
    fn is_collinear_opposite(&self, other: T, epsilon: f32) -> bool;

    /// Returns true if this vector is opposite collinear with the other vector (&self, isOnLine(&self, other) && hasOppositeDirection(&self, other)).
    fn is_collinear_opposite2(&self, other: T) -> bool;

    /// Returns Whether this vector is perpendicular with the other vector. True if the dot product is 0.
    fn is_perpendicular(&self, other: T) -> bool;

    /// # Parameters
    /// * `other` - the other vector
    /// * `epsilon` - a positive small number close to zero
    fn is_perpendicular_epsilon(&self, other: T, epsilon: f32) -> bool;

    /// # Returns
    /// Whether this vector has similar direction compared to the other vector. True if the normalized dot product is > 0.
    fn has_same_direction(&self, other: T) -> bool;

    /// # Returns
    /// Whether this vector has opposite direction compared to the other vector. True if the normalized dot product is < 0.
    fn has_opposite_direction(&self, other: T) -> bool;

    /// Compares this vector with the other vector, using the supplied epsilon for fuzzy equality testing.
    /// # Returns
    /// whether the vectors have fuzzy equality.
    fn epsilon_equals(&self, other: T, epsilon: f32) -> bool;

    /// First scale a supplied vector, then add it to this vector.
    /// # Parameters
    /// * `v` - addition vector
    /// * `scalar` - for scaling the addition vector
    fn mul_add(&mut self, v: T, scalar: f32) -> T;

    /// First scale a supplied vector, then add it to this vector.
    /// # Parameters
    /// * `v` - addition vector
    /// * `mul_vec` - vector by whose values the addition vector will be scaled
    fn mul_add2(&mut self, v: T, mul_vec: T) -> T;

    /// Sets the components of this vector to 0
    fn set_zero(&mut self, ) -> T;

    fn plus(&mut self, other: T) -> T {
        self.add(other)
    }
    fn minus(&mut self, other: T) -> T {
        self.sub(other)
    }
    fn unary_minus(&mut self, ) -> T {
        self.scl_f32(-1.0)
    }
    fn times(&mut self, other: T) -> T {
        self.scl_vec(other)
    }
}