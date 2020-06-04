mod prime {
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
                    i+=2;
                }
                true
            }
        }
    }
    pub fn prime_all(n: usize) -> Vec<usize> {
        let mut list = Vec::new();
        for i in 2..n + 1 {
            if is_prime(i) {
                list.push(i);
            }
        }
        list
    }
    
    // 
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
                n/=x;
                let index = list.len() - 1;
                list[index].1 += 1;
            }
            if is_prime(n) {
                list.push((n,1));
                break;
            }
        }
        list
    }
}