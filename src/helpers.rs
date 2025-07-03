pub fn celsius_to_kelvin(tc: f64) -> f64 {
    tc + 273.15
}

pub fn kelvin_to_celsius(tk: f64) -> f64 {
    tk - 273.15
}

pub fn kelvin_to_fahrenheit(tk: f64) -> f64 {
    (tk - 273.15) * 9.0 / 5.0 + 32.0
}

pub fn fahrenheit_to_celsius(tf: f64) -> f64 {
    (tf - 32.0) * 5.0 / 9.0
}

pub fn fahrenheit_to_kelvin(tf: f64) -> f64 {
    (tf + 459.67) * 5.0 / 9.0
}
