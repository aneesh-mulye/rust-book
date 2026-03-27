// Challenge 4.3 - Happy Path with `let...else`
//
// Implement:
// - `compute_bmi(weight_kg, height_m) -> Option<f64>`
// - `classify_bmi(bmi) -> &'static str`

pub fn compute_bmi(weight_kg: Option<f64>, height_m: Option<f64>) -> Option<f64> {
    let Some(weight) = weight_kg else {
        return None;
    };
    let Some(height) = height_m else {
        return None;
    };

    Some(weight / (height * height))
}

pub fn classify_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "underweight"
    } else if bmi < 25.0 {
        "normal"
    } else if bmi < 30.0 {
        "overweight"
    } else {
        "obese"
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
    use super::{classify_bmi, compute_bmi};

    #[test]
    fn computes_bmi_when_both_inputs_exist() {
        let bmi = compute_bmi(Some(70.0), Some(1.75)).unwrap_or(-1.0);
        assert!(
            (bmi - 22.857142857142858).abs() < 1e-10,
            "BMI should be about 22.8571. Got {bmi}."
        );
        assert_eq!(
            classify_bmi(bmi),
            "normal",
            "BMI around 22.9 should classify as normal."
        );
    }

    #[test]
    fn returns_none_when_any_input_is_missing() {
        assert_eq!(
            compute_bmi(None, Some(1.80)),
            None,
            "Missing weight should return None."
        );
        assert_eq!(
            compute_bmi(Some(90.0), None),
            None,
            "Missing height should return None."
        );
    }

    #[test]
    fn bmi_categories_follow_standard_thresholds() {
        assert_eq!(
            classify_bmi(17.5),
            "underweight",
            "BMI 17.5 should be underweight."
        );
        assert_eq!(classify_bmi(24.0), "normal", "BMI 24.0 should be normal.");
        assert_eq!(
            classify_bmi(27.0),
            "overweight",
            "BMI 27.0 should be overweight."
        );
        assert_eq!(classify_bmi(31.0), "obese", "BMI 31.0 should be obese.");
    }
}
