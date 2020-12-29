use std::mem::swap;
#[derive(Debug)]
struct UnionFind {
    data: Vec<usize>,
    rank: Vec<usize>,
}

// https://pyteyon.hatenablog.com/entry/2019/03/11/200000
impl UnionFind {
    /// return UnionFind  
    /// data was inited by i  
    /// rank was inited by 1
    fn new(n: usize) -> UnionFind {
        UnionFind {
            data: (0..n).map(|i| i).collect::<Vec<usize>>(),
            rank: (0..n).map(|_| 1).collect::<Vec<usize>>(),
        }
    }

    /// return root of x
    fn root(&mut self, x: usize) -> usize {
        if self.data[x] == x {
            return x;
        }
        self.data[x] = self.root(self.data[x]);
        self.data[x]
    }

    /// unite x and y
    fn unite(&mut self, x: usize, y: usize) {
        let mut root_x = self.root(x);
        let mut root_y = self.root(y);

        if root_x == root_y {
            return;
        }

        if self.rank[root_x] < self.rank[root_y] {
            swap(&mut root_x, &mut root_y);
        }
        self.rank[root_x] += self.rank[root_y];
        self.data[root_y] = root_x;
    }

    /// check if x and y belong to same root
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// check size of tree x belongs to
    fn rank(&mut self, x: usize) -> usize {
        let root_x = self.root(x);
        self.rank[root_x]
    }
}