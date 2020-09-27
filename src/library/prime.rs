mod prime {
    #[allow(dead_code)]
    pub fn is_prime(n: usize) -> bool {
        match n {
            0 | 1 => false,
            2 => true,
            _ => {
                if n % 2 == 0 {
                    return false;
                }
                let mut i = 3;
                while i <= (n as f64).sqrt() as usize + 1 {
                    if n % i == 0 {
                        return false;
                    }
                    i += 2;
                }
                true
            }
        }
    }

    /// # 素数列挙(prime_all)
    /// This enumerates prime numbers which are above 2 below N.
    /// ## Example
    /// ```rust
    /// let primes = prime::prime_all(24);
    /// assert_eq!(primes, {2, 3, 5, 7, 11, 13, 17, 19, 23});
    /// ```
    #[allow(dead_code)]
    pub fn prime_all(n: usize) -> Vec<usize> {
        let mut list = Vec::new();
        for i in 2..n + 1 {
            if is_prime(i) {
                list.push(i);
            }
        }
        list
    }

    /// # 素因数分解(factorize)
    /// This factorizes N and returns tuple(usize, usize).
    /// ## Example
    /// ```rust
    /// let facts = prime::factorize(24);
    /// assert_eq!(facts[0], (2, 3));
    /// assert_eq!(facts[1], (3, 1));    
    /// ```
    #[allow(dead_code)]
    pub fn factorize(mut n: usize) -> Vec<(usize, usize)> {
        let mut list = Vec::new();
        if is_prime(n) {
            list.push((n, 1));
            return list;
        }
        let primes = prime_all((n as f64).sqrt() as usize + 1);
        for x in &primes {
            let x = *x;
            if n % x != 0 {
                continue;
            }
            list.push((x, 0_usize));
            while n % x == 0 {
                n /= x;
                let index = list.len() - 1;
                list[index].1 += 1;
            }
            if is_prime(n) {
                list.push((n, 1));
                break;
            }
        }
        list
    }
}
