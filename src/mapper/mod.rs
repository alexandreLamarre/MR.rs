/// Mapper processes all input and yields the output of processed data
/// to the Reducer
pub struct Mapper {
    pub state: u8,
}

impl Mapper {
    pub fn new() -> Mapper {
        Mapper { state: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanity_palceholder_test_mapper() {
        let m: Mapper = Mapper::new();
        assert_eq!(m.state, 0);
    }
}
