use std::f32::consts::PI;

use crate::{Point, Rect};

pub fn superellipse_points(
    angles_in_degrees: impl Iterator<Item = u32>,
    n: f32,
    center: Point,
    radius: Point,
) -> impl Iterator<Item = Point> {
    angles_in_degrees
        .map(|degrees| PI * degrees as f32 / 180.0)
        .map(move |radians| {
            let x =
                center.x + (radians.cos().abs().powf(2.0 / n) * radius.x * radians.cos().signum());
            let y =
                center.y + (radians.sin().abs().powf(2.0 / n) * radius.y * radians.sin().signum());

            Point { x, y }
        })
}

pub fn superellipse_points_rect(
    angles_in_degrees: impl Iterator<Item = u32>,
    n: f32,
    rect: Rect,
) -> impl Iterator<Item = Point> {
    let width = rect.width();
    let height = rect.height();

    let center = Point {
        x: rect.left + (width / 2.0),
        y: rect.top + (height / 2.0),
    };
    let radius = Point {
        x: width / 2.0,
        y: height / 2.0,
    };

    superellipse_points(angles_in_degrees, n, center, radius)
}
