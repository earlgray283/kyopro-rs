pub struct UnionFind {
    parents: Vec<Option<usize>>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![None; n],
            sizes: vec![1; n],
        }
    }

    /// 頂点 x が属するグループの根を返す
    pub fn root(&mut self, x: usize) -> usize {
        if let Some(parent) = self.parents[x] {
            // 親を再帰で遡っていく
            self.parents[x] = Some(self.root(parent));
            self.parents[x].unwrap()
        } else {
            // x 自身が根だったら x を返す
            x
        }
    }

    /// 頂点 x と y が同じグループに属していたら true を返す
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        // x と y が同じグループ <=> 根が同じ
        self.root(x) == self.root(y)
    }

    /// 頂点 x が属するグループと頂点 y が属するグループを併合する  
    /// 既に同じグループの場合は false を返す
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let (mut root_x, mut root_y) = (self.root(x), self.root(y));
        if root_x == root_y {
            return false;
        }

        if self.sizes[root_x] < self.sizes[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parents[root_y] = Some(root_x); // 根 y の親を根 x にする
        self.sizes[root_y] += self.sizes[root_x];

        true
    }

    /// 頂点 x が属するグループのサイズを返す
    pub fn size(&mut self, x: usize) -> usize {
        // 頂点 x の根を求める
        let root_x = self.root(x);
        self.sizes[root_x]
    }
}
