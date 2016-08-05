define_vector2!(
    pub struct Point, f32;
    Copy, Default, Div, DivAssign, PartialEq, Neg);

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        if x.is_finite() && y.is_finite() {
            Point { x: x, y: y }
        } else {
            panic!("Cannot create point from nonfinite parameters");
        }
    }

    pub fn slope(self, other: Point) -> f32 {
        // f32 division by ±0.0 results in ± infinity, which is exactly what
        // we want.
        (other.y - self.y) / (other.x - self.x)
    }
}
