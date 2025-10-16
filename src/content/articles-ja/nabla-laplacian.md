---
title: '円筒座標・極座標のナブラとラプラシアン'
description: '円筒座標系や極座標系におけるナブラ（Nabla）とラプラシアン（Laplacian）を、一般座標系におけるナブラの表現をもとに導出します。'
pubDate: 2025-10-16
updatedDate: 2025-10-16
heroImage: ''
tags: ['mathematics', 'fluid dynamics']
---

流体力学の議論では円筒座標系や極座標系を用いることも多いので、各座標系でのナブラとラプラシアンを求めておこう。
いくつか手法はあるが、連鎖律（Chain Rule）からガリガリ計算するのは心が折れるので、一般座標系におけるナブラの表現を用いる方法で導出してみよう。

## 一般座標系でのナブラ

まず、3次元正規直交座標系（Cartesian Coordinate System）におけるナブラは次のように表される。
ここでは表記上の区別のため、3次元正規直交座標系のパラメタを$\tilde{x}_i$と表すことにする。

$$
\begin{equation}
\nabla = \boldsymbol{e}_i \frac{\partial}{\partial \tilde{x}_i}
\end{equation}
$$

これを一般座標系でも適用できるように一般化して表すと、(2)のようになる。
最低限、(2)--(4)の関係式を受け入れてしまえば、あとは円筒座標系や極座標系の位置ベクトルをパラメタで偏微分することで、各座標系でのナブラが求められる。

$$
\begin{equation}
\nabla = \boldsymbol{g}^i \frac{\partial}{\partial x^i}
\end{equation}
$$

ここで$\boldsymbol{g}_i$は共変基底ベクトル（Covariant Basis Vector）で、$\boldsymbol{g}^i$は反変基底ベクトル（Contravariant Basis Vector）である。
共変基底ベクトルは位置ベクトル$\boldsymbol{r}$をある座標系のパラメタで偏微分したもので、パラメタが微小に変化したときに、位置ベクトルの変化する方向を表す。

$$
\begin{equation}
\boldsymbol{g}_i = \frac{\partial \boldsymbol{r}}{\partial x^i}
\end{equation}
$$

反変基底ベクトルは、共変基底ベクトルと以下のような関係を満たすように定義される。

$$
\begin{equation}
\boldsymbol{g}_i \cdot \boldsymbol{g}^j = \delta_i^j
\end{equation}
$$

(2)の表現が(1)の一般化になっていることは、基底ベクトルの変換をすることで確認できる。
ある共変基底ベクトルから、ほかの共変基底ベクトルに変換する場合、次の関係が成り立つ。

$$
\begin{equation}
\boldsymbol{g}_i = \frac{\partial \bm{r}}{\partial x^i} = \frac{\partial \bar{x}^j}{\partial x^i} \frac{\partial \bm{r}}{\partial \bar{x}^j} = \frac{\partial \bar{x}^j}{\partial x^i} \bar{\bm{g}}_j
\end{equation}
$$

この関係を用いると、反変基底ベクトルに関する変換式も導出できる。

$$
\begin{equation}
\boldsymbol{g}^i = \frac{\partial x^i}{\partial \bar{x}^j} \bar{\boldsymbol{g}}^j
\end{equation}
$$

この関係を用いて、一般座標系のナブラを3次元正規直交座標系の基底に変換すると、よく知られている(1)の表現と一致していることが分かる。

$$
\begin{equation}
\nabla = \boldsymbol{g}^i \frac{\partial}{\partial x^i} = \boldsymbol{e}_j \frac{\partial x^i}{\partial \tilde{x}_j} \frac{\partial}{\partial x^i} = \boldsymbol{e}_j \frac{\partial}{\partial \tilde{x}_j}
\end{equation}
$$

## 円筒座標 / Cylindrical Coordinates

デカルト座標系パラメタは円筒座標系のパラメタを用いると以下のように表される。

$$
\begin{equation}
\boldsymbol{r} = x \boldsymbol{e}_x + y \boldsymbol{e}_y + z \boldsymbol{e}_z
= r \cos \theta\ \boldsymbol{e}_x + r \sin \theta\ \boldsymbol{e}_y + z \boldsymbol{e}_z
\end{equation}
$$

これより共変基底ベクトルを求めると以下のとおり。
これらのベクトルは必ずしも直交しないが、今回は円筒座標系を用いるので、互いに直交する3つのベクトルが得られる。

$$
\begin{align}
&\boldsymbol{g}_r = \frac{\partial \boldsymbol{r}}{\partial r} = \cos \theta \boldsymbol{e}_x + \sin \theta \boldsymbol{e}_y \\
&\boldsymbol{g}_\theta = \frac{\partial \boldsymbol{r}}{\partial \theta} = - r \sin \theta \boldsymbol{e}_x + r \cos \theta \boldsymbol{e}_y \\
&\boldsymbol{g}_z = \frac{\partial \boldsymbol{r}}{\partial z} = \boldsymbol{e}_z
\end{align}
$$

得られた共変基底ベクトルが互いに直交しているので、反変基底ベクトル$(\boldsymbol{g}^r, \boldsymbol{g}^\theta, \boldsymbol{g}^z)$はそれぞれの共変基底ベクトルと同じ方向を向き、長さが逆数になったものになる。
共変基底ベクトルを正規化したものを改めて$(\boldsymbol{e}_r, \boldsymbol{e}_\theta, \boldsymbol{e}_z)$とおくと、次のように円筒座標系での$\nabla$が得られる。

$$
\begin{equation}
\nabla = \boldsymbol{g}^r \frac{\partial}{\partial r} + \boldsymbol{g}^\theta \frac{\partial}{\partial \theta} + \boldsymbol{g}^z \frac{\partial}{\partial z}
= \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_z \frac{\partial}{\partial z}
\end{equation}
$$

円筒座標基底の偏微分を求めて、ナブラの内積を計算すると円筒座標系でのラプラシアンが求められる。

$$
\begin{equation}
\frac{\partial}{\partial \theta} \left[ \begin{array}{c}
\boldsymbol{e}_r \\ \boldsymbol{e}_\theta
\end{array} \right] =
\left[ \begin{array}{cc}
-\sin \theta & \cos \theta \\
- \cos \theta & - \sin \theta
\end{array} \right]
\left[ \begin{array}{c}
\boldsymbol{e}_x \\ \boldsymbol{e}_y
\end{array} \right] =
\left[ \begin{array}{c}
\boldsymbol{e}_\theta \\ - \boldsymbol{e}_r
\end{array} \right]
\end{equation}
$$

$$
\begin{align}
\Delta &= \nabla \cdot \nabla \notag \\
&= \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_z \frac{\partial}{\partial z} \right) \cdot \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_z \frac{\partial}{\partial z} \right) \notag \\
&= \frac{\partial^2}{\partial r^2} + \boldsymbol{e}_\theta \cdot \frac{1}{r} \left( \frac{\partial}{\partial \theta} \boldsymbol{e}_r \right) \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{\partial^2}{\partial z^2} \notag \\
&= \frac{\partial^2}{\partial r^2} + \frac{1}{r} \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{\partial^2}{\partial z^2} \\
\end{align}
$$

## 極座標 / Polar Coordinate

デカルト座標系パラメタは極座標系のパラメタを用いると以下のように表される。

$$
\begin{equation}
\boldsymbol{r} = x \boldsymbol{e}_x + y \boldsymbol{e}_y + z \boldsymbol{e}_z
= r \sin \theta \cos \phi\ \boldsymbol{e}_x + r \sin \theta \sin \phi\ \boldsymbol{e}_y + r \cos \theta \boldsymbol{e}_z
\end{equation}
$$

これより共変基底ベクトルを求めると以下のとおり。

$$
\begin{align}
&\boldsymbol{g}_r = \frac{\partial \boldsymbol{r}}{\partial r} = \sin \theta \cos \phi\ \boldsymbol{e}_x + \sin \theta \sin \phi\ \boldsymbol{e}_y + \cos \theta\ \boldsymbol{e}_z \\
&\boldsymbol{g}_\theta = \frac{\partial \boldsymbol{r}}{\partial \theta} = r \cos \theta \cos \phi\ \boldsymbol{e}_x + r \cos \theta \sin \phi\ \boldsymbol{e}_y - r \sin \theta\ \boldsymbol{e}_z \\
&\boldsymbol{g}_\phi = \frac{\partial \boldsymbol{r}}{\partial \phi} = - r \sin \theta \sin \phi\ \boldsymbol{e}_x + r \sin \theta \cos \phi\ \boldsymbol{e}_y
\end{align}
$$

今回も得られた共変基底ベクトルが互いに直交しているので、反変基底ベクトル$(\boldsymbol{g}^r, \boldsymbol{g}^\theta, \boldsymbol{g}^\phi)$はそれぞれの共変基底ベクトルと同じ方向を向き、長さが逆数になったものになる。
共変基底ベクトルを正規化したものを改めて$(\boldsymbol{e}_r, \boldsymbol{e}_{\theta}, \boldsymbol{e}_{\phi})$とおくと、次のように極座標系での$\nabla$が得られる。

$$
\begin{equation}
\nabla = \boldsymbol{g}^r \frac{\partial}{\partial r} + \boldsymbol{g}^\theta \frac{\partial}{\partial \theta} + \boldsymbol{g}^\phi \frac{\partial}{\partial \phi}
= \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_\phi \frac{1}{r \sin \theta} \frac{\partial}{\partial \phi}
\end{equation}
$$

極座標基底の偏微分を求めて、ナブラの内積を計算すると円筒座標系でのラプラシアンが求められる。

$$
\begin{align}
\frac{\partial}{\partial \theta} \left[ \begin{array}{c}
\boldsymbol{e}_r \\ \boldsymbol{e}_\theta \\ \boldsymbol{e}_\phi
\end{array} \right] =
\left[ \begin{array}{ccc}
\cos \theta \cos \phi & \cos \theta \sin \phi & - \sin \theta \\
- \sin \theta \cos \phi & - \sin \theta \sin \phi & - \cos \theta \\
0 & 0 & 0
\end{array} \right]
\left[ \begin{array}{c}
\boldsymbol{e}_x \\ \boldsymbol{e}_y \\ \boldsymbol{e}_z
\end{array} \right] =
\left[ \begin{array}{c}
\boldsymbol{e}_\theta \\ - \boldsymbol{e}_r \\ 0
\end{array} \right]
\end{align}
$$

$$
\begin{align}
\frac{\partial}{\partial \phi} \left[ \begin{array}{c}
\boldsymbol{e}_r \\ \boldsymbol{e}_\theta \\ \boldsymbol{e}_\phi
\end{array} \right] &=
\left[ \begin{array}{ccc}
- \sin \theta \sin \phi & \sin \theta \cos \phi & 0 \\
- \cos \theta \sin \phi & \cos \theta \cos \phi & 0 \\
- \cos \phi & - \sin \phi & 0
\end{array} \right]
\left[ \begin{array}{c}
\boldsymbol{e}_x \\ \boldsymbol{e}_y \\ \boldsymbol{e}_z
\end{array} \right] \notag \\
&=
\left[ \begin{array}{c}
\sin \theta\ \boldsymbol{e}_\phi \\ \cos \theta\ \boldsymbol{e}_\phi \\ - \sin \theta\ \boldsymbol{e}_r - \cos \theta\ \boldsymbol{e}_\theta
\end{array} \right]
\end{align}
$$

$$
\begin{align*}
\Delta &= \nabla \cdot \nabla \\
&= \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_\phi \frac{1}{r \sin \theta}\frac{\partial}{\partial \phi} \right) \cdot \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_\phi \frac{1}{r \sin \theta}\frac{\partial}{\partial \phi} \right) \\
&= \frac{\partial^2}{\partial r^2} + \frac{2}{r} \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{1}{r^2 \tan \theta} \frac{\partial}{\partial \theta} + \frac{1}{r^2 \sin^2 \theta} \frac{\partial^2}{\partial \phi^2}
\end{align*}
$$
