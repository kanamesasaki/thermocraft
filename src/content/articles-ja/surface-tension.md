---
title: '表面張力とヤング・ラプラスの式'
description: '異なる流体が曲率を持った表面を境界にして接している場合、圧力と表面張力を考慮して力のつりあい式を立てることが可能です。これはヤング・ラプラスの式と呼ばれ、境界となる表面の曲率、表面張力、圧力差の関係を表します。'
pubDate: 2025-09-01
updatedDate: 2025-09-22
heroImage: ''
tags: ['fluid dynamics']
---

異なる流体が曲率を持った表面を境界にして接している場合、圧力と表面張力を考慮して力のつりあい式を立てることが出来る。
これはヤング・ラプラスの式と呼ばれ、境界となる表面の曲率、表面張力、圧力差の関係を表す。今回はヤング・ラプラスの式を[[1]](#references)を参考に導出していこう。

## ヤング・ラプラスの式

2種類の流体がある表面を境界にして接している状況について考えよう。表面が曲面の場合、それぞれの流体の圧力は異なっており、この圧力差は表面の張力によって保持されていると考えられる。いま、表面の位置が$dz$だけ、流体1から流体2側に移動したとすると、そのときの仕事$\delta R$は次のように表すことができる。

$$
\begin{equation}
% \label{eq:LandauLifshitzFluid_61.1}
\delta R = - \int_A (p_1 - p_2) \delta z dA + \sigma \delta A
\end{equation}
$$

1項目は圧力による仕事で、2項目は表面の面積が変化することで張力のする仕事である。この係数$\sigma$はsurface-tension coefficient（あるいは単純にsurface tension）と呼ばれ、次元は$[\mathrm{N}/\mathrm{m}]$または$[\mathrm{J}/\mathrm{m}^2]$である。

![surface-tension-1](../figures/surface-tension-1.svg)
_Figure 1: Two Media Separated by the Curved Surface._

表面張力による仕事を求めたいので、まず表面が$\delta z$移動することによる、表面要素$dA= l_1 l_2$の面積の変化を評価しよう。ただし、表面の曲率半径は2方向にそれぞれ$R_1, R_2$で、$\delta z$の2次の項は十分小さいものとする。

$$
\begin{equation}
\frac{l_1}{R_1} (R_1 + \delta z) \frac{l_2}{R_2} (R_2 + \delta z) - l_1 l_2\simeq l_1 l_2 \delta z \left( \frac{1}{R_1} + \frac{1}{R_2} \right)
\end{equation}
$$

これを表面要素について積分してやれば、表面全体での面積変化$\delta A$が得られる。

$$
\begin{equation}
% \label{eq:LandauLifshitzFluid_61.2}
\delta A = \int_A l_1 l_2 \delta z \left( \frac{1}{R_1} + \frac{1}{R_2} \right) dA
\end{equation}
$$

(3)を(1)に代入する。

$$
\begin{equation}
\delta R = - \int_A \delta z \left \{ (p_1 - p_2) - \sigma \left( \frac{1}{R_1} + \frac{1}{R_2} \right) \right\} dA
\end{equation}
$$

平衡状態となるのは$\delta R = 0$となる場合で、この条件が各表面要素上で成り立つべきなので、以下の関係式(5)が得られる。これをヤング・ラプラスの式と呼ぶ。

$$
\begin{equation}
% \label{eq:LandauLifshitzFluid_61.3}
\Delta p = p_1- p_2 = \sigma \left( \frac{1}{R_1} + \frac{1}{R_2} \right)
\end{equation}
$$

## References

1. L D Landau, E.M. Lifshitz, "Fluid Mechanics, Second Edition", Pergamon Press, 1987, doi: [10.1016/C2013-0-03799-1](https://doi.org/10.1016/C2013-0-03799-1)
