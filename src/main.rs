struct Read<R: std::io::BufRead> {
    stdin: R,
    v: Vec<String>,
    i: usize,
}
impl<R:std::io::BufRead> Read<R> {
    pub fn new(stdin: R) -> Self {
        Read{
            stdin,
            v: Vec::new(),
            i: 0,
        }
    }
    pub fn read_until<T: std::str::FromStr>(&mut self) -> T {
        if self.i == self.v.len() {
            self.read_line();
        }
        let elem = &self.v[self.i];
        self.i += 1;
        elem.parse().unwrap_or_else(|_| panic!())
    }
    #[allow(dead_code)]
    pub fn read_i32(&mut self) -> i32 {
        if self.i == self.v.len() {
            self.read_line();
        }
        let elem = self.v[self.i].parse().unwrap();
        self.i += 1;
        elem
    }
    #[allow(dead_code)]
    pub fn read_string(&mut self) -> String {
        if self.i == self.v.len() {
            self.read_line();
        }
        let elem = &self.v[self.i];
        self.i += 1;
        elem.to_owned()
    }
    pub fn read_line(&mut self) {
        self.i = 0; self.v.clear();
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
}

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut read = Read::new(stdin);

    let a:i32 = read.read_until();
    let b:i32 = read.read_until();
    let x = a * b;
    println!("{}",x);

    
}