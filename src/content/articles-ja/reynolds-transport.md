---
title: 'レイノルズの輸送定理と連続の式'
description: '相加的な物理量の積分に関するラグランジュ微分の関係を表すレイノルズの輸送定理(Reynolds Transport Theorem)を導出します。具体的なレイノルズの輸送定理の適用例として連続の式（質量保存の式）を示します。'
pubDate: 2025-10-14
updatedDate: 2025-10-15
heroImage: ''
tags: ['fluid dynamics']
---

Reynoldsの輸送定理は、連続体力学の分野で用いられる定理で、
相加的な物理量を積分した値に関するラグランジュ微分の性質を表す。
この定理自体は、具体的な物理量を指定しないので、一見抽象的で物理的な意味をイメージしにくいが、
密度や運動量、エネルギーなどの物理量に適用することで、質量保存の式や運動方程式、エネルギー保存の式などを導出することができる。

## Reynoldsの輸送定理

ある時刻$t$、位置$(x_1, x_2, x_3)$におけるある物理量が$g(x_1, x_2, x_3, t)$で表されるとする。
この物理量を領域$V(t)$で積分したものを次のように表すことにする。

$$
\begin{equation}
f = \int_{V(t)} g(x_1, x_2, x_3, t) dV
\end{equation}
$$

この物理量$f$のLagrange（物質）微分は次のように表すことが出来る。
$V_0$は基準時刻$t_0$において、注目する物質が占めている領域で、ヤコビアン$J$は微小領域の体積変化率を表す。

$$
\begin{align}
\frac{Df}{Dt} &= \frac{D}{Dt} \left( \int_{V(t)} g(x_1, x_2, x_3, t) dV \right) = \frac{D}{Dt} \left( \int_{V_0} g J dV_0 \right) \notag \\
&= \int_{V_0} \left( \frac{Dg}{Dt} J + g \frac{DJ}{Dt} \right) dV_0
\end{align}
$$

ここで用いたヤコビアン$J$に対応するヤコビ行列は、(3)のように表され、連続体力学の分野では変形勾配テンソル（deformation gradient tensor）$\boldsymbol{F}$と呼ばれる。

$$
\begin{equation}
\bm{F} = F_{ij} \bm{e}_i \otimes \bm{e}_j = \frac{\partial x_i}{\partial X_j} \bm{e}_i \otimes \bm{e}_j
\end{equation}
$$

<!-- 基準時刻$t_0$における物質の位置$\bm{X}$が、時刻$t$における物質の位置$\bm{x}$に移動するとき、
変形勾配テンソルを用いると、$\bm{X}$の近傍のベクトル$d\bm{X}$は、$d\bm{x}$に次のように変換される。

$$
\begin{equation*}
d\bm{x} = \bm{F} \cdot d\bm{X}
\end{equation*}
$$ -->

この変形勾配テンソル（ヤコビ行列）の行列式がヤコビアン$J = \det(\boldsymbol{F})$で、変形前後の微小体積の変化率を表す。
このヤコビアンの時間微分には、以下の関係が成り立つ。

$$
\begin{equation}
\frac{DJ}{Dt} = J \left( \frac{\partial v_1}{\partial x_1} +  \frac{\partial v_2}{\partial x_2} +  \frac{\partial v_3}{\partial x_3} \right)
\end{equation}
$$

この関係を証明するために、いったん気合で3×3の行列式を展開して微分してしまおう。
3次元の場合、ヤコビアンは以下のように展開できる。

$$
\begin{align}
J = F_{11} (F_{22} F_{33} - F_{23} F_{32}) - F_{12} (F_{21} F_{33} - F_{23} F_{31}) + F_{13} (F_{21} F_{32} - F_{22} F_{31})
\end{align}
$$

これを時間微分すると、次のように整理できる。

$$
\begin{align}
\frac{DJ}{Dt} &= \dot{F}_{11} (F_{22} F_{33} - F_{23} F_{32}) - \dot{F}_{12} (F_{21} F_{33} - F_{23} F_{31}) + \dot{F}_{13} (F_{21} F_{32} - F_{22} F_{31}) \notag \\
&+ \dot{F}_{21} (F_{12} F_{33} - F_{13} F_{32}) - \dot{F}_{22} (F_{11} F_{33} - F_{13} F_{31}) + \dot{F}_{23} (F_{11} F_{32} - F_{12} F_{31}) \notag \\
&+ \dot{F}_{31} (F_{12} F_{23} - F_{13} F_{22}) - \dot{F}_{32} (F_{11} F_{23} - F_{13} F_{21}) + \dot{F}_{33} (F_{11} F_{22} - F_{12} F_{21})
\end{align}
$$

当たり前と言えば当たり前なのだが、(6)のように時間微分したパラメタごとにまとめると、係数部分がそのパラメタに関する余因子（cofactor）になっていることがわかる。
なので、余因子行列（adjugate matrix）$\mathrm{adj}~ F$を用いて、次のように簡潔に表すことができる。

$$
\begin{align}
\frac{DJ}{Dt} =& (\mathrm{adj}~ F)_{ji} \dot{F}_{ij}
\end{align}
$$

上式では、パラメタの総和として表したが、行列の演算として表すと、次のようになる。
これはヤコビの公式と呼ばれn次の正方行列に対して成り立つ。

$$
\begin{equation}
\frac{D}{Dt} \mathrm{det}~ \bm{F} = \mathrm{tr} \left( (\mathrm{adj}~ \bm{F})^{\mathrm{T}} \dot{\bm{F}} \right) = \mathrm{det}~ \bm{F} \cdot \mathrm{tr} \left( \bm{F}^{-1} \dot{\bm{F}} \right)
\end{equation}
$$

次に、$F_{ij}$の時間微分について確認しておく。

$$
\begin{align}
\dot{F}_{ij} &= \frac{D}{Dt} \left( \frac{\partial x_i}{\partial X_j} \right)
= \frac{\partial v_i}{\partial X_j}
= \frac{\partial v_i}{\partial x_k} \frac{\partial x_k}{\partial X_j}
= (\nabla v_i)_k F_{kj}
\end{align}
$$

行列の形で表すと、次のようになる。

$$
\begin{equation}
\dot{\bm{F}} = (\nabla \boldsymbol{v}) \bm{F}
= \left[ \begin{array}{ccc}
\frac{\partial v_1}{\partial x_1} & \frac{\partial v_1}{\partial x_2} & \frac{\partial v_1}{\partial x_3} \\
\frac{\partial v_2}{\partial x_1} & \frac{\partial v_2}{\partial x_2} & \frac{\partial v_2}{\partial x_3} \\
\frac{\partial v_3}{\partial x_1} & \frac{\partial v_3}{\partial x_2} & \frac{\partial v_3}{\partial x_3}
\end{array} \right]
\left[ \begin{array}{ccc}
\frac{\partial x_1}{\partial X_1} & \frac{\partial x_1}{\partial X_2} & \frac{\partial x_1}{\partial X_3} \\
\frac{\partial x_2}{\partial X_1} & \frac{\partial x_2}{\partial X_2} & \frac{\partial x_2}{\partial X_3} \\
\frac{\partial x_3}{\partial X_1} & \frac{\partial x_3}{\partial X_2} & \frac{\partial x_3}{\partial X_3}
\end{array} \right]
\end{equation}
$$

これを先ほどのヤコビの公式に代入すると、以下のようになる。
ただし、正方行列のトレースについて、$\mathrm{tr}(AB) = \mathrm{tr}(BA)$が成り立つことを利用した。

$$
\begin{align}
\frac{DJ}{Dt} &= J \cdot \mathrm{tr} \left( \bm{F}^{-1} (\nabla \boldsymbol{v}) \bm{F} \right) = J \cdot \mathrm{tr} (\nabla \boldsymbol{v}) = J \left( \frac{\partial v_1}{\partial x_1} +  \frac{\partial v_2}{\partial x_2} +  \frac{\partial v_3}{\partial x_3} \right)
\end{align}
$$

ヤコビアンの時間微分、および物理量$g$のラグランジュ微分をオイラー微分を用いて表すと、Reynoldsの輸送定理が得られる。

$$
\begin{align}
\frac{Df}{Dt} &= \int_{V_0} \left( \frac{Dg}{Dt} J + g J (\nabla \cdot \bm{v}) \right) dV_0 \notag \\
&= \int_{V_0} \left( \frac{\partial g}{\partial t} + (\bm{v} \cdot \nabla) g + g (\nabla \cdot \bm{v}) \right) J dV_0 \notag \\
&= \int_{V(t)} \left( \frac{\partial g}{\partial t} + \nabla \cdot (g \bm{v}) \right) dV
\end{align}
$$

## 連続の式（質量保存の式）

ここで、Reynoldsの輸送定理の具体的な適用例として、連続の式（質量保存の式）を導出してみよう。
連続の式は流体力学における基礎式の1つで「流体の質量は突然出てきたり，突然消えたりしないよ」というごく自然なことを主張している。

密度$\rho$を注目している物質の領域内で積分すると、その物質の質量$m$が得られる。
この質量（密度の積分値）をラグランジュ微分することを考える場合、積分領域は物質に追従して変化するので、質量変化は必ずゼロになる。

$$
\begin{equation}
\frac{Dm}{Dt} = 0, \quad \text{where} \quad m = \int_{V(t)} \rho dV
\end{equation}
$$

これをReynoldsの輸送定理に適用すると、以下のように表される。

$$
\begin{align}
\int_{V(t)} \left( \frac{\partial \rho}{\partial t} + \nabla \cdot (\rho \bm{v}) \right) dV = 0
\end{align}
$$

この関係は、任意の領域$V(t)$について成り立つので、微小領域についても以下の関係が成り立つ。
これがよく知られている連続の式（質量保存の式）である。

$$
\begin{equation}
\frac{\partial \rho}{\partial t} + \nabla \cdot (\rho \bm{v}) = 0
\end{equation}
$$

特に、圧縮性を考慮しない非圧縮流体の場合、密度は一定$\rho = \mathrm{const}$なので、連続の式は以下のように簡略化される。

$$
\begin{equation}
\nabla \cdot \bm{v} = 0
\end{equation}
$$

## References

1. 久田 俊明, 野口 裕久, "非線形有限要素法の基礎と応用", 丸善, 1996
2. 久田 俊明, "非線形有限要素法のためのテンソル解析の基礎", 丸善 , 1999
3. Karan S. Surana, "Classical Continuum Mechanics, Second Edition", CRC Press, 2022
