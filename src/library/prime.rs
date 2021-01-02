use std::collections::HashMap;
pub struct Prime {
    #[allow(dead_code)]
    primes: Vec<usize>,
    prime_map: HashMap<usize, bool>,
    spf: HashMap<usize, usize>,
}

impl Prime {
    /// Prime 構造体を返す  
    /// spf も求めてくれる親切設計  
    /// 計算量は O(NloglogN)
    pub fn new(n: usize) -> Prime {
        let mut primes = vec![2];
        let mut prime_map = HashMap::new();
        let mut spf = HashMap::new();
        prime_map.insert(1, false);
        prime_map.insert(2, true);
        spf.insert(1, 1);
        spf.insert(2, 2);

        for p in 3..=n {
            if p % 2 == 0 {
                spf.insert(p, 2);
                prime_map.insert(p, false);
                continue;
            }
            if prime_map.get(&p).is_some() {
                continue;
            }

            spf.insert(p, p);
            primes.push(p);

            prime_map.insert(p, true);
            let mut i = p + p;
            while i <= n {
                if prime_map.get(&i).is_some() {
                    i += p;
                    continue;
                }
                spf.insert(i, p);
                prime_map.insert(i, false);
                i += p;
            }
        }

        Prime {
            primes,
            prime_map,
            spf,
        }
    }

    /// N が素数か判定する  
    /// 計算量は O(1)
    /// ## Example
    /// ```rust
    /// assert!(prm.is_prime(1000000007));
    /// ```
    pub fn is_prime(&self, n: usize) -> bool {
        self.prime_map[&n]
    }

    /// N を素因数分解する  
    /// 計算量は O(logN)
    /// ## Example
    /// ```rust
    /// let facts = prm.factor(6);
    /// assert_eq!(vec![(2, 1), (3, 1), (6, 1)], facts);
    /// ```
    pub fn factor(&self, mut n: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        let mut p = self.spf[&n];

        if n == 1 {
            return vec![(0, 0)];
        }

        while n > 1 {
            let mut cnt = 0usize;
            while n % p == 0 {
                cnt += 1;
                n /= p;
            }
            if cnt != 0 {
                v.push((p, cnt))
            }
            p = self.spf[&n];
        }

        v
    }
}
