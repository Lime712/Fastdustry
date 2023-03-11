use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::arc_core::math::geom::vec2::Vec2;

pub trait ParticleConsumer {
    fn accept(&mut self, x: f32, y: f32, fin: f32, fout: f32);
}

lazy_static! {
    static ref VEC2: Mutex<Vec2> = Mutex::new(Vec2::new(0.0, 0.0));
}

// original code:
// public static float forwardDistance(float angle1, float angle2){
//         return Math.abs(angle1 - angle2);
//     }
//
//     public static float backwardDistance(float angle1, float angle2){
//         return 360 - Math.abs(angle1 - angle2);
//     }
//
//     public static boolean within(float a, float b, float margin){
//         return angleDist(a, b) <= margin;
//     }
//
//     public static float angleDist(float a, float b){
//         a = Mathf.mod(a, 360f);
//         b = Mathf.mod(b, 360f);
//         return Math.min((a - b) < 0 ? a - b + 360 : a - b, (b - a) < 0 ? b - a + 360 : b - a);
//     }
//
//     public static boolean near(float a, float b, float range){
//         return angleDist(a, b) < range;
//     }
//
//     public static float clampRange(float angle, float dest, float range){
//         float dst = angleDist(angle, dest);
//         return dst <= range ? angle : moveToward(angle, dest, dst - range);
//     }
//
//     public static float moveToward(float angle, float to, float speed){
//         if(Math.abs(angleDist(angle, to)) < speed) return to;
//         angle = Mathf.mod(angle, 360f);
//         to = Mathf.mod(to, 360f);
//
//         if(angle > to == backwardDistance(angle, to) > forwardDistance(angle, to)){
//             angle -= speed;
//         }else{
//             angle += speed;
//         }
//
//         return angle;
//     }
//
//     public static float angle(float x, float y){
//         return angle(0, 0, x, y);
//     }
//
//     public static float angle(float x, float y, float x2, float y2){
//         float ang = Mathf.atan2(x2 - x, y2 - y) * Mathf.radDeg;
//         if(ang < 0) ang += 360f;
//         return ang;
//     }
//
//     public static float angleRad(float x, float y, float x2, float y2){
//         return Mathf.atan2(x2 - x, y2 - y);
//     }
//
//     public static float trnsx(float angle, float len){
//         return len * Mathf.cosDeg(angle);
//     }
//
//     public static float trnsy(float angle, float len){
//         return len * Mathf.sinDeg(angle);
//     }
//
//     public static float trnsx(float angle, float x, float y){
//         return rv.set(x, y).rotate(angle).x;
//     }
//
//     public static float trnsy(float angle, float x, float y){
//         return rv.set(x, y).rotate(angle).y;
//     }
//
//     public static float mouseAngle(float cx, float cy){
//         Vec2 avector = Core.camera.project(cx, cy);
//         return angle(avector.x, avector.y, Core.input.mouseX(), Core.input.mouseY());
//     }
//
//     public static void circleVectors(int points, float length, Floatc2 pos){
//         for(int i = 0; i < points; i++){
//             float f = i * 360f / points;
//             pos.get(trnsx(f, length), trnsy(f, length));
//         }
//     }

pub fn forward_distance(angle1: f32, angle2: f32) -> f32 {
    (angle1 - angle2).abs()
}

pub fn backward_distance(angle1: f32, angle2: f32) -> f32 {
    360.0 - (angle1 - angle2).abs()
}

pub fn within(a: f32, b: f32, margin: f32) -> bool {
    angle_dist(a, b) <= margin
}

pub fn angle_dist(a: f32, b: f32) -> f32 {
    let a = a.rem_euclid(360.0);
    let b = b.rem_euclid(360.0);
    f32::min(
        if (a - b) < 0.0 {
            a - b + 360.0
        } else {
            a - b
        },
        if (b - a) < 0.0 {
            b - a + 360.0
        } else {
            b - a
        },
    )
}

pub fn near(a: f32, b: f32, range: f32) -> bool {
    angle_dist(a, b) < range
}

pub fn clamp_range(angle: f32, dest: f32, range: f32) -> f32 {
    let dst = angle_dist(angle, dest);
    if dst <= range {
        angle
    } else {
        move_toward(angle, dest, dst - range)
    }
}

pub fn move_toward(angle: f32, to: f32, speed: f32) -> f32 {
    if (angle_dist(angle, to)).abs() < speed {
        to
    } else {
        let angle = angle.rem_euclid(360.0);
        let to = to.rem_euclid(360.0);

        if (angle > to) == (backward_distance(angle, to) > forward_distance(angle, to)) {
            angle - speed
        } else {
            angle + speed
        }
    }
}

pub fn angle_xy(x: f32, y: f32) -> f32 {
    angle(0.0, 0.0, x, y)
}

pub fn angle(x: f32, y: f32, x2: f32, y2: f32) -> f32 {
    let ang = ((x2 - x) / (y2 - y)).atan() * (180.0 / std::f32::consts::PI);
    if ang < 0.0 {
        ang + 360.0
    } else {
        ang
    }
}

pub fn angle_rad(x: f32, y: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x) / (y2 - y).atan()
}

pub fn trns_x(angle: f32, len: f32) -> f32 {
    len * angle.to_radians().cos()
}

pub fn trns_y(angle: f32, len: f32) -> f32 {
    len * angle.to_radians().sin()
}

pub fn trns_x_y(angle: f32, x: f32, y: f32) -> f32 {
    VEC2.lock().unwrap().set(x, y).rotate(angle).x
}

pub fn trns_y_y(angle: f32, x: f32, y: f32) -> f32 {
    VEC2.lock().unwrap().set(x, y).rotate(angle).y
}

pub fn circle_vectors(points: i32, length: f32, pos: &mut dyn FnMut(f32, f32)) {
    for i in 0..points {
        let f = i as f32 * 360.0 / points as f32;
        pos(trns_x(f, length), trns_y(f, length));
    }
}

