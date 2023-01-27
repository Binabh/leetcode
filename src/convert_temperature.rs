pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    return vec![celsius + 273.15, celsius * 1.80 + 32.00];
}

pub fn test() {
    assert_eq!(convert_temperature(36.50), vec![309.65000, 97.70000]);
    assert_eq!(convert_temperature(122.11), vec![395.26000, 251.79800]);
}

pub const NAME: &str = "convert_temperature";
