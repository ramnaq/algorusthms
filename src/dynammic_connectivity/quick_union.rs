use crate::dynammic_connectivity::UnionFind;


/**
 * Quick union algorithm (lazy approach)
 */
#[derive( Clone)]
struct QuickUnion {
    pub vec: Vec<i32>,
}

impl QuickUnion {
    fn root(&self, x: i32) -> i32 {
        let x_parent = self.vec[x as usize];
        if x != x_parent {
            self.root(x_parent)
        } else {
            x
        }
    }
}

impl UnionFind for QuickUnion {

    // New p's root is q's root.
    fn union(&mut self, p: i32, q: i32) {
        let p_root = self.root(p);
        let q_root = self.root(q);
        self.vec[p_root as usize] = q_root;
    }

    // Check if p and q have the same root.
    fn connected(&self, p: i32, q: i32) -> bool {
        self.root(p) == self.root(q)
    }

}


pub fn test() {
    println!("### Test QuickUnion ###");

    let mut quick_union = QuickUnion {
        vec: (0..10).collect::<Vec<i32>>()
    };

    quick_union.union(4, 3);
    quick_union.union(3, 8);
    assert_eq!(vec![0,1,2,8,3,5,6,7,8,9], quick_union.vec);

    quick_union.union(6, 5);
    quick_union.union(9, 4);
    assert_eq!(vec![0,1,2,8,3,5,5,7,8,8], quick_union.vec);

    quick_union.union(2, 1);

    assert!(quick_union.connected(8,9));
    assert!(!quick_union.connected(5,4));

    quick_union.union(5, 0);
    quick_union.union(7, 2);
    quick_union.union(6, 1);
    assert_eq!(vec![1,1,1,8,3,0,5,1,8,8], quick_union.vec);
}
