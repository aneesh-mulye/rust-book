// Challenge 3.2 - First Match Finder
//
// Implement:
// - `find_first_above(numbers, threshold) -> Option<usize>`
// - `find_first_even(numbers) -> Option<i32>`

pub fn find_first_above(numbers: &[i32], threshold: i32) -> Option<usize> {
    for (i, n) in numbers.iter().enumerate() {
        match *n {
            n if n > threshold => {
                return Some(i);
            }
            _ => {
                continue;
            }
        }
    }
    None
}

pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for n in numbers {
        match (*n) % 2 {
            0 => {
                return Some(*n);
            }
            _ => {
                continue;
            }
        }
    }
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
    use super::{find_first_above, find_first_even};

    #[test]
    fn finds_first_index_above_threshold() {
        let data = [3, 7, 2, 9, 4, 11, 6];

        assert_eq!(
            find_first_above(&data, 5),
            Some(1),
            "First value strictly above 5 should be index 1 (value 7)."
        );
        assert_eq!(
            find_first_above(&data, 20),
            None,
            "No value is above 20, so result should be None."
        );
    }

    #[test]
    fn finds_first_even_value_not_index() {
        let data = [3, 7, 2, 9, 4, 11, 6];

        assert_eq!(
            find_first_even(&data),
            Some(2),
            "First even value should be 2, not its index."
        );
        assert_eq!(
            find_first_even(&[1, 3, 5]),
            None,
            "Array with no even numbers should return None."
        );
    }
}
