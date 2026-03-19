// Challenge 2.3 - UI Event Dispatcher
//
// Define `Event` with variants:
// - Click { x, y }
// - KeyPress(char)
// - TextInput(String)
// - Scroll(i32)
// - Close
//
// Implement:
// - `handle(event: &Event) -> String`
// - `process(events: &[Event]) -> Vec<String>`

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    TextInput(String),
    Scroll(i32),
    Close,
}

pub fn handle(event: &Event) -> String {
    match event {
        Event::Click { x, y } => {
            if 0 <= *x && *x <= 200 && 0 <= *y && *y <= 50 {
                String::from("Button area clicked")
            } else {
                format!("Click at ({x}, {y})")
            }
        }
        Event::KeyPress(c) => {
            if c.is_alphabetic() {
                format!("Letter key: {c}")
            } else if c.is_numeric() {
                String::from("Number key")
            } else {
                format!("{c}")
            }
        }
        Event::TextInput(text) => {
            format!("Text input: {} ({} chars)", text.clone(), text.len())
        }
        Event::Scroll(amount) => {
            if *amount <= 0 {
                String::from("Scroll down")
            } else {
                String::from("Scroll up")
            }
        }
        Event::Close => String::from("Window closed"),
    }
}

pub fn process(events: &[Event]) -> Vec<String> {
    events.iter().map(handle).collect()
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
    use super::{Event, handle, process};

    #[test]
    fn handles_each_variant_type() {
        assert_eq!(
            handle(&Event::Click { x: 50, y: 25 }),
            "Button area clicked",
            "Click inside 0..200 by 0..50 should be treated as a button click."
        );
        assert_eq!(
            handle(&Event::Click { x: 250, y: 90 }),
            "Click at (250, 90)",
            "Click outside the button area should report coordinates."
        );
        assert_eq!(
            handle(&Event::KeyPress('a')),
            "Letter key: a",
            "Alphabetic key should be labeled as a letter."
        );
        assert_eq!(
            handle(&Event::KeyPress('7')),
            "Number key",
            "Numeric key should be labeled as a number key."
        );
        assert_eq!(
            handle(&Event::TextInput(String::from("rust"))),
            "Text input: rust (4 chars)",
            "Text input should show the text and its character count."
        );
        assert_eq!(
            handle(&Event::Scroll(-3)),
            "Scroll down",
            "Negative scroll should be labeled as down."
        );
        assert_eq!(
            handle(&Event::Close),
            "Window closed",
            "Close event should say the window closed."
        );
    }

    #[test]
    fn processing_array_preserves_order() {
        let events = [Event::KeyPress('x'), Event::Scroll(2), Event::Close];
        let expected = vec![
            String::from("Letter key: x"),
            String::from("Scroll up"),
            String::from("Window closed"),
        ];
        let actual = process(&events);

        assert_eq!(
            actual, expected,
            "process() should handle events in order and return one output per event. Got {:?}.",
            actual
        );
    }
}
