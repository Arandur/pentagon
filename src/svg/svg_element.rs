use super::super::geometry::point::Point;

use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum SVGElement {
    SVG {
        width: i32,
        height: i32,
        content: Option<Box<SVGElement>>
    },
    Line (Point, Point),
    Path {
        elements: Vec<PathElement>
    }
}

impl SVGElement {
    fn fmt_svg(width: i32, height: i32, content: &Option<Box<SVGElement>>,
               f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }

    fn fmt_line(a: Point, b: Point, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }

    fn fmt_path(elements: &Vec<PathElement>, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum PathElement {
    MoveTo(Point),
    LineTo(Point),
    ArcTo(ArcToParameters),
    Close
}

#[derive(Clone, PartialEq, Debug)]
pub enum ArcToParameters {
    OfCircle {
        radius: f32,
        large_arc: bool,
        sweep: SweepFlag,
        to: Point
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SweepFlag {
    Clockwise,
    Counterclockwise
}

impl fmt::Display for SVGElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SVGElement::SVG { width, height, ref content } => 
                SVGElement::svg_fmt(width, height, &content, &mut f),
            &SVGElement::Line(a, b) =>
                SVGElement::line_fmt(a, b, &mut f),
            &SVGElement::Path { ref elements } =>
                SVGElement::path_fmt(&elements, &mut f)
        }
    }
/*
        try!(write!(f, r#"<svg xmlns="http://www.w3.org/2000/svg" "#));
        try!(write!(f, r#"width="{}" height="{}">"#, self.width, self.height));

        if let Some(ref content) = self.content {
            try!(fmt::Display::fmt(&content, &mut f));
        }

        try!(write!(f, "</svg>"));
    }
*/
}

impl fmt::Display for PathElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl fmt::Display for ArcToParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl fmt::Display for SweepFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}
