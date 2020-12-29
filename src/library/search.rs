struct Search {
    goal: (usize, usize),
    memo: Vec<Vec<i32>>,
    h: usize,
    w: usize,
    c: Vec<Vec<char>>,
}

impl Search {
    fn new(h: usize, w: usize, c: Vec<Vec<char>>, goal: (usize, usize)) -> Search {
        Search {
            h,
            w,
            c,
            goal,
            memo: vec![vec![-1; w]; h],
        }
    }

    fn bfs(&mut self, start: (usize, usize)) {
        let dx: [i32; 4] = [1, 0, -1, 0];
        let dy: [i32; 4] = [0, -1, 0, 1];

        let mut queue = VecDeque::new();
        let mut x = 0;
        self.memo[start.0][start.1] = x;

        queue.push_back(start);
        while !queue.is_empty() {
            for i in 0..4 {

            }
        }
    }

    #[allow(dead_code)]
    fn dfs(&mut self, now: (usize, usize), x: i32) {
        let dx: [i32; 4] = [1, 0, -1, 0];
        let dy: [i32; 4] = [0, -1, 0, 1];

        self.memo[now.0][now.1] = x;

        for i in 0..4 {
            let mut next = now;
            match dy[i].is_negative() {
                true => {
                    if dy[i].abs() > 0 && next.0 == 0 {
                        continue;
                    }
                    next.0 = next.0 - dy[i].abs() as usize;
                }
                false => {
                    if next.0 + dy[i].abs() as usize >= self.h {
                        continue;
                    }
                    next.0 = next.0 + dy[i].abs() as usize;
                }
            };
            match dx[i].is_negative() {
                true => {
                    if dx[i].abs() > 0 && next.1 == 0 {
                        continue;
                    }
                    next.1 = next.1 - dx[i].abs() as usize;
                }
                false => {
                    if next.1 + dx[i].abs() as usize >= self.w {
                        continue;
                    }
                    next.1 = next.1 + dx[i].abs() as usize;
                }
            };

            if self.c[next.0][next.1] == '#' {
                continue;
            }

            if self.memo[next.0][next.1] != -1 && self.memo[next.0][next.1] < x {
                continue;
            }

            self.dfs(next, x + 1);
        }
    }

    fn checker(target: usize, x: usize) -> bool {

    }
}