use std::f32::consts::{PI, TAU};

use ggez::glam::Vec2;

pub mod colors;
pub mod config;
pub mod grid;

pub fn lerp_angle(a: f32, b: f32, t: f32, choosing_larger_angle: bool) -> f32 {
    let mut a = a.normalize_angle();
    let mut b = b.normalize_angle();

    if !choosing_larger_angle && (b - a).abs() > PI || choosing_larger_angle && (b - a).abs() < PI {
        (a, b) = if a < b {(a+TAU, b)} else { (a, b+TAU) };
    }
    a + (b - a) * t
}

pub fn angle_between(a: f32, b: f32) -> f32 {
    let mut a = if a < 0.0 { a + TAU } else { a };
    let mut b = if b < 0.0 { b + TAU } else { b };

    if b - a > PI {
        a += TAU;
    } else if a - b > PI {
        b += TAU;
    }
    (b - a).abs()
}

pub trait AngleNormalizer {
    fn normalize_angle(self) -> f32;
}

impl AngleNormalizer for f32 {
    fn normalize_angle(self) -> f32 {
        let mut angle = self % TAU;
        if angle < 0.0 {
            angle += TAU;
        }
        angle
    }
}

pub trait AngleCalc {
    fn angle(&self) -> f32;
}

impl AngleCalc for Vec2 {
    fn angle(&self) -> f32 {
        self.y.atan2(self.x).normalize_angle()
    }
    
}