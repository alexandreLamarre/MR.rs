/// Object to Combine downstream data from reducers
pub struct Combiner {
    state: u8,
}

impl Combiner {
    pub fn new() -> Combiner {
        Combiner { state: 0 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanity_palceholder_test_combiner() {
        let c = Combiner { state: 0 };
        assert_eq!(c.state, 0);
    }
}
