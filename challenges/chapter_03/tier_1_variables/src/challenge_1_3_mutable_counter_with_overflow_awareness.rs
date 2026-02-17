// Challenge 1.3 - Mutable Counter with Overflow Awareness
//
// Implement `checked_counter_run` using `checked_add` (not plain `+`).
//
// Behavior:
// - Start from `start`.
// - Attempt to increment `attempts` times.
// - Each successful increment should be pushed into the returned sequence.
// - If overflow would occur, stop immediately.
//
// Return:
// - `Vec<u8>`: values produced after each successful increment.
// - `bool`: `true` if an overflow attempt was detected and the function stopped early.

pub fn checked_counter_run(start: u8, attempts: u8) -> (Vec<u8>, bool) {
    let mut counter = start;
    let mut v: Vec<u8> = Vec::new();
    for _ in 0..attempts {
        match counter.checked_add(1) {
            Some(counter_inc) => {
                counter = counter_inc;
                v.push(counter);
            }
            None => return (v, true),
        }
    }
    (v, false)
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
    use super::checked_counter_run;

    #[test]
    fn stops_before_overflow_in_challenge_scenario() {
        let (values, overflowed) = checked_counter_run(250, 6);

        assert_eq!(
            values,
            vec![251, 252, 253, 254, 255],
            "For start=250 and 6 attempts, expected values 251..=255, then stop before overflow. Got {:?}.",
            values
        );
        assert!(
            overflowed,
            "Overflow flag should be true when trying to go past 255. Got false."
        );
    }

    #[test]
    fn reports_no_overflow_when_attempts_fit_exactly() {
        let (values, overflowed) = checked_counter_run(250, 5);

        assert_eq!(
            values,
            vec![251, 252, 253, 254, 255],
            "For start=250 and 5 attempts, expected exactly 251..=255. Got {:?}.",
            values
        );
        assert!(
            !overflowed,
            "Overflow flag should be false when no overflow attempt happens. Got true."
        );
    }

    #[test]
    fn overflows_immediately_when_starting_at_max() {
        let (values, overflowed) = checked_counter_run(255, 1);

        assert!(
            values.is_empty(),
            "No increments should succeed when starting at 255 and attempting one increment. Got {:?}.",
            values
        );
        assert!(
            overflowed,
            "Overflow flag should be true when incrementing 255. Got false."
        );
    }
}
