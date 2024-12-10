use std::usize;

use mockall::automock;

pub struct Filter<const N: usize> {
    buffer: [u32; N],
}

impl<const N: usize> Filter<N> {
    pub fn new() -> Self {
        Filter { buffer: [0; N] }
    }
    pub fn filter_data(&self) -> u32 {
        let sum: u32 = self.buffer.iter().map(|&x| x).sum();
        sum / self.buffer.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::config::BUFFER_SIZE;
    use rstest::{fixture, rstest};
    use super::*;

    #[fixture]
    fn filter() -> Filter<BUFFER_SIZE> {
        let filter = Filter::<BUFFER_SIZE>::new();
        filter
    }

    fn set_buffer_value(filter: &mut Filter<BUFFER_SIZE>, value: u32) {
        filter.buffer = filter.buffer.map(|_x| value);
    }

    #[rstest]
    fn test_empty_buffer(filter: Filter<BUFFER_SIZE>) {
        for i in filter.buffer {
            assert_eq!(i, 0)
        }
    }

    #[rstest]
    fn test_filter_data(mut filter: Filter<BUFFER_SIZE>) {
        set_buffer_value(&mut filter, 1);
        assert_eq!(1, filter.filter_data())
    }
}
