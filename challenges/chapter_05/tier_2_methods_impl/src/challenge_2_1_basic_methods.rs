// Challenge 2.1 - Basic Methods
//
// Add methods to `Color`:
// - `is_grayscale(&self) -> bool`
// - `brightness(&self) -> u8` (average of channels, with safe intermediate type)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn is_grayscale(&self) -> bool {
        let _ = self;
        false
    }

    pub fn brightness(&self) -> u8 {
        let _ = self;
        0
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
    fn matches_prompt_examples() {
        let gray = Color {
            r: 100,
            g: 100,
            b: 100,
        };
        let vivid = Color {
            r: 200,
            g: 50,
            b: 10,
        };

        assert!(
            gray.is_grayscale(),
            "(100,100,100) should be grayscale because all channels are equal."
        );
        assert_eq!(
            gray.brightness(),
            100,
            "Brightness of (100,100,100) should be 100. Got {}.",
            gray.brightness()
        );

        assert!(
            !vivid.is_grayscale(),
            "(200,50,10) should not be grayscale because channels differ."
        );
        assert_eq!(
            vivid.brightness(),
            86,
            "Brightness of (200,50,10) should be floor((200+50+10)/3)=86. Got {}.",
            vivid.brightness()
        );
    }

    #[test]
    fn avoids_u8_overflow_when_summing_channels() {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
        };
        assert_eq!(
            white.brightness(),
            255,
            "Brightness of (255,255,255) should be 255 and must not overflow intermediate sum."
        );
    }
}
