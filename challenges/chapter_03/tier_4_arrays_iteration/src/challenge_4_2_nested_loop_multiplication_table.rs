// Challenge 4.2 - Nested Loop Multiplication Table
//
// Implement `multiplication_table_5x5` so it returns a 5x5 table where
// table[row][col] = (row + 1) * (col + 1).
//
// In the original prompt, this would be printed with width formatting.

pub fn multiplication_table_5x5() -> [[u32; 5]; 5] {
    [[0; 5]; 5]
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
    use super::multiplication_table_5x5;

    #[test]
    fn has_expected_first_and_last_rows() {
        let table = multiplication_table_5x5();

        assert_eq!(
            table[0],
            [1, 2, 3, 4, 5],
            "First row should be [1, 2, 3, 4, 5]."
        );
        assert_eq!(
            table[4],
            [5, 10, 15, 20, 25],
            "Last row should be [5, 10, 15, 20, 25]."
        );
    }

    #[test]
    fn each_cell_matches_row_times_column_rule() {
        let table = multiplication_table_5x5();

        for row in 0..5 {
            for col in 0..5 {
                let expected = (row as u32 + 1) * (col as u32 + 1);
                let actual = table[row][col];
                assert_eq!(
                    actual, expected,
                    "Cell ({row}, {col}) should be {expected}, got {actual}."
                );
            }
        }
    }
}
