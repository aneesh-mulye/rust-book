// Challenge 4.2 - Mutable Slice Operations
//
// Implement `double_all(numbers: &mut [i32])` to double every element.

pub fn double_all(numbers: &mut [i32]) {
    let _ = numbers;
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
    use super::double_all;

    #[test]
    fn matches_prompt_scenario() {
        let mut arr = [1, 2, 3, 4, 5];
        double_all(&mut arr[..3]);

        assert_eq!(
            arr,
            [2, 4, 6, 4, 5],
            "After doubling first three elements, expected [2, 4, 6, 4, 5]. Got {:?}.",
            arr
        );
    }

    #[test]
    fn doubles_entire_slice_when_requested() {
        let mut arr = [2, 4, 6];
        double_all(&mut arr);

        assert_eq!(
            arr,
            [4, 8, 12],
            "Doubling entire slice [2,4,6] should yield [4,8,12]. Got {:?}.",
            arr
        );
    }
}
