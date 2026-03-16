// Challenge 4.4 - Vehicle Fleet Analyzer
//
// Define `FuelType` and `Vehicle`, then implement:
// - `Vehicle::new`
// - `is_green`
// - `cost_per_km`
// - `drive`
// - `describe` (return String for testability)
// - `cheapest_vehicle_name`
// - `green_vehicle_names`

#[derive(Debug, Clone, PartialEq)]
pub enum FuelType {
    Gasoline,
    Diesel,
    Electric,
    Hybrid(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vehicle {
    pub name: String,
    pub fuel: FuelType,
    pub efficiency: f64,
    pub odometer: f64,
}

impl Vehicle {
    pub fn new(name: &str, fuel: FuelType, efficiency: f64) -> Vehicle {
        let _ = (name, &fuel, efficiency);
        Vehicle {
            name: String::new(),
            fuel,
            efficiency: 0.0,
            odometer: 0.0,
        }
    }

    pub fn is_green(&self) -> bool {
        let _ = self;
        false
    }

    pub fn cost_per_km(&self) -> f64 {
        let _ = self;
        0.0
    }

    pub fn drive(&mut self, km: f64) {
        let _ = km;
    }

    pub fn describe(&self) -> String {
        let _ = self;
        String::new()
    }
}

pub fn cheapest_vehicle_name(fleet: &[Vehicle]) -> Option<&str> {
    let _ = fleet;
    None
}

pub fn green_vehicle_names(fleet: &[Vehicle]) -> Vec<String> {
    let _ = fleet;
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
    use super::{cheapest_vehicle_name, green_vehicle_names, FuelType, Vehicle};

    #[test]
    fn green_logic_and_cost_rules_work() {
        let gas = Vehicle::new("Gas Sedan", FuelType::Gasoline, 15.0);
        let diesel = Vehicle::new("Diesel Van", FuelType::Diesel, 20.0);
        let electric = Vehicle::new("EV", FuelType::Electric, 5.0);
        let hybrid = Vehicle::new("Hybrid", FuelType::Hybrid(0.6), 20.0);

        assert!(!gas.is_green(), "Gasoline vehicle should not be green.");
        assert!(!diesel.is_green(), "Diesel vehicle should not be green.");
        assert!(electric.is_green(), "Electric vehicle should be green.");
        assert!(hybrid.is_green(), "Hybrid with 60% electric share should be green.");

        assert!((gas.cost_per_km() - 0.1).abs() < 1e-10, "Gasoline cost should be 1.5 / 15 = 0.1.");
        assert!((diesel.cost_per_km() - 0.065).abs() < 1e-10, "Diesel cost should be 1.3 / 20 = 0.065.");
        assert!((electric.cost_per_km() - 0.08).abs() < 1e-10, "Electric cost should be 0.4 / 5 = 0.08.");
        assert!(
            (hybrid.cost_per_km() - 0.042).abs() < 1e-10,
            "Hybrid cost should use weighted energy cost based on electric percentage. Expected 0.042."
        );
    }

    #[test]
    fn driving_updates_odometer_and_description_mentions_key_fields() {
        let mut vehicle = Vehicle::new("EV", FuelType::Electric, 5.0);
        vehicle.drive(12.5);

        assert!((vehicle.odometer - 12.5).abs() < 1e-10, "Driving 12.5 km should update odometer to 12.5.");

        let text = vehicle.describe();
        assert!(text.contains("EV"), "Description should include the vehicle name. Got '{text}'.");
        assert!(text.contains("12.5"), "Description should include the odometer value. Got '{text}'.");
    }

    #[test]
    fn fleet_helpers_find_cheapest_and_green_vehicles() {
        let fleet = [
            Vehicle::new("Gas Sedan", FuelType::Gasoline, 15.0),
            Vehicle::new("Diesel Van", FuelType::Diesel, 20.0),
            Vehicle::new("EV", FuelType::Electric, 5.0),
            Vehicle::new("Hybrid", FuelType::Hybrid(0.6), 20.0),
        ];

        assert_eq!(
            cheapest_vehicle_name(&fleet),
            Some("Hybrid"),
            "Hybrid should be cheapest with the given cost formula."
        );
        assert_eq!(
            green_vehicle_names(&fleet),
            vec![String::from("EV"), String::from("Hybrid")],
            "Green vehicles should be the electric car and the 60% hybrid."
        );
    }
}
