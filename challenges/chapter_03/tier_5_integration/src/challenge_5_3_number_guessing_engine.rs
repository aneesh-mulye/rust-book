// Challenge 5.3 - Number Guessing Engine
//
// Keep this deterministic:
// - Use `SECRET` as the target.
// - Iterate through guesses.
// - For each guess, produce one output line:
//   "Guess {i}: {guess} - Too low!"
//   "Guess {i}: {guess} - Too high!"
//   "Guess {i}: {guess} - Correct! Found it in {i} guesses."
// - Stop after a correct guess.
//
// Return value:
// - `Vec<String>` with the generated lines in order.
// - `Option<usize>` = Some(attempt_number) when found, otherwise None.

use std::cmp::Ordering;

pub const SECRET: i32 = 42;

pub fn evaluate_guesses(secret: i32, guesses: &[i32]) -> (Vec<String>, Option<usize>) {
    let mut results: Vec<String> = Vec::new();
    let mut attempts: Option<usize> = None;
    for i in 0..guesses.len() {
        let guess = guesses[i];
        let attempt = i + 1;
        match guesses[i].cmp(&secret) {
            Ordering::Less => results.push(format!("Guess {attempt}: {guess} - Too low!")),
            Ordering::Greater => results.push(format!("Guess {attempt}: {guess} - Too high!")),
            Ordering::Equal => {
                results.push(format!(
                    "Guess {attempt}: {guess} - Correct! Found it in {attempt} guesses."
                ));
                attempts = Some(attempt);
                break;
            }
        }
    }

    (results, attempts)
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
    use super::{SECRET, evaluate_guesses};

    #[test]
    fn stops_when_correct_guess_is_found() {
        let guesses = [25, 50, 37, 42, 99];
        let (lines, found_at) = evaluate_guesses(SECRET, &guesses);

        let expected = vec![
            "Guess 1: 25 - Too low!".to_string(),
            "Guess 2: 50 - Too high!".to_string(),
            "Guess 3: 37 - Too low!".to_string(),
            "Guess 4: 42 - Correct! Found it in 4 guesses.".to_string(),
        ];

        assert_eq!(
            lines, expected,
            "Engine output lines are wrong when a correct guess exists.\nExpected: {:?}\nActual:   {:?}",
            expected, lines
        );
        assert_eq!(
            found_at,
            Some(4),
            "Expected to report success on attempt 4, got {:?}.",
            found_at
        );
    }

    #[test]
    fn reports_failure_when_secret_not_found() {
        let guesses = [10, 11, 12];
        let (lines, found_at) = evaluate_guesses(SECRET, &guesses);

        assert_eq!(
            lines.len(),
            guesses.len(),
            "When secret is never guessed, should emit one line per guess. Expected {}, got {}.",
            guesses.len(),
            lines.len()
        );
        assert_eq!(
            found_at, None,
            "Expected None when secret is not guessed, got {:?}.",
            found_at
        );
        assert!(
            lines.last().unwrap_or(&String::new()).contains("Too low")
                || lines.last().unwrap_or(&String::new()).contains("Too high"),
            "Final line should still be a high/low message when no guess is correct. Got {:?}.",
            lines.last()
        );
    }
}
