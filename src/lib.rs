mod combiner;
mod mapper;
mod reducer;
/// Client Side control of distributed Map Reduce
struct Client {
    nodes: Vec<Node>,
}

/// Specifies Node type the client/ other nodes is communicating with
enum Node {
    Local,
    Remote,
    Nfs,
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder_test() {
        assert_eq!(2 + 2, 4);
    }
}
