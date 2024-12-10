fn celsius_to_kelvin(celsius: f32) -> f32 {
    celsius + 273.15
}
fn celsius_decigrades_to_celsius(decigrades: f32) -> f32 {
    decigrades / 10.0
}
fn kelvin_to_celisus(kelvin: f32) -> f32 {
    kelvin - 273.15
}
#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(0.0, 273.15)]
    #[case(3.0, 276.15)]
    #[case(-30.0, 243.15)]
    fn test_celsius_to_kelvin(#[case] input: f32, #[case] expected: f32) {
        assert_eq!(celsius_to_kelvin(input), expected);
    }
    #[rstest]
    #[case(100.0, 10.0)]
    #[case(30.0, 3.0)]
    #[case(-10.0, -1.0)]
    fn test_celsius_decigrades_to_celsius(#[case] input: f32, #[case] expected: f32) {
        assert_eq!(celsius_decigrades_to_celsius(input), expected);
    }
    #[rstest]
    #[case(273.15, 0.0)]
    #[case(276.15, 3.0)]
    #[case(243.15, -30.0)]
    fn test_kelvin_to_celisus(#[case] input: f32, #[case] expected: f32){
        assert_eq!(kelvin_to_celisus(input), expected);
    }
}
