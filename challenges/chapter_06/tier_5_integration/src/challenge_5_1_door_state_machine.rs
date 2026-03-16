// Challenge 5.1 - Door State Machine
//
// Define:
// - `DoorState`
// - `Action`
//
// Implement:
// - `transition(state, action) -> DoorState`
// - `DoorState::describe(&self) -> &'static str`
// - `requires_key(action) -> bool`
// - `run_sequence(start, actions) -> Vec<String>`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorState {
    Locked,
    Closed,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    InsertKey,
    Turn,
    Push,
    Pull,
    RemoveKey,
}

impl DoorState {
    pub fn describe(&self) -> &'static str {
        let _ = self;
        ""
    }
}

pub fn transition(state: DoorState, action: &Action) -> DoorState {
    let _ = action;
    state
}

pub fn requires_key(action: &Action) -> bool {
    let _ = action;
    false
}

pub fn run_sequence(start: DoorState, actions: &[Action]) -> Vec<String> {
    let _ = (start, actions);
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
    use super::{requires_key, run_sequence, transition, Action, DoorState};

    #[test]
    fn tuple_style_transitions_match_rules() {
        assert_eq!(
            transition(DoorState::Locked, &Action::Turn),
            DoorState::Closed,
            "Locked + Turn should unlock the door to Closed."
        );
        assert_eq!(
            transition(DoorState::Closed, &Action::Push),
            DoorState::Open,
            "Closed + Push should open the door."
        );
        assert_eq!(
            transition(DoorState::Open, &Action::Pull),
            DoorState::Closed,
            "Open + Pull should close the door."
        );
        assert_eq!(
            transition(DoorState::Locked, &Action::Push),
            DoorState::Locked,
            "Undefined transitions should leave state unchanged."
        );
    }

    #[test]
    fn requires_key_distinguishes_key_actions_from_manual_actions() {
        assert!(requires_key(&Action::Turn), "Turning the lock should require a key.");
        assert!(!requires_key(&Action::Push), "Pushing the door should not require a key.");
    }

    #[test]
    fn prompt_sequence_produces_expected_state_progression() {
        let actions = [
            Action::InsertKey,
            Action::Turn,
            Action::Push,
            Action::Pull,
            Action::Turn,
            Action::RemoveKey,
        ];
        let expected = vec![
            String::from("[InsertKey] Locked -> Locked (no state change)"),
            String::from("[Turn] Locked -> Closed"),
            String::from("[Push] Closed -> Open"),
            String::from("[Pull] Open -> Closed"),
            String::from("[Turn] Closed -> Locked"),
            String::from("[RemoveKey] Locked -> Locked (no state change)"),
        ];
        let actual = run_sequence(DoorState::Locked, &actions);

        assert_eq!(
            actual, expected,
            "Door state sequence should match the prompt's transition table. Got {:?}.",
            actual
        );
    }
}
