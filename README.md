# イテレータ関連 iter()
## all
要素全てが条件を満たしているかどうかを判定する
```rust
let v = vec![3, 1, 4, 1, 5];
assert!(v.iter().all(|&x| x > 0));
assert!(!v.iter().all(|&x| x < 4));
```

## sum
要素の総和をもとめる
```rust
let v = vec![3, 1, 4, 1, 5];
assert_eq!(14, v.iter().sum::<i32>());
```

## max, min
要素の中の最大値、最小値を求める
```rust
let v = vec![3, 1, 4, 1, 5];
assert_eq!(&5, v.iter().max().unwrap()); // v.len() == 0 のときに panic になるから注意
assert_eq!(&5, v.iter().min().unwrap()); // v.len() == 0 のときに panic になるから注意
```

## Option のメソッド
### 1. unwrap
　`Ok` だったら中の値を返し、`None` だったら `panic` を起こす。
```rust
let opt = Some(2);
assert_eq!(2, opt.unwrap());
let opt = None::<i32>;
assert_eq!(2, opt.unwrap()); // panic
```

### 2. unwrap_or
 `Ok` だったら中の値を返し、`None` だったらデフォルト値を返す。
```rust
let opt = Some(2);
assert_eq!(2, opt.unwrap_or(3));
let opt = None::<i32>;
assert_eq!(3, opt.unwrap_or(3));
```

### 3. unwrap_or_else
 `Ok` だったら中の値を返し、`None` だったらクロージャ式を実行する。
```rust
let opt = Some(2);
assert_eq!(2, opt.unwrap_or_else());
let opt = None::<i32>;
assert_eq!(2, opt.unwrap_or_else(|| &2));
```



# 文字列関連
## 添え字参照
```rust
let s = String::from("makabe")l
assert_eq!('m', s[0]); // コンパイルエラー！

let s: Vec<char> = s.chars().collect();
assert_eq!('m', s[0]); // ok
```

rust は UTF-8 でエンコードしているため、`String` 型に対してそのまま添え字アクセスすることはできない。  
`chars()` で1文字を返すイテレータを生成し、`collect()` で `Vec<char>` に変換する必要がある。