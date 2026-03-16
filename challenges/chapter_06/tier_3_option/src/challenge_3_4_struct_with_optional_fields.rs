// Challenge 3.4 - Struct with Optional Fields
//
// Define `SensorReading` with:
// - `sensor_id: u32`
// - `temperature: Option<f64>`
// - `humidity: Option<f64>`
//
// Implement:
// - `new`
// - `with_temperature`
// - `with_humidity`
// - `is_complete`
// - `heat_index`
// - `display` (return a String for testability)

#[derive(Debug, Clone, PartialEq)]
pub struct SensorReading {
    pub sensor_id: u32,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
}

impl SensorReading {
    pub fn new(id: u32) -> SensorReading {
        let _ = id;
        SensorReading {
            sensor_id: 0,
            temperature: None,
            humidity: None,
        }
    }

    pub fn with_temperature(self, t: f64) -> SensorReading {
        let _ = t;
        self
    }

    pub fn with_humidity(self, h: f64) -> SensorReading {
        let _ = h;
        self
    }

    pub fn is_complete(&self) -> bool {
        let _ = self;
        false
    }

    pub fn heat_index(&self) -> Option<f64> {
        let _ = self;
        None
    }

    pub fn display(&self) -> String {
        let _ = self;
        String::new()
    }
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
    use super::SensorReading;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn builder_methods_fill_optional_fields() {
        let reading = SensorReading::new(1)
            .with_temperature(23.5)
            .with_humidity(65.0);

        assert_eq!(reading.sensor_id, 1, "Sensor id should be preserved as 1.");
        assert_eq!(reading.temperature, Some(23.5), "Temperature should be Some(23.5).");
        assert_eq!(reading.humidity, Some(65.0), "Humidity should be Some(65.0).");
        assert!(reading.is_complete(), "Reading with both values present should be complete.");
        assert!(
            approx_eq(reading.heat_index().unwrap_or(-1.0), 26.75),
            "Heat index should be temperature + 0.05 * humidity = 26.75."
        );
    }

    #[test]
    fn incomplete_readings_report_none_for_heat_index() {
        let temp_only = SensorReading::new(2).with_temperature(19.0);
        let empty = SensorReading::new(3);

        assert!(!temp_only.is_complete(), "Temperature-only reading should not be complete.");
        assert_eq!(temp_only.heat_index(), None, "Missing humidity should produce None heat index.");
        assert_eq!(empty.heat_index(), None, "Missing both readings should produce None heat index.");
    }

    #[test]
    fn display_formats_missing_values_as_dashes() {
        let full = SensorReading::new(1).with_temperature(23.5).with_humidity(65.0);
        let temp_only = SensorReading::new(2).with_temperature(19.0);
        let empty = SensorReading::new(3);

        assert_eq!(
            full.display(),
            "Sensor 1: temp=23.5, humidity=65.0, heat index=26.75",
            "Display for full reading should show both values and computed heat index."
        );
        assert_eq!(
            temp_only.display(),
            "Sensor 2: temp=19.0, humidity=--, heat index=--",
            "Display should use '--' for missing humidity and heat index."
        );
        assert_eq!(
            empty.display(),
            "Sensor 3: temp=--, humidity=--, heat index=--",
            "Display should use '--' for all missing values."
        );
    }
}
