struct Filter<const N: usize>{
    buffer: [u8; N]
}

impl<const N: usize> Filter<N>{
    pub fn new() -> Self {
        Filter{
            buffer: [0; N]
        }
    }

    pub fn filter_data(&self) -> u8 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn filter() -> Filter<10>{
        let filter = Filter::<10>::new();
        filter
    }

    #[rstest]
    fn test_empty_buffer(filter: Filter<10>){
        for i in filter.buffer{
            assert_eq!(i, 0)
        }
    }

    #[rstest]
    fn test_filter_data(filter: Filter<10>){
        assert_eq!(1, filter.filter_data())
    }
}
