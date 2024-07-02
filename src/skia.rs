use skia_safe::{rrect::Corner, Path, Point, RRect, Rect};

use crate::{Corners, Superellipse, SuperellipseRect};

impl Superellipse {
    pub fn from_skia_rect(rect: Rect, exponent: f32) -> Self {
        crate::Rect::from(rect).superellipse(exponent)
    }
    pub fn skia_path(&self) -> Path {
        generate_path(self.points())
    }
}
impl SuperellipseRect {
    pub fn from_skia_rrect(rrect: RRect, corner_exponents: Corners<f32>) -> Self {
        Self {
            rounded_rect: rrect.into(),
            corner_exponents,
        }
    }
    pub fn skia_path(&self) -> Path {
        generate_path(self.points())
    }
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
        let corner_radii = crate::Corners {
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
