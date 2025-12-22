---
title: 'Xorshiftによる擬似乱数生成'
description: 'Xorshiftはシンプルで高速な擬似乱数生成器（RNG）アルゴリズムです。本記事では、Xorshiftの基本的なコンセプトについて解説し、有効なXorshiftパラメータを見つけるためのRustコードを実装します。'
pubDate: 2025-12-13
updatedDate: 2025-12-23
heroImage: ''
tags: ['programming', 'numerical analysis']
---

<div style="background: linear-gradient(135deg, #8b0000 0%, #c41e3a 100%); padding: 0.5rem; border-radius: 8px; margin: 2rem 0;">
  <p style="color: white; margin: 0; text-align: left; font-size: 1.1rem;">
    🎄 <a href="https://qiita.com/advent-calendar/2025/first-time" style="color: white; text-decoration: underline; font-weight: bold;">はじめてのアドベントカレンダー Advent Calendar 2025</a> 13日目 🎄
  </p>
</div>

擬似乱数を発生させるアルゴリズムには、線形合同法、Xorshift、メルセンヌ・ツイスタ、などいくつも有名なものがある。
その中でも操作が単純で高速なXorshiftについて、ざっくり何をやっているのかをMarsagliaによる論文[[1,2]](#reference)を参照しながら考えてみたい。
Xorshiftの実装自体は難しくないが、ここではアルゴリズムの背後にあるモチベーション的な部分を考えていこう。

## バイナリ行列による擬似乱数の生成

擬似乱数を発生させる方法には様々なアプローチがあり得るが、大きな方向性として、
n次のバイナリ$\beta$に対して、$n \times n$バイナリ行列$T$を作用させていき、$\beta T,~\beta T^2,~\cdots$という数列を作ることで、擬似乱数を発生させる方法を考えよう。

このとき、以下の主張は同値になる。

1. ゼロでない任意のn次バイナリに対して、$n \times n$バイナリ行列$T$を作用させていくと、ゼロ以外のn次バイナリを全て作ってくれる。
2. $n \times n$バイナリ行列$T$の周期は$2^n-1$。

同値性に関して、元の論文[[2]](#reference)では固有多項式を用いたりして議論しているが、感覚的には自然に納得できると思う。

1 $\Leftarrow$ 2 に関して：

$T$の周期が$k = 2^n-1$ということは、あるバイナリ$\beta$に対して$\beta T^k= \beta$で、当然それまで（$\beta T,~\beta T^2,~\cdots$）に$\beta$は踏んでいない。
なのでその間、n次バイナリのとりうる元が$2^n$個あるうち、ゼロと初期バイナリを除いたすべての元を踏んでいかないといけないことが分かる（途中で同じものを踏むと、そこでループする）。

1 $\Rightarrow$ 2 に関して：

行列$T$を作用させると、あるバイナリ$\beta$に対して、一意にバイナリ$\beta T$を与えるので、とりうるバイナリを全て発生させていくには、毎回異なるバイナリを発生させ続けなければならない。なので、$2^n-1$回の操作で元のバイナリに戻るという周期になるはずだ。

なので、周期が$2^n-1$のバイナリ行列$T$を見つけることができれば、ゼロを除く任意の初期バイナリから始めて、出現回数に偏りなく（出現の順番には偏りがあるかもしれないが）すべてのバイナリを発生させることができる。

## バイナリ行列

Xorshiftアルゴリズムの中身について議論する前に、バイナリ行列の性質を簡単に振り返っておこう。
まず最初の注意として、ここで扱うバイナリ行列とその演算は、ブール代数の定義とは異なる（Boolean行列の基礎については、Luceの論文[[3]](#reference)などを参照すると良い）。
基本的な演算（加算と乗算）は、(1)および(2)のように定義される。
ブール代数との違いは、加算がORではなくXORで定義されている点である。
今回の定義の下では、通常の実数/複素数行列の性質の多くがバイナリ行列の場合にも適用できる。

$$
\begin{align}
\left\{\begin{array}{l}
0 + 0 = 0\\
0 + 1 = 1\\
1 + 0 = 1\\
1 + 1 = 0
\end{array}\right.
\end{align}
$$

$$
\begin{align}
\left\{ \begin{array}{l}
0 \cdot 0 = 0\\
0 \cdot 1 = 0\\
1 \cdot 0 = 0\\
1 \cdot 1 = 1
\end{array} \right.
\end{align}
$$

バイナリ行列の計算の順序は、基本的に普通の行列と同じで、そこに先ほどの加算と乗算のルールを適用していく。
例えば、$3 \times 3$のバイナリ行列$A$と$B$を(3)のように定義した場合、加算と乗算の演算は次のように表される。

$$
\begin{align}
A = \begin{bmatrix}
1 & 0 & 1 \\
0 & 1 & 0 \\
0 & 0 & 1
\end{bmatrix}, \quad
B = \begin{bmatrix}
0 & 0 & 1 \\
1 & 1 & 1 \\
0 & 1 & 0
\end{bmatrix}
\end{align}
$$

$$
\begin{align}
A + B = \begin{bmatrix}
1 & 0 & 0 \\
1 & 0 & 1 \\
0 & 1 & 1
\end{bmatrix}, \quad
AB = \begin{bmatrix}
0 & 1 & 1 \\
1 & 1 & 1 \\
0 & 1 & 0
\end{bmatrix}
\end{align}
$$

## Xorshiftを使う理由

さて、周期$2^n-1$の$n \times n$バイナリ行列$T$はどうやって見つければいいだろうか。
重要な特徴として、行列$T$は正則でなければならない。これは以下のように考えることができる。

- 行列$T$は全ての$n$次バイナリを生成できる必要があるので、$n$次バイナリ全体の集合に関して全単射でなければならない。
- 行列$T$が全単射である ⇔ 行列$T$には逆行列$T^{-1}$が存在する。

ただこれだけでは行列の形は絞れないし、実際、周期の条件を満たすような行列は大量に存在する。

むしろ方針としては、どのような行列の作り方をすれば行列計算のコストがかからないか、という視点から行列の候補を出していき、それらが周期の条件を満たすかチェックしていきたい。
そこで出てくるのがXorshiftという操作だ。

ビットシフトの操作はコンピュータ内で高速に実行可能な操作で、行列ではshift matrixとして表される。
例えば$4 \times 4$の行列では、左に1ビットシフトさせる行列$L^1$、右に1ビットシフトさせる行列$R^1$、はそれぞれ(5)のように表される。
行ベクトルを$L^1$に適用すると、右に1ビットシフトしたように見え、行ベクトルを$R^1$に適用すると、左に1ビットシフトしたように見える。
ここでは「左シフト」はMSB（最上位ビット）方向にビットをシフトし、「右シフト」はLSB（最下位ビット）方向にビットをシフトすること、という風に理解しておこう。

$$
\begin{align}
L^1 =
\begin{bmatrix}
0 & 1 & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1 \\
0 & 0 & 0 & 0 \\
\end{bmatrix},
\quad
R^1 =
\begin{bmatrix}
0 & 0 & 0 & 0 \\
1 & 0 & 0 & 0 \\
0 & 1 & 0 & 0 \\
0 & 0 & 1 & 0 \\
\end{bmatrix}
\end{align}
$$

これらは正則でないので、XOR（排他的論理和）の操作を加えることで$T=I+L^a$または$T=I+R^b$とすると、正則かつ計算コストが少ない操作を作ることが出来る。
この操作は行列計算を行わずにバイナリ演算として実行できる。32ビットの場合の、1ステップのXorshift操作を行うコード例を以下に示す。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryVector32 {
    row: u32,
}

impl BinaryVector32 {
    pub fn new() -> Self {
        Self { row: 0 }
    }

    pub fn xorshift_r(mut self, bits: usize) -> Self {
        self.row ^= self.row >> bits;
        self
    }

    pub fn xorshift_l(mut self, bits: usize) -> Self {
        self.row ^= self.row << bits;
        self
    }
}
```

正則行列の積は正則になるので、これらの行列を掛け合わせて行列$T$の候補を作ることができる。
元の論文[[1]](#reference)によると、$n=32$の場合、$T=(I+L^a)(I+R^b)$または$T=(I+R^b)(I+L^a)$という形で、周期の条件を満たすものはなく、$T=(I+L^a)(I+R^b)(I+R^c)$の形であれば、条件を満たす$(a, b, c)$の組み合わせが多数存在するそうだ。

## 周期の条件を満たすパラメタの探索

では、周期の条件を満たすパラメータ$(a, b, c)$を探してみよう。
$32 \times 32$バイナリ行列$T$の周期は$2^{32}-1$でなければならない。
ただし、$2^{32}-1$は以下のように因数分解できる。

$$
\begin{align}
2^{32} - 1
% &= (2^{16} + 1)(2^{16} - 1) \notag \\
% &= (2^{16} + 1)(2^8 + 1)(2^8 - 1) \notag \\
% &= (2^{16} + 1)(2^8 + 1)(2^4 + 1)(2^4 - 1) \notag \\
&= (2^{16} + 1)(2^8 + 1)(2^4 + 1)(2^2 + 1)(2^2 - 1) \notag \\
&= 65537 \cdot 257 \cdot 17 \cdot 5 \cdot 3
\end{align}
$$

いま確認したいのは、行列$T$が$2^{32}-1$回の乗算で単位行列$I$になること、かつ、それより短い周期で$I$にならないこと、の２点である。
これを言い換えると、以下のように表すことができる（条件(8)--(12)を確認すると、$T^{3} \neq I,~ T^{5} \neq I,~ \cdots$ であることが確認できる）。

$$
\begin{align}
&T^{2^{32}-1} = I \quad \text{or} \quad T^{2^{32}} = T \\
&T^{(2^{32}-1) / 3} \neq I \\
&T^{(2^{32}-1) / 5} \neq I \\
&T^{(2^{32}-1) / 17} \neq I \\
&T^{(2^{32}-1) / 257} \neq I \\
&T^{(2^{32}-1) / 65537} \neq I
\end{align}
$$

以下のRustコードは、$32 \times 32$バイナリ行列を実装して、上記の条件を確認する。
バイナリ行列の累乗に関しては、繰り返し２乗法を用いて計算している。

```rust
/// A 32x32 binary matrix implemented using bitwise operations for efficiency
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryMatrix32 {
    /// Each u32 represents a row of 32 binary values
    rows: [u32; 32],
}

impl BinaryMatrix32 {
    /// Create a new zero matrix
    pub fn new() -> Self {
        Self { rows: [0; 32] }
    }

    /// Create an identity matrix
    pub fn identity() -> Self {
        let mut matrix = Self::new();
        for i in 0..32 {
            matrix.rows[i] = 1 << i;
        }
        matrix
    }

    /// Get a specific bit at (row, col)
    pub fn get(&self, row: usize, col: usize) -> bool {
        if row >= 32 || col >= 32 {
            panic!("Index out of bounds: ({}, {})", row, col);
        }
        (self.rows[row] & (1 << col)) != 0
    }

    /// Set a specific bit at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        if row >= 32 || col >= 32 {
            panic!("Index out of bounds: ({}, {})", row, col);
        }
        if value {
            self.rows[row] |= 1 << col;
        } else {
            self.rows[row] &= !(1 << col);
        }
    }

    /// Binary matrix addition
    pub fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..32 {
            result.rows[i] = self.rows[i] ^ other.rows[i];
        }
        result
    }

    /// Create a shift matrix for right shift by `bits` positions
    /// Example for 3 x 3 matrix with bits = 1
    ///                              |0, 0, 0|
    /// [v1, v2, 0] = [v0, v1, v2] * |1, 0, 0|
    ///                              |0, 1, 0|
    pub fn shift_right(bits: usize) -> Self {
        let mut matrix = Self::new();
        if bits >= 32 || bits == 0 {
            return matrix;
        }

        for i in 0..(32 - bits) {
            matrix.rows[i + bits] = 1 << i;
        }
        matrix
    }

    /// Create a shift matrix for left shift by `bits` positions
    /// Example for 3 x 3 matrix with bits = 1
    ///                              |0, 1, 0|
    /// [0, v0, v1] = [v0, v1, v2] * |0, 0, 1|
    ///                              |0, 0, 0|
    pub fn shift_left(bits: usize) -> Self {
        let mut matrix = Self::new();
        if bits >= 32 || bits == 0 {
            return matrix;
        }

        for i in bits..32 {
            matrix.rows[i - bits] = 1 << i;
        }
        matrix
    }

    /// Binary matrix multiplication
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..32 {
            for j in 0..32 {
                let mut sum = false;
                for k in 0..32 {
                    sum ^= self.get(i, k) && other.get(k, j);
                }
                if sum {
                    result.rows[i] |= 1 << j;
                }
            }
        }
        result
    }

    pub fn pow(&self, n: u64) -> Self {
        if n == 0 {
            return Self::identity(); // T^0 = I
        }
        if n == 1 {
            return *self; // T^1 = T
        }

        // binary exponentation in iterative form
        let mut base = *self;
        let mut result = Self::identity();
        let mut exp = n;

        while exp > 0 {
            if exp & 1 == 1 {               // if LSB is odd, multiply base
                result = result.mul(&base);
            }
            base = base.mul(&base);         // calculate next base
            exp >>= 1;                      // shift right by 1
        }
        result
    }

    /// Comprehensive check for correct xorshift period (2^32 - 1)
    /// Verifies that T^(2^32) = T and that T doesn't have smaller periods
    pub fn has_correct_full_period(&self) -> bool {
        // First check: T^(2^32) = T
        let matrix_pow_2_32 = self.pow(2_u64.pow(32));
        if *self != matrix_pow_2_32 {
            return false;
        }

        let identity = Self::identity();

        // Check that no smaller periods exist by testing T^k != I for proper divisors k of (2^32-1)
        // 2^32 - 1 = 3 × 5 × 17 × 257 × 65537, so check each quotient
        let quotients = [
            (2_u64.pow(32) - 1) / 3,     // Missing factor 3
            (2_u64.pow(32) - 1) / 5,     // Missing factor 5
            (2_u64.pow(32) - 1) / 17,    // Missing factor 17
            (2_u64.pow(32) - 1) / 257,   // Missing factor 257
            (2_u64.pow(32) - 1) / 65537, // Missing factor 65537
        ];

        !quotients.iter().any(|&quotient| self.pow(quotient) == identity)
    }

    /// Test if a triplet (a, b, c) creates a valid xorshift matrix
    pub fn test_xorshift_triplet(a: usize, b: usize, c: usize) -> bool {
        let identity = Self::identity();
        let il_a = identity.add(&Self::shift_left(a));
        let ir_b = identity.add(&Self::shift_right(b));
        let il_c = identity.add(&Self::shift_left(c));

        let xorshift = il_a.mul(&ir_b).mul(&il_c);
        xorshift.has_correct_full_period()
    }

    /// Search for all valid xorshift parameter triplets in the range 1..32
    pub fn search_valid_triplets() -> Vec<(usize, usize, usize)> {
        let mut valid_triplets = Vec::new();
        let mut tested = 0;
        const TOTAL: usize = 15376; // Total cases (a, b, c) with a <= c

        for a in 1..32 {
            for b in 1..32 {
                for c in a..32 {
                    tested += 1;

                    if tested % 1000 == 0 {
                        println!("Progress: {}/{} ({:.1}%)", tested, TOTAL,
                               (tested as f64 / TOTAL as f64) * 100.0);
                    }

                    if Self::test_xorshift_triplet(a, b, c) {
                        valid_triplets.push((a, b, c));
                    }
                }
            }
        }

        valid_triplets
    }
}
```

このコードを実行すると、周期の条件を満たすパラメータ$(a, b, c)$の組み合わせが見つけられる。
実際に結果を表示してみると、元の論文[[1]](#reference)と一致することが確認できる。

```console
Found 81 valid triplets:
| 1, 3,10| 1, 5,16| 1, 5,19| 1, 9,29| 1,11, 6| 1,11,16| 1,19, 3| 1,21,20| 1,27,27|
| 2, 5,15| 2, 5,21| 2, 7, 7| 2, 7, 9| 2, 7,25| 2, 9,15| 2,15,17| 2,15,25| 2,21, 9|
| 3, 1,14| 3, 3,26| 3, 3,28| 3, 3,29| 3, 5,20| 3, 5,22| 3, 5,25| 3, 7,29| 3,13, 7|
| 3,23,25| 3,25,24| 3,27,11| 4, 3,17| 4, 3,27| 4, 5,15| 5, 3,21| 5, 7,22| 5, 9, 7|
| 5, 9,28| 5, 9,31| 5,13, 6| 5,15,17| 5,17,13| 5,21,12| 5,27, 8| 5,27,21| 5,27,25|
| 5,27,28| 6, 1,11| 6, 3,17| 6,17, 9| 6,21, 7| 6,21,13| 7, 1, 9| 7, 1,18| 7, 1,25|
| 7,13,25| 7,17,21| 7,25,12| 7,25,20| 8, 7,23| 8, 9,23| 9, 5,14| 9, 5,25| 9,11,19|
| 9,21,16|10, 9,21|10, 9,25|11, 7,12|11, 7,16|11,17,13|11,21,13|12, 9,23|13, 3,17|
|13, 3,27|13, 5,19|13,17,15|14, 1,15|14,13,15|15, 1,29|17,15,20|17,15,23|17,15,26|
```

## Reference

1. Marsaglia, G. (2003). Xorshift RNGs. Journal of Statistical Software, 8(14), 1–6. doi: [10.1109/IEEESTD.2019.8766229](https://doi.org/10.18637/jss.v008.i14).
2. George Marsaglia, Liang-Huei Tsay, Matrices and the structure of random number sequences, Linear Algebra and its Applications, Volume 67, 1985, Pages 147-156, ISSN 0024-3795, doi: [10.1016/0024-3795(85)90192-2](<https://doi.org/10.1016/0024-3795(85)90192-2>).
3. Luce, R. Duncan. "A note on Boolean matrix theory." Proceedings of the American Mathematical Society 3.3, 1952, 382-388, doi: [10.2307/2031888](https://doi.org/10.2307/2031888).
