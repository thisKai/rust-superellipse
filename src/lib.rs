pub mod partial;

#[cfg(feature = "skia")]
pub mod skia;

/// This can draw several different shapes depending on the value of n:
/// superellipse: n > 2
/// ellipse: n = 2
/// convex curved rhombus: 1 < n < 2
/// rhombus: n = 1
/// concave curved rhombus: 0 < n < 1
/// rectangle: n = ±∞
pub fn superellipse_points(n: f32, center: Point, radius: Point) -> impl Iterator<Item = Point> {
    partial::superellipse_points(0..360, n, center, radius)
}
pub fn superellipse_points_rect(n: f32, rect: Rect) -> impl Iterator<Item = Point> {
    partial::superellipse_points_rect(0..360, n, rect)
}

pub fn superellipse_rounded_rect_points(
    n: f32,
    rounded_rect: RoundedRect,
) -> impl Iterator<Item = Point> {
    let RoundedRect { rect, corner_radii } = rounded_rect;

    let radius_bottom_right = corner_radii.bottom_right;
    let radius_bottom_left = corner_radii.bottom_left;
    let radius_top_left = corner_radii.top_left;
    let radius_top_right = corner_radii.top_right;

    let bottom_right_center = Point {
        x: rect.right - radius_bottom_right.x,
        y: rect.bottom - radius_bottom_right.y,
    };
    let bottom_right_points =
        partial::superellipse_points(0..=90, n, bottom_right_center, radius_bottom_right);

    let bottom_left_center = Point {
        x: rect.left + radius_bottom_left.x,
        y: rect.bottom - radius_bottom_left.y,
    };
    let bottom_left_points =
        partial::superellipse_points(90..=180, n, bottom_left_center, radius_bottom_left);

    let top_left_center = Point {
        x: rect.left + radius_top_left.x,
        y: rect.top + radius_top_left.y,
    };
    let top_left_points =
        partial::superellipse_points(180..=270, n, top_left_center, radius_top_left);

    let top_right_center = Point {
        x: rect.right - radius_top_right.x,
        y: rect.top + radius_top_right.y,
    };
    let top_right_points =
        partial::superellipse_points(270..=360, n, top_right_center, radius_top_right);

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
impl Rect {
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }
}

pub struct RoundedRect {
    pub rect: Rect,
    pub corner_radii: CornerRadii,
}

pub struct CornerRadii {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}
