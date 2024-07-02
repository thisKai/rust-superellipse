use std::f32::consts::PI;

use skia_safe::{rrect::Corner, Path, RRect};

/// if n == 2.0 this draws a circle
pub fn superellipse(n: f32, center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Path {
    let mut path = Path::new();
    for angle in 0..360 {
        let radians = PI * angle as f32 / 180.0;

        let x = center_x + (radians.cos().abs().powf(2.0 / n) * radius_x * radians.cos().signum());
        let y = center_y + (radians.sin().abs().powf(2.0 / n) * radius_y * radians.sin().signum());

        if angle == 0 {
            path.move_to((x, y));
        } else {
            path.line_to((x, y));
        }
    }
    path.close();
    path
}

pub fn superellipse_xywh(n: f32, x: f32, y: f32, width: f32, height: f32) -> Path {
    let center_x = x + (width / 2.0);
    let center_y = y + (height / 2.0);

    let radius_x = width / 2.0;
    let radius_y = height / 2.0;

    superellipse(n, center_x, center_y, radius_x, radius_y)
}

pub fn superellipse_rrect(n: f32, rrect: RRect) -> Path {
    let radius_top_right = rrect.radii(Corner::UpperRight);
    let radius_bottom_right = rrect.radii(Corner::LowerRight);
    let radius_bottom_left = rrect.radii(Corner::LowerLeft);
    let radius_top_left = rrect.radii(Corner::UpperLeft);

    let bottom_right_center_x = rrect.bounds().right - radius_bottom_right.x;
    let bottom_right_center_y = rrect.bounds().bottom - radius_bottom_right.y;
    let bottom_right_points = superellipse_points(
        0..=90,
        n,
        bottom_right_center_x,
        bottom_right_center_y,
        radius_bottom_right.x,
        radius_bottom_right.y,
    );

    let bottom_left_center_x = rrect.bounds().left + radius_bottom_left.x;
    let bottom_left_center_y = rrect.bounds().bottom - radius_bottom_left.y;
    let bottom_left_points = superellipse_points(
        90..=180,
        n,
        bottom_left_center_x,
        bottom_left_center_y,
        radius_bottom_left.x,
        radius_bottom_left.y,
    );

    let top_left_center_x = rrect.bounds().left + radius_top_left.x;
    let top_left_center_y = rrect.bounds().top + radius_top_left.y;
    let top_left_points = superellipse_points(
        180..=270,
        n,
        top_left_center_x,
        top_left_center_y,
        radius_top_left.x,
        radius_top_left.y,
    );

    let top_right_center_x = rrect.bounds().right - radius_top_right.x;
    let top_right_center_y = rrect.bounds().top + radius_top_right.y;
    let top_right_points = superellipse_points(
        270..=360,
        n,
        top_right_center_x,
        top_right_center_y,
        radius_top_right.x,
        radius_top_right.y,
    );

    let mut points = bottom_right_points
        .chain(bottom_left_points)
        .chain(top_left_points)
        .chain(top_right_points);

    let mut path = Path::new();

    let point = points.next().unwrap();
    path.move_to(point);

    for point in points {
        path.line_to(point);
    }

    path.close();
    path
}

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
