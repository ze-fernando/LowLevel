pub mod Converter {
    const KM_TO_MILES_CONSTANT: f64 = 0.621371;

    pub fn celsius_to_fahrenheit(value: f64) -> f64 {
        (value * 9.0 / 5.0) + 32.0
    }
    
    pub fn fahrenheit_to_celsius(value: f64) -> f64 {
        (value - 32.0) * 5.0 / 9.0
    }

    pub fn km_to_milles(value: f64) -> f64 {
        value * KM_TO_MILES_CONSTANT
    }
    
    pub fn milles_to_km(value: f64) -> f64 {
        value / KM_TO_MILES_CONSTANT
    }
}