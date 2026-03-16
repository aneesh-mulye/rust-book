// Challenge 5.2 - Inventory with Pricing Rules
//
// Define:
// - `Category`
// - `Condition`
// - `Item`
//
// Implement item methods:
// - `new`
// - `effective_price`
// - `is_sellable`
// - `tax_rate`
// - `final_price`
// - `describe` (return String for testability)
// - `cheapest_sellable_name`

#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Electronics,
    Clothing,
    Food { perishable: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    New,
    Refurbished(u8),
    Damaged(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub base_price: f64,
    pub category: Category,
    pub condition: Condition,
    pub in_stock: bool,
}

impl Item {
    pub fn new(name: &str, price: f64, category: Category, condition: Condition) -> Item {
        let _ = (name, price, &category, &condition);
        Item {
            name: String::new(),
            base_price: 0.0,
            category,
            condition,
            in_stock: true,
        }
    }

    pub fn effective_price(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn is_sellable(&self) -> bool {
        let _ = self;
        false
    }

    pub fn tax_rate(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn final_price(&self) -> Option<f64> {
        let _ = self;
        None
    }

    pub fn describe(&self) -> String {
        let _ = self;
        String::new()
    }
}

pub fn cheapest_sellable_name(items: &[Item]) -> Option<&str> {
    let _ = items;
    None
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
    use super::{cheapest_sellable_name, Category, Condition, Item};

    #[test]
    fn effective_price_and_tax_rules_follow_category_and_condition() {
        let laptop = Item::new("Laptop", 1000.0, Category::Electronics, Condition::New);
        let shirt = Item::new("Shirt", 50.0, Category::Clothing, Condition::Refurbished(80));
        let apples = Item::new(
            "Apples",
            20.0,
            Category::Food { perishable: true },
            Condition::New,
        );

        assert!((laptop.effective_price() - 1000.0).abs() < 1e-10, "New item should keep full base price.");
        assert!((shirt.effective_price() - 40.0).abs() < 1e-10, "Refurbished(80) should scale price to 80%.");
        assert!((laptop.tax_rate() - 0.15).abs() < 1e-10, "Electronics tax rate should be 0.15.");
        assert!((shirt.tax_rate() - 0.08).abs() < 1e-10, "Clothing tax rate should be 0.08.");
        assert!((apples.tax_rate() - 0.0).abs() < 1e-10, "Perishable food tax rate should be 0.0.");
    }

    #[test]
    fn sellable_and_final_price_respect_damage_and_stock() {
        let laptop = Item::new("Laptop", 1000.0, Category::Electronics, Condition::New);
        let damaged = Item::new(
            "Cracked Phone",
            500.0,
            Category::Electronics,
            Condition::Damaged(String::from("screen")),
        );
        let mut out_of_stock = Item::new("Shirt", 50.0, Category::Clothing, Condition::New);
        out_of_stock.in_stock = false;

        assert!(laptop.is_sellable(), "New in-stock item should be sellable.");
        assert!(!damaged.is_sellable(), "Damaged item should not be sellable.");
        assert!(!out_of_stock.is_sellable(), "Out-of-stock item should not be sellable.");
        assert_eq!(damaged.final_price(), None, "Unsellable items should return None final price.");
        assert!(
            (laptop.final_price().unwrap_or(-1.0) - 1150.0).abs() < 1e-10,
            "Laptop final price should be 1000 * 1.15 = 1150."
        );
    }

    #[test]
    fn helpers_describe_items_and_find_cheapest_sellable() {
        let items = [
            Item::new("Laptop", 1000.0, Category::Electronics, Condition::New),
            Item::new("Shirt", 50.0, Category::Clothing, Condition::Refurbished(80)),
            Item::new(
                "Apples",
                20.0,
                Category::Food { perishable: true },
                Condition::New,
            ),
            Item::new(
                "Cracked Phone",
                500.0,
                Category::Electronics,
                Condition::Damaged(String::from("screen")),
            ),
        ];

        let text = items[0].describe();
        assert!(text.contains("Laptop"), "Description should include the item name. Got '{text}'.");
        assert!(text.contains("Electronics"), "Description should mention the category. Got '{text}'.");
        assert_eq!(
            cheapest_sellable_name(&items),
            Some("Apples"),
            "Cheapest sellable item should be Apples."
        );
    }
}
