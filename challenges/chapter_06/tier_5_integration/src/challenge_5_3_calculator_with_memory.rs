// Challenge 5.3 - Calculator with Memory
//
// Define:
// - `Op`
// - `Calculator`
//
// Implement:
// - `Calculator::new`
// - `execute`
// - `show`
// - `run_program(program) -> Vec<String>`

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add(f64),
    Sub(f64),
    Mul(f64),
    Div(f64),
    Store,
    Recall,
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Calculator {
    pub display: f64,
    pub memory: Option<f64>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display: -1.0,
            memory: Some(-1.0),
        }
    }

    pub fn execute(&mut self, op: &Op) {
        let _ = op;
    }

    pub fn show(&self) -> String {
        let _ = self;
        String::new()
    }
}

pub fn run_program(program: &[Op]) -> Vec<String> {
    let _ = program;
    Vec::new()
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
    use super::{run_program, Calculator, Op};

    #[test]
    fn execute_updates_display_and_memory_for_happy_path_ops() {
        let mut calc = Calculator::new();

        calc.execute(&Op::Add(10.0));
        calc.execute(&Op::Mul(3.0));
        calc.execute(&Op::Store);
        calc.execute(&Op::Sub(5.0));
        calc.execute(&Op::Div(5.0));

        assert!((calc.display - 5.0).abs() < 1e-10, "Display should be 5 after ((0 + 10) * 3 - 5) / 5.");
        assert_eq!(calc.memory, Some(30.0), "Memory should still contain 30.0 after Store.");
    }

    #[test]
    fn division_by_zero_and_empty_recall_preserve_state() {
        let mut calc = Calculator::new();
        calc.execute(&Op::Add(25.0));
        calc.execute(&Op::Div(0.0));

        assert!((calc.display - 25.0).abs() < 1e-10, "Division by zero should leave display unchanged at 25.");

        calc.execute(&Op::Clear);
        calc.execute(&Op::Recall);

        assert!((calc.display - 0.0).abs() < 1e-10, "Recall with empty memory should leave display at 0 after Clear.");
        assert_eq!(calc.memory, None, "Memory should still be empty after failed Recall.");
    }

    #[test]
    fn full_program_emits_one_state_line_per_operation() {
        let program = [
            Op::Add(10.0),
            Op::Mul(3.0),
            Op::Store,
            Op::Sub(5.0),
            Op::Div(0.0),
            Op::Div(5.0),
            Op::Recall,
            Op::Clear,
            Op::Recall,
        ];
        let lines = run_program(&program);

        assert_eq!(
            lines.len(),
            9,
            "run_program should return one output line per operation. Got {} lines.",
            lines.len()
        );
        assert!(
            lines.iter().any(|line| line.contains("division by zero")),
            "Program output should include a division-by-zero message. Got {:?}.",
            lines
        );
        assert!(
            lines.iter().any(|line| line.contains("memory empty")),
            "Program output should include a memory-empty message after Recall on empty memory. Got {:?}.",
            lines
        );
    }
}
