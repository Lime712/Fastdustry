use std::char::from_digit;
use std::cmp::{max, min};

pub const WHITE: Color = Color::new(1., 1., 1., 1.);
pub const LIGHT_GRAY: Color = Color::from_rgba8888(0xbfbfbfff);
pub const GRAY: Color = Color::from_rgba8888(0x7f7f7fff);
pub const DARK_GRAY: Color = Color::from_rgba8888(0x3f3f3fff);
pub const BLACK: Color = Color::new(0., 0., 0., 1.);
pub const CLEAR: Color = Color::new(0., 0., 0., 0.);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Color {
    tmp_hsv: [f32; 3],
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        let mut c = Self { tmp_hsv: [0., 0., 0.], r, g, b, a };
        c.clamp();
        c
    }

    pub const fn value_of(hex: &str) -> Self {
        let hex = hex.to_string();
        let offset = if hex.starts_with("#") { 1 } else { 0 };


        let r = Color::parse_hex(hex.clone(), offset, offset + 2);
        let g = Color::parse_hex(hex.clone(), offset + 2, offset + 4);
        let b = Color::parse_hex(hex.clone(), offset + 4, offset + 6);
        let a = if hex.len() - offset != 8 {
            255
        } else {
            Color::parse_hex(hex.clone(), offset + 6, offset + 8)
        };
        Self::new(r as f32 / 255., g as f32 / 255., b as f32 / 255., a as f32 / 255.)
    }

    pub fn parse_hex(string: String, from: usize, to: usize) -> u8 {
        let mut total = 0;
        for i in from..to {
            let c = string.chars().nth(i).unwrap();
            // lets hope this is correct lmao
            // todo: definitly needs testing lmfao
            total += from_digit(c.to_digit(16).unwrap(), 16).unwrap() as u8 * 16u8.pow((to - i - 1) as u32);
        }
        total
    }

    /// Packs the color components into a 32-bit integer with the format ABGR and then converts it to a float. Note that no range
    /// checking is performed for higher performance.
    /// # Arguments
    /// * `r`, `g`, `b`, `a` - The red component of the color, 0 - 255
    /// # Returns
    /// The packed color as a float
    /// see also [Color::int_to_float_color]
    pub fn to_float_bits(r: u8, g: u8, b: u8, a: u8) -> f32 {
        let color = (a << 24) | (b << 16) | (g << 8) | r;
        Color::int_to_float_color(color)
    }

    // todo: add the other functions that turn like floats to color and stuff

    pub fn rgba8888(r: f32, g: f32, b: f32, a: f32) -> i32 {
        // some bitwise magic i dont get (hopefully it works)
        return (((r * 255) as i32) << 24) | (((g * 255) as i32) << 16) | (((b * 255) as i32) << 8) | (a * 255) as i32;
    }

    pub fn self_rgba8888(&self) -> i32 {
        Color::rgba8888(self.r, self.g, self.b, self.a)
    }

    pub fn from_rgba8888(value: i32) -> Color {
        let r = ((value >> 24) & 0xff) as f32 / 255.;
        let g = ((value >> 16) & 0xff) as f32 / 255.;
        let b = ((value >> 8) & 0xff) as f32 / 255.;
        let a = (value & 0xff) as f32 / 255.;
        Color::new(r, g, b, a)
    }

    /// Creates a grayscale color.
    pub fn gray(value: f32) -> Self {
        Self::new(value, value, value, 1.)
    }

    /// Creates a color from 0-255 scaled RGB values.
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r / 255., g / 255., b / 255., 1.)
    }

    /// Converts the color from a float ABGR encoding to an int ABGR encoding. The alpha is expanded from 0-254 in the float
    /// encoding (see {@link #intToFloatColor(int)}) to 0-255, which means converting from int to float and back to int can be
    /// lossy.
    /// please only give positive numbers, else idk what happens
    pub fn float_to_int_color(float_bits: f32) -> i32 {
        let mut int_bits = float_bits.to_bits() as i32;
        int_bits |= ((int_bits >> 24) * (255. / 254.)) << 24;
        int_bits
    }

    pub fn mul(&mut self, other: &Color) -> Self {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
        self.clamp();
        *self
    }

    pub fn mul_float(&mut self, other: f32) -> Self {
        self.r *= other;
        self.g *= other;
        self.b *= other;
        self.clamp();
        *self
    }

    pub fn mul_float_a(&mut self, other: f32) {
        self.r *= other;
        self.g *= other;
        self.b *= other;
        self.a *= other;
        self.clamp();
    }

    pub fn add(&mut self, other: &Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
        self.clamp();
    }

    pub fn sub(&mut self, other: &Color) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
        self.clamp();
    }

    pub const fn clamp(&mut self) {
        self.r = self.r.max(0.).min(1.);
        self.g = self.g.max(0.).min(1.);
        self.b = self.b.max(0.).min(1.);
        self.a = self.a.max(0.).min(1.);
    }

    pub fn sum(&self) -> f32 {
        self.r + self.g + self.b
    }

    /// attention: this doesnt clamp the color and doesnt return a pointer to the color but rather the value of it
    pub fn inv(&mut self) -> Color {
        self = &mut Color::new(1. - self.r, 1. - self.g, 1. - self.b, self.a);
        *self
    }

    pub fn r(&mut self, r: f32) {
        self.r = r;
    }

    pub fn g(&mut self, g: f32) {
        self.g = g;
    }

    pub fn b(&mut self, b: f32) {
        self.b = b;
    }

    pub fn a(&mut self, a: f32) -> Self {
        self.a = a;
        *self
    }

    /// Linearly interpolates between this color and the target color by t which is in the range [0,1]. The result is stored in
    /// this color.
    pub fn lerp(&mut self, target: &Color, t: f32) {
        self.r += (target.r - self.r) * t;
        self.g += (target.g - self.g) * t;
        self.b += (target.b - self.b) * t;
        self.a += (target.a - self.a) * t;
    }

    /// Multiplies the RGB values by the alpha.
    pub fn premultiply_alpha(&mut self) {
        self.r *= self.a;
        self.g *= self.a;
        self.b *= self.a;
    }

    pub fn write(&self, mut to: Color) -> Color {
        to.r = self.r;
        to.g = self.g;
        to.b = self.b;
        to.a = self.a;
        to
    }

    pub fn hue(&self) -> f32 {
        self.to_hsv(self.tmp_hsv).0
    }

    pub fn saturation(&self) -> f32 {
        self.to_hsv(self.tmp_hsv).1
    }

    pub fn value(&self) -> f32 {
        self.to_hsv(self.tmp_hsv).2
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        let x = (h / 60. + 6.) % 6.;
        let i = x as i32;
        let f = x - i as f32;
        let p = v * (1. - s);
        let q = v * (1. - f * s);
        let t = v * (1. - (1. - f) * s);
        let mut c = match i {
            0 => Color::new(v, t, p, 1.),
            1 => Color::new(q, v, p, 1.),
            2 => Color::new(p, v, t, 1.),
            3 => Color::new(p, q, v, 1.),
            4 => Color::new(t, p, v, 1.),
            5 => Color::new(v, p, q, 1.),
            _ => Color::new(0., 0., 0., 1.),
        };
        c.clamp();
        c
    }

    pub fn to_hsv(&self, mut hsv: [f32; 3]) -> (f32, f32, f32) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let range = max - min;
        if range == 0. {
            hsv[0] = 0.;
        } else if max == self.r {
            hsv[0] = (self.g - self.b) / range % 6. * 60.;
        } else if max == self.g {
            hsv[0] = (self.b - self.r) / range + 2. * 60.;
        } else {
            hsv[0] = (self.r - self.g) / range + 4. * 60.;
        }

        if max > 0. {
            hsv[1] = range / max;
        } else {
            hsv[1] = 0.;
        }

        hsv[2] = max;

        (hsv[0], hsv[1], hsv[2])
    }

    pub fn hsv_to_rgb(mut h: f32, mut s: f32, mut v: f32) -> Color {
        if h == 360. { h = 359. }
        let (mut r, mut g, mut b);
        let mut i;
        let (mut f, mut p, mut q, mut t);
        h = max(0., min(360., h)) as f32;
        s = max(0., min(100., s)) as f32;
        v = max(0., min(100., v)) as f32;
        s /= 100.;
        v /= 100.;
        h /= 60.;
        i = h.floor() as i32;
        f = h - i as f32;
        p = v * (1. - s);
        q = v * (1. - s * f);
        t = v * (1. - s * (1. - f));
        let mut c = match i {
            0 => Color::new(v, t, p, 1.),
            1 => Color::new(q, v, p, 1.),
            2 => Color::new(p, v, t, 1.),
            3 => Color::new(p, q, v, 1.),
            4 => Color::new(t, p, v, 1.),
            5 => Color::new(v, p, q, 1.),
            _ => Color::new(0., 0., 0., 1.),
        };

        c.clamp();
        c
    }

    pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let (mut h, mut s, mut v);
        let (mut min, mut max, mut delta);
        min = r.min(g).min(b);
        max = r.max(g).max(b);
        v = max;
        delta = max - min;
        if max != 0. {
            s = delta / max;
        } else {
            s = 0.;
            h = -1.;
            return (h, s, v);
        }
        if r == max {
            h = (g - b) / delta;
        } else if g == max {
            h = 2. + (b - r) / delta;
        } else {
            h = 4. + (r - g) / delta;
        }
        h *= 60.;
        if h < 0. {
            h += 360.;
        }
        s *= 100.;
        v *= 100.;

        (h, s, v)
    }

    pub fn clampf(value: f32) -> f32 {
        min(max(value, 0.), 255.)
    }

    /// return R value of an RGBA packed color
    pub fn ri(rgba: u32) -> u32 {
        (rgba & 0xff000000) >> 24
    }

    // todo: implement the magic bit shift stuff
}