// Challenge 3.1 - Internal Method Calls
//
// Define `Rect` and implement:
// - area
// - perimeter
// - is_square
// - center
// - contains_point
// - overlaps

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn area(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn perimeter(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn is_square(&self) -> bool {
        let _ = self;
        false
    }

    pub fn center(&self) -> (f64, f64) {
        let _ = self;
        (0.0, 0.0)
    }

    pub fn contains_point(&self, px: f64, py: f64) -> bool {
        let _ = (self, px, py);
        false
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        let _ = (self, other);
        false
    }
}


// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .

#[cfg(test)]
mod tests {
    use super::Rect;

    #[test]
    fn computes_geometry_basics() {
        let r = Rect {
            x: 10.0,
            y: 5.0,
            width: 4.0,
            height: 2.0,
        };

        assert!(
            (r.area() - 8.0).abs() < 1e-12,
            "Area should be width*height = 8. Got {}.",
            r.area()
        );
        assert!(
            (r.perimeter() - 12.0).abs() < 1e-12,
            "Perimeter should be 2*(w+h)=12. Got {}.",
            r.perimeter()
        );
        assert!(!r.is_square(), "4x2 rectangle is not a square.");
        assert_eq!(
            r.center(),
            (12.0, 6.0),
            "Center should be (x + w/2, y + h/2) = (12,6). Got {:?}.",
            r.center()
        );
    }

    #[test]
    fn contains_point_checks_inside_bounds() {
        let r = Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };

        assert!(r.contains_point(5.0, 5.0), "(5,5) should be inside the rectangle.");
        assert!(r.contains_point(0.0, 0.0), "Top-left corner should count as inside.");
        assert!(
            !r.contains_point(-0.1, 0.0),
            "Point left of rectangle should be outside."
        );
        assert!(
            !r.contains_point(10.1, 5.0),
            "Point right of rectangle should be outside."
        );
    }

    #[test]
    fn overlap_detection_handles_overlap_and_separation() {
        let a = Rect {
            x: 0.0,
            y: 0.0,
            width: 4.0,
            height: 4.0,
        };
        let b = Rect {
            x: 2.0,
            y: 2.0,
            width: 4.0,
            height: 4.0,
        };
        let c = Rect {
            x: 5.0,
            y: 5.0,
            width: 2.0,
            height: 2.0,
        };

        assert!(a.overlaps(&b), "Rectangles A and B should overlap.");
        assert!(
            !a.overlaps(&c),
            "Rectangles A and C should not overlap when separated."
        );
    }
}
