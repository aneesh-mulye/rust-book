// Challenge 3.2 - Struct Composition
//
// Define `Coordinate` and `City`, then implement:
// - `Coordinate::distance_to`
// - `City::new`
// - `City::is_bigger_than`
// - `City::distance_to` delegating to `Coordinate::distance_to`

use core::f64;

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
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl City {
    pub fn new(name: &str, population: u64, x: f64, y: f64) -> City {
        let _ = (name, population, x, y);
        City {
            name: name.to_string(),
            population,
            location: Coordinate { x, y },
        }
    }

    pub fn is_bigger_than(&self, other: &City) -> bool {
        self.population > other.population
    }

    pub fn distance_to(&self, other: &City) -> f64 {
        self.location.distance_to(&other.location)
    }
}

pub fn closest_pair(cities: &[City; 3]) -> (&str, &str) {
    let mut min_pair = ("", "");
    let mut min_dist = f64::MAX;
    for c1 in cities {
        for c2 in cities {
            if c1 == c2 {
                continue;
            }
            let this_dist = c1.distance_to(c2);
            if this_dist < min_dist {
                min_dist = this_dist;
                min_pair = (&c1.name, &c2.name);
            }
        }
    }
    min_pair
}

pub fn largest_city_name(cities: &[City; 3]) -> &str {
    let mut largest_name = "";
    for city in cities {
        if city.name.len() > largest_name.len() {
            largest_name = &city.name;
        }
    }
    largest_name
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
    use super::{City, Coordinate, closest_pair, largest_city_name};

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
