use std::f32::consts::PI;

use skia_safe::Path;

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
