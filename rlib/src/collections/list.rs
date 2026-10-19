
use core::ptr;

const NONE: usize = usize::MAX;

struct Node<T> {
    value: T,
    next: usize,
}

pub struct List<T, const N: usize> {
    nodes: [Option<Node<T>>; N],
    head: usize,
    len: usize,
}

impl<T, const N: usize> List<T, N> {
    pub fn new() -> Self {
        Self {
            nodes: core::array::from_fn(|_| None),
            head: NONE,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) -> bool {
        if self.len >= N {
            return false;
        }

        let index = self.nodes.iter().position(|n| n.is_none()).unwrap();

        self.nodes[index] = Some(Node {
            value,
            next: self.head,
        });

        self.head = index;
        self.len += 1;

        true
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
