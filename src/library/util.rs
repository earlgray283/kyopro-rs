/// s に番兵を加えたものを返す
/// ## Example
/// ```rust
/// let v = vec![
///     "..#",
///     ".#.",
///     "#.."
/// ];
/// let v = add_sentinel(v);
/// assert_eq!(v, vec![
///     "#####", 
///     "#..##", 
///     "#.#.#", 
///     "##..#", 
///     "#####"]
/// );
/// ```

pub fn add_sentinel(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut t = vec![vec!['.'; s[0].len() + 2]; s.len() + 2];
    
    for i in 1..t.len() - 1 {
        for j in 1..t[i].len() - 1 {
            t[i][j] = s[i - 1][j - 1];
        }
    }

    t
}