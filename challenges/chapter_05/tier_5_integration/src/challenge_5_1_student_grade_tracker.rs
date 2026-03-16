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
        Student {
            name: String::from(name),
            scores,
        }
    }

    pub fn average(&self) -> f64 {
        let mut total = 0.0;
        for score in self.scores {
            total += score;
        }
        total / (self.scores.len() as f64)
    }

    pub fn highest(&self) -> f64 {
        let mut max_score = f64::MIN;
        for score in self.scores {
            max_score = max_score.max(score)
        }
        max_score
    }

    pub fn lowest(&self) -> f64 {
        let mut min_score = f64::MAX;
        for score in self.scores {
            min_score = min_score.min(score)
        }
        min_score
    }

    pub fn letter_grade(&self) -> char {
        let avg = self.average();
        if avg > 90.0 {
            'A'
        } else if avg > 80.0 {
            'B'
        } else if avg > 70.0 {
            'C'
        } else if avg > 60.0 {
            'D'
        } else {
            'F'
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "--- {} ---\nScores: {:?}\nAverage: {:?}\nHighest: {:?}\nLowest: {:?}\nGrade: {}",
            self.name,
            self.scores,
            self.average(),
            self.highest(),
            self.lowest(),
            self.letter_grade()
        )
    }
}

pub fn best_student(students: &[Student]) -> (&str, f64) {
    let mut best = ("", 0.0);
    for student in students {
        let student_avg = student.average();
        if student_avg > best.1 {
            best.0 = &student.name;
            best.1 = student_avg; // Yeah yeah inefficent
        }
    }
    best
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
    use super::{Student, best_student};

    #[test]
    fn computes_stats_for_student() {
        let alice = Student::new("Alice", [92.0, 88.0, 95.0, 78.0, 90.0]);

        assert!(
            (alice.average() - 88.6).abs() < 1e-10,
            "Alice average should be 88.6. Got {}.",
            alice.average()
        );
        assert!(
            (alice.highest() - 95.0).abs() < 1e-10,
            "Alice highest should be 95. Got {}.",
            alice.highest()
        );
        assert!(
            (alice.lowest() - 78.0).abs() < 1e-10,
            "Alice lowest should be 78. Got {}.",
            alice.lowest()
        );
        assert_eq!(
            alice.letter_grade(),
            'B',
            "Alice grade should be B based on average 88.6."
        );
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
        assert!(
            (avg - 88.6).abs() < 1e-10,
            "Best average should be 88.6. Got {avg}."
        );
    }

    #[test]
    fn summary_contains_core_lines() {
        let bob = Student::new("Bob", [55.0, 62.0, 70.0, 45.0, 58.0]);
        let summary = bob.summary();

        assert!(
            summary.contains("--- Bob ---"),
            "Summary should include header with student name.\nGot:\n{summary}"
        );
        assert!(
            summary.contains("Grade: F"),
            "Summary should include computed letter grade.\nGot:\n{summary}"
        );
    }
}
