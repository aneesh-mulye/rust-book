// Challenge 2.1 - Shape Area Calculator
//
// Define `Shape`:
// - Circle(radius)
// - Rectangle(width, height)
// - RightTriangle(base, height)
//
// Implement:
// - `area(shape: &Shape) -> f64`
// - `describe(shape: &Shape) -> String`
// - `largest_area_index(shapes: &[Shape]) -> Option<usize>`

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    RightTriangle(f64, f64),
}

pub fn area(shape: &Shape) -> f64 {
    let _ = shape;
    0.0
}

pub fn describe(shape: &Shape) -> String {
    let _ = shape;
    String::new()
}

pub fn largest_area_index(shapes: &[Shape]) -> Option<usize> {
    let _ = shapes;
    None
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
    use super::{area, describe, largest_area_index, Shape};
    use std::f64::consts::PI;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn computes_area_for_each_variant() {
        assert!(
            approx_eq(area(&Shape::Circle(2.0)), PI * 4.0),
            "Circle with radius 2 should have area PI * 2^2."
        );
        assert!(
            approx_eq(area(&Shape::Rectangle(3.0, 5.0)), 15.0),
            "Rectangle 3x5 should have area 15."
        );
        assert!(
            approx_eq(area(&Shape::RightTriangle(4.0, 6.0)), 12.0),
            "Right triangle with base 4 and height 6 should have area 12."
        );
    }

    #[test]
    fn describe_mentions_dimensions_and_area() {
        let text = describe(&Shape::Rectangle(3.0, 5.0));

        assert!(
            text.contains("Rectangle"),
            "Description should mention the variant name. Got '{text}'."
        );
        assert!(
            text.contains("3") && text.contains("5"),
            "Description should mention the rectangle dimensions. Got '{text}'."
        );
        assert!(
            text.contains("15"),
            "Description should include the computed area. Got '{text}'."
        );
    }

    #[test]
    fn finds_largest_shape_in_collection() {
        let shapes = [
            Shape::Circle(1.0),
            Shape::Rectangle(2.0, 4.0),
            Shape::RightTriangle(10.0, 6.0),
            Shape::Circle(4.0),
            Shape::Rectangle(3.0, 9.0),
        ];

        assert_eq!(
            largest_area_index(&shapes),
            Some(3),
            "Largest area should be the circle of radius 4 at index 3."
        );
    }
}
