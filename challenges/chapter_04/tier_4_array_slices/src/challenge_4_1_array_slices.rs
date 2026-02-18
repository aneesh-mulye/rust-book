// Challenge 4.1 - Array Slices
//
// Implement `sum(numbers: &[i32]) -> i32`.

pub fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for i in numbers {
        total += i;
    }
    total
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
    use super::sum;

    #[test]
    fn matches_prompt_examples() {
        let arr = [10, 20, 30, 40, 50];

        assert_eq!(sum(&arr), 150, "Expected total 150 for entire array.");
        assert_eq!(sum(&arr[..3]), 60, "Expected first 3 sum to be 60.");
        assert_eq!(sum(&arr[3..]), 90, "Expected last 2 sum to be 90.");
    }

    #[test]
    fn handles_empty_slice() {
        let data: [i32; 0] = [];
        assert_eq!(sum(&data), 0, "Sum of empty slice should be 0.");
    }
}
