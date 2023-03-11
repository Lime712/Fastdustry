use lazy_static::lazy_static;
use crate::arc_core::math::geom::mathf::*;

pub trait Interpolation {
    fn apply(&self, a: f32) -> f32;
}


pub struct Interp {
    pub f: fn(f32) -> f32,
}

impl Interp {
    pub fn new(f: fn(f32) -> f32) -> Interp {
        Interp { f }
    }
}

impl Interpolation for Interp {
    fn apply(&self, x: f32) -> f32 {
        (self.f)(x)
    }
}

pub struct Pow {
    pub power: f32,
}

impl Pow {
    pub fn new(power: f32) -> Pow {
        Pow { power }
    }
}

impl Interpolation for Pow {
    fn apply(&self, x: f32) -> f32 {
    //     if(a <= 0.5f) return (float)Math.pow(a * 2, power) / 2;
        //             return (float)Math.pow((a - 1) * 2, power) / (power % 2 == 0 ? -2 : 2) + 1;
        if x <= 0.5 {
            (x * 2.0).powf(self.power) / 2.0
        } else {
            (x - 1.0) * (2.0 as f32)
                .powf(self.power)
                / (if self.power % 2.0 == 0.0 { -2.0 } else { 2.0 })
                + 1.0
        }
    }
}

pub struct PowIn {
    pub power: f32,
}

impl PowIn {
    pub fn new(power: f32) -> PowIn {
        PowIn { power }
    }
}

impl Interpolation for PowIn {
    fn apply(&self, x: f32) -> f32 {
        x.powf(self.power)
    }
}

pub struct PowOut {
    pub power: f32,
}

impl PowOut {
    pub fn new(power: f32) -> PowOut {
        PowOut { power }
    }
}

impl Interpolation for PowOut {
    fn apply(&self, x: f32) -> f32 {
        // return (float)Math.pow(a - 1, power) * (power % 2 == 0 ? -1 : 1) + 1;
        (x - 1.0).powf(self.power) * (if self.power % 2.0 == 0.0 { -1.0 } else { 1.0 }) + 1.0
    }
}

pub struct Exp {
    pub value: f32,
    pub power: f32,
    pub min: f32,
    pub scale: f32,
}

impl Exp {
    pub fn new(value: f32, power: f32) -> Exp {
        let min = value.powf(power);
        let scale = 1.0 / (1.0 - min);
        Exp {
            value,
            power,
            min,
            scale,
        }
    }
}

impl Interpolation for Exp {
    fn apply(&self, x: f32) -> f32 {
  //       if(a <= 0.5f) return ((float)Math.pow(value, power * (a * 2 - 1)) - min) * scale / 2;
        //             return (2 - ((float)Math.pow(value, -power * (a * 2 - 1)) - min) * scale) / 2;
        if x <= 0.5 {
            ((self.value).powf(self.power * (x * 2.0 - 1.0)) - self.min) * self.scale / 2.0
        } else {
            (2.0 - ((self.value).powf(-self.power * (x * 2.0 - 1.0)) - self.min) * self.scale) / 2.0
        }
  }
}

pub struct ExpIn {
    pub value: f32,
    pub power: f32,
    pub min: f32,
    pub scale: f32,
}

impl ExpIn {
    pub fn new(value: f32, power: f32) -> ExpIn {
        let min = value.powf(power);
        let scale = 1.0 / (1.0 - min);
        ExpIn {
            value,
            power,
            min,
            scale,
        }
    }
}

impl Interpolation for ExpIn {
    fn apply(&self, x: f32) -> f32 {
        ((self.value).powf(self.power * (x - 1.0)) - self.min) * self.scale
    }
}

pub struct ExpOut {
    pub value: f32,
    pub power: f32,
    pub min: f32,
    pub scale: f32,
}

impl ExpOut {
    pub fn new(value: f32, power: f32) -> ExpOut {
        let min = value.powf(power);
        let scale = 1.0 / (1.0 - min);
        ExpOut {
            value,
            power,
            min,
            scale,
        }
    }
}

impl Interpolation for ExpOut {
    fn apply(&self, x: f32) -> f32 {
        1.0 - ((self.value).powf(-self.power * x) - self.min) * self.scale
    }
}

pub struct Elastic {
    pub value: f32,
    pub power: f32,
    pub scale: f32,
    pub bounces: f32,
}

impl Elastic {
    pub fn new(value: f32, power: f32, bounces: i32, scale: f32) -> Elastic {
        Elastic {
            value,
            power,
            scale,
            bounces: bounces as f32 * PI * (if bounces % 2 == 0 { 1.0 } else { -1.0 }),
        }
    }
}

impl Interpolation for Elastic {
    fn apply(&self, x: f32) -> f32 {
    //     if(a <= 0.5f){
        //                 a *= 2;
        //                 return (float)Math.pow(value, power * (a - 1)) * Mathf.sin(a * bounces) * scale / 2;
        //             }
        //             a = 1 - a;
        //             a *= 2;
        //             return 1 - (float)Math.pow(value, power * (a - 1)) * Mathf.sin((a) * bounces) * scale / 2;
        if x <= 0.5 {
            (self.value).powf(self.power * (x * 2.0 - 1.0))
                * (x * 2.0 * self.bounces).sin()
                * self.scale
                / 2.0
        } else {
            let a = 1.0 - x;
            1.0
                - (self.value).powf(self.power * (a * 2.0 - 1.0))
                    * (a * 2.0 * self.bounces).sin()
                    * self.scale
                    / 2.0
        }
    }
}

pub struct ElasticIn {
    pub value: f32,
    pub power: f32,
    pub scale: f32,
    pub bounces: f32,
}

impl ElasticIn {
    pub fn new(value: f32, power: f32, bounces: i32, scale: f32) -> ElasticIn {
        ElasticIn {
            value,
            power,
            scale,
            bounces: bounces as f32 * PI * (if bounces % 2 == 0 { 1.0 } else { -1.0 }),
        }
    }
}

impl Interpolation for ElasticIn {
    fn apply(&self, x: f32) -> f32 {
        (self.value).powf(self.power * (x - 1.0)) * (x * self.bounces).sin() * self.scale
    }
}

pub struct ElasticOut {
    pub value: f32,
    pub power: f32,
    pub scale: f32,
    pub bounces: f32,
}

impl ElasticOut {
    pub fn new(value: f32, power: f32, bounces: i32, scale: f32) -> ElasticOut {
        ElasticOut {
            value,
            power,
            scale,
            bounces: bounces as f32 * PI * (if bounces % 2 == 0 { 1.0 } else { -1.0 }),
        }
    }
}

impl Interpolation for ElasticOut {
    fn apply(&self, x: f32) -> f32 {
        1.0 - (self.value).powf(-self.power * x) * (x * self.bounces).sin() * self.scale
    }
}

lazy_static! {
    // original code
    // Interp reverse = a -> 1f - a;
    //     /** Aka "smoothstep". */
    //     Interp smooth = a -> a * a * (3 - 2 * a);
    //
    //     //
    //     Interp smooth2 = a -> {
    //         a = a * a * (3 - 2 * a);
    //         return a * a * (3 - 2 * a);
    //     };
    //
    //     Interp one = a -> 1f;
    //     Interp zero = a -> 0f;
    //     Interp slope = Mathf::slope;
    //
    //     //
    //     /** By Ken Perlin. */
    //     Interp smoother = a -> a * a * a * (a * (a * 6 - 15) + 10);
    //     Interp fade = smoother;
    //     Pow pow2 = new Pow(2);
    //     /** Slow, then fast. */
    //     PowIn pow2In = new PowIn(2);
    //     PowIn slowFast = pow2In;
    //     /** Fast, then slow. */
    //     PowOut pow2Out = new PowOut(2);
    //     PowOut fastSlow = pow2Out;
    //     Interp pow2InInverse = a -> (float)Math.sqrt(a);
    //     Interp pow2OutInverse = a -> 1 - (float)Math.sqrt(-(a - 1));
    //     Pow pow3 = new Pow(3);
    //     PowIn pow3In = new PowIn(3);
    //     PowOut pow3Out = new PowOut(3);
    //     Interp pow3InInverse = a -> (float)Math.cbrt(a);
    //     Interp pow3OutInverse = a -> 1 - (float)Math.cbrt(-(a - 1));
    //     Pow pow4 = new Pow(4);
    //     PowIn pow4In = new PowIn(4);
    //     PowOut pow4Out = new PowOut(4);
    //     Pow pow5 = new Pow(5);
    //     PowIn pow5In = new PowIn(5);
    //     PowIn pow10In = new PowIn(10);
    //     PowOut pow10Out = new PowOut(10);
    //     PowOut pow5Out = new PowOut(5);
    //     Interp sine = a -> (1 - Mathf.cos(a * Mathf.PI)) / 2;
    //     Interp sineIn = a -> 1 - Mathf.cos(a * Mathf.PI / 2);
    //     Interp sineOut = a -> Mathf.sin(a * Mathf.PI / 2);
    //     Exp exp10 = new Exp(2, 10);
    //     ExpIn exp10In = new ExpIn(2, 10);
    //     ExpOut exp10Out = new ExpOut(2, 10);
    //     Exp exp5 = new Exp(2, 5);
    //     ExpIn exp5In = new ExpIn(2, 5);
    //     ExpOut exp5Out = new ExpOut(2, 5);
    //     Interp circle = a -> {
    //         if(a <= 0.5f){
    //             a *= 2;
    //             return (1 - (float)Math.sqrt(1 - a * a)) / 2;
    //         }
    //         a--;
    //         a *= 2;
    //         return ((float)Math.sqrt(1 - a * a) + 1) / 2;
    //     };
    //     Interp circleIn = a -> 1 - (float)Math.sqrt(1 - a * a);
    //     Interp circleOut = a -> {
    //         a--;
    //         return (float)Math.sqrt(1 - a * a);
    //     };
    //     Elastic elastic = new Elastic(2, 10, 7, 1);
    //     ElasticIn elasticIn = new ElasticIn(2, 10, 6, 1);
    //     ElasticOut elasticOut = new ElasticOut(2, 10, 7, 1);
    //     Swing swing = new Swing(1.5f);
    //     SwingIn swingIn = new SwingIn(2f);
    //     SwingOut swingOut = new SwingOut(2f);
    //     Bounce bounce = new Bounce(4);
    //     BounceIn bounceIn = new BounceIn(4);
    //     BounceOut bounceOut = new BounceOut(4);
    pub static ref LINEAR: Interp = Interp::new(|x| x);
    pub static ref REVERSE: Interp = Interp::new(|x| 1.0 - x);
    /// aka "smoothstep"
    pub static ref SMOOTH: Interp = Interp::new(|x| x * x * (3.0 - 2.0 * x));
    pub static ref SMOOTH2: Interp = Interp::new(|x| {
        let x = x * x * (3.0 - 2.0 * x);
        x * x * (3.0 - 2.0 * x)
    });
    pub static ref ONE: Interp = Interp::new(|_| 1.0);
    pub static ref ZERO: Interp = Interp::new(|_| 0.0);
    pub static ref SLOPE: Interp = Interp::new(slope);
    /// by Ken Perlin
    pub static ref SMOOTHER: Interp = Interp::new(|x| {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    });
    pub static ref FADE: Interp = Interp::new(|x| {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    });
    pub static ref POW2: Pow = Pow::new(2.0);
    /// slow, then fast
    pub static ref POW2_IN: PowIn = PowIn::new(2.0);
    pub static ref SLOW_FAST: PowIn = PowIn::new(2.0);
    /// fast, then slow
    pub static ref POW2_OUT: PowOut = PowOut::new(2.0);
    pub static ref FAST_SLOW: PowOut = PowOut::new(2.0);
    pub static ref POW2_IN_INVERSE: Interp = Interp::new(|x| x.sqrt());
    pub static ref POW2_OUT_INVERSE: Interp = Interp::new(|x| 1.0 - (-x + 1.0).sqrt());
    pub static ref POW3: Pow = Pow::new(3.0);
    pub static ref POW3_IN: PowIn = PowIn::new(3.0);
    pub static ref POW3_OUT: PowOut = PowOut::new(3.0);
    pub static ref POW3_IN_INVERSE: Interp = Interp::new(|x| x.cbrt());
    pub static ref POW3_OUT_INVERSE: Interp = Interp::new(|x| 1.0 - (-x + 1.0).cbrt());
    pub static ref POW4: Pow = Pow::new(4.0);
    pub static ref POW4_IN: PowIn = PowIn::new(4.0);
    pub static ref POW4_OUT: PowOut = PowOut::new(4.0);
    pub static ref POW5: Pow = Pow::new(5.0);
    pub static ref POW5_IN: PowIn = PowIn::new(5.0);
    pub static ref POW10_IN: PowIn = PowIn::new(10.0);
    pub static ref POW10_OUT: PowOut = PowOut::new(10.0);
    pub static ref POW5_OUT: PowOut = PowOut::new(5.0);
    pub static ref SINE: Interp = Interp::new(|x| (1.0 - (x * std::f32::consts::PI).cos()) / 2.0);
    pub static ref SINE_IN: Interp = Interp::new(|x| 1.0 - (x * std::f32::consts::PI / 2.0).cos());
    pub static ref SINE_OUT: Interp = Interp::new(|x| (x * std::f32::consts::PI / 2.0).sin());
}