// Challenge 4.3 - Borrowing Discipline
//
// Implement:
// - `find_max(numbers: &[i32]) -> i32`
// - `replace_max(numbers: &mut [i32], replacement: i32)`
//
// `replace_max` should replace only the first occurrence of the maximum value.

pub fn find_max(numbers: &[i32]) -> i32 {
    let _ = numbers;
    0
}

pub fn replace_max(numbers: &mut [i32], replacement: i32) {
    let _ = (numbers, replacement);
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
    use super::{find_max, replace_max};

    #[test]
    fn matches_prompt_flow() {
        let mut arr = [3, 7, 2, 9, 4];

        let max_before = find_max(&arr);
        replace_max(&mut arr, 0);
        let max_after = find_max(&arr);

        assert_eq!(max_before, 9, "Expected initial max 9. Got {max_before}.");
        assert_eq!(
            arr,
            [3, 7, 2, 0, 4],
            "After replacing max with 0, expected [3, 7, 2, 0, 4]. Got {:?}.",
            arr
        );
        assert_eq!(max_after, 7, "Expected new max 7 after replacement. Got {max_after}.");
    }

    #[test]
    fn replace_max_changes_only_first_maximum_occurrence() {
        let mut arr = [5, 9, 3, 9, 1];
        replace_max(&mut arr, -1);

        assert_eq!(
            arr,
            [5, -1, 3, 9, 1],
            "Should replace only first max occurrence in [5,9,3,9,1]. Got {:?}.",
            arr
        );
    }
}
