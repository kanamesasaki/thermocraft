---
title: 'ナビエ・ストークス方程式の導出'
description: '流体力学の基礎となるナビエ・ストークス方程式（Navier-Stokes Equations）の導出過程を、レイノルズの輸送定理から解説します。連続体の運動方程式を起点に、どのような構成則を設定することで、粘性と圧縮性が導入されるかを考察します。'
pubDate: 2025-10-15
updatedDate: 2025-10-15
heroImage: ''
tags: ['fluid dynamics']
---

流体力学の基礎方程式を導出する場合、弾性力学と対応するような形で理解しておくと見通しがよい。どちらも同じ連続体の運動量の式（運動方程式）からスタートして、

- ラグランジュ（物質）微分を用いるか、オイラー（空間）微分を用いるか
- 物質の物理的特性（応力と変位・速度の関係）を表す構成則をどう仮定するか

によって、異なる物理考察が生まれてくる。このことを意識しつつナビエ・ストークス方程式（Navier-Stokes Equations）を導出するところまでを見ていこう。

## Reynoldsの輸送定理

運動方程式について考える前に、準備としてReynoldsの輸送定理を導出しておこう。
Reynoldsの輸送定理は、相加的な物理量に関するラグランジュ微分の公式で、具体的に質量、運動量、エネルギーなどに適用することで、それぞれの保存則を導出することができる。

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

ここで、Reynoldsの輸送定理の具体的な適用例として、質量保存の式を導出してみよう。
物質の密度$\rho$の積分を、ラグランジュ微分することを考えると、質量変化がゼロであることから、以下の関係が成り立つ。

$$
\begin{align}
\int_{V(t)} \left( \frac{\partial \rho}{\partial t} + \nabla \cdot (\rho \bm{v}) \right) dV = 0
\end{align}
$$

この関係は、任意の領域$V(t)$について成り立つので、微小領域についても以下の関係が成り立つ。

$$
\begin{equation}
\frac{\partial \rho}{\partial t} + \nabla \cdot (\rho \bm{v}) = 0
\end{equation}
$$

## コーシーの第一運動法則

物体に作用する力として物体力$\rho \boldsymbol{f}$と表面力$\boldsymbol{t}$を考慮して、運動量の式を立てる。

$$
\begin{equation}
% \label{eq:Hisada1992_4.13}
\frac{D}{Dt} \left( \int \rho \boldsymbol{v} dV \right) = \int \rho \boldsymbol{f} dV + \int \boldsymbol{t} ds
\end{equation}
$$

運動量のラグランジュ微分を、Reynoldsの輸送定理および質量保存の式を用いて以下のように書き直す。

$$
\begin{align}
&\frac{D}{Dt} \left( \int \rho v_i dV \right) = \int \left( \rho \frac{\partial v_i}{\partial t} + v_i \frac{\partial \rho}{\partial t} + \nabla \cdot (\rho v_i \boldsymbol{v}) \right) dV \notag \\
&= \int \left( \rho \frac{\partial v_i}{\partial t} + v_i \frac{\partial \rho}{\partial t} + v_i \nabla \cdot (\rho \bm{v}) + \rho \bm{v} \cdot \nabla v_i \right) dV \notag \\
&= \int \left( \rho \frac{\partial v_i}{\partial t} + \rho \bm{v} \cdot \nabla v_i \right) dV
\end{align}
$$

被積分関数の中身がオイラー微分の形になっていることに注意すると、これをラグランジュ微分に戻すことができる。

$$
\begin{align}
\frac{D}{Dt} \left( \int \rho v_i dV \right) = \int \rho \frac{Dv_i}{Dt} dV
\end{align}
$$

$\frac{D\bm{v}}{Dt} = \bm{a}$と表せば、運動量の式は以下のように変形できる。
ただし、表面力$\boldsymbol{t}$はCauchy応力テンソル$\boldsymbol{T}$を用いて、$\boldsymbol{t} = \boldsymbol{T} \cdot \boldsymbol{n}$と表され、発散定理を用いて面積分を体積分に変換した。

$$
\begin{equation}
\int \rho (\boldsymbol{a} - \boldsymbol{f}) dV = \int \mathrm{div}~ \boldsymbol{T} dV
\end{equation}
$$

これが物体の任意の一部分について成り立つので、Cauchy's First Law of Motionが(19)のように得られる。
この式は、ラグランジュ微分を用いた弾性力学のための運動方程式であり、注目する物質の変形を追いかけていく形で表されている。

$$
\begin{equation}
% \label{Hisada1992_4.28}
\rho \boldsymbol{a} = \mathrm{div}~ \boldsymbol{T} + \rho \boldsymbol{f}
\end{equation}
$$

一方で、先ほど出てきたオイラー微分の形を用いると、ある領域に注目した、流体力学における運動方程式が得られる。

$$
\begin{equation}
% \label{Hisada1992_4.29}
\rho \frac{\partial \boldsymbol{v}}{\partial t} + \rho \boldsymbol{v} \cdot \nabla \boldsymbol{v} = \mathrm{div}~ \boldsymbol{T} + \rho \boldsymbol{f}
\end{equation}
$$

## 構成則

次に重要なのが物質の物理特性を表す構成則をどう仮定するかだ。これによって応力がどう発生するか、つまり具体的な$T$の形が決まる。
オイラー方程式（Equation of Motion for Ideal Fluid）を求める際には、表面力を発生させる場として圧力のスカラー場を考える。この場合、微小表面には面に対して垂直な方向にのみ表面力が働く。
このため、オイラー方程式では流体の圧縮は考慮されるが、流体内の剪断応力に由来する粘性の影響は考慮されない。
ここでは、剪断も含めた表面力を発生させる場として、応力テンソルによって表されるテンソル場を考える。Cauchy応力テンソル$\boldsymbol{T}$は慣性デカルト座標系で以下のように表す。

$$
\begin{equation}
\boldsymbol{T} = T_{ij} \boldsymbol{e}_i \otimes \boldsymbol{e}_j
\end{equation}
$$

弾性力学では、物質の変形や変形履歴に応じて物質内部に発生する応力を応力テンソルとして表し、これは物理現象をもとに仮定される。最も単純なものの例として、線形等方弾性体の構成式のHookeの法則（要するに力に比例して伸びたり縮んだりする）がある。
流体を扱う場合、流体の速度勾配に応じた応力が発生すると考える。速度勾配テンソル（velocity gradient tensor）$\boldsymbol{L}$は名前の通り物質の速度勾配を表し、次のように物質の位置の差を速度差に変換する。

$$
\begin{equation}
d\boldsymbol{v} = \boldsymbol{L} \cdot d\boldsymbol{x}
\end{equation}
$$

これより、具体的には以下のように表される。

$$
\begin{equation}
\boldsymbol{L} = \boldsymbol{v} \otimes \nabla = v_i \boldsymbol{e}_i \otimes \frac{\partial}{\partial x_j} \boldsymbol{e}_j = \frac{\partial v_i}{\partial x_j} \boldsymbol{e}_i \otimes \boldsymbol{e}_j = L_{ij} \boldsymbol{e}_i \otimes \boldsymbol{e}_j
\end{equation}
$$

これを対称成分の変形速度テンソル（deformation rate tensor or strain rate tensor）$\boldsymbol{D}$と反対称成分のスピンテンソル（spin tensor or rotation rate tensor）$\boldsymbol{W}$に、$\boldsymbol{L} = \boldsymbol{D} + \boldsymbol{W}$となるよう分離する。それぞれ以下のように表される。

$$
\begin{equation}
\boldsymbol{D} = \frac{1}{2} \left( \frac{\partial v_i}{\partial x_j} + \frac{\partial v_j}{\partial x_i} \right) \boldsymbol{e}_i \otimes \boldsymbol{e}_j, \quad
\boldsymbol{W} = \frac{1}{2} \left( \frac{\partial v_i}{\partial x_j} - \frac{\partial v_j}{\partial x_i} \right) \boldsymbol{e}_i \otimes \boldsymbol{e}_j
\end{equation}
$$

スピンテンソルはある軸周りでの回転を表すもので、このような渦流れは応力を生じないと考える。これよりStokes流体の構成式として以下のように仮定する。

$$
\begin{equation}
% \label{Hisada1992_5.119}
\boldsymbol{T} = p \boldsymbol{I} + \boldsymbol{f} (\boldsymbol{D})
\end{equation}
$$

$p$は圧力で、$\boldsymbol{f} (\boldsymbol{D})$は変形速度テンソル$\boldsymbol{D}$を変数とし、$\boldsymbol{f} (\boldsymbol{O})= 0$を満たすテンソル値関数である。特に$\boldsymbol{f} (\boldsymbol{D})$が$\boldsymbol{D}$について線型であるようなものをNewton流体（Newtonian Fluid）と呼び、次のように表す。

$$
\begin{equation}
% \label{Hisada1992_5.122}
\boldsymbol{T} = \left\{ -p + \left( \kappa - \frac{2}{3} \mu \right) \mathrm{tr}~\boldsymbol{D} \right\} \boldsymbol{I} + 2 \mu \boldsymbol{D}
\end{equation}
$$

ここで$\kappa$は体積粘性率（bulk viscosity, bulk modulus or modulus of compression）、$\mu$せん断粘性率（shear viscosity, shear modulus or modulus of rigidity）と呼ばれる。流体は等方で、応力テンソルの対角項は圧力$p$と圧縮方向の粘性$\kappa (\mathrm{tr}~\boldsymbol{D})$によって表される。$- \frac{2}{3} \mu (\mathrm{tr}~\boldsymbol{D}) \boldsymbol{I}$の項は$2\mu \boldsymbol{D}$によって生じる対角項を打ち消すために挿入されている。非圧縮性を仮定すると、応力テンソルは次のように簡略化される。

$$
\begin{equation}
\mathrm{div}\ \boldsymbol{v} = \frac{\partial v_i}{\partial x_i}= \mathrm{tr}~\boldsymbol{L} = \mathrm{tr}~\boldsymbol{D} = 0
\end{equation}
$$

$$
\begin{equation}
% \label{Hisada1992_5.124}
\boldsymbol{T} = -p \boldsymbol{I} + 2 \mu \boldsymbol{D}
\end{equation}
$$

## ナビエ・ストークス方程式

オイラー微分を使った運動方程式(20)に、圧縮性を考慮したNewton流体の構成式(26)適用すると、Navier-Stokes方程式が得られる。

$$
\begin{equation}
\rho \frac{\partial \boldsymbol{v}}{\partial t} + \rho \boldsymbol{v} \cdot \nabla \boldsymbol{v} = - \nabla p + \mu \Delta \boldsymbol{v} + \left( \kappa + \frac{1}{3} \mu \right) \nabla (\nabla \cdot \boldsymbol{v}) + \rho \boldsymbol{f}
\end{equation}
$$

もちろん以下のように書いてもよい。

$$
\begin{equation}
% \label{eq:LandauLifshitzVol6_15.6}
\rho \left[ \frac{\partial \boldsymbol{v}}{\partial t} + (\boldsymbol{v} \cdot \mathrm{grad}\ \boldsymbol{v}) \right]
= - \mathrm{grad}\ p + \mu \Delta \boldsymbol{v} + \left(\kappa + \frac{1}{3} \mu \right) \mathrm{grad}\ \mathrm{div}\ \boldsymbol{v} + \rho \boldsymbol{f}
\end{equation}
$$

ただし、応力テンソルのdivergenceは次のように整理した。

$$
\begin{align*}
&\nabla \cdot \boldsymbol{T} = \boldsymbol{e}_l \frac{\partial}{\partial x_l} \cdot \left[ \left\{ -p + \left( \kappa - \frac{2}{3} \mu \right) \mathrm{tr}~\boldsymbol{D} \right\} \boldsymbol{I} + 2 \mu \boldsymbol{D} \right] \\
&= - \boldsymbol{e}_l \frac{\partial p}{\partial x_l} \cdot \delta_{mn} (\boldsymbol{e}_m \otimes \boldsymbol{e}_n) + \boldsymbol{e}_l \frac{\partial}{\partial x_l} \cdot \delta_{mn} \left( \kappa - \frac{2}{3} \mu \right) D_{kk} (\boldsymbol{e}_m \otimes \boldsymbol{e}_n) \\
&\hspace{12pt}+ \boldsymbol{e}_l \frac{\partial}{\partial x_l} \cdot 2\mu D_{ij} (\boldsymbol{e}_i \otimes \boldsymbol{e}_j) \\
&= - \delta_{mn} \frac{\partial p}{\partial x_l} (\boldsymbol{e}_l \cdot \boldsymbol{e}_m) \boldsymbol{e}_n + \delta_{mn} \left( \kappa - \frac{2}{3} \mu \right) \frac{\partial D_{kk}}{\partial x_l} (\boldsymbol{e}_l \cdot \boldsymbol{e}_m) \boldsymbol{e}_n + 2 \mu \frac{\partial D_{ij}}{\partial x_l} (\boldsymbol{e}_l \cdot \boldsymbol{e}_i) \boldsymbol{e}_j \\
&= - \frac{\partial p}{\partial x_n} \boldsymbol{e}_n + \left( \kappa - \frac{2}{3} \mu \right) \frac{\partial D_{kk}}{\partial x_n} \boldsymbol{e}_n + 2 \mu \frac{\partial D_{ij}}{\partial x_i} \boldsymbol{e}_j \\
&= - \frac{\partial p}{\partial x_n} \boldsymbol{e}_n + \left( \kappa - \frac{2}{3} \mu \right) \frac{\partial}{\partial x_n} \left( \frac{\partial v_k}{\partial x_k} \right) \boldsymbol{e}_n + \mu \frac{\partial}{\partial x_i} \left( \frac{\partial v_i}{\partial x_j} + \frac{\partial v_j}{\partial x_i} \right) \boldsymbol{e}_j \\
&= - \frac{\partial p}{\partial x_n} \boldsymbol{e}_n + \left( \kappa + \frac{1}{3} \mu \right) \frac{\partial}{\partial x_n} \left( \frac{\partial v_k}{\partial x_k} \right) \boldsymbol{e}_n + \mu \frac{\partial^2 v_j}{\partial {x_i}^2} \boldsymbol{e}_j \\
&= - \nabla p + \mu \Delta \boldsymbol{v} + \left( \kappa + \frac{1}{3} \mu \right) \nabla (\nabla \cdot \boldsymbol{v})
\end{align*}
$$

非圧縮を仮定した場合$\nabla \cdot \boldsymbol{v}=0$であり、運動方程式は簡単な形に書き換えられる。

$$
\begin{equation}
% \label{eq:LandauLifshitzVol6_15.7}
\rho \left[ \frac{\partial \boldsymbol{v}}{\partial t} + (\boldsymbol{v} \cdot \nabla \boldsymbol{v}) \right] = - \nabla p + \mu \Delta \boldsymbol{v} + \rho \boldsymbol{f}
\end{equation}
$$

このとき、粘性係数は$\mu$のみで、動粘性係数$\nu = \mu / \rho$が代わりに用いられることも多い。

## References

1. 久田 俊明, 野口 裕久, "非線形有限要素法の基礎と応用", 丸善, 1996
2. 久田 俊明, "非線形有限要素法のためのテンソル解析の基礎", 丸善 , 1999
3. Karan S. Surana, "Classical Continuum Mechanics, Second Edition", CRC Press, 2022
