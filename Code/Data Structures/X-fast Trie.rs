use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct XFastTrie {
    root: Node,
    max_bits: usize,
}
