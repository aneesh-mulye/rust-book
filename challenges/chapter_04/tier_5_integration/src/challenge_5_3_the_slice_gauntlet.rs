// Challenge 5.3 - The Slice Gauntlet
//
// Implement:
// - `analyze(data: &[i32]) -> (i32, i32, bool)` returning (min, max, is_sorted)
// - `normalize(data: &mut [i32])` subtracting the min from every element

pub fn analyze(data: &[i32]) -> (i32, i32, bool) {
    let _ = data;
    (0, 0, false)
}

pub fn normalize(data: &mut [i32]) {
    let _ = data;
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
    use super::{analyze, normalize};

    #[test]
    fn matches_prompt_example_before_and_after_normalize() {
        let mut data = [5, 3, 8, 1, 9];

        let before = analyze(&data);
        normalize(&mut data);
        let after = analyze(&data);

        assert_eq!(
            before,
            (1, 9, false),
            "Initial analysis should be (min=1, max=9, sorted=false). Got {:?}.",
            before
        );
        assert_eq!(
            data,
            [4, 2, 7, 0, 8],
            "After normalize, expected [4, 2, 7, 0, 8]. Got {:?}.",
            data
        );
        assert_eq!(
            after,
            (0, 8, false),
            "Post-normalize analysis should be (min=0, max=8, sorted=false). Got {:?}.",
            after
        );
    }

    #[test]
    fn sorted_detection_works() {
        let sorted = [1, 2, 2, 5];
        let unsorted = [2, 1, 3];

        assert!(
            analyze(&sorted).2,
            "Expected sorted=true for [1,2,2,5]."
        );
        assert!(
            !analyze(&unsorted).2,
            "Expected sorted=false for [2,1,3]."
        );
    }
}
