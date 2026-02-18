// Challenge 5.3 - The Slice Gauntlet
//
// Implement:
// - `analyze(data: &[i32]) -> (i32, i32, bool)` returning (min, max, is_sorted)
// - `normalize(data: &mut [i32])` subtracting the min from every element

pub fn analyze(data: &[i32]) -> (i32, i32, bool) {
    if data.is_empty() {
        return (i32::MAX, i32::MIN, true);
    }
    let mut max = data[0];
    let mut min = data[0];
    let mut last = data[0];
    let mut sorted = true;

    for i in data[1..].iter() {
        max = i32::max(*i, max);
        min = i32::min(*i, min);
        if *i < last {
            sorted = false;
        }
        last = *i;
    }

    (min, max, sorted)
}

pub fn normalize(data: &mut [i32]) {
    let (min, _, _) = analyze(data);
    for i in data.iter_mut() {
        *i -= min;
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

        assert!(analyze(&sorted).2, "Expected sorted=true for [1,2,2,5].");
        assert!(!analyze(&unsorted).2, "Expected sorted=false for [2,1,3].");
    }
}
