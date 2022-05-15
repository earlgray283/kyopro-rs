pub struct Sieve {
    table: Vec<bool>,
}

impl Sieve {
    pub fn new(n: usize) -> Self {
        let mut table = vec![true; n + 1];
        table[0] = false;
        table[1] = false;
        for i in 2..=n {
            if !table[i] {
                continue;
            }
            let mut j = i * 2;
            while j <= n {
                table[j] = false;
                j += i;
            }
        }
        Self { table }
    }

    pub fn is_prime<T>(&self, n: T) -> bool
    where
        T: std::slice::SliceIndex<[bool], Output = bool>,
    {
        self.table[n]
    }

    pub fn prime_list<T: std::convert::From<usize>>(&self) -> Vec<T> {
        self.table
            .iter()
            .enumerate()
            .filter(|(_, &p)| p)
            .map(|(i, _)| i.into())
            .collect()
    }
}

pub fn make_prime_list(n: usize) -> Vec<usize> {
    let mut prime_list = Vec::with_capacity(n / 4);
    for i in 2..=n {
        let mut ok = true;
        for &p in &prime_list {
            if i < p * p {
                break;
            }
            if i % p == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            prime_list.push(i);
        }
    }
    prime_list
}
