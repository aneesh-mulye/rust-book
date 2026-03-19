// Challenge 1.2 - Planet Weight Calculator
//
// Define `Planet` with variants:
// - Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune
//
// Implement:
// - `surface_gravity_ratio(planet: &Planet) -> f64`
// - `weight_on(earth_weight: f64, planet: &Planet) -> f64`
// - `all_planets() -> [Planet; 8]`

use std::num::Saturating;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

pub fn surface_gravity_ratio(planet: &Planet) -> f64 {
    match planet {
        Planet::Mercury => 0.38,
        Planet::Venus => 0.91,
        Planet::Earth => 1.0,
        Planet::Mars => 0.38,
        Planet::Jupiter => 2.53,
        Planet::Saturn => 1.07,
        Planet::Uranus => 0.89,
        Planet::Neptune => 1.13,
    }
}

pub fn weight_on(earth_weight: f64, planet: &Planet) -> f64 {
    earth_weight * surface_gravity_ratio(planet)
}

pub fn all_planets() -> [Planet; 8] {
    [
        Planet::Mercury,
        Planet::Venus,
        Planet::Earth,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
    ]
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
    use super::{Planet, all_planets, surface_gravity_ratio, weight_on};

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn manual_planet_array_contains_all_eight_in_order() {
        let planets = all_planets();
        let expected = [
            Planet::Mercury,
            Planet::Venus,
            Planet::Earth,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
        ];

        assert_eq!(
            planets, expected,
            "all_planets() should manually list the eight planets in solar-system order. Got {:?}.",
            planets
        );
    }

    #[test]
    fn gravity_ratios_match_expected_approximations() {
        assert!(
            approx_eq(surface_gravity_ratio(&Planet::Mercury), 0.38),
            "Mercury ratio should be about 0.38."
        );
        assert!(
            approx_eq(surface_gravity_ratio(&Planet::Earth), 1.0),
            "Earth ratio should be exactly 1.0."
        );
        assert!(
            approx_eq(surface_gravity_ratio(&Planet::Jupiter), 2.53),
            "Jupiter ratio should be about 2.53."
        );
        assert!(
            approx_eq(surface_gravity_ratio(&Planet::Neptune), 1.13),
            "Neptune ratio should be about 1.13."
        );
    }

    #[test]
    fn weight_on_uses_earth_weight_times_ratio() {
        let earth_weight = 75.0;

        assert!(
            approx_eq(weight_on(earth_weight, &Planet::Earth), 75.0),
            "Weight on Earth should stay unchanged at 75.0 kg."
        );
        assert!(
            approx_eq(weight_on(earth_weight, &Planet::Mercury), 28.5),
            "75.0 kg on Mercury should be about 28.5 kg."
        );
        assert!(
            approx_eq(weight_on(earth_weight, &Planet::Jupiter), 189.75),
            "75.0 kg on Jupiter should be about 189.75 kg."
        );
    }
}
