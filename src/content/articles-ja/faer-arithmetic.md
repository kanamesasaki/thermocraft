---
title: 'Pure Rust 線形代数ライブラリfaer入門② 演算パフォーマンス'
description: 'faerにおける基本的な行列演算について、演算子を用いた記述方法から、より効率的なzip!マクロやmatmulを用いた実装方法まで解説します。'
pubDate: 2025-12-24
updatedDate: 2025-12-24
heroImage: ''
tags: ['programming', 'numerical analysis', 'mathematics']
---

前回の記事「[Pure Rust 線形代数ライブラリfaer入門①](https://thermocraft.space/ja/articles/faer-basics/)」で、faerの基本的なデータ型であるMat（所有）とMatRef（ビュー）の内部構造について解説した。
これらは行列データを効率的かつ安全に管理するための仕組みだったが、今回はそれらを使って行う基礎的な演算に焦点を当てる。
とくにfaerが提供する行列演算APIを、「書きやすさ」と「性能」の観点から比較し、用途に応じた使い分けを紹介する。

今回の記事でも、2025年12月時点での最新バージョンである0.23.2を使用する。

```toml
[dependencies]
faer = "0.23.2"
```

## 演算子による行列演算

faerでは、標準的な算術演算子（`+`, `-`, `*`）が実装されており、直感的に行列計算を書くことができる。
ただし`*`は要素ごとの積ではなく、行列積を表すことに注意しよう。

```rust
use faer::{Mat, MatRef, mat};

fn main() {
    let a = mat![[1.0, 2.0], [3.0, 4.0]];
    let b = mat![[5.0, 6.0], [7.0, 8.0]];

    // 1. 行列の和
    let c: Mat<f64> = &a + &b;
    println!("{:?}", c);
    // [
    // [6.0, 8.0],
    // [10.0, 12.0],
    // ]

    // 2. 行列の差
    let d: Mat<f64> = &a - &b;
    println!("{:?}", d);
    // [
    // [-4.0, -4.0],
    // [-4.0, -4.0],
    // ]

    // 3. 行列の積
    let e: Mat<f64> = &a * &b;
    println!("{:?}", e);
    // [
    // [19.0, 22.0],
    // [43.0, 50.0],
    // ]
}
```

## より効率的な行列演算

これらの書き方は直感的で便利だが、行列演算が行われるたびに結果が新しい行列として確保されるため、複雑な行列の演算を行う場合には非効率になることがある。
このセクションでは、演算子を用いた記述方法と、より効率的なzip!マクロやmatmulを用いた実装方法を比較する。

### Case 1: 3つの行列の足し算 (D = A + B + C)

足し算、引き算、スカラー倍などの、要素ごとの演算については、faerは`zip`関数を提供している。
これを用いると、中間結果を保持せずに複数の行列を一度に演算できる。

```rust
use faer::{zip, unzip, Mat};
use std::time::Instant;

fn main() {
    // 1024x1024 の f64 行列は約8MB
    let n: usize = 4096;
    let a: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i + j) as f64);
    let b: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i as isize - j as isize) as f64);
    let c: Mat<f64> = Mat::<f64>::full(n, n, 1.0);

    // ==========================================
    // Case 1: 3つの行列の足し算 (D = A + B + C)
    // ==========================================
    println!("--- Case 1: D = A + B + C ---");

    // [Bad Pattern] 演算子
    let start = Instant::now();
    let d_op = &a + &b + &c;
    println!("Operators: {:.2?}", start.elapsed());

    // [Good Pattern] zip!
    let start = Instant::now();
    let mut d_zip = Mat::zeros(n, n);
    zip!(&mut d_zip, &a, &b, &c).for_each(|unzip!(d, a, b, c)| {
        *d = *a + *b + *c;
    });
    println!("zip!: {:.2?}", start.elapsed());

    // --- Case 1: D = A + B + C ---
    // Operators : 3.01ms
    // zip!      : 1.94ms
}
```

`&a + &b + &c`の計算では中間結果が生成されるのに対し、`zip!`を用いた場合は1回の結果行列の確保のみで済むため、メモリ使用量と処理時間が削減されている。
このテストケースでは、`zip!`を用いることで約1.5倍の高速化が達成された。

### Case 2: 積和演算 (E = A + B \* C)

行列積を含む演算の場合、行列積の演算ルール上、なんらかの形で中間結果を保持することは避けられない。
しかし、この場合でも書き方によって、計算速度に大きな差が出ることがある。

```rust
use faer::linalg::matmul::matmul;
use faer::{zip, unzip, Mat};
use std::time::Instant;

fn main() {
    println!("--- Case 2: E = A + B * C ---");
    let n: usize = 4096;
    let a: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i + j) as f64);
    let b: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i as isize - j as isize) as f64);
    let c: Mat<f64> = Mat::<f64>::full(n, n, 1.0);

    // ==========================================
    // 1. Operators
    // ==========================================
    let start = Instant::now();
    // Process:
    // 1. tmp = B * C (Alloc + matmul Replace)
    // 2. res = A + tmp (Alloc + Zip Add)
    let e1 = &a + &b * &c;
    println!("operators                 : {:.2?}", start.elapsed());

    // ==========================================
    // 2. clone + Add
    // ==========================================
    let start = Instant::now();
    // Process:
    // 1. res = A.clone() (Alloc + Copy)
    // 2. res += B * C (matmul Add)
    let mut e2 = a.clone();
    matmul(
        e2.as_mut(),
        Accum::Add,
        &b,
        &c,
        1.0,
        get_global_parallelism()
    );
    println!("copy + matmul Add         : {:.2?}", start.elapsed());

    // ==========================================
    // 3. Replace + Zip
    // ==========================================
    let start = Instant::now();
    // Process:
    // 1. res (Alloc)
    // 2. res = B * C (matmul Replace)
    // 3. res += A (Zip Add)
    let mut e3 = Mat::<f64>::zeros(n, n);
    matmul(
        e3.as_mut(),
        Accum::Replace,
        &b,
        &c,
        1.0,
        get_global_parallelism()
    );
    zip!(&mut e3, &a).for_each(|unzip!(e, a)| *e += *a); // equivalent to e3 += &a;
    println!("matmul Replace + zip Add  : {:.2?}", start.elapsed());

    // --- Case 2: E = A + B * C ---
    // 1. operators                 : 13.62ms
    // 2. copy + matmul Add         : 9.50ms
    // 3. matmul Replace + zip Add  : 9.51ms

}
```

この例では、3つの方法で`E = A + B * C`を計算している。

1. 演算子を使った方法では、`B * C`の結果が中間行列として確保され、その後`A`との和が計算されるため、2回の行列確保が発生している。
2. `A`のクローンを結果行列として作成し、その上に`B * C`の結果を加算している。
3. 最初にゼロで初期化した結果行列を確保し、そこに`B * C`の結果を上書きし、その後`A`を加算している。

実行時間を比較すると、2番目と3番目の方法がほぼ同じ速度であり、1番目の方法よりも高速であることがわかる。
このテストコードでは、メモリ確保のコストが大きく、中間行列の生成を避けることでパフォーマンスが向上していると考えられる。
ただし、2番目と3番目の方法で実行時間がほぼ同じになるのは、`cargo run --release`で最適化された場合の結果であり、Debugビルドでは挙動が異なるので注意が必要である。

### Case 3: 複数回の行列積 (F = A \* B \* C \* D \* E)

複数回の行列積を行う場合、各積ごとに中間結果が生成されるため、メモリ使用量が増加し、パフォーマンスが低下する可能性がある。
例えば`A * B * C * D * E`という、5つの行列の積について考えてみよう。

```rust
use faer::linalg::matmul::matmul;
use faer::{mat, zip, unzip, Accum, Mat, Scale, get_global_parallelism};
use std::time::Instant;
use std::hint::black_box;

fn main() {
    println!("--- Chain: A * B * C * D * E ---");
    let n = 1024;
    let loop_count = 50;

    let a = Mat::<f64>::from_fn(n, n, |i, j| (i + j) as f64);
    let b = Mat::<f64>::from_fn(n, n, |i, j| (i as isize - j as isize) as f64);
    let c = Mat::<f64>::full(n, n, 1.0);
    let d = Mat::<f64>::from_fn(n, n, |i, j| (j as isize - i as isize) as f64);
    let e = Mat::<f64>::from_fn(n, n, |i, j| (i * j) as f64);

    // ==========================================
    // 1. operators
    // ==========================================
    let start = Instant::now();
    for _ in 0..loop_count {
        let res = &a * &b * &c * &d * &e;
        black_box(res);
    }
    println!("operators (4 allocs) : {:.2?}", start.elapsed() / loop_count);

    // ==========================================
    // 2. ping-pong buffering
    // ==========================================
    let par = get_global_parallelism();
    let start = Instant::now();

    for _ in 0..loop_count {
        let mut buf1 = Mat::<f64>::zeros(n, n);
        let mut buf2 = Mat::<f64>::zeros(n, n);

        matmul(buf1.as_mut(), Accum::Replace, &a, &b, 1.0, par);
        matmul(buf2.as_mut(), Accum::Replace, &buf1, &c, 1.0, par);
        matmul(buf1.as_mut(), Accum::Replace, &buf2, &d, 1.0, par);
        matmul(buf2.as_mut(), Accum::Replace, &buf1, &e, 1.0, par);
        black_box(buf2);
    }
    println!("ping-pong buffering (2 allocs): {:.2?}", start.elapsed() / loop_count);

    // --- Chain: A * B * C * D * E ---
    // 1. operators (4 allocs)  : 34.40ms
    // 2. ping-pong (2 allocs)  : 32.57ms
}
```

この例では、2つの方法で5つの行列の積を計算している。

1. 演算子を使った方法では、各積ごとに中間行列が生成されるため、4回の行列確保が発生している。
2. Ping-Pong Bufferingと呼ばれる方法では、2つのバッファ領域を交互に使用して中間結果を保存することで、行列確保の回数を2回に抑えている。

テストコード内で50回ループを回して平均をとっているのは、あくまで計測の安定化を図るためであり、`A * B * C * D * E`の速度計測を行うことが目的である。
なので、Ping-Pong Bufferingの場合も、ループ内で2回バッファを確保している。
実行時間を比較すると、Ping-Pong Bufferingの方がわずかに高速であることがわかる。
メモリ確保のコストは、行列積のコストと比較して小さいため、劇的な改善にはならないが、パフォーマンスにシビアな計算を行う場合には考慮する価値がある。

## まとめ

今回はRustの線形代数ライブラリfaerの基本的な行列演算について解説した。
行列の和、差、積などの演算1回のプロセスはfaerが実装する演算子で簡単に記述でき、高速に実装できるが、複数回の演算を組み合わせる場合には、中間結果の生成を避けるために`zip!`や`matmul`関数を用いた明示的な実装が必要になる場合がある。
次回は、faerが提供するより高度な線形代数機能について解説していきたい。
