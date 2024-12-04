fn celsius_to_kelvin(celsius : f32) -> f32 {
    celsius + 273.15
}

#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    fn test_celsius_to_kelvin(){
        assert_eq!(celsius_to_kelvin(0.0), 273.15);
        assert_eq!(celsius_to_kelvin(3.0), 276.15);
        assert_eq!(celsius_to_kelvin(-30.0), 243.15);
    }
}
