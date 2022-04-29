pub fn modpow<T>(mut n: T, mut p: T, m: T) -> T
where
    T: num_traits::NumOps
        + num_traits::NumAssignOps
        + std::ops::ShrAssign
        + std::cmp::PartialEq
        + PartialOrd
        + num::Zero
        + num::One
        + std::ops::BitAnd<Output = T>
        + Copy,
{
    if p == num::zero() {
        return num::one();
    }

    let mut ans = num::one();
    while p > num::zero() {
        if p & num::one() == num::one() {
            ans *= n;
            ans %= m;
        }
        n *= n;
        n %= m;
        p >>= num::one();
    }
    ans
}
