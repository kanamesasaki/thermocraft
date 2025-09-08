---
title: '標準大気モデル（U.S. Standard Atmosphere 1976、ISA、ICAO）'
description: '地球大気の温度、圧力等の高度による変化を表すモデルとしてよく知られている、U.S. Standard Atmosphere 1976、ISA（国際標準大気）、ICAO Standard Atmosphereを紹介し、各種パラメタの計算方法について解説します。'
pubDate: 2025-09-08
updatedDate: 2025-09-08
heroImage: ''
tags: ['thermal']
---

<!-- Using the following tool, you can calculate the temperature and pressure at a given altitude based on the U.S. Standard Atmosphere 1976 model.
The model accepts altitude ranges from sea level to 86 km. -->

以下のツールを用いると、U.S. Standard Atmosphere 1976に基づいて、指定した高度における温度、圧力をはじめとした各種パラメタを計算することが出来ます。

<!-- markdownlint-disable MD033 MD045 -->
<div align="center">
  <iframe
    src="/widgets/atmosphere-calculator.html"
    title="U.S. Standard Atmosphere 1976 Calculator"
    width="100%"
    height="470"
    loading="lazy"
    style="max-width: 720px; width: 100%; border:1px solid #ddd; border-radius:8px; background:#fff;">
  </iframe>
</div>
<!-- markdownlint-enable MD033 MD045 -->

## 大気モデルの種類

<!-- The Standard Atmosphere is a model that describes the variation of atmospheric properties such as temperature, pressure, and density with altitude.
The following standards are the commonly used in various fields: -->

標準大気モデルは、温度、圧力などの大気の性質が高度によってどのように変化するかを表すもので、
いくつかの代表的なモデルが存在する。

- **International Standard Atmosphere (ISA)** [[1]](#references)
- **U.S. Standard Atmosphere 1976** [[2]](#references)
- **ICAO STANDARD ATMOSPHERE** [[3]](#references)

<!-- The temperature variation with respect to altitude is basically the same among these standards.
The atmosphere is divided into several layers, and the temperature variation is represented by a liner function or a constant value in each layer. -->

ISA（国際標準大気）、ICAO Standard Atmosphereはそれぞれ、ISO（International Organization for Standardization）およびICAO（International Civil Aviation Organization）によって定められた標準大気モデルで、おおよそ地表から高度80kmまでの範囲での大気の性質を定義している。
U.S. Standard Atmosphere 1976は、NOAA（National Oceanic and Atmospheric Administration）、NASA（National Aeronautics and Space Administration）、USAF（United States Air Force）によって定められたもので、地表から高度1000kmまでの範囲での大気の性質を定義しており、
特に地表から86kmまでの範囲では、U.S. Standard Atmosphere 1976の定義はISAおよびICAO Standard Atmosphereとほぼ同じである。

## Geopotential Height（ジオポテンシャル高度）

いずれの大気モデルを参照する場合でも、Geopotential Height（ジオポテンシャル高度）を用いて高度を表すことが一般的なので、まずはその定義について確認しておこう。
まず、重力ポテンシャル$\Phi(Z)$が定義されているものとする。このとき、以下の関係が成り立つ。

$$
\begin{equation}
\Phi(Z) = \int_{0}^{Z} g(Z) ~dZ
\end{equation}
$$

ここで、Geopotential Altitudeを、次のように定義する。
本来、重力加速度$g(Z)$は高度によって変化するため、重力ポテンシャルの増加も一定ではないが、
重力ポテンシャルの変化が、高度パラメタの変化に一定の値$g_0$で比例するように、高度を少しずつ押しつぶすように変換したものがGeopotential Heightである。
これを用いることで、後で圧力の計算を行う際に、高度依存のパラメタが少なくなり評価が容易になる。

$$
\begin{equation}
H = \frac{\Phi(Z)}{g_0}
\end{equation}
$$

重力の高度による変化は、以下のように表される。

$$
\begin{equation}
g(Z) = g_0 \left( \frac{r_0}{r_0 + Z} \right)^2
\end{equation}
$$

これらの式より、Geopotential Height $H$とGeometric Height $Z$の関係は以下のように表される。

$$
\begin{equation}
H = \int_{0}^{Z} \left( \frac{r_0}{r_0 + Z} \right)^2 ~dZ = \frac{r_0 Z}{r_0 + Z}
\end{equation}
$$

$$
\begin{equation}
Z = \frac{r_0 H}{r_0 - H}
\end{equation}
$$

## Molecular Scale Temperature

Molecular Scale Temperatureとは、気体の平均分子量$M$が高度によって変化する場合に、温度を補正するための概念である。
気体の状態方程式を用いる際に適用することで、平均分子量が高度によって変化する場合でも、地表付近の平均分子量$M_0$を用いて計算を行うことが出来る。

$$
\begin{equation}
T_M = T \frac{M_0}{M}
\end{equation}
$$

ただし、U.S. Standard Atmosphere 1976では、地表から高度80kmまでの範囲で、平均分子量は一定値$M_0$であると仮定しており、それ以上の高度では、平均分子量が高度によって変化するものとしている。
そのため、地表から高度80kmまでの範囲では、Molecular Scale Temperature $T_M$は通常の温度$T$と同じものと考えてよい。
ISAおよびICAO Standard Atmosphereでは、モデルの適用範囲を高度80kmまでとしており、そもそもMolecular Scale Temperatureの考え方を導入していない。

## 高度による温度変化

これらの標準大気による定義では、地表からの高度に応じて、大気はいくつかの層に分けられ、各層ごとに温度の高度変化が線形または一定で表される。
Table 1に、U.S. Standard Atmosphere 1976による各層の温度変化を示す。
高度はGeopotential Heightで表されており、最大高度の84.852kmは、Geometric Heightの高度86kmに相当する。
この記事では言及できていないが、U.S. Standard Atmosphere 1976では、高度86kmから高度1000kmまでの範囲で大気の性質についても定義されている。

_Table 1: Temperature Variation in U.S. Standard Atmosphere 1976_

| Layer | Geopotential height $H$ [km] | Temperature gradient $\frac{dT_M}{dH}$ [K/km] |
| ----- | ---------------------------- | --------------------------------------------- |
| 0     | 0                            | -6.5                                          |
| 1     | 11                           | 0                                             |
| 2     | 20                           | 1.0                                           |
| 3     | 32                           | 2.8                                           |
| 4     | 47                           | 0                                             |
| 5     | 51                           | -2.8                                          |
| 6     | 71                           | -2.0                                          |
| 7     | 84.852                       | 0                                             |

同様に、ISA、ICAO Standard Atmosphereにおける各層の温度変化をTable 2に示す。
地表付近および高度80km付近での適用範囲に違いがあるものの、基本的な温度変化は同じである。

_Table 2: Temperature Variation in ISA and ICAO Standard Atmosphere_

| Geopotential height (km) | Temperature (K)          | Temperature gradient (K/km) |
| ------------------------ | ------------------------ | --------------------------- |
| -2.00 (-5.00 for ICAO)   | 301.15 (320.65 for ICAO) | -6.5                        |
| 0.00                     | 288.15                   | -6.5                        |
| 11.00                    | 216.65                   | 0                           |
| 20.00                    | 216.65                   | 1.0                         |
| 32.00                    | 228.65                   | 2.8                         |
| 47.00                    | 270.65                   | 0                           |
| 51.00                    | 270.65                   | -2.8                        |
| 71.00                    | 214.65                   | -2.0                        |
| 80.00                    | 196.65                   |                             |

## 高度による圧力変化

圧力の高度による変化は、先ほど示した温度変化のモデルと、静水圧平衡（hydrostatic equilibrium）の仮定を用いて推定することが出来る。
静水圧平衡とは、Figure 1に示すように、微小な高度変化による圧力変化が、その領域にある大気にかかる重力による力と釣り合っている状態を指す。
式で表すと以下のようになる。

$$
\begin{equation}
dP = -\rho g(Z) ~dZ
\end{equation}
$$

また、Geopotential Heightを用いると、以下のように書き換えることができる。

$$
\begin{equation}
dP = -\rho g_0 ~dH
\end{equation}
$$

![standard-atmosphere-1](../figures/standard-atmosphere-1.svg)
_Figure 1: Hydrostatic Equilibrium._

理想気体の状態方程式が成り立つことを仮定すれば、以下の関係が成り立つ。

$$
\begin{equation}
P = \rho \frac{R^*}{M} T
\end{equation}
$$

これをMolecular Scale Temperatureを用いて書き換えると、以下のようになる。

$$
\begin{equation}
P = \rho \frac{R^*}{M_0} T_M
\end{equation}
$$

Geopotential Heightを用いた静水圧平衡の式と、Molecular Scale Temperatureを用いた気体の状態方程式から、以下の式が得られる。

$$
\begin{equation}
\frac{dP}{P} = -\frac{M_0g_0}{R^*T_M} ~dH
\end{equation}
$$

この式を地表から高度$H$まで積分すれば、圧力を温度の関数として表すことが出来る。

$$
\begin{equation}
\ln P - \ln P_0 = -\frac{M_0g_0}{R^*} \int_{0}^{H} \frac{1}{T_M} ~dH
\end{equation}
$$

温度の高度変化は各層で線形または一定であるため、各層ごとに積分を行い、順番に足し合わせていくことで、任意の高度における圧力を求めることが出来る。

温度変化が線形の場合の積分結果は次のように表される。

$$
\begin{align}
\ln P - \ln P_i &= -\frac{M_0g_0}{R^*} \left[ \frac{1}{L_i} \ln (L_i(H- H_i) + T_i) \right]_{H_i}^{H} \notag \\
&= -\frac{M_0g_0}{R^*L_i} \Big\{ \ln(L_i(H- H_i) + T_i) - \ln T_i \Big\} \notag \\
&= -\frac{M_0g_0}{R^*L_i} \ln \left( \frac{L_i(H- H_i) + T_i}{T_i} \right)
\end{align}
$$

$$
\begin{align}
P &= P_i \left( \frac{L_i(H- H_i) + T_i}{T_i} \right)^{-\frac{M_0 g_0}{R^* L_i}}
\end{align}
$$

温度一定の場合の積分結果は次のように表される。

$$
\begin{align}
\ln P - \ln P_i &= -\frac{M_0g_0}{R^*} \left[ \frac{H}{T_i} \right]_{H_i}^{H} = -\frac{M_0g_0}{R^*T_i} (H - H_i)
\end{align}
$$

$$
\begin{align}
P &= P_i \exp\left\{ -\frac{M_0 g_0}{R^* T_i} (H - H_i) \right\}
\end{align}
$$

## その他のパラメタ

密度や動粘性係数など、その他のパラメタについても、温度と圧力から計算することが出来る。
密度は理想気体の状態方程式から以下のように求められる。

$$
\begin{equation}
\rho = \frac{P M_0}{R^* T_M}
\end{equation}
$$

粘性係数（dynamic viscosity）は、以下のように求められる。
ここで用いられている定数$\beta$と$S$は、実験に基づく経験的な定数であり、
以下の式も高度86kmを超える領域では有効ではないとされている。

$$
\begin{equation}
\mu = \frac{\beta T^{\frac{3}{2}}}{T + S}
\end{equation}
$$

粘性係数と密度から、動粘性係数（kinematic viscosity）を以下のように求められる。

$$
\begin{equation}
\nu = \frac{\mu}{\rho}
\end{equation}
$$

大気の熱伝導率$k$は、以下のように求められる。

$$
\begin{equation}
k = \frac{2.64638 \times 10^{-3} T^\frac{3}{2}}{T + 245.4 \times 10^{-\frac{12}{T}}}
\end{equation}
$$

<!-- ## High-Altitude Extension of the U.S. Standard Atmosphere 1976

U.S. Standard Atmosphere 1976 では、86 km から 1000 km までの大気についてもモデル化が行われている点が、他の標準大気と異なる。 -->

## Nomenclature

| Symbol  | Description              | Value                   |
| ------- | ------------------------ | ----------------------- |
| $R^*$   | Gas constant             | 8.31432 J/(mol·K)       |
| $M_0$   | Molar mass of air        | 0.0289644 kg/mol        |
| $g_0$   | Gravity at sea level     | 9.80665 m/s²            |
| $P_0$   | Pressure at sea level    | 101325 Pa               |
| $T_0$   | Temperature at sea level | 288.15 K                |
| $\beta$ |                          | 1.458e-6 kg/(m·s·K^0.5) |
| $S$     | Sutherland constant      | 110.4 K                 |
| $Z$     | Geometric height         | m                       |
| $H$     | Geopotential height      | m                       |
| $r_0$   | Radius of the Earth      | 6356766 m               |

## References

1. Standard Atmosphere (identical with the ICAO and WMO Standard Atmospheres from 2 to 32 km), First Edition 1975-05-15, ISO 2533-1975
2. U.S. Standard Atmosphere, 1976, NOAA-S/T 76-1562
3. Manual of the ICAO Standard Atmosphere, extended to 80 kilometers (262 500 feet), 1993, Third Edition, Doc 7488/3
