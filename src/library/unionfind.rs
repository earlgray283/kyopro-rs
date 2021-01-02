use std::mem::swap;
#[derive(Debug)]
pub struct UnionFind {
    data: Vec<usize>,
    rank: Vec<usize>,
}

// https://pyteyon.hatenablog.com/entry/2019/03/11/200000
impl UnionFind {
    /// UnionFind を返す  
    /// data は i で初期化され、rank は 1 で初期化される。 
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            data: (0..n).map(|i| i).collect::<Vec<usize>>(),
            rank: (0..n).map(|_| 1).collect::<Vec<usize>>(),
        }
    }

    /// x の根を返す
    pub fn root(&mut self, x: usize) -> usize {
        if self.data[x] == x {
            return x;
        }
        self.data[x] = self.root(self.data[x]);
        self.data[x]
    }

    /// x と y を結合する
    pub fn unite(&mut self, x: usize, y: usize) {
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

    /// x と y が同じ根に属しているかをチェックする
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// x が属している木のサイズを返す
    pub fn rank(&mut self, x: usize) -> usize {
        let root_x = self.root(x);
        self.rank[root_x]
    }
}