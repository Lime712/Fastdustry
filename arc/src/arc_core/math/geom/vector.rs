pub trait Vector<T: Vector<T>> {
    // original code:
    // /** @return a copy of this vector */
    //     T cpy();
    //
    //     /** @return The euclidean length */
    //     float len();
    //
    //     /**
    //      * This method is faster than {@link Vector#len()} because it avoids calculating a square root. It is useful for comparisons,
    //      * but not for getting exact lengths, as the return value is the square of the actual length.
    //      * @return The squared euclidean length
    //      */
    //     float len2();
    //
    //     /**
    //      * Limits the length of this vector, based on the desired maximum length.
    //      * @param limit desired maximum length for this vector
    //      * @return this vector for chaining
    //      */
    //     T limit(float limit);
    //
    //     /**
    //      * Limits the length of this vector, based on the desired maximum length squared.
    //      * <p/>
    //      * This method is slightly faster than limit().
    //      * @param limit2 squared desired maximum length for this vector
    //      * @return this vector for chaining
    //      * @see #len2()
    //      */
    //     T limit2(float limit2);
    //
    //     /**
    //      * Sets the length of this vector. Does nothing if this vector is zero.
    //      * @param len desired length for this vector
    //      * @return this vector for chaining
    //      */
    //     T setLength(float len);
    //
    //     /**
    //      * Sets the length of this vector, based on the square of the desired length. Does nothing if this vector is zero.
    //      * <p/>
    //      * This method is slightly faster than setLength().
    //      * @param len2 desired square of the length for this vector
    //      * @return this vector for chaining
    //      * @see #len2()
    //      */
    //     T setLength2(float len2);
    //
    //     /**
    //      * Clamps this vector's length to given min and max values
    //      * @param min Min length
    //      * @param max Max length
    //      * @return This vector for chaining
    //      */
    //     T clamp(float min, float max);
    //
    //     /**
    //      * Sets this vector from the given vector
    //      * @param v The vector
    //      * @return This vector for chaining
    //      */
    //     T set(T v);
    //
    //     /**
    //      * Subtracts the given vector from this vector.
    //      * @param v The vector
    //      * @return This vector for chaining
    //      */
    //     T sub(T v);
    //
    //     /**
    //      * Normalizes this vector. Does nothing if it is zero.
    //      * @return This vector for chaining
    //      */
    //     T nor();
    //
    //     /**
    //      * Adds the given vector to this vector
    //      * @param v The vector
    //      * @return This vector for chaining
    //      */
    //     T add(T v);
    //
    //     /**
    //      * @param v The other vector
    //      * @return The dot product between this and the other vector
    //      */
    //     float dot(T v);
    //
    //     /**
    //      * Scales this vector by a scalar
    //      * @param scalar The scalar
    //      * @return This vector for chaining
    //      */
    //     T scl(float scalar);
    //
    //     /**
    //      * Scales this vector by another vector
    //      * @return This vector for chaining
    //      */
    //     T scl(T v);
    //
    //     /** Inverse of scl() */
    //     T div(T other);
    //
    //     /**
    //      * @param v The other vector
    //      * @return the distance between this and the other vector
    //      */
    //     float dst(T v);
    //
    //     /**
    //      * This method is faster than {@link Vector#dst(Vector)} because it avoids calculating a square root. It is useful for
    //      * comparisons, but not for getting accurate distances, as the return value is the square of the actual distance.
    //      * @param v The other vector
    //      * @return the squared distance between this and the other vector
    //      */
    //     float dst2(T v);

    /// Returns a copy of this vector.
    fn cpy() -> T;

    /// Returns the euclidean length.
    fn len() -> f32;

    /// This method is faster than `len()` because it avoids calculating a square root. It is useful for comparisons, but not for getting exact lengths, as the return value is the square of the actual length.
    /// Returns the squared euclidean length.
    fn len2() -> f32;

    /// Limits the length of this vector, based on the desired maximum length.
    fn limit(limit: f32) -> T;

    /// Limits the length of this vector, based on the desired maximum length squared.
    /// This method is slightly faster than `limit()`.
    fn limit2(limit2: f32) -> T;

    /// Sets the length of this vector. Does nothing if this vector is zero.
    fn set_length(len: f32) -> T;

    /// Sets the length of this vector, based on the square of the desired length. Does nothing if this vector is zero.
    /// This method is slightly faster than `set_length()`.
    fn set_length2(len2: f32) -> T;

    /// Clamps this vector's length to given min and max values.
    fn clamp(min: f32, max: f32) -> T;

    /// Sets this vector from the given vector.
    fn set(v: T) -> T;

    /// Subtracts the given vector from this vector.
    fn sub(v: T) -> T;

    /// Normalizes this vector. Does nothing if it is zero.
    fn nor() -> T;

    /// Adds the given vector to this vector.
    fn add(v: T) -> T;

    /// Returns the dot product between this and the other vector.
    fn dot(v: T) -> f32;

    /// Scales this vector by a scalar.
    fn scl_f32(scalar: f32) -> T;

    /// Scales this vector by another vector.
    fn scl_vec(v: T) -> T;

    /// Inverse of `scl()`.
    fn div(other: T) -> T;

    /// Returns the distance between this and the other vector.
    fn dst(v: T) -> f32;

    /// This method is faster than `dst(v)` because it avoids calculating a square root. It is useful for comparisons, but not for getting accurate distances, as the return value is the square of the actual distance.
    /// Returns the squared distance between this and the other vector.
    fn dst2(v: T) -> f32;

    // /**
    //      * Linearly interpolates between this vector and the target vector by alpha which is in the range [0,1]. The result is stored
    //      * in this vector.
    //      * @param target The target vector
    //      * @param alpha The interpolation coefficient
    //      * @return This vector for chaining.
    //      */
    //     T lerp(T target, float alpha);
    //
    //     /**
    //      * Interpolates between this vector and the given target vector by alpha (within range [0,1]) using the given Interpolation
    //      * method. the result is stored in this vector.
    //      * @param target The target vector
    //      * @param alpha The interpolation coefficient
    //      * @param interpolator An Interpolation object describing the used interpolation method
    //      * @return This vector for chaining.
    //      */
    //     T interpolate(T target, float alpha, Interp interpolator);
    //
    //     /**
    //      * Sets this vector to the unit vector with a random direction
    //      * @return This vector for chaining
    //      */
    //     T setToRandomDirection();
    //
    //     /** @return Whether this vector is a unit length vector */
    //     boolean isUnit();
    //
    //     /** @return Whether this vector is a unit length vector within the given margin. */
    //     boolean isUnit(final float margin);
    //
    //     /** @return Whether this vector is a zero vector */
    //     boolean isZero();
    //
    //     /** @return Whether the length of this vector is smaller than the given margin */
    //     boolean isZero(final float margin);
    //
    //     /** @return true if this vector is in line with the other vector (either in the same or the opposite direction) */
    //     boolean isOnLine(T other, float epsilon);
    //
    //     /** @return true if this vector is in line with the other vector (either in the same or the opposite direction) */
    //     boolean isOnLine(T other);
    //
    //     /**
    //      * @return true if this vector is collinear with the other vector ({@link #isOnLine(Vector, float)} &&
    //      * {@link #hasSameDirection(Vector)}).
    //      */
    //     boolean isCollinear(T other, float epsilon);
    //
    //     /**
    //      * @return true if this vector is collinear with the other vector ({@link #isOnLine(Vector)} &&
    //      * {@link #hasSameDirection(Vector)}).
    //      */
    //     boolean isCollinear(T other);
    //
    //     /**
    //      * @return true if this vector is opposite collinear with the other vector ({@link #isOnLine(Vector, float)} &&
    //      * {@link #hasOppositeDirection(Vector)}).
    //      */
    //     boolean isCollinearOpposite(T other, float epsilon);
    //
    //     /**
    //      * @return true if this vector is opposite collinear with the other vector ({@link #isOnLine(Vector)} &&
    //      * {@link #hasOppositeDirection(Vector)}).
    //      */
    //     boolean isCollinearOpposite(T other);
    //
    //     /** @return Whether this vector is perpendicular with the other vector. True if the dot product is 0. */
    //     boolean isPerpendicular(T other);

    /// Linearly interpolates between this vector and the target vector by alpha which is in the range [0,1]. The result is stored in this vector.
    fn lerp(target: T, alpha: f32) -> T;

    /// Interpolates between this vector and the given target vector by alpha (within range [0,1]) using the given Interpolation method. the result is stored in this vector.
    fn interpolate(target: T, alpha: f32, interpolator: Interp) -> T;

    /// Sets this vector to the unit vector with a random direction
    fn set_to_random_direction() -> T;

    /// Returns true if this vector is a unit length vector
    fn is_unit() -> bool;

    /// Returns true if this vector is a unit length vector within the given margin.
    fn is_unit_margin(margin: f32) -> bool;

    /// Returns true if this vector is a zero vector
    fn is_zero() -> bool;

    /// Returns true if the length of this vector is smaller than the given margin
    fn is_zero_margin(margin: f32) -> bool;

    /// Returns true if this vector is in line with the other vector (either in the same or the opposite direction)
    fn is_on_line(other: T, epsilon: f32) -> bool;

    /// Returns true if this vector is in line with the other vector (either in the same or the opposite direction)
    fn is_on_line2(other: T) -> bool;

    /// Returns true if this vector is collinear with the other vector (isOnLine(other, epsilon) && hasSameDirection(other)).
    fn is_collinear(other: T, epsilon: f32) -> bool;

    /// Returns true if this vector is collinear with the other vector (isOnLine(other) && hasSameDirection(other)).
    fn is_collinear2(other: T) -> bool;

    /// Returns true if this vector is opposite collinear with the other vector (isOnLine(other, epsilon) && hasOppositeDirection(other)).
    fn is_collinear_opposite(other: T, epsilon: f32) -> bool;

    /// Returns true if this vector is opposite collinear with the other vector (isOnLine(other) && hasOppositeDirection(other)).
    fn is_collinear_opposite2(other: T) -> bool;

    /// Returns Whether this vector is perpendicular with the other vector. True if the dot product is 0.
    fn is_perpendicular(other: T) -> bool;

    // /**
    //      * @param epsilon a positive small number close to zero
    //      * @return Whether this vector is perpendicular with the other vector. True if the dot product is 0.
    //      */
    //     boolean isPerpendicular(T other, float epsilon);
    //
    //     /** @return Whether this vector has similar direction compared to the other vector. True if the normalized dot product is > 0. */
    //     boolean hasSameDirection(T other);
    //
    //     /** @return Whether this vector has opposite direction compared to the other vector. True if the normalized dot product is < 0. */
    //     boolean hasOppositeDirection(T other);
    //
    //     /**
    //      * Compares this vector with the other vector, using the supplied epsilon for fuzzy equality testing.
    //      * @return whether the vectors have fuzzy equality.
    //      */
    //     boolean epsilonEquals(T other, float epsilon);
    //
    //     /**
    //      * First scale a supplied vector, then add it to this vector.
    //      * @param v addition vector
    //      * @param scalar for scaling the addition vector
    //      */
    //     T mulAdd(T v, float scalar);
    //
    //     /**
    //      * First scale a supplied vector, then add it to this vector.
    //      * @param v addition vector
    //      * @param mulVec vector by whose values the addition vector will be scaled
    //      */
    //     T mulAdd(T v, T mulVec);
    //
    //     /**
    //      * Sets the components of this vector to 0
    //      * @return This vector for chaining
    //      */
    //     T setZero();
    //
    //     default T plus(T other){
    //         return add(other);
    //     }
    //
    //     default T minus(T other){
    //         return sub(other);
    //     }
    //
    //     default T unaryMinus(){
    //         return scl(-1);
    //     }
    //
    //     default T times(T other){
    //         return scl(other);
    //     }

    /// # Parameters
    /// * `other` - the other vector
    /// * `epsilon` - a positive small number close to zero
    fn is_perpendicular_epsilon(other: T, epsilon: f32) -> bool;

    /// # Returns
    /// Whether this vector has similar direction compared to the other vector. True if the normalized dot product is > 0.
    fn has_same_direction(other: T) -> bool;

    /// # Returns
    /// Whether this vector has opposite direction compared to the other vector. True if the normalized dot product is < 0.
    fn has_opposite_direction(other: T) -> bool;

    /// Compares this vector with the other vector, using the supplied epsilon for fuzzy equality testing.
    /// # Returns
    /// whether the vectors have fuzzy equality.
    fn epsilon_equals(other: T, epsilon: f32) -> bool;

    /// First scale a supplied vector, then add it to this vector.
    /// # Parameters
    /// * `v` - addition vector
    /// * `scalar` - for scaling the addition vector
    fn mul_add(v: T, scalar: f32) -> T;

    /// First scale a supplied vector, then add it to this vector.
    /// # Parameters
    /// * `v` - addition vector
    /// * `mul_vec` - vector by whose values the addition vector will be scaled
    fn mul_add2(v: T, mul_vec: T) -> T;

    /// Sets the components of this vector to 0
    fn set_zero() -> T;

    fn plus(other: T) -> T;
    fn minus(other: T) -> T;
    fn unary_minus() -> T;
    fn times(other: T) -> T;

}