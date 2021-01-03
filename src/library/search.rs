struct Search<T> {
    memo: Vec<Vec<i32>>,
    h: usize,
    w: usize,
    c: Vec<Vec<T>>,
}

impl<T> Search<T> {
    fn new(h: usize, w: usize, c: Vec<Vec<T>>) -> Search<T> {
        Search {
            h,
            w,
            c,
            memo: vec![vec![-1; w]; h],
        }
    }

    fn init(&mut self) {
        self.memo = vec![vec![-1; self.w]; self.h];
    }

    fn bfs1(&mut self, start: (usize, usize), f: fn(&Vec<Vec<T>>, (usize, usize)) -> bool) -> i32 {
        let dxy = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        let mut ans = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);

        self.memo[start.1][start.0] = 0;

        while !queue.is_empty() {
            let now = queue.pop_front().unwrap();
            for i in 0..4 {
                let res = now.add(dxy[i]);
                if res.is_none() {
                    continue;
                }

                let next = res.unwrap();
                if next.0 >= self.w || next.1 >= self.h {
                    continue;
                }

                if !f(&self.c, next) {
                    continue;
                }

                if self.memo[next.1][next.0] != -1 {
                    continue;
                }

                queue.push_back(next);
                self.memo[next.1][next.0] = self.memo[now.1][now.0] + 1;
                ans = ans.max(self.memo[next.1][next.0]);
            }
        }

        ans
    }
}