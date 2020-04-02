
pub mod quick_find;
pub mod quick_union;

pub trait UnionFind {
    fn union(&mut self, p: i32, q: i32);
    fn connected(self, p: i32, q: i32) -> bool;
}
