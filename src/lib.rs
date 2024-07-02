#[cfg(feature = "skia")]
pub mod skia;

use std::f32::consts::PI;

/// if n == 2.0 this draws a circle
pub fn superellipse_points(
    angles_in_degrees: impl Iterator<Item = u32>,
    n: f32,
    center_x: f32,
    center_y: f32,
    radius_x: f32,
    radius_y: f32,
) -> impl Iterator<Item = (f32, f32)> {
    angles_in_degrees
        .map(|degrees| PI * degrees as f32 / 180.0)
        .map(move |radians| {
            let x =
                center_x + (radians.cos().abs().powf(2.0 / n) * radius_x * radians.cos().signum());
            let y =
                center_y + (radians.sin().abs().powf(2.0 / n) * radius_y * radians.sin().signum());

            (x, y)
        })
}
