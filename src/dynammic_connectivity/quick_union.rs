use crate::dynammic_connectivity::UnionFind;

struct QuickUnion<T> {
    pub vec: Vec<T>,
}

impl UnionFind for QuickUnion::<i32> {
    fn union(&mut self, p: i32, q: i32) {
    }

    fn connected(self, p: i32, q: i32) -> bool {
        false
    }
}
