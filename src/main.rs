struct Read<R: std::io::BufRead> {
    stdin: R,
    v: Vec<String>,
    i: usize,
}
impl<R: std::io::BufRead> Read<R> {
    pub fn new(stdin: R) -> Self {
        Read {
            stdin: stdin,
            v: Vec::new(),
            i: 0,
        }
    }
    #[allow(dead_code)]
    pub fn read_one_line(&mut self) -> String {
        self.i = 0;
        self.v.clear();
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_owned()
    }
    pub fn read_line(&mut self) {
        self.i = 0;
        self.v.clear();
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        let buf = buf.trim().to_owned();
        let mut ws = buf.split_whitespace();
        loop {
            let x = ws.next();
            if x.is_none() {
                break;
            }
            let x = x.unwrap().to_owned();
            self.v.push(x);
        }
    }
    #[allow(dead_code)]
    pub fn read_a<T: std::str::FromStr>(&mut self) -> T {
        if self.i == self.v.len() {
            self.read_line();
        }
        let elem = &self.v[self.i];
        self.i += 1;
        elem.parse().unwrap_or_else(|_| panic!())
    }
}

use std::cmp;
fn main() {
    let mut stdin = std::io::stdin();
    let stdin = std::io::BufReader::new(stdin.lock());
    let mut r = Read::new(stdin);

    let mut n: usize = r.read_a();
    let mut v = vec![vec![0; n]; n - 1];
    for i in 0..n - 1 {
        for j in i + 1..n {
            v[i][j] = r.read_a();
        }
    }
    let mut ans = std::i32::MIN;
    for mut x in 0..3_i32.pow(n as u32) {
        let mut i = 0;
        let mut list = vec![0; n];
        while x > 0 {
            let t = x % 3;
            x /= 3;
            list[i] = t;
            i += 1;
        }
        let mut sum = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                //dbg!(i, j);
                if list[i] == list[j] {
                    sum += v[i][j];
                }
            }
        }
        ans = cmp::max(ans, sum);
    }

    println!("{}", ans);
}
