// Challenge 5.1 - Student Grade Tracker
//
// Define `Student` with scores and implement methods:
// - new
// - average
// - highest
// - lowest
// - letter_grade (calls average)
// - summary (calls other methods)

#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
    pub scores: [f64; 5],
}

impl Student {
    pub fn new(name: &str, scores: [f64; 5]) -> Student {
        let _ = (name, scores);
        Student {
            name: String::new(),
            scores: [0.0; 5],
        }
    }

    pub fn average(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn highest(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn lowest(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn letter_grade(&self) -> char {
        let _ = self;
        '?'
    }

    pub fn summary(&self) -> String {
        let _ = self;
        String::new()
    }
}

pub fn best_student(students: &[Student]) -> (&str, f64) {
    let _ = students;
    ("", 0.0)
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
    use super::{best_student, Student};

    #[test]
    fn computes_stats_for_student() {
        let alice = Student::new("Alice", [92.0, 88.0, 95.0, 78.0, 90.0]);

        assert!((alice.average() - 88.6).abs() < 1e-10, "Alice average should be 88.6. Got {}.", alice.average());
        assert!((alice.highest() - 95.0).abs() < 1e-10, "Alice highest should be 95. Got {}.", alice.highest());
        assert!((alice.lowest() - 78.0).abs() < 1e-10, "Alice lowest should be 78. Got {}.", alice.lowest());
        assert_eq!(alice.letter_grade(), 'B', "Alice grade should be B based on average 88.6.");
    }

    #[test]
    fn picks_best_student_by_average() {
        let students = vec![
            Student::new("Alice", [92.0, 88.0, 95.0, 78.0, 90.0]),
            Student::new("Bob", [55.0, 62.0, 70.0, 45.0, 58.0]),
            Student::new("Cara", [80.0, 85.0, 82.0, 88.0, 84.0]),
        ];

        let (name, avg) = best_student(&students);
        assert_eq!(name, "Alice", "Best student should be Alice. Got '{name}'.");
        assert!((avg - 88.6).abs() < 1e-10, "Best average should be 88.6. Got {avg}.");
    }

    #[test]
    fn summary_contains_core_lines() {
        let bob = Student::new("Bob", [55.0, 62.0, 70.0, 45.0, 58.0]);
        let summary = bob.summary();

        assert!(summary.contains("--- Bob ---"), "Summary should include header with student name.\nGot:\n{summary}");
        assert!(summary.contains("Grade: F"), "Summary should include computed letter grade.\nGot:\n{summary}");
    }
}
