// Challenge 4.2 - Borrowing Struct Fields
//
// Define `Inventory` with:
// - `name: String`
// - `items: [i32; 5]`
//
// Implement standalone functions:
// - `print_name(name: &str)`
// - `total_items(items: &[i32]) -> i32`
//
// For testability, `print_name` returns the formatted string.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    pub name: String,
    pub items: [i32; 5],
}

pub fn print_name(name: &str) -> String {
    let _ = name;
    String::new()
}

pub fn total_items(items: &[i32]) -> i32 {
    let _ = items;
    0
}

pub fn inventory_demo() -> (String, i32, i32, [i32; 5]) {
    let inv = Inventory {
        name: String::new(),
        items: [0; 5],
    };

    let name_line = print_name(&inv.name);
    let before_total = total_items(&inv.items);
    let after_total = 0;

    (name_line, before_total, after_total, inv.items)
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
    use super::{inventory_demo, print_name, total_items};

    #[test]
    fn total_items_sums_slice() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(
            total_items(&data),
            15,
            "Sum of [1,2,3,4,5] should be 15. Got {}.",
            total_items(&data)
        );
    }

    #[test]
    fn demo_shows_field_level_borrowing_and_mutation() {
        let (name_line, before_total, after_total, items) = inventory_demo();

        assert_eq!(
            name_line,
            "Inventory: Main Warehouse",
            "Name line should be formatted using borrowed name field. Got '{name_line}'."
        );
        assert_eq!(
            before_total, 15,
            "Before mutation, total should be 15 for [1,2,3,4,5]. Got {before_total}."
        );
        assert_eq!(
            after_total, 22,
            "After setting index 2 from 3 to 10, total should be 22. Got {after_total}."
        );
        assert_eq!(
            items,
            [1, 2, 10, 4, 5],
            "Expected updated items [1,2,10,4,5]. Got {:?}.",
            items
        );
    }

    #[test]
    fn print_name_formats_consistently() {
        assert_eq!(
            print_name("Shelf A"),
            "Inventory: Shelf A",
            "print_name should prefix the provided name with 'Inventory: '."
        );
    }
}
