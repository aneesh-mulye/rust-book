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

use core::f64;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FuelType {
    Gasoline,
    Diesel,
    Electric,
    Hybrid(f64),
}

impl fmt::Display for FuelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gasoline => write!(f, "Gasoline"),
            Self::Electric => write!(f, "Electric"),
            Self::Diesel => write!(f, "Diesel"),
            Self::Hybrid(p) => write!(f, "Hybrid ({})", p),
        }
    }
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
        Vehicle {
            name: String::from(name),
            fuel,
            efficiency,
            odometer: 0.0,
        }
    }

    pub fn is_green(&self) -> bool {
        match self.fuel {
            FuelType::Electric => true,
            FuelType::Hybrid(p) => p > 0.5,
            _ => false,
        }
    }

    pub fn cost_per_km(&self) -> f64 {
        match self.fuel {
            FuelType::Gasoline => 1.5 / self.efficiency,
            FuelType::Diesel => 1.3 / self.efficiency,
            FuelType::Electric => 0.4 / self.efficiency,
            FuelType::Hybrid(p) => (p * 0.4 + (1.0 - p) * 1.5) / self.efficiency,
        }
    }

    pub fn drive(&mut self, km: f64) {
        self.odometer += km;
    }

    pub fn describe(&self) -> String {
        format!(
            "{} {} {} {}",
            self.name, self.fuel, self.efficiency, self.odometer
        )
    }
}

pub fn cheapest_vehicle_name(fleet: &[Vehicle]) -> Option<&str> {
    let mut cheapest_name: Option<&str> = None;
    let mut lowest_cost = f64::MAX;
    for vehicle in fleet {
        let cpk = vehicle.cost_per_km();
        if cpk < lowest_cost {
            lowest_cost = cpk;
            cheapest_name = Some(&vehicle.name);
        }
    }
    cheapest_name
}

pub fn green_vehicle_names(fleet: &[Vehicle]) -> Vec<String> {
    fleet
        .iter()
        .filter(|vehicle| vehicle.is_green())
        .map(|vehicle| vehicle.name.clone())
        .collect()
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
    use super::{FuelType, Vehicle, cheapest_vehicle_name, green_vehicle_names};

    #[test]
    fn green_logic_and_cost_rules_work() {
        let gas = Vehicle::new("Gas Sedan", FuelType::Gasoline, 15.0);
        let diesel = Vehicle::new("Diesel Van", FuelType::Diesel, 20.0);
        let electric = Vehicle::new("EV", FuelType::Electric, 5.0);
        let hybrid = Vehicle::new("Hybrid", FuelType::Hybrid(0.6), 20.0);

        assert!(!gas.is_green(), "Gasoline vehicle should not be green.");
        assert!(!diesel.is_green(), "Diesel vehicle should not be green.");
        assert!(electric.is_green(), "Electric vehicle should be green.");
        assert!(
            hybrid.is_green(),
            "Hybrid with 60% electric share should be green."
        );

        assert!(
            (gas.cost_per_km() - 0.1).abs() < 1e-10,
            "Gasoline cost should be 1.5 / 15 = 0.1."
        );
        assert!(
            (diesel.cost_per_km() - 0.065).abs() < 1e-10,
            "Diesel cost should be 1.3 / 20 = 0.065."
        );
        assert!(
            (electric.cost_per_km() - 0.08).abs() < 1e-10,
            "Electric cost should be 0.4 / 5 = 0.08."
        );
        assert!(
            (hybrid.cost_per_km() - 0.042).abs() < 1e-10,
            "Hybrid cost should use weighted energy cost based on electric percentage. Expected 0.042."
        );
    }

    #[test]
    fn driving_updates_odometer_and_description_mentions_key_fields() {
        let mut vehicle = Vehicle::new("EV", FuelType::Electric, 5.0);
        vehicle.drive(12.5);

        assert!(
            (vehicle.odometer - 12.5).abs() < 1e-10,
            "Driving 12.5 km should update odometer to 12.5."
        );

        let text = vehicle.describe();
        assert!(
            text.contains("EV"),
            "Description should include the vehicle name. Got '{text}'."
        );
        assert!(
            text.contains("12.5"),
            "Description should include the odometer value. Got '{text}'."
        );
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
