---
title: '円筒座標での連続の式とナビエ・ストークス方程式'
description: '管内流れを議論する場合、円筒座標系で表した流体力学の基礎方程式が必要になります。連続の式およびナビエ・ストークス方程式を円筒座標系で表現し、いくつかの簡略化した場合についても確認します。'
pubDate: 2025-10-20
updatedDate: 2025-10-20
heroImage: ''
tags: ['fluid dynamics']
---

管内流れを議論する場合、円筒座標系で表した流体力学の基礎方程式が必要になる。
基本的には、これまでに導出した連続の式およびナビエ・ストークス方程式に対して、円筒座標系のナブラとラプラシアンを代入してやればよい。

## 円筒座標系での連続の式

連続の式は以下のように表すことが出来る[（レイノルズの輸送定理と連続の式）](https://thermocraft.space/ja/articles/reynolds-transport/)。

$$
\begin{equation}
% \label{eq:LandauLifshitzVol6_1.2}
\frac{\partial \rho}{\partial t} + \nabla \cdot (\rho \bm{v}) = 0
\end{equation}
$$

また、円筒座標系でのナブラ$\nabla$は次のように表すことができる[（円筒座標・極座標のナブラとラプラシアン）](https://thermocraft.space/ja/articles/nabla-laplacian/)。

$$
\begin{equation}
% \label{eq:nabla}
\nabla = \bm{e}_r \frac{\partial}{\partial r} + \bm{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \bm{e}_z \frac{\partial}{\partial z}
\end{equation}
$$

円筒座標系での連続の式を得るには、(1)に(2)をそのまま代入してやればよい。

$$
\begin{equation}
\frac{\partial \rho}{\partial t} + \frac{1}{r} \frac{\partial}{\partial r} (r \rho v_r) + \frac{1}{r} \frac{\partial}{\partial \theta} (\rho v_\theta) + \frac{\partial}{\partial z} (\rho v_z) = 0
\end{equation}
$$

## 円筒座標系のナビエ・ストークス方程式

ナビエ・ストークス方程式は以下のように表すことができる[（ナビエ・ストークス方程式の導出）](https://thermocraft.space/ja/articles/navier-stokes/)。
ただし、$\zeta$は体積粘性率（bulk viscosity）、$\eta$せん断粘性率（shear viscosity） である。また今回は、重力などの流体質量に直接働く力はないものとする。

$$
\begin{equation}
% \label{eq:navier-stokes}
\rho \frac{\partial \bm{v}}{\partial t} + \rho \bm{v} \cdot \nabla \bm{v} = - \nabla p + \eta \Delta \bm{v} + \left( \zeta + \frac{1}{3} \eta \right) \nabla (\nabla \cdot \bm{v})
\end{equation}
$$

また、円筒座標系でのラプラシアンは次のように表すことができた[（円筒座標・極座標のナブラとラプラシアン）](https://thermocraft.space/ja/articles/nabla-laplacian/)。

$$
\begin{equation}
% \label{eq:laplacian}
\Delta = \frac{\partial^2}{\partial r^2} + \frac{1}{r} \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{\partial^2}{\partial z^2}
\end{equation}
$$

よって(4)に(2)と(5)を代入すれば、円筒座標系でのナビエ・ストークス方程式が得られる。

$$
\begin{align}
&\rho \frac{\partial}{\partial t} \left( v_r \bm{e}_r + v_\theta \bm{e}_\theta + v_z \bm{e}_z \right) + \rho \left( v_r \frac{\partial}{\partial r} + \frac{v_\theta}{r} \frac{\partial}{\partial \theta} + v_z \frac{\partial}{\partial z} \right) \left( v_r \bm{e}_r + v_\theta \bm{e}_\theta + v_z \bm{e}_z \right) \notag \\
&= - \left( \bm{e}_r \frac{\partial p}{\partial r} + \bm{e}_\theta \frac{1}{r} \frac{\partial p}{\partial \theta} + \bm{e}_z \frac{\partial p}{\partial z} \right) \notag \\
&\quad+ \eta \left( \frac{\partial^2}{\partial r^2} + \frac{1}{r} \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{\partial^2}{\partial z^2} \right) \left( v_r \bm{e}_r + v_\theta \bm{e}_\theta + v_z \bm{e}_z \right) \notag \\
&\quad+ \left( \zeta + \frac{1}{3} \eta \right) \left( \bm{e}_r \frac{\partial}{\partial r} + \bm{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \bm{e}_z \frac{\partial}{\partial z} \right) \left( \frac{\partial v_r}{\partial r} + \frac{v_r}{r} + \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \frac{\partial v_z}{\partial z} \right)
\end{align}
$$

ただし、粘性項に含まれる$(\nabla \cdot \bm{v})$については以下のように展開した。

$$
\begin{align}
&\nabla \cdot \bm{v}
= \left(  \bm{e}_r \frac{\partial}{\partial r} + \bm{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \bm{e}_z \frac{\partial}{\partial z} \right) \cdot \left( v_r \bm{e}_r + v_\theta \bm{e}_\theta + v_z \bm{e}_z \right) \notag \\
&= \bm{e}_r \cdot \frac{\partial}{\partial r} (v_r \bm{e}_r) + \bm{e}_r \cdot \frac{\partial}{\partial r} (v_\theta \bm{e}_\theta) + \bm{e}_z \cdot \frac{\partial}{\partial r} (v_z \bm{e}_z) \notag \\
&\quad+ \bm{e}_\theta \cdot \frac{1}{r} \frac{\partial}{\partial \theta} (v_r \bm{e}_r) + \bm{e}_\theta \cdot \frac{1}{r} \frac{\partial}{\partial \theta} (v_\theta \bm{e}_\theta) + \bm{e}_\theta \cdot \frac{1}{r} \frac{\partial}{\partial \theta} (v_z \bm{e}_z) \notag \\
&\quad+ \bm{e}_z \cdot \frac{\partial}{\partial z} (v_r \bm{e}_r) + \bm{e}_z \cdot \frac{\partial}{\partial z} (v_\theta \bm{e}_\theta) + \bm{e}_z \cdot \frac{\partial}{\partial z} (v_z \bm{e}_z) \notag \\
&=\frac{\partial v_r}{\partial r} (\bm{e}_r \cdot \bm{e}_r) + \frac{\partial v_\theta}{\partial r} (\bm{e}_r \cdot \bm{e}_\theta) + \frac{\partial v_z}{\partial r} (\bm{e}_r \cdot \bm{e}_z) \notag \\
&\quad+ \bm{e}_\theta \cdot \frac{1}{r} \left( \bm{e}_r \frac{\partial v_r}{\partial \theta} + v_r \bm{e}_\theta \right) + \bm{e}_\theta \cdot \frac{1}{r} \left( \bm{e}_\theta \frac{\partial v_\theta}{\partial \theta} - v_\theta \bm{e}_r \right) + \frac{1}{r} \frac{\partial v_z}{\partial \theta} (\bm{e}_\theta \cdot  \bm{e}_z) \notag \\
&\quad+ \frac{\partial v_r}{\partial r} (\bm{e}_z \cdot \bm{e}_r) + \frac{\partial v_\theta}{\partial z} (\bm{e}_z \cdot \bm{e}_\theta) + \frac{\partial v_z}{\partial z} (\bm{e}_z \cdot \bm{e}_z) \notag \\
&=\frac{\partial v_r}{\partial r} + \frac{v_r}{r} + \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \frac{\partial v_z}{\partial z}
\end{align}
$$

これを$r$成分、$\theta$成分、$z$成分に分けて書き下すと以下のようになる。

$$
\begin{align}
&\rho \frac{\partial v_r}{\partial t} + \rho v_r \frac{\partial v_r}{\partial r} + \rho v_\theta \frac{1}{r} \frac{\partial v_r}{\partial \theta} - \rho \frac{v_\theta^2}{r} + \rho v_z \frac{\partial v_r}{\partial z} \notag \\
&\quad= - \frac{\partial p}{\partial r} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_r}{\partial r} \right) + \frac{\eta}{r^2} \frac{\partial^2 v_r}{\partial \theta^2}
- \frac{2\eta}{r^2} \frac{\partial v_\theta}{\partial \theta} - \eta\frac{v_r}{r^2} + \eta \frac{\partial^2 v_r}{\partial z^2} \notag \\
&\hspace{21pt}+ \left( \zeta + \frac{1}{3} \eta \right) \frac{\partial}{\partial r} \left( \frac{\partial v_r}{\partial r} + \frac{v_r}{r} + \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \frac{\partial v_z}{\partial z} \right)
\end{align}
$$

$$
\begin{align}
& \rho \frac{\partial v_\theta}{\partial t} + \rho v_r \frac{\partial v_\theta}{\partial r} + \rho v_\theta \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \rho \frac{v_\theta v_r}{r} + \rho v_z \frac{\partial v_\theta}{\partial z} \notag \\
&\quad= - \frac{1}{r} \frac{\partial p}{\partial \theta} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_\theta}{\partial r} \right) + \frac{\eta}{r^2} \frac{\partial^2 v_\theta}{\partial \theta^2}
+ \frac{2\eta}{r^2} \frac{\partial v_r}{\partial \theta} - \eta \frac{v_\theta}{r^2} + \eta \frac{\partial^2 v_\theta}{\partial z^2}\notag \\
&\hspace{21pt}+ \left( \zeta + \frac{1}{3} \eta \right) \frac{1}{r} \frac{\partial}{\partial \theta} \left( \frac{\partial v_r}{\partial r} + \frac{v_r}{r} + \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \frac{\partial v_z}{\partial z} \right)
\end{align}
$$

$$
\begin{align}
& \rho \frac{\partial v_z}{\partial t} + \rho v_r \frac{\partial v_z}{\partial r} + \rho v_\theta \frac{1}{r} \frac{\partial v_z}{\partial \theta} + \rho v_z \frac{\partial v_z}{\partial z} \notag \\
&\quad= - \frac{\partial p}{\partial z} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_z}{\partial r} \right) + \frac{\eta}{r^2} \frac{\partial^2 v_z}{\partial \theta^2} + \eta \frac{\partial^2 v_z}{\partial z^2} \notag \\
&\hspace{21pt}+ \left( \zeta + \frac{1}{3} \eta \right) \frac{\partial}{\partial z} \left( \frac{\partial v_r}{\partial r} + \frac{v_r}{r} + \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \frac{\partial v_z}{\partial z} \right)
\end{align}
$$

## 軸対称・非圧縮などの場合について

ナビエ・ストークス方程式をそのままの形で扱うのは難しいので、扱う問題に応じて適切な簡略化がしばしば行われる。
ここでは、いくつかの仮定をおいた場合、連続の式とナビエ・ストークス方程式がどう変形されるかを確認しておく。

旋回のない軸対称問題$v_\theta = 0$であれば、連続の式とナビエ・ストークス方程式は以下のように簡略化できる。

$$
\begin{equation}
\frac{\partial \rho}{\partial t} + \frac{1}{r} \frac{\partial}{\partial r} (r \rho v_r) + \frac{\partial}{\partial z} (\rho v_z) = 0
\end{equation}
$$

$$
\begin{align}
\rho \frac{\partial v_r}{\partial t} + \rho v_r \frac{\partial v_r}{\partial r} + \rho v_z \frac{\partial v_r}{\partial z}
=& - \frac{\partial p}{\partial r} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_r}{\partial r} \right) - \eta \frac{v_r}{r^2} + \eta \frac{\partial^2 v_r}{\partial z^2} \notag \\
&+ \left( \zeta + \frac{1}{3} \eta \right) \frac{\partial}{\partial r} \left( \frac{\partial v_r}{\partial r} + \frac{v_r}{r} + \frac{\partial v_z}{\partial z} \right)
\end{align}
$$

$$
\begin{align}
\rho \frac{\partial v_z}{\partial t} + \rho v_r \frac{\partial v_z}{\partial r} + \rho v_z \frac{\partial v_z}{\partial z}
=& - \frac{\partial p}{\partial z} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_z}{\partial r} \right) + \eta \frac{\partial^2 v_z}{\partial z^2} \notag \\
&+ \left( \zeta + \frac{1}{3} \eta \right) \frac{\partial}{\partial z} \left( \frac{\partial v_r}{\partial r} + \frac{v_r}{r} + \frac{\partial v_z}{\partial z} \right)
\end{align}
$$

さらに半径方向にも一様である場合、一次元流れとして以下のように簡略化できる。

$$
\begin{equation}
\frac{\partial \rho}{\partial t} + \frac{\partial}{\partial z} (\rho v_z) = 0
\end{equation}
$$

$$
\begin{equation}
\rho \frac{\partial v_z}{\partial t} + \rho v_z \frac{\partial v_z}{\partial z}
= - \frac{\partial p}{\partial z} + \left( \zeta + \frac{4}{3} \eta \right) \frac{\partial^2 v_z}{\partial z^2}
\end{equation}
$$

圧縮性を考慮しない$\rho = \mathrm{const}$の場合、連続の式とナビエ・ストークス方程式は以下のように表される。
このとき、粘性係数は$\eta$のみとなり、動粘性係数$\nu = \eta/\rho$を用いることも多い。

$$
\begin{equation}
\frac{1}{r} \frac{\partial}{\partial r} (r v_r) + \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \frac{\partial v_z}{\partial z} = 0
\end{equation}
$$

$$
\begin{align}
&\rho \frac{\partial v_r}{\partial t} + \rho v_r \frac{\partial v_r}{\partial r} + \rho v_\theta \frac{1}{r} \frac{\partial v_r}{\partial \theta} - \rho \frac{v_\theta^2}{r} + \rho v_z \frac{\partial v_r}{\partial z} \notag \\
&\quad= - \frac{\partial p}{\partial r} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_r}{\partial r} \right) + \frac{\eta}{r^2} \frac{\partial^2 v_r}{\partial \theta^2}
- \frac{2\eta}{r^2} \frac{\partial v_\theta}{\partial \theta} - \eta\frac{v_r}{r^2} + \eta \frac{\partial^2 v_r}{\partial z^2}
\end{align}
$$

$$
\begin{align}
& \rho \frac{\partial v_\theta}{\partial t} + \rho v_r \frac{\partial v_\theta}{\partial r} + \rho v_\theta \frac{1}{r} \frac{\partial v_\theta}{\partial \theta} + \rho \frac{v_\theta v_r}{r} + \rho v_z \frac{\partial v_\theta}{\partial z} \notag \\
&\quad= - \frac{1}{r} \frac{\partial p}{\partial \theta} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_\theta}{\partial r} \right) + \frac{\eta}{r^2} \frac{\partial^2 v_\theta}{\partial \theta^2}
+ \frac{2\eta}{r^2} \frac{\partial v_r}{\partial \theta} - \eta \frac{v_\theta}{r^2} + \eta \frac{\partial^2 v_\theta}{\partial z^2}
\end{align}
$$

$$
\begin{align}
& \rho \frac{\partial v_z}{\partial t} + \rho v_r \frac{\partial v_z}{\partial r} + \rho v_\theta \frac{1}{r} \frac{\partial v_z}{\partial \theta} + \rho v_z \frac{\partial v_z}{\partial z} \notag \\
&\quad= - \frac{\partial p}{\partial z} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_z}{\partial r} \right) + \frac{\eta}{r^2} \frac{\partial^2 v_z}{\partial \theta^2} + \eta \frac{\partial^2 v_z}{\partial z^2}
\end{align}
$$

旋回のない軸対称問題$v_\theta = 0$であれば、以下のように簡略化できる。

$$
\begin{equation}
\frac{1}{r} \frac{\partial}{\partial r} (r v_r) + \frac{\partial v_z}{\partial z} = 0
\end{equation}
$$

$$
\begin{align}
&\rho \frac{\partial v_r}{\partial t} + \rho v_r \frac{\partial v_r}{\partial r} + \rho v_z \frac{\partial v_r}{\partial z}
= - \frac{\partial p}{\partial r} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_r}{\partial r} \right) - \eta\frac{v_r}{r^2} + \eta \frac{\partial^2 v_r}{\partial z^2}
\end{align}
$$

$$
\begin{align}
\rho \frac{\partial v_z}{\partial t} + \rho v_r \frac{\partial v_z}{\partial r} + \rho v_z \frac{\partial v_z}{\partial z}
= - \frac{\partial p}{\partial z} + \frac{\eta}{r} \frac{\partial}{\partial r} \left( r \frac{\partial v_z}{\partial r} \right) + \eta \frac{\partial^2 v_z}{\partial z^2}
\end{align}
$$
