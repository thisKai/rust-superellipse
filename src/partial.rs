use std::f32::consts::PI;

use crate::Point;

pub fn superellipse_points(
    angles_in_degrees: impl Iterator<Item = u32>,
    n: f32,
    center_x: f32,
    center_y: f32,
    radius_x: f32,
    radius_y: f32,
) -> impl Iterator<Item = Point> {
    angles_in_degrees
        .map(|degrees| PI * degrees as f32 / 180.0)
        .map(move |radians| {
            let x =
                center_x + (radians.cos().abs().powf(2.0 / n) * radius_x * radians.cos().signum());
            let y =
                center_y + (radians.sin().abs().powf(2.0 / n) * radius_y * radians.sin().signum());

            Point { x, y }
        })
}

pub fn superellipse_points_xywh(
    angles_in_degrees: impl Iterator<Item = u32>,
    n: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> impl Iterator<Item = Point> {
    let center_x = x + (width / 2.0);
    let center_y = y + (height / 2.0);

    let radius_x = width / 2.0;
    let radius_y = height / 2.0;

    superellipse_points(angles_in_degrees, n, center_x, center_y, radius_x, radius_y)
}
