#![allow(unused_imports, dead_code, unused_variables, unused_mut)]
use std::cmp;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem::swap;
use permutohedron::LexicalPermutation;
const INF: i32 = std::i32::MAX;
const MOD: i32 = 1e9 as i32 + 7;

struct Get<R: std::io::BufRead> {
    stdin: R,
    v: Vec<String>,
    i: usize,
}
impl<R: std::io::BufRead> Get<R> {
    pub fn new(stdin: R) -> Self {
        Get {
            stdin,
            v: Vec::new(),
            i: 0,
        }
    }
    fn split_line(&mut self) {
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
    pub fn once<T: std::str::FromStr>(&mut self) -> T {
        if self.i == self.v.len() {
            self.split_line();
        }
        let elem = &self.v[self.i];
        self.i += 1;
        elem.parse().unwrap_or_else(|_| panic!())
    }
    #[allow(dead_code)]
    pub fn line<T: std::str::FromStr>(&mut self) -> String {
        self.i = 0;
        self.v.clear();
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        let buf = buf.trim().to_owned();
        buf
    }
}

fn main() {
    let stdin = std::io::stdin();
    let stdin = std::io::BufReader::new(stdin.lock());
    let get = Get::new(stdin);
    solve(get);
}

fn solve<R: std::io::BufRead>(mut get: Get<R>) {
    
}
