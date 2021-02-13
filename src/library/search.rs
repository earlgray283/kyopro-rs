/// ## verify
/// + https://atcoder.jp/contests/atc001/tasks/dfs_a
struct Grid {
    h: usize,
    w: usize,
    m: Vec<Vec<char>>,
    memo: Vec<Vec<i32>>,
}

impl Grid {
    #[allow(dead_code)]
    fn bfs(
        &mut self,
        start: (usize, usize),
        f: fn(&Vec<Vec<char>>, (usize, usize)) -> bool,
    ) {
        use std::collections::VecDeque;
        #[allow(non_upper_case_globals)]
        const dxy: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

        let mut queue = VecDeque::new();
        queue.push_back(start);
        self.memo[start.1][start.0] = 0;

        while !queue.is_empty() {
            let now = queue.pop_front().unwrap();

            for i in 0..4 {
                let next = (now.0 as i32 + dxy[i].0, now.1 as i32 + dxy[i].1);
                if next.0.is_negative()
                    || next.1.is_negative()
                    || next.0 >= self.w as i32
                    || next.1 >= self.h as i32
                {
                    continue;
                }

                let next = (next.0 as usize, next.1 as usize);
                if self.memo[next.1][next.0] != -1 {
                    continue;
                }

                if !f(&self.m, next) {
                    continue;
                }

                self.memo[next.1][next.0] = self.memo[now.1][now.0] + 1;

                queue.push_back(next);
            }
        }
    }

    #[allow(dead_code)]
    fn dfs(
        &mut self,
        now: (usize, usize),
        cost: i32,
        f: fn(&Vec<Vec<char>>, (usize, usize)) -> bool,
    ) {
        #[allow(non_upper_case_globals)]
        const dxy: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

        self.memo[now.1][now.0] = cost + 1;

        for i in 0..4 {
            let next = (now.0 as i32 + dxy[i].0, now.1 as i32 + dxy[i].1);
            if next.0.is_negative()
                || next.1.is_negative()
                || next.0 >= self.w as i32
                || next.1 >= self.h as i32
            {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);
            if self.memo[next.1][next.0] != -1 {
                continue;
            }

            if !f(&self.m, next) {
                continue;
            }

            self.dfs(next, cost + 1, f);
        }
    }
}