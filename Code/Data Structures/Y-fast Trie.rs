use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct YFastTrie {
    root: Node,
    max_bits: usize,
}
