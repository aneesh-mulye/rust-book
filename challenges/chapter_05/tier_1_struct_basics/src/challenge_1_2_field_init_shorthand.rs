// Challenge 1.2 - Field Init Shorthand
//
// Define `Sensor` and implement:
// `create_sensor(id: u32, label: String, reading: f64) -> Sensor`
//
// The point is to use field init shorthand where parameter names match field names.

#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    pub id: u32,
    pub label: String,
    pub reading: f64,
}

pub fn create_sensor(id: u32, label: String, reading: f64) -> Sensor {
    let _ = (id, label, reading);
    Sensor {
        id: 0,
        label: String::new(),
        reading: 0.0,
    }
}

pub fn sensor_examples() -> (Sensor, Sensor) {
    (
        create_sensor(1, String::from("temperature"), 21.5),
        create_sensor(2, String::from("humidity"), 48.0),
    )
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
    use super::{create_sensor, sensor_examples, Sensor};

    #[test]
    fn creates_sensor_with_matching_fields() {
        let sensor = create_sensor(42, String::from("pressure"), 101.3);

        assert_eq!(sensor.id, 42, "Sensor id should be 42. Got {}.", sensor.id);
        assert_eq!(
            sensor.label, "pressure",
            "Sensor label should be 'pressure'. Got '{}'.",
            sensor.label
        );
        assert!(
            (sensor.reading - 101.3).abs() < 1e-10,
            "Sensor reading should be 101.3. Got {}.",
            sensor.reading
        );
    }

    #[test]
    fn builds_both_prompt_style_sensors() {
        let (a, b) = sensor_examples();

        assert_eq!(
            a,
            Sensor {
                id: 1,
                label: String::from("temperature"),
                reading: 21.5,
            },
            "First example sensor is incorrect. Got {:?}.",
            a
        );
        assert_eq!(
            b,
            Sensor {
                id: 2,
                label: String::from("humidity"),
                reading: 48.0,
            },
            "Second example sensor is incorrect. Got {:?}.",
            b
        );
    }
}
