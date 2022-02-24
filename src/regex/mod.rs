// Fast Thompson NFA Regex implementation
// Reference : [Regular Expression Matching Can Be Simple And Fast](https://swtch.com/~rsc/regexp/regexp1.html)

struct Re {
    pattern: String,
}

impl Re {
    fn compile(pattern: String) -> Re {
        Re { pattern: pattern }
    }
}
/// NFA state
/// the integer int represents one of the following three NFA fragments
/// depending on the value of c
/// - c < 256, i.e. matches a single character
///     c -----> out
/// - c = 256, split between choices
///          |---> out
///      c --|
///          |---> out1
/// c = 257, completed a match
///     c ------> match!
///
struct State {
    c: u16,
    out: Box<State>,
    out1: Box<State>,
    lastlist: u32,
}

struct Frag {
    start: State,
    out: Vec<State>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_regex() {
        Re::compile("a*".to_string());
    }
}
