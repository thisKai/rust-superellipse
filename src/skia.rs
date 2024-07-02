use skia_safe::{rrect::Corner, Path, Point, RRect, Rect};

pub fn superellipse(n: f32, center: Point, radius: Point) -> Path {
    generate_path(crate::superellipse_points(n, center.into(), radius.into()))
}

pub fn superellipse_rect(n: f32, rect: Rect) -> Path {
    generate_path(crate::superellipse_points_rect(n, rect.into()))
}

pub fn superellipse_rrect(n: f32, rrect: RRect) -> Path {
    generate_path(crate::superellipse_rounded_rect_points(n, rrect.into()))
}

fn generate_path(mut points: impl Iterator<Item = crate::Point>) -> Path {
    let mut path = Path::new();

    let point = points.next().unwrap();
    path.move_to(point);

    for point in points {
        path.line_to(point);
    }

    path.close();
    path
}

impl From<crate::Point> for Point {
    fn from(value: crate::Point) -> Point {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<Point> for crate::Point {
    fn from(value: Point) -> crate::Point {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<Rect> for crate::Rect {
    fn from(value: Rect) -> crate::Rect {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}
impl From<RRect> for crate::RoundedRect {
    fn from(value: RRect) -> crate::RoundedRect {
        let corner_radii = crate::CornerRadii {
            top_right: value.radii(Corner::UpperRight).into(),
            bottom_right: value.radii(Corner::LowerRight).into(),
            bottom_left: value.radii(Corner::LowerLeft).into(),
            top_left: value.radii(Corner::UpperLeft).into(),
        };

        Self {
            rect: From::from(*value.rect()),
            corner_radii,
        }
    }
}
