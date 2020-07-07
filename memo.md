# イテレータ関連
## 要素全てが条件を満たしているかどうかを判定する
```rust
let v = vec![3, 1, 4, 1, 5];
assert!(v.iter().all(|&x| x > 0));
assert!(!v.iter().all(|&x| x < 4));
```

## 要素の総和をもとめる
```rust
let v = vec![3, 1, 4, 1, 5];
assert_eq!(14, v.iter().fold(0, |sum, x|, sum + x));
```

# 比較
```rust
// 大きいほう
assert_eq!(std::cmp::max(3, 1), 3);
// 小さいほう
assert_eq!(std::cmp::min(3, 1), 1);
```

# 文字列関連
## 添え字参照
```rust
let s = String::from("makabe")l
assert_eq!('m', s[0]); // ng

let s: Vec<char> = s.chars().collect();
assert_eq!('m', s[0]); // ok
```

rust は UTF-8 でエンコードしているため、```String``` 型に対してそのまま添え字アクセスすることはできない。  
```chars()``` で1文字を返すイテレータを生成し、```collect()``` で ```Vec<char>``` に変換する必要がある。