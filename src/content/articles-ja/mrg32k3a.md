---
title: 'MRG32k3aを用いたWebアプリ用の乱数発生'
description: '並列計算で用いられる乱数発生アルゴリズムにMRG32k3aがあります。この記事では、WebGL2を用いてGPU上で実行できるMRG32k3aアルゴリズムを実装します。'
pubDate: 2025-07-24
updatedDate: 2025-07-24
heroImage: ''
tags: ['numerical analysis', 'programming']
---

## Introduction

モンテカルロシミュレーションは、1回1回の実行結果を相互にやり取りする必要がないはずなので、計算を並列化すると効率よくシミュレーションを回せそうだ。
WebGL2を用いてウェブアプリケーションとしてモンテカルロシミュレーションをする場合、ひとつハードルになるのが乱数発生だ。
OpenGL ES 3.0には乱数発生させるビルトイン関数は存在しないので、自作で用意してやる必要がある。

基本的に乱数発生アルゴリズムは、初期シードを元に順々に値を発生させていくので、並列化するには何らかの工夫が必要になる。
もっとも単純な手法は、各計算スレッドがまず数列上の異なる点まで大きくスキップして、そこからひとつひとつ値を生成していくという方法だ。
これを実現するには、順々に値を発生させるアルゴリズムに加えて、効率的にスキップするアルゴリズムが必要になる。

![mrg32k3a-1](../figures/mrg32k3a-1.svg)
_Figure 1: Idea of Parallel Random Number Generation._

今回は並列計算でもよく用いられる乱数発生アルゴリズム[[1]](#reference)のうちMRG32k3aについて、L'Ecuyerの論文 [[2,3]](#reference)を参考にアルゴリズムの概要と具体的な実装方法について考えていきたい。

## MRG32k3aとは

乱数発生アルゴリズムのうちMultiple Recursive Generator (MRG)と呼ばれるものは、(1)のように過去の状態の線形和を計算することで状態を更新していく。

$$
\begin{equation}
x_n = (a_1 x_{n-1} + \cdots + a_k x_{n-k}) \quad \mathrm{mod}~m
\end{equation}
$$

このようなMRGを複数組み合わせて、かつパラメタをうまく設定してやることで、より長い周期、良いランダム特性を実現しようとしたものにMRG32k3aがある。
MRG32k3aは以下の式(2)に従って、乱数を発生させていく。

$$
\begin{align}
&x_{1,n} = (1403580 \times x_{1,n-2} - 810728 \times x_{1,n-3})\quad\mathrm{mod}~m_1 \\
&x_{2,n} = (527612 \times x_{2,n-1} - 1370589 \times x_{2,n-3})\quad\mathrm{mod}~m_2 \\
&z_n = (x_{1,n} - x_{2,n})\quad\mathrm{mod}~m_1
\end{align}
$$

where

$$
\begin{align}
m_1 = 2^{32} - 209 = 4294967087, \quad
m_2 = 2^{32} - 22853 = 4294944443 \notag
\end{align}
$$

これらをOpenGL ES 3.0で実装するのは意外と厄介だ。
というのも、32bit型しか使えない（かつ32bitフルで使う）ので、掛け算すると簡単にオーバーフローするし、足し算でもちょくちょくオーバーフローする。
なので、オーバーフローに気を付けつつ掛け算してModをとる関数を用意してやる必要がある。
$a \times b$を計算する場合、基本的には$a$を何度も足し合わせることで積を求めていく。
ただし繰り返し2乗法と似た要領で計算回数を減らして、足し算のたびにModを確認するようにした。

```glsl
uint addModM(uint a, uint b, uint m) {
    uint amodm = a % m;
    uint bmodm = b % m;
    uint blim = m - amodm;
    if (bmodm <= blim) {
        return amodm + bmodm;
    }
    else {
        return amodm - (m - bmodm);
    }
}

uint diffModM(uint a, uint b, uint m) {
    uint amodm = a % m;
    uint bmodm = b % m;
    if (amodm >= bmodm) {
        return amodm - bmodm;
    }
    else {
        return amodm + (m - bmodm);
    }
}

uint multModM(uint a, uint b, uint m) {
    uint amodm = a % m;
    uint bmodm = b % m;
    uint res = 0u;
    while (bmodm > 0u) {
        if ((bmodm&0x1u) == 0x1u) {
            res = addModM(res, amodm, m);
        }
        bmodm = bmodm >> 1;
        amodm = addModM(amodm, amodm, m);
    }
    return res;
}
```

これらを用いるとMRG32k3aは以下のように実装できる。

```glsl
// MRG32k3a parameters
uint[3] x1 = uint[3](0u, 0u, 1234567u);
uint[3] x2 = uint[3](0u, 0u, 1234567u);
const uint m1 = 4294967087u;
const uint m2 = 4294944443u;
const uint a11 = 1403580u;
const uint a10 = 810728u;
const uint a22 = 527612u;
const uint a20 = 1370589u;

uint stepMRG32k3a(void) {
    uint x1i = diffModM(multModM(x1[1], a11, m1), multModM(x1[0], a10, m1), m1);
    uint x2i = diffModM(multModM(x2[2], a22, m2), multModM(x2[0], a20, m2), m2);
    x1[0] = x1[1];
    x1[1] = x1[2];
    x1[2] = x1i;
    x2[0] = x2[1];
    x2[1] = x2[2];
    x2[2] = x2i;
    return diffModM(x1i, x2i, m1);
}
```

## MRG32k3aのスキップ機能

さて一方で、MRG32k3aを行列の形に書き直すと、1ステップ前進は次のように表される。

$$
\begin{align}
\left( \begin{array}{c} x_{1,n-2} \\ x_{1,n-1} \\ x_{1,n} \end{array} \right) &=
\left( \begin{array}{ccc} 0 & 1 & 0 \\ 0 & 0 & 1 \\ -810728 & 1403580 & 0 \end{array} \right) \notag
\left( \begin{array}{c} x_{1,n-3} \\ x_{1,n-2} \\ x_{1,n-1} \end{array} \right) \\ &=
\left( \begin{array}{ccc} 0 & 1 & 0 \\ 0 & 0 & 1 \\ 4294156359 & 1403580 & 0 \end{array} \right)
\left( \begin{array}{c} x_{1,n-3} \\ x_{1,n-2} \\ x_{1,n-1} \end{array} \right)
\end{align}
$$

$$
\begin{align}
\left( \begin{array}{c} x_{2,n-2} \\ x_{2,n-1} \\ x_{2,n} \end{array} \right) &=
\left( \begin{array}{ccc} 0 & 1 & 0 \\ 0 & 0 & 1 \\ -1370589 & 0 & 527612 \end{array} \right) \notag
\left( \begin{array}{c} x_{2,n-3} \\ x_{2,n-2} \\ x_{2,n-1} \end{array} \right) \\ &=
\left( \begin{array}{ccc} 0 & 1 & 0 \\ 0 & 0 & 1 \\ 4293573854 & 0 & 527612 \end{array} \right)
\left( \begin{array}{c} x_{2,n-3} \\ x_{2,n-2} \\ x_{2,n-1} \end{array} \right)
\end{align}
$$

1ステップ後退は次のように表される。

$$
\begin{align}
x_{1,n} &= (184888585 \times x_{1,n+1} + 1945170933 \times x_{1,n+3})~\mathrm{mod}~m_1 \\
x_{2,n} &= (360366334 \times x_{2,n+2} + 4225571728 \times x_{2,n+3})~\mathrm{mod}~m_2 \\
&= (360366334 \times x_{2,n+2} - 69372715 \times x_{2,n+3})~\mathrm{mod}~m_2
\end{align}
$$

$$
\begin{equation}
\left( \begin{array}{c} x_{1,n} \\ x_{1,n+1} \\ x_{1,n+2} \end{array} \right) =
\left( \begin{array}{ccc} 184888585 & 0 & 1945170933 \\ 1 & 0 & 0 \\ 0 & 1 & 0 \end{array} \right)
\left( \begin{array}{c} x_{1,n+1} \\ x_{1,n+2} \\ x_{1,n+3} \end{array} \right)
\end{equation}
$$

$$
\begin{equation}
\left( \begin{array}{c} x_{1,n} \\ x_{1,n+1} \\ x_{1,n+2} \end{array} \right) =
\left( \begin{array}{ccc} 0 & 360366334 & 4225571728 \\ 1 & 0 & 0 \\ 0 & 1 & 0 \end{array} \right)
\left( \begin{array}{c} x_{1,n+1} \\ x_{1,n+2} \\ x_{1,n+3} \end{array} \right)
\end{equation}
$$

1ステップ前進・後退を行列の形で書くことが出来たので、この行列の累乗を求めてやれば、一気に複数ステップ前進・後退させることができる。
ここでも繰り返し2乗法と同じ要領で実装すれば、行列を1回1回かけていくよりも効率よく累乗を求めることが出来る。

まず行列の積に関しては、先ほどのModをとる関数を用いて、uintの1次元配列からべた書きする（OpenGL ES 3.0には2次元配列がないのと、mat3はfloat型しかないため）。

$$
\begin{equation}
\left[ \begin{array}{ccc} a_0 & a_1 & a_2 \\ a_3 & a_4 & a_5 \\ a_6 & a_7 & a_8 \end{array} \right]
\left[ \begin{array}{ccc} b_0 & b_1 & b_2 \\ b_3 & b_4 & b_5 \\ b_6 & b_7 & b_8 \end{array} \right] =
\left[ \begin{array}{ccc} c_0 & c_1 & c_2 \\ c_3 & c_4 & c_5 \\ c_6 & c_7 & c_8 \end{array} \right]
\end{equation}
$$

$$
\begin{align*}
c_0 &= a_0 b_0 + a_1 b_3 + a_2 b_6 \\
c_1 &= a_0 b_1 + a_1 b_4 + a_2 b_7 \\
c_2 &= a_0 b_2 + a_1 b_5 + a_2 b_8 \\
c_3 &= a_3 b_0 + a_4 b_3 + a_5 b_6 \\
c_4 &= a_3 b_1 + a_4 b_4 + a_5 b_7 \\
c_5 &= a_3 b_2 + a_4 b_5 + a_5 b_8 \\
c_6 &= a_6 b_0 + a_7 b_3 + a_8 b_6 \\
c_7 &= a_6 b_1 + a_7 b_4 + a_8 b_7 \\
c_8 &= a_6 b_2 + a_7 b_5 + a_8 b_8
\end{align*}
$$

```glsl
// calculate c = a*b mod m
void matMultModM(uint[9] a, uint[9] b, uint m, out uint[9] c) {
    c[0] = addModM(addModM(multModM(a[0],b[0],m), multModM(a[1],b[3],m), m), multModM(a[2],b[6],m), m);
    c[1] = addModM(addModM(multModM(a[0],b[1],m), multModM(a[1],b[4],m), m), multModM(a[2],b[7],m), m);
    c[2] = addModM(addModM(multModM(a[0],b[2],m), multModM(a[1],b[5],m), m), multModM(a[2],b[8],m), m);
    c[3] = addModM(addModM(multModM(a[3],b[0],m), multModM(a[4],b[3],m), m), multModM(a[5],b[6],m), m);
    c[4] = addModM(addModM(multModM(a[3],b[1],m), multModM(a[4],b[4],m), m), multModM(a[5],b[7],m), m);
    c[5] = addModM(addModM(multModM(a[3],b[2],m), multModM(a[4],b[5],m), m), multModM(a[5],b[8],m), m);
    c[6] = addModM(addModM(multModM(a[6],b[0],m), multModM(a[7],b[3],m), m), multModM(a[8],b[6],m), m);
    c[7] = addModM(addModM(multModM(a[6],b[1],m), multModM(a[7],b[4],m), m), multModM(a[8],b[7],m), m);
    c[8] = addModM(addModM(multModM(a[6],b[2],m), multModM(a[7],b[5],m), m), multModM(a[8],b[8],m), m);
}
```

これを用いて、行列の累乗を計算する。

```glsl
// calculate b = a**n mod m
void matPowModM(uint[9] a, uint n, uint m, out uint[9] b) {
    uint[9] apow = a;
    b = uint[9](1u, 0u, 0u, 0u, 1u, 0u, 0u, 0u, 1u);
    while (n > 0u) {
        if ((n&0x1u) == 0x1u) {
            matMultModM(apow, b, m, b);
        }
        n = n >> 1;
        matMultModM(apow, apow, m, apow);
    }
}
```

最終的に行列の累乗を用いて状態ベクトルを更新してやれば、スキップの機能を実装することが出来る。

```glsl
const uint[9] a1 = uint[9](0u, 1u, 0u, 0u, 0u, 1u, 4294156359u, 1403580u, 0u);
const uint[9] a2 = uint[9](0u, 1u, 0u, 0u, 0u, 1u, 4293573854u, 0u, 527612u);

uint skipMRG32k3a(uint n) {
    uint[9] a1pow;
    uint[9] a2pow;
    matPowModM(a1, n, m1, a1pow);
    matPowModM(a2, n, m2, a2pow);
    matVecMultModM(a1pow, x1, m1, x1);
    matVecMultModM(a2pow, x2, m2, x2);
    return diffModM(x1[0], x2[0], m1);
}
```

## Reference

1. T. Bradley, J. du Toit, R. Tong, M. Giles, P. Woodhams, Parallelization techniques for random numbers generators, in: GPU Computing Gems, Gems Emerald ed., 2011, pp. 231–246, [10.1016/B978-0-12-384988-5.00016-4](https://doi.org/10.1016/B978-0-12-384988-5.00016-4).
2. Pierre L'Ecuyer, (1999) Good Parameters and Implementations for Combined Multiple Recursive Random Number Generators. Operations Research 47(1):159-164, [10.1287/opre.47.1.159](https://doi.org/10.1287/opre.47.1.159).
3. Pierre L'Ecuyer, Richard Simard, E. Jack Chen, W. David Kelton, (2002) An Object-Oriented Random-Number Package with Many Long Streams and Substreams. Operations Research 50(6):1073-1075, [10.1287/opre.50.6.1073.358](https://doi.org/10.1287/opre.50.6.1073.358).
