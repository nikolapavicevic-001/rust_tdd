use crate::filter::FilterMethods;

struct ThermalReader {
    filter: Box<dyn FilterMethods>,
}

impl ThermalReader {
    pub fn new(filter: Box<dyn FilterMethods>) -> Self {
        Self { filter }
    }
    pub fn read_filtered_temperature(&self) -> u32 {
        self.filter.filter_data()
    }
}

#[cfg(test)]
mod test {
    use rstest::{fixture, rstest};

    use crate::filter::MockFilterMethods;

    use super::*;

    // mock! {
    //     pub Filter{ }
    //     impl FilterMethods for Filter{
    //         fn filter_data(&self) -> u32;
    //     }
    // }

    #[fixture]
    fn thermal_reader() -> ThermalReader {
        let mut mock_filter = MockFilterMethods::new();
        mock_filter.expect_filter_data().return_const(40 as u32);

        ThermalReader::new(Box::new(mock_filter))
    }

    #[rstest]
    fn test_thermal_reader(thermal_reader: ThermalReader) {
        assert_eq!(thermal_reader.read_filtered_temperature(), 40);
    }
}
