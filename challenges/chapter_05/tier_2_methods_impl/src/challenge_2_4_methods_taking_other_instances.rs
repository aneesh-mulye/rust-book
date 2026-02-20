// Challenge 2.4 - Methods Taking Other Instances
//
// Add methods to `Color`:
// - `mix(&self, other: &Color) -> Color`
// - `distance(&self, other: &Color) -> f64` (Euclidean RGB distance)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn mix(&self, other: &Color) -> Color {
        let _ = (self, other);
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn distance(&self, other: &Color) -> f64 {
        let _ = (self, other);
        0.0
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
    use super::Color;

    #[test]
    fn red_blue_mix_matches_prompt() {
        let red = Color { r: 255, g: 0, b: 0 };
        let blue = Color { r: 0, g: 0, b: 255 };
        let mixed = red.mix(&blue);

        assert_eq!(
            mixed,
            Color {
                r: 127,
                g: 0,
                b: 127
            },
            "Mix of red and blue should average each channel to (127,0,127). Got {:?}.",
            mixed
        );
    }

    #[test]
    fn distance_between_red_and_blue_is_correct() {
        let red = Color { r: 255, g: 0, b: 0 };
        let blue = Color { r: 0, g: 0, b: 255 };
        let dist = red.distance(&blue);
        let expected = 360.62445840513925_f64;
        let diff = (dist - expected).abs();

        assert!(
            diff < 1e-9,
            "RGB distance between red and blue should be about {expected}. Got {dist} (diff {diff})."
        );
        assert!(
            (red.distance(&blue) - blue.distance(&red)).abs() < 1e-12,
            "Distance should be symmetric: d(a,b) == d(b,a)."
        );
    }
}
