---
title: 'Hertz-Knudsen Equationの導出'
description: 'Hertz-Knudsen Equation（Hertz-Knudsen-Langmuir Equation）は、凝縮・蒸発による質量流量を表す古典的なモデルとしてよく知られている。今回は、マクスウェル・ボルツマン分布を出発点としてHertz-Knudsen Equationを導出する。'
pubDate: 2025-08-30
updatedDate: 2025-08-31
heroImage: ''
tags: ['thermal', 'statistical mechanics']
---

Hertz-Knudsen Equation（Hertz-Knudsen-Langmuir Equationとも呼ばれる）は、凝縮・蒸発による質量流量を表す古典的なモデルとしてよく知られている。
Liquid-Vapor Interface付近の分子の速度分布としてマクスウェル・ボルツマン分布を仮定すると、凝縮・蒸発それぞれの質量流量を推定することができる。
これらを足し合わせて、全体としての質量流量を表現したものがHertz-Knudsen Equationである。

## マクスウェル・ボルツマン分布

まずはマクスウェル・ボルツマン分布の表記式を確認しておこう。

$$
\begin{equation}
% \label{eq:M-B}
dw_v = \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}}
\exp \left[ -\frac{m(v_x^2+v_y^2+v_z^2)}{2k_\mathrm{B}T} \right] dv_x dv_y dv_z
\end{equation}
$$

速度に関する確率分布である(1)に、単位体積あたりの分子数をかけてやれば、ある速度を持った粒子が何個見つかるか、という分布が得られる。

$$
\begin{equation}
dN_v = \frac{N}{V} \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}}
\exp \left[ -\frac{m(v_x^2+v_y^2+v_z^2)}{2k_\mathrm{B}T} \right] dv_x dv_y dv_z
\end{equation}
$$

このマクスウェル・ボルツマン分布に基づく個数分布の式を用いて、Liquid-Vapor Interfaceに衝突する分子がどれだけあるかを見積もっていこう。

## Liquid-Vapor Interfaceに衝突する質量流量の推定

速度$v_z$かつ$z=0$にある壁面から$v_z \Delta t$以下の距離にある分子は、$\Delta t$以内に壁面に衝突する。このことから、単位面積・単位時間に壁面に衝突する分子の数は次のように表すことができる。

$$
\begin{align*}
\nu_z &= \int_{-\infty}^{\infty} \int_{-\infty}^{\infty} \int_{-\infty}^{0}
\frac{N}{V} \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}}
\exp \left[ -\frac{m(v_x^2 + v_y^2 + v_z^2)}{2k_\mathrm{B}T} \right]
|v_z|~dv_x dv_y dv_z \\

&= \frac{N}{V} \sqrt{\frac{m}{2\pi k_\mathrm{B} T}} \int_{-\infty}^{0}
\exp \left[ -\frac{m v_z^2}{2k_\mathrm{B}T} \right] |v_z|~dv_z
= \frac{N}{V} \sqrt{\frac{m}{2\pi k_\mathrm{B} T}}
\left[ \frac{k_\mathrm{B}T}{m} \exp \left[ - \frac{m v_z^2}{2k_\mathrm{B}T} \right] \right]_{-\infty}^{0} \\

&= \frac{Nk_\mathrm{B}T}{mV} \sqrt{\frac{m}{2\pi k_\mathrm{B} T}}
= \frac{P}{\sqrt{2\pi m k_\mathrm{B} T}}
\end{align*}
$$

ただし、$v_x,~v_y$の積分にはガウス積分の結果(3)を用いた。

$$
\begin{equation}
% \label{eq:gauss}
\int_{-\infty}^{\infty} \exp \left[ -\alpha x^2 \right] dx = \sqrt{\frac{\pi}{\alpha}}
\end{equation}
$$

壁面に衝突する分子数$\nu_z$が求められたので、これに分子の質量をかければLiquid-Vapor Interfaceに衝突する質量流量(Mass Flux)が得られる。

$$
\begin{equation}
j = m\nu_z = P \sqrt{\frac{m}{2\pi k_\mathrm{B} T}}
\end{equation}
$$

本来マクスウェル・ボルツマン分布を用いることが出来るのは、自由に飛びまわっている気体分子が平衡状態にあるとき、という前提条件がある。
今回注目している領域は、気体分子とLiquid-Vapor Interfaceが相互作用する領域で、気体分子が自由に飛び回っている状態ではない。
また、全体として凝縮・蒸発が進んでいれば非平衡な状態である。
そのため、マクスウェル・ボルツマン分布の導入は、あくまで分子の速度分布を簡略化して扱うための仮定であることに注意しよう。

## Hertz-Knudsen Equation

さらにLiquid-Vapor Interfaceに衝突した分子のうちどれだけが、実際に凝縮または蒸発したのかを表す係数、凝縮係数$\sigma_c$と蒸発係数$\sigma_e$を導入しよう[[1]](#reference)。

$$
\begin{equation}
% \label{eq:Marek_Straub_2001_Eq3}
\sigma_e = \frac{\text{number of molecules transferred to the vapor phase}}{\text{number of molecules emitted from the interface}}
\end{equation}
$$

$$
\begin{equation}
% \label{eq:Marek_Straub_2001_Eq4}
\sigma_c = \frac{\text{number of molecules absorbed by the liquid phase}}{\text{number of molecules impinging on the interface}}
\end{equation}
$$

これらを用いると凝縮・蒸発にともなう質量流量は次のように表される。ただし、蒸発方向を正としている。

$$
\begin{equation}
j_e = \sigma_e \sqrt{\frac{m}{2\pi k_\mathrm{B}}} \frac{P_\mathrm{sat}(T_l)}{\sqrt{T_l}}
\end{equation}
$$

$$
\begin{equation}
j_c = - \sigma_c \sqrt{\frac{m}{2\pi k_\mathrm{B}}} \frac{P_v}{\sqrt{T_v}}
\end{equation}
$$

凝縮・蒸発の流量を足し合わせれば、全体としての質量流量を表すHertz-Knudsen Equationが得られる。

$$
\begin{equation}
% \label{eq:hertz-knudsen}
j^{LV} = \sqrt{\frac{m}{2\pi k_\mathrm{B}}} \left( \sigma_e \frac{P_\mathrm{sat}(T_l)}{\sqrt{T_l}} - \sigma_c \frac{P_v}{\sqrt{T_v}} \right)
\end{equation}
$$

この式について残る問題は、凝縮係数$\sigma_c$と蒸発係数$\sigma_e$の値をどのように取るかである。
まず、Liquid-Vapor Interfaceにおいて平衡状態が成り立っている場合には、質量流量$j^{LV}$は全体としてゼロになる。
また、$T_l = T_v$および$P_v = P_{sat}(T_l)$も成り立つので、$\sigma_e = \sigma_c$が導かれる。
一方で、蒸発あるいは凝縮が進んでいる非平衡状態においては、$\sigma_e$と$\sigma_c$が等しいとは限らず、これらの値は実験やシミュレーションで決定する必要がある。
例えば、水の凝縮係数$\sigma_c$と蒸発係数$\sigma_e$に関しては、多くの研究が行われているが、
値の推定手法によって結果にばらつきが大きく、温度と圧力を指定しても$\sigma_e$と$\sigma_c$の値を一意に決定することは難しい[[1]](#reference)。

## 気液平衡

先ほどの議論ではさらっとごまかしたが、Liquid-Vapor Interfaceにおける平衡状態（気液平衡）とは何を意味するかについて確認しておこう[[2]](#reference)。
今回は簡単のため、１種類の成分かつ気相と液相のみからなる系を考える。
まず、系全体が孤立系であること、それと全体として平衡状態にあることを仮定する。
これらの仮定は、以下のように表すことが出来る。

$$
\begin{align}
dU &= 0 \\
dS &= dS_l + dS_v = 0 \\
dV &= dV_l + dV_v = 0 \\
dN &= dN_l + dN_v = 0
\end{align}
$$

一方で、系のエネルギーの微小変化$dU$は、以下のように表される。
ただし、化学ポテンシャル$\mu$は、1molあたりのGibbsの自由エネルギーに対応する。

$$
\begin{equation}
dU = T_l dS_l + T_v dS_v - P_l dV_l - P_v dV_v + \mu_l dN_l + \mu_v dN_v
\end{equation}
$$

これを上の仮定を用いて変形すると、次のように表せる。

$$
\begin{equation}
dU = (T_v - T_l)dS_v - (P_v - P_l)dV_v + (\mu_v - \mu_l)dN_v = 0
\end{equation}
$$

$dS_v, dV_v, dN_v$は独立に変化しうるので、各係数がそれぞれ0である必要がある。
つまり、気液平衡においては以下の関係が成り立つ。

$$
\begin{align}
P_l &= P_v \\
T_l &= T_v \\
\mu_l &= \mu_v
\end{align}
$$

これらの各条件は、以下のような物理的な意味に対応している。

1. 液相と気相の圧力が等しい -> 界面で力が釣り合い、界面が移動しない
2. 液相と気相の温度が等しい -> 熱の移動がない
3. 液相と気相の化学ポテンシャルが等しい -> 質量の移動がない

このSectionで使用した記号を以下にまとめておく。

| Symbol                | Description        | Unit (SI)              |
| --------------------- | ------------------ | ---------------------- |
| $T$                   | 温度               | $\mathrm{K}$           |
| $N$                   | 物質量             | $\mathrm{mol}$         |
| $V$                   | 体積               | $\mathrm{m^3}$         |
| $P_\mathrm{sat}$      | 飽和蒸気圧         | $\mathrm{Pa}$          |
| $P$                   | 圧力               | $\mathrm{Pa}$          |
| $\sigma_e,\ \sigma_c$ | 蒸発係数・凝縮係数 |                        |
| $U$                   | 内部エネルギー     | $\mathrm{J}$           |
| $S$                   | エントロピー       | $\mathrm{J\,K^{-1}}$   |
| $\mu$                 | 化学ポテンシャル   | $\mathrm{J\,mol^{-1}}$ |

## Reference

1. R. Marek, J. Straub, "Analysis of the evaporation coefficient and the condensation coefficient of water", International Journal of Heat and Mass Transfer, Volume 44, Issue 1, 2001, Pages 39-53, doi: [10.1016/S0017-9310(00)00086-7](<https://doi.org/10.1016/S0017-9310(00)00086-7>).
2. John M. Prausnitz, Rüdiger N. Lichtenthaler, Edmundo Gomes de Azevedo, "Molecular Thermodynamics of Fluid-Phase Equilibria", 3rd Edition, Prentice Hall, 1999.
