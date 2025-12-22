---
title: 'Pure Rust 線形代数ライブラリfaer入門① データ構造とライフタイム'
description: 'faerはPure Rustで高速な大規模行列計算を実現するライブラリです。行列(Mat)およびビュー(MatRef)のデータ構造と、メモリレイアウト、ライフタイムの仕組みについて解説します。'
pubDate: 2025-12-23
updatedDate: 2025-12-23
heroImage: ''
tags: ['programming', 'numerical analysis', 'mathematics']
---

Rustで科学計算、特に線形代数の計算を行う場合、ndarrayやnalgebraといった行列計算ライブラリがよく知られているが、最近新たにfaerという線形代数ライブラリが注目され始めている。それぞれの特徴を簡単に比較すると以下のようになる。

- ndarray: PythonのNumPyに近い汎用的な多次元配列ライブラリ。データの保持や基本操作はRustのみで完結するが、高速な行列積や高度な線形代数演算を行うには、OpenBLASやIntel MKLといった外部のライブラリに依存する必要がある。
- nalgebra: コンピュータグラフィックスやロボティクスで重宝される、Pure Rustの線形代数ライブラリ。ジェネリクスで行列サイズを固定できるため、小規模な行列計算には最適だが、巨大な行列の計算は必ずしも得意ではない。
- faer: 今回紹介する、大規模な2次元行列計算に特化したライブラリ。Pure Rustでありながら、SIMD最適化やマルチスレッドを活用して高速な行列演算を実現し、OpenBLASやC++のEigenに匹敵する速度を叩き出すらしい（[benchmarks | faer](https://faer.veganb.tw/benchmarks/)）。

faerは外部ライブラリに依存せずpure Rustで大規模行列計算を高速に行える点が最大の特徴だ。
これにより、環境構築が容易になり、Rustらしいコードを書けることが大きな魅力となっている。

## Installation

faerはCargo.tomlに以下のように記述することでインストールできる。
今回は、2025年12月時点での最新バージョンである0.23.2を指定する。

```toml
[dependencies]
faer = "0.23.2"
```

## 行列（Mat）の作成

まずは基本的な行列の作成方法を見てみよう。faerでは`Mat<T>`という型で2次元行列を表現する。
ゼロ行列、単位行列、全要素が1の行列、定数行列などは、それぞれ専用の関数で簡単に生成できる。

```rust
use faer::Mat;
use faer::mat;

fn main() {
    // 1. ゼロ行列 (Zero Matrix)
    let zeros = Mat::<f64>::zeros(3, 3);
    println!("Zero Matrix:\n{:?}", zeros);
    // Zero Matrix (3, 3):
    // [
    // [0.0, 0.0, 0.0],
    // [0.0, 0.0, 0.0],
    // [0.0, 0.0, 0.0],
    // ]

    // 2. 単位行列 (Identity Matrix)
    let identity = Mat::<f64>::identity(3, 3);
    println!("Identity Matrix:\n{:?}", identity);
    // Identity Matrix (3, 3):
    // [
    // [1.0, 0.0, 0.0],
    // [0.0, 1.0, 0.0],
    // [0.0, 0.0, 1.0],
    // ]

    // 3. 全要素が1の行列 (Ones Matrix)
    let ones: Mat<f64> = Mat::<f64>::ones(3, 3);
    println!("Ones Matrix {:?}:\n{:?}", ones.shape(), ones);
    // Ones Matrix (3, 3):
    // [
    // [1.0, 1.0, 1.0],
    // [1.0, 1.0, 1.0],
    // [1.0, 1.0, 1.0],
    // ]

    // 4. 定数行列 (Constant Matrix)
    let constants = Mat::<f64>::full(3, 3, 5.0);
    println!("Constant Matrix:\n{:?}", constants);
    // Constant Matrix (3, 3):
    // [
    // [5.0, 5.0, 5.0],
    // [5.0, 5.0, 5.0],
    // [5.0, 5.0, 5.0],
    // ]
}
```

Rustのクロージャやマクロを使うことで、より柔軟に行列を初期化することもできる。
クロージャを使う方法では、行列の各要素をインデックスに基づいて計算できる。
巨大な行列を作成する際も、一時的なメモリを使わずに効率的に初期化できる。

```rust
use faer::Mat;
use faer::mat;

fn main() {
    // 5. クロージャによる初期化 (Closure Initialization)
    let a = Mat::from_fn(3, 3, |i, j| (i * 10 + j) as f64);
    println!("Matrix A:\n{:?}", a);
    // Matrix A (3, 3):
    // [
    // [0.0, 1.0, 2.0],
    // [10.0, 11.0, 12.0],
    // [20.0, 21.0, 22.0],
    // ]

    // 6. マクロによる初期化 (Macro Initialization)
    let b = mat![
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, 3.0],
        [0.0, 8.0, 0.0],
    ];
    println!("Matrix B {:?}:\n{:?}", b.shape(), b);
    // Matrix B (4, 3):
    // [
    // [1.0, 0.0, 0.0],
    // [0.0, 2.0, 0.0],
    // [0.0, 0.0, 3.0],
    // [0.0, 8.0, 0.0],
    // ]
}
```

## faerでの行列表現

faerにおいて、２次元行列のデータがどのように表現されているかをもう少し詳しく見ておこう。
ざっくりとしたイメージとして、２次元行列のデータ型として最低限必要な情報は、以下の4つである。

- 行列要素のデータが格納されているメモリの先頭へのポインタ
- 行数
- 列数
- 行列要素のデータ型

これらの情報があれば、メモリ上に並んだバイナリデータを行列要素に対応させることが出来る。
faerではたいていの場合、行列は`Mat<T>`という型で表されるが、これはユーザー用にラップされた表現で、中身を理解するにはもう少し包みを剥がす必要がある。

faerの実装を見てみると、`generic::Mat<Own<T, Rows, Cols>>`と定義されている行列オブジェクトを、`Mat<T, Rows = usize, Cols = usize>`という型エイリアスで名前付けしていることが分かる。
とくに行数と列数を動的に`usize`として扱う場合、それらを省略して`Mat<T>`と書ける。

```rust
pub type Mat<T, Rows = usize, Cols = usize> = generic::Mat<Own<T, Rows, Cols>>;
```

`Mat<T>`のデータ型としての実体は`Own<T, Rows: Shape = usize, Cols: Shape = usize>`という構造体として定義されている。
`Rows`と`Cols`は行数と列数を表す型パラメータであり、faerは大規模な２次元行列を扱うことを目的としているので、普通は`usize`として動的に指定する。
ただしこれらが、`Shape`トレイトを満たすようなジェネリクスとなっているのは、固定サイズの行列を効率的に扱えるようにするためである。
`RawMat<T>`は行列データの生のメモリ情報を保持する構造体であり、先頭ポインタや確保されたメモリ容量、レイアウト情報などを含んでいる。

```rust
pub struct Own<T, Rows: Shape = usize, Cols: Shape = usize> {
	raw: RawMat<T>,
	nrows: Rows,
	ncols: Cols,
}

struct RawMat<T> {
	ptr: NonNull<T>,
	row_capacity: usize,
	col_capacity: usize,
	layout: StackReq,
	__marker: PhantomData<T>,
}
```

さて、このような行列`Mat<T>`のデータの中身は、ヒープに確保された連続したメモリ領域に格納されており、
faerではColumn-major（つまり、同じ列に属する要素がメモリ上で連続した形）で配置されている。
ただし、SIMD最適化のために、行数がSIMD幅（例えば64byte = f64の8個分）の倍数になるように確保する領域がpaddingされることがある。
このことは、`unsafe`を利用して次のようなコードを実行してみると確認できる。
3×2の行列を作成し、ポインタの値とポインタの指す値を順番に表示してみると、1列目の要素が3つ連続してメモリ上に並び、その後5つ分のpaddingを挟んだ後、2列目の要素が表示される。

```rust
use faer::Mat;

fn main() {
    let mat: Mat<f64> = Mat::from_fn(3, 2, |i, j| ((i + 1) * 10 + (j + 1)) as f64);
    println!("--- Matrix Data {:?} ---\n{:?}", mat.shape(), mat);
    // --- Matrix Data (3, 2) ---
    // [
    // [11.0, 12.0],
    // [21.0, 22.0],
    // [31.0, 32.0],
    // ]

    println!("--- Sequential Data on Memory ---");
    let base_ptr = mat.as_ptr();
    let row_stride = mat.row_stride();
    let col_stride = mat.col_stride();
    let len = mat.ncols() * col_stride as usize;
    println!("Shape: {:?}, Row Stride: {}, Col Stride: {}", mat.shape(), row_stride, col_stride);

    unsafe {
        for offset in 0..len {
            let ptr = base_ptr.add(offset);
            println!("Offset +{}, {:?}: {:.1}", offset, ptr, *ptr);
        }
    }
    // --- Sequential Data on Memory ---
    // Shape: (3, 2), Row Stride: 1, Col Stride: 8
    // Offset +0, 0x562139354780: 11.0  <-- mat[0,0]
    // Offset +1, 0x562139354788: 21.0  <-- mat[1,0]
    // Offset +2, 0x562139354790: 31.0  <-- mat[2,0]
    // Offset +3, 0x562139354798: 0.0
    // Offset +4, 0x5621393547a0: 0.0
    // Offset +5, 0x5621393547a8: 0.0
    // Offset +6, 0x5621393547b0: 0.0
    // Offset +7, 0x5621393547b8: 0.0
    // Offset +8, 0x5621393547c0: 12.0  <-- mat[0,1]
    // Offset +9, 0x5621393547c8: 22.0  <-- mat[1,1]
    // Offset +10, 0x5621393547d0: 32.0 <-- mat[2,1]
    // Offset +11, 0x5621393547d8: 0.0
    // Offset +12, 0x5621393547e0: 0.0
    // Offset +13, 0x5621393547e8: 0.0
    // Offset +14, 0x5621393547f0: 0.0
    // Offset +15, 0x5621393547f8: 0.0
}
```

## MatRef<'a, T>

ここまで見てきた`Mat<T>`は、行列データの所有権を持つ型である。
一方で、行列データの所有権を持たず、参照として振る舞う型として`MatRef<'a, T>`がある。
`MatRef<'a, T>`は、単なる`Mat<T>`の参照`&Mat<T>`というだけでなく、行列データの一部を参照するスライスとしても機能し、これをうまく用いると、転置やピボット操作、部分行列の抽出などを効率的に行うことができる。
このあたりの仕組みを意識しながらfaerを使うと、不用意にメモリを確保したりデータをコピーしたりせずに、効率的な行列計算が可能になる。

### MatRefの使い方

まずは、MatRefの作成方法を確認しよう。
行列全体のビュー、あるいは部分行列のビューは以下のように作成できる。
部分行列を作成する場合、行列の行数・列数は変化するが、行ストライド・列ストライドは変化しない。

```rust
use faer::{Mat, MatRef};

fn main() {
    // 4x4 の行列を作成
    // (i, j) -> i * 10 + j
    let mat: Mat<f64> = Mat::from_fn(4, 4, |i, j| ((i + 1) * 10 + (j + 1)) as f64);
    println!("mat {:?}:\n{:?}", mat.shape(), mat);
    // mat (4, 4):
    // [
    // [11.0, 12.0, 13.0, 14.0],
    // [21.0, 22.0, 23.0, 24.0],
    // [31.0, 32.0, 33.0, 34.0],
    // [41.0, 42.0, 43.0, 44.0],
    // ]

    // 行列全体のビューを取得
    let view: MatRef<f64> = mat.as_ref();
    println!("view {:?}:\n{:?}", view.shape(), view);
    // view (4, 4):
    // [
    // [11.0, 12.0, 13.0, 14.0],
    // [21.0, 22.0, 23.0, 24.0],
    // [31.0, 32.0, 33.0, 34.0],
    // [41.0, 42.0, 43.0, 44.0],
    // ]

    // 部分行列のスライスを取得
    let sub_matrix: MatRef<f64> = mat.as_ref().submatrix(1, 1, 2, 3);
    println!("sub_matrix {:?}:\n{:?}", sub_matrix.shape(), sub_matrix);
    // sub_matrix (2, 3):
    // [
    // [22.0, 23.0, 24.0],
    // [32.0, 33.0, 34.0],
    // ]

    // 複数行のスライスを取得
    let sub_rows: MatRef<f64> = mat.as_ref().subrows(1, 2);
    println!("sub_rows {:?}:\n{:?}", sub_rows.shape(), sub_rows);
    // sub_rows (2, 4):
    // [
    // [21.0, 22.0, 23.0, 24.0],
    // [31.0, 32.0, 33.0, 34.0],
    // ]

    // 複数列のスライスを取得
    let sub_cols: MatRef<f64> = mat.as_ref().subcols(0, 2);
    println!("sub_cols {:?}:\n{:?}", sub_cols.shape(), sub_cols);
    // sub_cols (4, 2):
    // [
    // [11.0, 12.0],
    // [21.0, 22.0],
    // [31.0, 32.0],
    // [41.0, 42.0],
    // ]
}
```

一方で、行ストライド・列ストライド、および先頭ポインタを変化させることで、いくつかの便利な行列のビューを作成できる。
例えば転置行列の場合、先頭ポインタはそのまま、行ストライドと列ストライドを入れ替えることで実現できる。
もともとの行列では、行方向インデックスを1増やすとメモリ上で`f64`1つ分進み、列方向インデックスを1増やすと`f64`8つ分進んでいた。
これを行方向インデックスを1増やすと`f64`8つ分（元の行列で列方向に1つ分）進み、列方向インデックスを1増やすと`f64`1つ分（元の行列で行方向に1つ分）進むように変更することで、新しいデータをメモリ上に並べることなく転置行列の振る舞いを実現できる。
同様の考え方で、行反転行列や列反転行列のビューも作成できる。

```rust
use faer::{Mat, MatRef};

fn main() {

    // 4x4 の行列を作成
    // (i, j) -> i * 10 + j
    let mat: Mat<f64> = Mat::from_fn(4, 4, |i, j| ((i + 1) * 10 + (j + 1)) as f64);
    println!("mat {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", mat.shape(), mat.row_stride(), mat.col_stride(), mat.as_ptr(), mat);
    // mat (4, 4), row stride 1, col stride 8, ptr 0x556fd7e5ca00:
    // [
    // [11.0, 12.0, 13.0, 14.0],
    // [21.0, 22.0, 23.0, 24.0],
    // [31.0, 32.0, 33.0, 34.0],
    // [41.0, 42.0, 43.0, 44.0],
    // ]

    // 転置行列のビューを取得
    let transpose: MatRef<f64> = mat.as_ref().transpose();
    println!("transpose {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", transpose.shape(), transpose.row_stride(), transpose.col_stride(), transpose.as_ptr(), transpose);
    // transpose (4, 4), row stride 8, col stride 1, ptr 0x556fd7e5ca00:
    // [
    // [11.0, 21.0, 31.0, 41.0],
    // [12.0, 22.0, 32.0, 42.0],
    // [13.0, 23.0, 33.0, 43.0],
    // [14.0, 24.0, 34.0, 44.0],
    // ]

    // 行反転行列のビューを取得
    let reverse_row: MatRef<f64> = mat.as_ref().reverse_rows();
    println!("reverse_row {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", reverse_row.shape(), reverse_row.row_stride(), reverse_row.col_stride(), reverse_row.as_ptr(), reverse_row);
    // reverse_row (4, 4), row stride -1, col stride 8, ptr 0x556fd7e5ca18:
    // [
    // [41.0, 42.0, 43.0, 44.0],
    // [31.0, 32.0, 33.0, 34.0],
    // [21.0, 22.0, 23.0, 24.0],
    // [11.0, 12.0, 13.0, 14.0],
    // ]

    // 列反転行列のビューを取得
    let reverse_col: MatRef<f64> = mat.as_ref().reverse_cols();
    println!("reverse_col {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", reverse_col.shape(), reverse_col.row_stride(), reverse_col.col_stride(), reverse_col.as_ptr(), reverse_col);
    // reverse_col (4, 4), row stride 1, col stride -8, ptr 0x556fd7e5cac0:
    // [
    // [14.0, 13.0, 12.0, 11.0],
    // [24.0, 23.0, 22.0, 21.0],
    // [34.0, 33.0, 32.0, 31.0],
    // [44.0, 43.0, 42.0, 41.0],
    // ]
}
```

行ベクトル、列ベクトル、対角行列のビューはそれぞれ専用の型が存在し、以下のように作成できる。

```rust
use faer::diag::DiagRef;
use faer::{Mat, MatRef, RowRef, ColRef};

fn main() {
    // 4x4 の行列を作成
    // (i, j) -> i * 10 + j
    let mat: Mat<f64> = Mat::from_fn(4, 4, |i, j| ((i + 1) * 10 + (j + 1)) as f64);
    println!("mat {:?}:\n{:?}", mat.shape(), mat);
    // mat (4, 4):
    // [
    // [11.0, 12.0, 13.0, 14.0],
    // [21.0, 22.0, 23.0, 24.0],
    // [31.0, 32.0, 33.0, 34.0],
    // [41.0, 42.0, 43.0, 44.0],
    // ]

    // 行ベクトルのスライスを取得
    let row_vector: RowRef<f64> = mat.as_ref().row(2);
    println!("row_vector {:?}:\n{:?}", row_vector.shape(), row_vector);
    // row_vector (1, 4):
    // [31.0, 32.0, 33.0, 34.0]

    // 列ベクトルのスライスを取得
    let col_vector: ColRef<f64> = mat.as_ref().col(3);
    println!("col_vector {:?}:\n{:?}", col_vector.shape(), col_vector);
    // col_vector (4, 1):
    // [14.0, 24.0, 34.0, 44.0]

    // 対角成分のビューを取得
    let diagonal: DiagRef<f64> = mat.as_ref().diagonal();
    println!("diagonal {:?}:\n{:?}", diagonal.dim(), diagonal);
    // diagonal 4:
    // [11.0, 22.0, 33.0, 44.0]
}
```

### MatRefのライフタイム

行列のビューをRustを用いて実現するうえで重要となるのは、ライフタイムを適切に管理することである。
`MatRef<'a, T>`のライフタイムは参照先の`Mat<T>`のライフタイムの範囲内に収まってないといけない。
このことを、どのようにコンパイル時に保証しているかを見てみよう。

まず、`MatRef<'a, T>`は、`generic::Mat<Ref<'a, T, Rows, Cols, RStride, CStride>>`の型エイリアスで、実際の中身は`Ref<'a, T, Rows, Cols, RStride, CStride>`という構造体であることが分かる。

```rust
pub type MatRef<
	'a,
	T,
	Rows = usize,
	Cols = usize,
	RStride = isize,
	CStride = isize,
> = generic::Mat<Ref<'a, T, Rows, Cols, RStride, CStride>>;
```

行列の参照を扱う上で、本質的に必要な情報は、メモリ上にある行列データへのポインタ、行数、列数、行ストライド、列ストライド、データ型である。
これらのデータは`MatView`という構造体で保持され、`Ref`のPublicメンバになっているが、`MatView`自体はライフタイムパラメータを持たないので、ポインタの先のデータが有効かどうかは保証していない。
`MatRef<'a, T>`のライフタイムを示すパラメータ`'a`は、`Ref`の中の`PhantomData<&'a T>`というダミーデータにくっつけられており、`Ref`全体のライフタイムが`'a`に依存していることを示している。

```rust
pub struct Ref<'a, T, Rows = usize, Cols = usize, RStride = isize, CStride = isize,> {
	pub(super) imp: MatView<T, Rows, Cols, RStride, CStride>,
	pub(super) __marker: PhantomData<&'a T>,
}

pub(crate) struct MatView<T: ?Sized, Rows, Cols, RStride, CStride> {
	ptr: NonNull<T>,
	nrows: Rows,
	ncols: Cols,
	row_stride: RStride,
	col_stride: CStride,
}
```

ライフタイム`'a`がもともとのデータ`Mat<T>`の参照のライフタイムなんだぞ、ということは`mat.as_ref()`によって`MatRef<'a, T>`を作成するときにコンパイラに伝えている。
`as_ref()`メソッドは`generic::Mat<Inner>`に対して実装されており、`Inner`が`Reborrow`トレイトを満たすこと（`rb()`メソッドが実装されていること）を要求している。
これにより、ユーザー的には`Mat`、`MatRef`、`MatMut`など様々な型に対して`as_ref()`メソッドが使えることになるのだが、特に重要なのは`Mat`に対する実装である。

```rust
impl<
	T,
	Rows: Shape,
	Cols: Shape,
	RStride: Stride,
	CStride: Stride,
	Inner: for<'short> Reborrow<
			'short,
			Target = Ref<'short, T, Rows, Cols, RStride, CStride>,
		>,
> generic::Mat<Inner>
{
	/// returns a view over `self`
	#[inline]
	pub fn as_ref(&self) -> MatRef<'_, T, Rows, Cols, RStride, CStride> {
		self.rb()
	}
}

pub mod generic {
    pub struct Mat<Inner>(pub Inner);

    impl<'short, Inner: Reborrow<'short>> Reborrow<'short> for Mat<Inner> {
		type Target = Mat<Inner::Target>;

		#[inline(always)]
		fn rb(&'short self) -> Self::Target {
			Mat(self.0.rb())
		}
	}
}
```

`Mat`に対して`Reborrow`トレイトがどのように実装されているかを見てみよう。
`rb()`メソッドは`&self`を受け取り、この`&self`のライフタイムを`'short`として、`Ref`オブジェクトを作成している。以下のコード内では明示的に書かれていないが、`Ref`の中の`PhantomData<&'short T>`によって、`Ref`全体のライフタイムが`'short`となっている。
つまり、`Mat`の参照`&Mat`のライフタイムを、`PhantomData`にくっつけることで、返却される`MatRef`のライフタイムが`&Mat`のライフタイムである、ということをコンパイラに伝えているのだ。

```rust
impl<'short, T, Rows: Shape, Cols: Shape> Reborrow<'short>
	for Own<T, Rows, Cols>
{
	type Target = Ref<'short, T, Rows, Cols>;

	#[inline]
	fn rb(&'short self) -> Self::Target {
		Ref {
			imp: MatView {
				ptr: self.raw.ptr,
				nrows: self.nrows,
				ncols: self.ncols,
				row_stride: 1,
				col_stride: self.raw.row_capacity as isize,
			},
			__marker: PhantomData,
		}
	}
}
```

## まとめ

今回はRustの線形代数ライブラリfaerの基本的な使い方と、行列データの内部表現について解説した。
特に`MatRef`を用いた行列のビュー操作は、効率的な行列操作を行う上で重要な概念である。
次回は、faerを用いた基本的な行列演算の方法について紹介する。
