pub struct Reducer {
    state: u8,
}

impl Reducer {
    fn new() -> Reducer {
        Reducer { state: 0 }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn sanity_palceholder_test_reducer() {
        let r = Reducer::new();
        assert_eq!(r.state, 0);
    }
}
