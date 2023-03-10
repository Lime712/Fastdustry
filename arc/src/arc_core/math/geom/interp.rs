use lazy_static::lazy_static;

pub struct Interp {
    pub f: fn(f32) -> f32,
}

impl Interp {
    pub fn new(f: fn(f32) -> f32) -> Interp {
        Interp { f }
    }

    pub fn apply(&self, x: f32) -> f32 {
        (self.f)(x)
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
    pub static ref SMOOTH: Interp = Interp::new(|x| x * x * (3.0 - 2.0 * x));
    pub static ref SMOOTH2: Interp = Interp::new(|x| {
        let a = x * x * (3.0 - 2.0 * x);
        a * a * (3.0 - 2.0 * a)
    });
    pub static ref ONE: Interp = Interp::new(|_x| 1.0);
    pub static ref ZERO: Interp = Interp::new(|_x| 0.0);
    pub static ref SLOPE: Interp = Interp::new(|x| x);
    pub static ref SMOOTHER: Interp = Interp::new(|x| {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    });
    pub static ref FADE: Interp = Interp::new(|x| {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    });
    pub static ref POW2: Interp = Interp::new(|x| x * x);
    pub static ref POW2_IN: Interp = Interp::new(|x| x * x);
    pub static ref SLOW_FAST: Interp = Interp::new(|x| x * x);
    pub static ref POW2_OUT: Interp = Interp::new(|x| 1.0 - (1.0 - x) * (1.0 - x));
    pub static ref FAST_SLOW: Interp = Interp::new(|x| 1.0 - (1.0 - x) * (1.0 - x));
    pub static ref POW2_INVERSE: Interp = Interp::new(|x| x.sqrt());
    pub static ref POW2_OUT_INVERSE: Interp = Interp::new(|x| 1.0 - (1.0 - x).sqrt());
    pub static ref POW3: Interp = Interp::new(|x| x * x * x);
    pub static ref POW3_IN: Interp = Interp::new(|x| x * x * x);
    pub static ref POW3_OUT: Interp = Interp::new(|x| 1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x));
    pub static ref POW3_INVERSE: Interp = Interp::new(|x| x.cbrt());
    pub static ref POW3_OUT_INVERSE: Interp = Interp::new(|x| 1.0 - (1.0 - x).cbrt());
    pub static ref POW4: Interp = Interp::new(|x| x * x * x * x);
    pub static ref POW4_IN: Interp = Interp::new(|x| x * x * x * x);
    pub static ref POW4_OUT: Interp = Interp::new(|x| 1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x) * (1.0 - x));
    pub static ref POW5: Interp = Interp::new(|x| x * x * x * x * x);
    pub static ref POW5_IN: Interp = Interp::new(|x| x * x * x * x * x);
    pub static ref POW5_OUT: Interp = Interp::new(|x| 1.0

}