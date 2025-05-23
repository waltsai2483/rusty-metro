use std::f32::consts::{PI, TAU};

pub mod colors;
pub mod config;

pub fn lerp_angle(a: f32, b: f32, t: f32, choosing_larger_angle: bool) -> f32 {
    let mut a = if a < 0.0 { a + TAU } else { a };
    let mut b = if b < 0.0 { b + TAU } else { b };

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