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
            display: 0.0,
            memory: None,
        }
    }

    pub fn execute(&mut self, op: &Op) {
        match *op {
            Op::Add(n) => {
                self.display += n;
            }
            Op::Sub(n) => {
                self.display -= n;
            }
            Op::Mul(n) => {
                self.display *= n;
            }
            Op::Div(n) => {
                if n != 0.0 {
                    self.display /= n;
                }
            }
            Op::Store => self.memory = Some(self.display),
            Op::Recall => {
                if let Some(mem) = self.memory {
                    self.display = mem;
                }
            }
            Op::Clear => {
                self.display = 0.0;
                self.memory = None;
            }
        }
    }

    pub fn show(&self) -> String {
        format!(
            "display: {}, memory: {}",
            self.display,
            match self.memory {
                None => String::from("empty"),
                Some(mem) => format!("{mem}"),
            }
        )
    }
}

pub fn run_program(program: &[Op]) -> Vec<String> {
    let mut calc = Calculator::new();
    let mut output: Vec<String> = Vec::new();
    for op in program {
        calc.execute(op);
        output.push(calc.show());
    }
    output
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
    use super::{Calculator, Op, run_program};

    #[test]
    fn execute_updates_display_and_memory_for_happy_path_ops() {
        let mut calc = Calculator::new();

        calc.execute(&Op::Add(10.0));
        calc.execute(&Op::Mul(3.0));
        calc.execute(&Op::Store);
        calc.execute(&Op::Sub(5.0));
        calc.execute(&Op::Div(5.0));

        assert!(
            (calc.display - 5.0).abs() < 1e-10,
            "Display should be 5 after ((0 + 10) * 3 - 5) / 5."
        );
        assert_eq!(
            calc.memory,
            Some(30.0),
            "Memory should still contain 30.0 after Store."
        );
    }

    #[test]
    fn division_by_zero_and_empty_recall_preserve_state() {
        let mut calc = Calculator::new();
        calc.execute(&Op::Add(25.0));
        calc.execute(&Op::Div(0.0));

        assert!(
            (calc.display - 25.0).abs() < 1e-10,
            "Division by zero should leave display unchanged at 25."
        );

        calc.execute(&Op::Clear);
        calc.execute(&Op::Recall);

        assert!(
            (calc.display - 0.0).abs() < 1e-10,
            "Recall with empty memory should leave display at 0 after Clear."
        );
        assert_eq!(
            calc.memory, None,
            "Memory should still be empty after failed Recall."
        );
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
        assert_eq!(
            lines[0],
            "display: 10, memory: empty",
            "After Add(10.0), the first state snapshot should show display 10 and empty memory."
        );
        assert_eq!(
            lines[4],
            "display: 25, memory: 30",
            "After Div(0.0), output should still show the unchanged state because the invalid division is ignored."
        );
        assert_eq!(
            lines[6],
            "display: 30, memory: 30",
            "After Recall with stored memory, display should return to 30 while memory stays 30."
        );
        assert_eq!(
            lines[8],
            "display: 0, memory: empty",
            "After Recall on empty memory, output should show the unchanged cleared state."
        );
    }
}
