use crate::dynammic_connectivity::UnionFind;

/**
 * Quick find algorithm (eager approach): numbers p, q are in the same connected
 * component if the positions [p] and [q] in the array have the same identifier
 * (an integer).
 *
 * Example:
 *      ids = [0, 1, 1, 8, 8, 0] -> connected_components = {{0, 5}, {1, 2}, {3, 4}}
 *
 * Complexity:
 *  Initialization - O(n)
 *  Union - O(n)
 *  Find - O(1)
 *
 *  For n numbers, n unions takes n² memory accesses. That's huge! This complexity
 *  it not acceptable for big problems, because it doesn't scale. For instance,
 *  let's say we have 10^9 numbers and do 10^9 unions, thats 10^18 memory accesses.
 *  If tomorrow we get a computer 10x faster, we expect solving 10x bigger problems,
 *  but quadratic algorithms don't take this advantage:
 *      10^18 / 10 = 10^17  (Same problem, 10x faster computer)
 *      10 * 10^17 = 10^18  (10x that problem, with the new computer)
 */
struct QuickFind<T> {
    pub vec: Vec<T>,
}

impl UnionFind for QuickFind::<i32> {
    fn union(&mut self, p: i32, q: i32) {
        let p_id = self.vec[p as usize];
        let q_id = self.vec[q as usize];

        if p_id == q_id {
            return;
        }

        // p and all its connections get updated to q's id.
        self.vec.iter_mut().for_each(|x| {
            if *x == p_id {
                *x = q_id;
            }
        });
    }

    fn connected(self, p: i32, q: i32) -> bool {
        return self.vec[p as usize] == self.vec[q as usize];
    }
}


fn quick_find() {
    let mut quick_find = QuickFind::<i32> {
        vec: (0..10).collect::<Vec<i32>>()
    };

    quick_find.union(4, 3);
    quick_find.union(3, 8);
    assert_eq!(vec![0,1,2,8,8,5,6,7,8,9], quick_find.vec);

    quick_find.union(6, 5);
    quick_find.union(9, 4);
    assert_eq!(vec![0,1,2,8,8,5,5,7,8,8], quick_find.vec);

    quick_find.union(2, 1);
    quick_find.union(8, 9);
    quick_find.union(5, 0);
    quick_find.union(7, 2);
    quick_find.union(6, 1);
    assert_eq!(vec![1,1,1,8,8,1,1,1,8,8], quick_find.vec);
}


fn main() {
    quick_find();
}
