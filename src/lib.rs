use std::f32::consts::PI;

#[cfg(feature = "skia")]
pub mod skia;

/// This can appear as several different shapes depending on the value of `exponent`:
/// superellipse: `exponent > 2`
/// ellipse: `exponent = 2`
/// convex curved rhombus: `1 < exponent < 2`
/// rhombus: `exponent = 1`
/// concave curved rhombus: `0 < n < 1`
/// rectangle: exponent = `±∞`
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Superellipse {
    pub exponent: f32,
    pub center: Point,
    pub radius: Point,
}
impl Superellipse {
    pub fn points(&self) -> impl Iterator<Item = Point> {
        self.partial_points(0..360)
    }
    pub fn partial_points(
        &self,
        angles_in_degrees: impl Iterator<Item = u32>,
    ) -> impl Iterator<Item = Point> {
        let Self {
            exponent,
            center,
            radius,
        } = *self;

        angles_in_degrees
            .map(|degrees| PI * degrees as f32 / 180.0)
            .map(move |radians| {
                let x = center.x
                    + (radians.cos().abs().powf(2.0 / exponent)
                        * radius.x
                        * radians.cos().signum());
                let y = center.y
                    + (radians.sin().abs().powf(2.0 / exponent)
                        * radius.y
                        * radians.sin().signum());

                Point { x, y }
            })
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct SuperellipseRect {
    pub rounded_rect: RoundedRect,
    pub corner_exponents: Corners<f32>,
}
impl SuperellipseRect {
    pub fn points(&self) -> impl Iterator<Item = Point> {
        let Corners {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        } = self.superellipse_corners();

        bottom_right
            .partial_points(0..=90)
            .chain(bottom_left.partial_points(90..=180))
            .chain(top_left.partial_points(180..=270))
            .chain(top_right.partial_points(270..=360))
    }
    pub fn superellipse_corners(&self) -> Corners<Superellipse> {
        let Self {
            rounded_rect:
                RoundedRect {
                    rect,
                    corner_radii:
                        Corners {
                            top_left,
                            top_right,
                            bottom_left,
                            bottom_right,
                        },
                },
            corner_exponents,
        } = *self;

        Corners {
            bottom_right: Superellipse {
                exponent: corner_exponents.bottom_right,
                center: Point {
                    x: rect.right - bottom_right.x,
                    y: rect.bottom - bottom_right.y,
                },
                radius: bottom_right,
            },
            bottom_left: Superellipse {
                exponent: corner_exponents.bottom_left,
                center: Point {
                    x: rect.left + bottom_left.x,
                    y: rect.bottom - bottom_left.y,
                },
                radius: bottom_left,
            },
            top_left: Superellipse {
                exponent: corner_exponents.top_left,
                center: Point {
                    x: rect.left + top_left.x,
                    y: rect.top + top_left.y,
                },
                radius: top_left,
            },
            top_right: Superellipse {
                exponent: corner_exponents.top_right,
                center: Point {
                    x: rect.right - top_right.x,
                    y: rect.top + top_right.y,
                },
                radius: top_right,
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
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
    pub fn superellipse(&self, exponent: f32) -> Superellipse {
        let width = self.width();
        let height = self.height();

        let center = Point {
            x: self.left + (width / 2.0),
            y: self.top + (height / 2.0),
        };
        let radius = Point {
            x: width / 2.0,
            y: height / 2.0,
        };

        Superellipse {
            exponent,
            center,
            radius,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct RoundedRect {
    pub rect: Rect,
    pub corner_radii: Corners<Point>,
}
impl RoundedRect {
    pub fn with_superellipse_corners(self, corner_exponents: Corners<f32>) -> SuperellipseRect {
        SuperellipseRect {
            rounded_rect: self,
            corner_exponents,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Corners<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_left: T,
    pub bottom_right: T,
}
impl<T: Clone> Corners<T> {
    pub fn all(value: T) -> Self {
        Self {
            top_left: value.clone(),
            top_right: value.clone(),
            bottom_left: value.clone(),
            bottom_right: value,
        }
    }
}
