/// return x^n % m
fn modpow(mut x: isize, mut n: isize, m: isize) -> isize {
    let mut ans = 1;
    while n > 0 {
        if n & 1 != 0 {
            ans *= x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    ans
}

/// return nCr % m
fn modncr(n: isize, r: isize, m: isize) -> isize {
    let mut x = 1;
    let mut y = 1;

    for i in 0..r {
        x *= n - i;
        x %= m;
        y *= r - i;
        y %= m;
    }

    let ans = modpow(y, m - 2, m) % m;
    let ans = (ans * x) % m;
    ans
}