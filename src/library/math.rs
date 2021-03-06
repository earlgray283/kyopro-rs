/// nCr を求める
pub fn ncr(n: i128, r: i128) -> i128 {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ncr(n, r - 1) * (n - r + 1) / r,
    }
}

/// x^n を m で割った余りを求める
pub fn modpow(mut x: i128, mut n: i128, m: i128) -> i128 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 != 0 {
            ans *= x % m;
            ans %= m;
        }
        x = x * x % m;
        n >>= 1;
    }

    ans
}

/// nCr を m で割った余りを求める
pub fn modncr(n: i128, r: i128, m: i128) -> i128 {
    let mut x = 1;
    let mut y = 1;

    for i in 0..r {
        x *= n - i;
        x %= m;
        y *= r - i;
        y %= m;
    }

    let ans = modpow(y, m - 2, m) % m;
    (ans * x) % m
}
