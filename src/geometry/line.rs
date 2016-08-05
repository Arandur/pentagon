use super::point::Point;

use std::f32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Line {
    Normal {
        slope: f32,
        intercept: f32
    },
    Vertical {
        x: f32
    }
}

impl Line {
    pub fn from_points(a: Point, b: Point) -> Line {
        if a == b {
            panic!("Cannot form line from two equal points");
        }

        Line::from_point_and_slope(a, Point::slope(a, b))
    }

    pub fn from_point_and_slope(point: Point, slope: f32) -> Line {
        if slope.is_finite() {
            let intercept = Line::intercept(point, slope);

            Line::Normal { slope: slope, intercept: intercept }
        } else if slope.is_infinite() {
            Line::Vertical { x: point.x }
        } else {
            panic!("slope must be a number");
        }
    }

    pub fn vertical(x: f32) -> Line {
        Line::Vertical { x: x }
    }

    pub fn horizontal(y: f32) -> Line {
        Line::Normal { slope: 0.0, intercept: y }
    }

    pub fn intersect(self, other: Line) -> Intersection {
        if self == other {
            Intersection::All(self)
        } else {
            match self {
                Line::Normal { slope, intercept } => other.intersect_parts(slope, intercept),
                Line::Vertical { x } => other.at_x(x)
            }
        }
    }

    pub fn at_x(self, x: f32) -> Intersection {
        match self {
            Line::Normal { slope, intercept } => if slope == 0.0 {
                Intersection::at_point(x, intercept)
            } else {
                Intersection::at_point(x, slope * x + intercept)
            },
            Line::Vertical { x: x2 } => if x == x2 {
                Intersection::All(self)
            } else {
                Intersection::None
            }
        }
    }

    pub fn at_y(self, y: f32) -> Intersection {
        match self {
            Line::Normal { slope, intercept } => if slope == 0.0 {
                if intercept == y {
                    Intersection::All(self)
                } else {
                    Intersection::None
                }
            } else {
                Intersection::at_point((y - intercept) / slope, y)
            },
            Line::Vertical { x } => Intersection::at_point(x, y)
        }
    }

    pub fn is_horizontal(self) -> bool {
        match self {
            Line::Normal { slope, .. } => slope == 0.0,
            Line::Vertical { .. } => false
        }
    }

    pub fn is_vertical(self) -> bool {
        match self {
            Line::Normal { .. } => false,
            Line::Vertical { .. } => true
        }
    }

    fn intercept(point: Point, slope: f32) -> f32 {
        point.y - slope * point.x
    }

    fn intersect_parts(self, m2: f32, b2: f32) -> Intersection {
        debug_assert!(m2.is_finite());

        if m2 == 0.0 {
            self.at_y(b2)
        } else {
            match self {
                Line::Normal { slope: m1, intercept: b1 } => {
                    let x = (b2 - b1) / (m1 - m2);
                    let y = m1 * x + b1;

                    Intersection::at_point(x, y)
                },
                Line::Vertical { x } => 
                    Intersection::at_point(x, m2 * x + b2)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Intersection {
    None,
    One(Point),
    All(Line)
}

impl Intersection {
    pub fn at_point(x: f32, y: f32) -> Intersection {
        Intersection::One(Point::new(x, y))
    }
}
