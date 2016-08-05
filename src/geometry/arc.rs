use super::point::Point;
use super::line::{Line, Intersection};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Arc {
    pub begin: Point,
    pub end: Point,
    pub center: Point
}

impl Arc {
    pub fn from_points_along_arc(begin: Point, middle: Point, end: Point) -> Option<Arc> {
        let p1 = (begin + middle) / 2.0;
        let p2 = (middle + end) / 2.0;

        let m1 = -Point::slope(begin, middle).recip();
        let m2 = -Point::slope(middle, end).recip();

        let l1 = Line::from_point_and_slope(p1, m1);
        let l2 = Line::from_point_and_slope(p2, m2);

        if let Intersection::One(center) = l1.intersect(l2) {
            Some(Arc { begin: begin, end: end, center: center })
        } else { None }
    }
}
