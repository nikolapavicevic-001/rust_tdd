fn celsius_to_kelvin(celsius: f32) -> f32 {
    celsius + 273.15
}
fn celsius_decigrades_to_celsius(decigrades: f32) -> f32 {
    0.0
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_celsius_to_kelvin() {
        assert_eq!(celsius_to_kelvin(0.0), 273.15);
        assert_eq!(celsius_to_kelvin(3.0), 276.15);
        assert_eq!(celsius_to_kelvin(-30.0), 243.15);
    }
    #[test]
    fn test_celsius_decigrades_to_celsius() {
        assert_eq!(celsius_decigrades_to_celsius(100.0), 10.0);
        assert_eq!(celsius_decigrades_to_celsius(30.0), 3.0);
        assert_eq!(celsius_decigrades_to_celsius(-10.0), -1.0);
    }
}
