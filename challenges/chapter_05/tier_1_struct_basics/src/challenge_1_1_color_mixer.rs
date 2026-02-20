// Challenge 1.1 - Color Mixer
//
// Define `Color` with `r`, `g`, `b` (`u8` fields), then build the prompt examples:
// - red = (255, 0, 0)
// - white = (255, 255, 255)
// - check whether red has any blue component

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn color_mixer_examples() -> (Color, Color, bool) {
    (
        Color { r: 0, g: 0, b: 0 },
        Color { r: 0, g: 0, b: 0 },
        true,
    )
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
    use super::{color_mixer_examples, Color};

    #[test]
    fn builds_prompt_colors() {
        let (red, white, red_has_blue) = color_mixer_examples();

        assert_eq!(
            red,
            Color { r: 255, g: 0, b: 0 },
            "Red should be Color {{ r: 255, g: 0, b: 0 }}. Got {:?}.",
            red
        );
        assert_eq!(
            white,
            Color {
                r: 255,
                g: 255,
                b: 255
            },
            "White should be Color {{ r: 255, g: 255, b: 255 }}. Got {:?}.",
            white
        );
        assert!(
            !red_has_blue,
            "Red should report no blue component (`false`). Got {red_has_blue}."
        );
    }
}
