use crate::arc_core::math::geom::angle::angle;

pub trait Position {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn angle_to(&self, other: Box<dyn Position>) -> f32 {
        angle(self.get_x(), self.get_y(), other.get_x(), other.get_y())
    }
}
