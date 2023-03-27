use std::fmt::Display;

use rand::random;

use crate::arc_core::math::geom::interp::{Interp, Interpolation};
use crate::arc_core::math::geom::mathf::{clamp, zero, DEGREES_TO_RADIANS, PI};
use crate::arc_core::math::geom::position::Position;
use crate::arc_core::math::geom::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Creates a new vector with the given components.
    /// # Parameters
    /// * `x` - The x component.
    /// * `y` - The y component.
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn trns(&mut self, angle: f32, amount: f32) -> Vec2 {
        self.set(amount, 0.0).rotate(angle)
    }

    pub fn trns_xy(&mut self, angle: f32, x: f32, y: f32) -> Vec2 {
        self.set(x, y).rotate(angle)
    }

    /// Snaps this vector's coordinates to integers.
    pub fn snap(&mut self) -> Vec2 {
        self.x = self.x.round();
        self.y = self.y.round();
        *self
    }

    pub fn set_pos(&mut self, v: Box<dyn Position>) -> Vec2 {
        self.x = v.get_x();
        self.y = v.get_y();
        *self
    }

    pub fn set(&mut self, x: f32, y: f32) -> Vec2 {
        self.x = x;
        self.y = y;
        *self
    }

    pub fn rotate(&mut self, degrees: f32) -> Vec2 {
        self.rotate_rad(degrees * DEGREES_TO_RADIANS)
    }

    pub fn rotate_rad(&mut self, radians: f32) -> Vec2 {
        let cos = radians.cos();
        let sin = radians.sin();
        let new_x = self.x * cos - self.y * sin;
        let new_y = self.x * sin + self.y * cos;
        self.x = new_x;
        self.y = new_y;
        *self
    }
}

impl Position for Vec2 {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vector<Vec2> for Vec2 {
    fn cpy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn len2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn limit(&mut self, limit: f32) -> Vec2 {
        self.limit2(limit * limit)
    }

    fn limit2(&mut self, limit2: f32) -> Vec2 {
        let len2 = self.len2();
        if len2 > limit2 {
            self.scl_f32((limit2 / len2).sqrt())
        } else {
            self.cpy()
        }
    }

    fn set_length(&mut self, len: f32) -> Vec2 {
        self.set_length2(len * len)
    }

    fn set_length2(&mut self, len2: f32) -> Vec2 {
        let old_len2 = self.len2();
        if old_len2 == 0.0 || old_len2 == len2 {
            self.cpy()
        } else {
            self.scl_f32((len2 / old_len2).sqrt())
        }
    }

    fn clamp(&mut self, min: f32, max: f32) -> Vec2 {
        self.x = clamp(self.x, min, max);
        self.y = clamp(self.y, min, max);
        *self
    }

    fn set(&mut self, v: Vec2) -> Vec2 {
        self.set(v.x, v.y)
    }

    fn sub(&mut self, v: Vec2) -> Vec2 {
        self.x -= v.x;
        self.y -= v.y;
        *self
    }

    fn nor(&mut self) -> Vec2 {
        let len = self.len();
        if len != 0.0 {
            self.scl_f32(1.0 / len)
        } else {
            *self
        }
    }

    fn add(&mut self, v: Vec2) -> Vec2 {
        self.x += v.x;
        self.y += v.y;
        *self
    }

    fn dot(&self, v: Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    fn scl_f32(&mut self, scalar: f32) -> Vec2 {
        self.x *= scalar;
        self.y *= scalar;
        *self
    }

    fn scl_vec(&mut self, v: Vec2) -> Vec2 {
        self.x *= v.x;
        self.y *= v.y;
        *self
    }

    fn div(&mut self, other: Vec2) -> Vec2 {
        self.x /= other.x;
        self.y /= other.y;
        *self
    }

    fn dst(&self, v: Vec2) -> f32 {
        let x_d = v.x - self.x;
        let y_d = v.y - self.y;
        (x_d * x_d + y_d * y_d).sqrt()
    }

    fn dst2(&self, v: Vec2) -> f32 {
        let x_d = v.x - self.x;
        let y_d = v.y - self.y;
        x_d * x_d + y_d * y_d
    }

    fn lerp(&mut self, target: Vec2, alpha: f32) -> Vec2 {
        let inv_alpha = 1.0 - alpha;
        self.x = self.x * inv_alpha + target.x * alpha;
        self.y = self.y * inv_alpha + target.y * alpha;
        *self
    }

    fn interpolate(&mut self, target: Vec2, alpha: f32, interpolator: Interp) -> Vec2 {
        self.lerp(target, interpolator.apply(alpha))
    }

    fn set_to_random_direction(&mut self) -> Vec2 {
        let theta: f32 = random::<f32>() * 2.0 * PI;
        self.x = theta.cos();
        self.y = theta.sin();
        *self
    }

    fn is_unit(&self) -> bool {
        self.is_unit_margin(0.000000001)
    }

    fn is_unit_margin(&self, margin: f32) -> bool {
        (self.len2() - 1.0).abs() < margin
    }

    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    fn is_zero_margin(&self, margin: f32) -> bool {
        self.len2() < margin
    }

    fn is_on_line(&self, other: Vec2, epsilon: f32) -> bool {
        zero(self.x * other.y - self.y * other.x, Some(epsilon))
    }

    fn is_on_line2(&self, other: Vec2) -> bool {
        zero(self.x * other.y - self.y * other.x, None)
    }

    fn is_collinear(&self, other: Vec2, epsilon: f32) -> bool {
        self.is_on_line(other, epsilon) && self.dot(other) > 0.0
    }

    fn is_collinear2(&self, other: Vec2) -> bool {
        self.is_on_line2(other) && self.dot(other) > 0.0
    }

    fn is_collinear_opposite(&self, other: Vec2, epsilon: f32) -> bool {
        self.is_on_line(other, epsilon) && self.dot(other) < 0.0
    }

    fn is_collinear_opposite2(&self, other: Vec2) -> bool {
        self.is_on_line2(other) && self.dot(other) < 0.0
    }

    fn is_perpendicular(&self, other: Vec2) -> bool {
        zero(self.dot(other), None)
    }

    fn is_perpendicular_epsilon(&self, other: Vec2, epsilon: f32) -> bool {
        zero(self.dot(other), Some(epsilon))
    }

    fn has_same_direction(&self, other: Vec2) -> bool {
        self.dot(other) > 0.0
    }

    fn has_opposite_direction(&self, other: Vec2) -> bool {
        self.dot(other) < 0.0
    }

    fn epsilon_equals(&self, other: Vec2, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon && (self.y - other.y).abs() < epsilon
    }

    fn mul_add(&mut self, v: Vec2, scalar: f32) -> Vec2 {
        self.x += v.x * scalar;
        self.y += v.y * scalar;
        *self
    }

    fn mul_add2(&mut self, v: Vec2, mul_vec: Vec2) -> Vec2 {
        self.x += v.x * mul_vec.x;
        self.y += v.y * mul_vec.y;
        *self
    }

    fn set_zero(&mut self) -> Vec2 {
        self.x = 0.0;
        self.y = 0.0;
        *self
    }
}
