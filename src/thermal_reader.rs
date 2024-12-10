use crate::{config::BUFFER_SIZE, filter::Filter};

struct ThermalReader {
    filter: Filter<BUFFER_SIZE>,
}

impl ThermalReader{
    pub fn new(filter: Filter<BUFFER_SIZE>) -> Self{
        Self{
            filter
        }
    }
    pub fn read_filtered_temperature(&self) -> u32 {
        self.filter.filter_data()
    }
}

#[cfg(test)]
mod test{
    use rstest::fixture;

    use super::*;

    #[fixture]
    fn thermal_reader() -> ThermalReader{
    }
}
