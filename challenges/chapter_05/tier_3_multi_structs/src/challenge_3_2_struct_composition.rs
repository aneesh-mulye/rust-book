// Challenge 3.2 - Struct Composition
//
// Define `Coordinate` and `City`, then implement:
// - `Coordinate::distance_to`
// - `City::new`
// - `City::is_bigger_than`
// - `City::distance_to` delegating to `Coordinate::distance_to`

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct City {
    pub name: String,
    pub population: u64,
    pub location: Coordinate,
}

impl Coordinate {
    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        let _ = (self, other);
        0.0
    }
}

impl City {
    pub fn new(name: &str, population: u64, x: f64, y: f64) -> City {
        let _ = (name, population, x, y);
        City {
            name: String::new(),
            population: 0,
            location: Coordinate { x: 0.0, y: 0.0 },
        }
    }

    pub fn is_bigger_than(&self, other: &City) -> bool {
        let _ = (self, other);
        false
    }

    pub fn distance_to(&self, other: &City) -> f64 {
        let _ = (self, other);
        0.0
    }
}

pub fn closest_pair(cities: &[City; 3]) -> (&str, &str) {
    let _ = cities;
    ("", "")
}

pub fn largest_city_name(cities: &[City; 3]) -> &str {
    let _ = cities;
    ""
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
    use super::{closest_pair, largest_city_name, City, Coordinate};

    #[test]
    fn city_methods_delegate_and_compare_correctly() {
        let a = City::new("Alpha", 1_000_000, 0.0, 0.0);
        let b = City::new("Beta", 500_000, 3.0, 4.0);

        assert!(
            a.is_bigger_than(&b),
            "Alpha should be bigger than Beta based on population."
        );
        assert!(
            !b.is_bigger_than(&a),
            "Beta should not be bigger than Alpha."
        );

        let dist = a.distance_to(&b);
        assert!(
            (dist - 5.0).abs() < 1e-12,
            "Distance between (0,0) and (3,4) should be 5. Got {dist}."
        );
    }

    #[test]
    fn coordinate_distance_standalone_works() {
        let p = Coordinate { x: -1.0, y: -1.0 };
        let q = Coordinate { x: 2.0, y: 3.0 };
        let dist = p.distance_to(&q);

        assert!(
            (dist - 5.0).abs() < 1e-12,
            "Coordinate distance should match Euclidean formula. Expected 5, got {dist}."
        );
    }

    #[test]
    fn finds_closest_pair_and_largest_city() {
        let cities = [
            City::new("Alpha", 1_000_000, 0.0, 0.0),
            City::new("Beta", 500_000, 1.0, 1.0),
            City::new("Gamma", 750_000, 10.0, 10.0),
        ];

        let pair = closest_pair(&cities);
        assert_eq!(
            pair,
            ("Alpha", "Beta"),
            "Closest pair should be Alpha/Beta for points (0,0), (1,1), (10,10). Got {:?}.",
            pair
        );

        let largest = largest_city_name(&cities);
        assert_eq!(
            largest, "Alpha",
            "Largest city by population should be Alpha. Got '{largest}'."
        );
    }
}
