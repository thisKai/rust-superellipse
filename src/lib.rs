pub mod partial;

#[cfg(feature = "skia")]
pub mod skia;

/// if n == 2.0 this draws a circle
pub fn superellipse_points(
    n: f32,
    center_x: f32,
    center_y: f32,
    radius_x: f32,
    radius_y: f32,
) -> impl Iterator<Item = Point> {
    partial::superellipse_points(0..360, n, center_x, center_y, radius_x, radius_y)
}
pub fn superellipse_points_xywh(
    n: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> impl Iterator<Item = Point> {
    partial::superellipse_points_xywh(0..360, n, x, y, width, height)
}

pub fn superellipse_rounded_rect_points(
    n: f32,
    rect: Rect,
    corner_radii: CornerRadii,
) -> impl Iterator<Item = Point> {
    let radius_bottom_right = corner_radii.bottom_right;
    let radius_bottom_left = corner_radii.bottom_left;
    let radius_top_left = corner_radii.top_left;
    let radius_top_right = corner_radii.top_right;

    let bottom_right_center_x = rect.right - radius_bottom_right.x;
    let bottom_right_center_y = rect.bottom - radius_bottom_right.y;
    let bottom_right_points = partial::superellipse_points(
        0..=90,
        n,
        bottom_right_center_x,
        bottom_right_center_y,
        radius_bottom_right.x,
        radius_bottom_right.y,
    );

    let bottom_left_center_x = rect.left + radius_bottom_left.x;
    let bottom_left_center_y = rect.bottom - radius_bottom_left.y;
    let bottom_left_points = partial::superellipse_points(
        90..=180,
        n,
        bottom_left_center_x,
        bottom_left_center_y,
        radius_bottom_left.x,
        radius_bottom_left.y,
    );

    let top_left_center_x = rect.left + radius_top_left.x;
    let top_left_center_y = rect.top + radius_top_left.y;
    let top_left_points = partial::superellipse_points(
        180..=270,
        n,
        top_left_center_x,
        top_left_center_y,
        radius_top_left.x,
        radius_top_left.y,
    );

    let top_right_center_x = rect.right - radius_top_right.x;
    let top_right_center_y = rect.top + radius_top_right.y;
    let top_right_points = partial::superellipse_points(
        270..=360,
        n,
        top_right_center_x,
        top_right_center_y,
        radius_top_right.x,
        radius_top_right.y,
    );

    bottom_right_points
        .chain(bottom_left_points)
        .chain(top_left_points)
        .chain(top_right_points)
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct CornerRadii {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}
