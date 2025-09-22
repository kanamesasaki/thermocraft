---
title: 'ハニカムコアの密度と熱伝導率'
description: '人工衛星等のシステムの一部としてハニカムパネルの熱解析をする際には、実効密度・実効熱伝導率を推定してモデルに組み込む必要があります。今回は、特にハニカムコア部分に注目して実効密度と実効熱伝導率の推定方法を紹介します。'
pubDate: 2025-09-02
updatedDate: 2025-09-11
heroImage: ''
tags: ['thermal']
---

以下の計算ツールを用いることで、ハニカムコアの実効密度と実効熱伝導率を推定することが出来ます。
ハニカムコアの寸法を入力し、コア材料の密度と熱伝導率を設定してください。

<!-- markdownlint-disable MD033 MD045 -->
<div align="center">
  <iframe
    src="/widgets/honeycomb-calculator.html"
    title="Honeycomb Core Thermal Conductivity Calculator"
    width="100%"
    height="490"
    loading="lazy"
    style="max-width: 720px; width: 100%; border:1px solid #ddd; border-radius:8px; background:#fff;">
  </iframe>
</div>
<!-- markdownlint-enable MD033 MD045 -->

![honeycomb-conductivity-1](../figures/honeycomb-conductivity-1.svg)
_Figure 1: Honeycomb Core Dimensions._

## 実効密度と実効熱伝導率の推定方法

ハニカムパネルは、軽量・高剛性な優れた構造材料として宇宙分野でもよく用いられるが、軽量であるがゆえ熱伝導率はサイズに対して低く、剛性と同様に方向に依存した特性を持つ。
人工衛星等のシステムの一部としてハニカムパネルの熱解析をする際には、ハニカムコアの形状をそのままモデル化するのは現実的でないので、ハニカムパネルとして等価な密度・熱伝導率を事前に計算して、材料パラメタとしてモデルに入れ込んでやる必要がある。

ハニカムコアの実効密度と実効熱伝導率を推定する場合、コアの寸法から各方向の熱伝導率を計算する方法[[1]](#references)が最も一般的である。
今回はFigure 1に示すようにコアの形状は正六角形とするが、特定の方向に潰れた六角形だったりしても基本的な考え方は同じである。
また、ハニカムコアはシート状のコア材料を整形して貼り合わせて作るので、コア壁面の厚さは方向によって異なる。

まず、密度$\rho_{\mathrm{eff}}$に関しては、
水平方向の断面を見たときの、コア材料の占める割合を考えればよい。

$$
\begin{equation}
\mathrm{Reference~Area:}\quad \frac{S}{2} \times \left( \frac{S}{2\sqrt{3}} + \frac{S}{\sqrt{3}} \right) = \frac{\sqrt{3}}{4}S^2
\end{equation}
$$

$$
\begin{equation}
\mathrm{Core~Material~Area:}\quad \delta \times \left( \frac{S}{\sqrt{3}} + \frac{S}{\sqrt{3}} \right) = \frac{2}{\sqrt{3}}S\delta
\end{equation}
$$

$$
\begin{equation}
\rho_\mathrm{eff} = \frac{\frac{2}{\sqrt{3}}S\delta \times \rho}{\frac{\sqrt{3}}{4}S^2} = \frac{8\delta \rho}{3S}
\end{equation}
$$

厚み方向の熱伝導を、コア材料の熱伝導率$k$と、厚み方向の実効熱伝導率$k_H$を用いてそれぞれ表す。

$$
\begin{equation}
\frac{k\times \frac{2}{\sqrt{3}}S\delta}{h} = \frac{2kS\delta}{\sqrt{3}h}\quad[\mathrm{W/K}]
\end{equation}
$$

$$
\begin{equation}
\frac{k_H\times \frac{\sqrt{3}}{4}S^2}{h} = \frac{\sqrt{3}k_H S^2}{4h}\quad[\mathrm{W/K}]
\end{equation}
$$

これらが等しいので、厚み方向の実効熱伝導率$k_H$は次のように表される。

$$
\begin{equation}
k_H = \frac{8}{3} \frac{k\delta}{S}
\end{equation}
$$

面内方向の熱伝導率についても同様の手順で求められる。
L方向の熱伝導を、コア材料の熱伝導率$k$と、L方向の実効熱伝導率$k_L$を用いてそれぞれ表すと次のようになる。

$$
\begin{equation}
\frac{k\times \delta h}{\frac{2}{\sqrt{3}}S} = \frac{\sqrt{3}k\delta h}{2S}\quad[\mathrm{W/K}]
\end{equation}
$$

$$
\begin{equation}
\frac{k_L\times \frac{S}{2}h}{\frac{\sqrt{3}}{2}S} = \frac{k_L h}{\sqrt{3}}\quad[\mathrm{W/K}]
\end{equation}
$$

これより、L方向の実効熱伝導率$k_L$は次のように表される。

$$
\begin{equation}
k_L = \frac{3}{2} \frac{k\delta}{S}
\end{equation}
$$

W方向の熱伝導は、コア材料の熱伝導率$k$と、W方向の実効熱伝導率$k_W$を用いると次のように表される。

$$
\begin{equation}
\frac{k\times \delta h}{\frac{2}{\sqrt{3}}S} = \frac{\sqrt{3}k\delta h}{2S}\quad[\mathrm{W/K}]
\end{equation}
$$

$$
\begin{equation}
\frac{k_W\times h\left(\frac{S}{2\sqrt{3}} + \frac{S}{\sqrt{3}} \right)}{S} = \frac{\sqrt{3}k_W h}{2}\quad[\mathrm{W/K}]
\end{equation}
$$

これより、W方向の実効熱伝導率$k_W$は次のように表される。

$$
\begin{equation}
k_W = \frac{k\delta}{S}
\end{equation}
$$

## References

1. David G. Gilmore, "Spacecraft Thermal Control Handbook. Vol. 1: Fundamental Technologies", The Aerospace Corporation Press, California, 2002, doi: [10.2514/4.989117](https://doi.org/10.2514/4.989117)

<!--

| Parameter                      | Values              |
| ------------------------------ | ------------------- |
| Cell size $S$ [inch]           | 1/8, 3/16, 1/4, 3/8 |
| Foil thickness $\delta$ [inch] | 0.001, 0.002, 0.003 |

コアの材料パラメタは、例としてAL5052のものを用いる。

| Property                 | Value        |
| ------------------------ | ------------ |
| Density $\rho$           | 2700 kg/m³   |
| Specific heat $c_p$      | 900 J/(kg·K) |
| Thermal conductivity $k$ | 160 W/(m·K)  |

## Swann–Pittman Model

厚み方向の熱伝導率に関しては、Swann-Pittmanモデル[^2]という熱輻射を含んだモデルが引用されることも多い。
アイデアとしては、伝導と輻射を考慮したノード熱モデルを作って、最終的にフェイスシート間の温度差と熱流速をもとに実効熱伝導率を求めようというものだ。用いる熱モデルは、フェイスシートを円板、ハニカムコアの壁面形状を円柱で近似し、厚さ方向に10分割した次のようなものを用いる。

<div align=center><img src=".\swann-pittman.svg" width="400"></div>

このモデルに関する熱伝達の関係を、行列形式でおおよそ次のような形で表すことを考えよう。

$$
\begin{equation}
\left[ ~c~ \right] \left[ \Delta T \right]
= \left\{
\left[ ~k~ \right] \left[ T_i \right] + \left[ R \right] \left[ T^4 \right] + [~q~]
\right\} \Delta t
\end{equation}
$$

左辺と、右辺の熱伝導に関する部分の具体的な行列表記は次のようになる。
ただし熱伝導係数はノード間で対称で$k_{i,j}=k_{j,i}$である。

$$
\begin{gather}
\left[ ~c~ \right] \left[ \Delta T \right] =
\left[ \begin{array}{ccccc}
c_1 & 0 &  & 0 & 0 \\
0 & c_2 &  & 0 & 0 \\
  &   & \ddots &  &  \\
0 & 0 &  & c_{11} & 0 \\
0 & 0 &  & 0 & c_{12}
\end{array} \right]
\left[ \begin{array}{c}
\Delta T_1 \\ \Delta T_2 \\ \vdots \\ \Delta T_{11} \\ \Delta T_{12}
\end{array} \right] \\
\left[ k_{ij} \right] \left[ T_i \right] =
\left[ \begin{array}{ccccc}
-k_{1,2} & k_{2,1} &  & 0 & 0 \\
k_{1,2} & -k_{2,1}-k_{2,3} &  & 0 & 0 \\
  &   & \ddots &  &  \\
0 & 0 &  & -k_{11,10}-k_{11,12} & k_{12,11} \\
0 & 0 &  & k_{11,12} & -k_{12,11}
\end{array} \right]
\left[ \begin{array}{c}
T_1 \\ T_2 \\ \vdots \\ T_{11} \\ T_{12}
\end{array} \right]
\end{gather}
$$

輻射に関する行列$\left[ R_{ij} \right]$は、輻射放熱、各ノードから直接入ってくる輻射熱、1度反射して入ってくる輻射熱、2度反射して入ってくる輻射熱、・・・、を含むので次のように分解しよう。

$$
\begin{equation}
\left[ R \right] \left[ T^4 \right] =
\left\{
\left[ R_{\mathrm{out}} \right] + \left[ R_{\mathrm{in0}} \right] + \left[ R_{\mathrm{in1}} \right] + \left[ R_{\mathrm{in2}} \right] + \cdots
\right\} \left[ T^4 \right]
\end{equation}
$$

各ノードが輻射によって放出する熱量は次のように表される。

$$
\begin{gather}
\left[ R_{\mathrm{out}} \right] \left[ T^4 \right] = -\sigma
\left[ \begin{array}{ccccc}
\epsilon_1 A_1 & 0 &  & 0 & 0 \\
0 & \epsilon_2 A_2 &  & 0 & 0 \\
  &   & \ddots &  &  \\
0 & 0 &  & \epsilon_{11} A_{11} & 0 \\
0 & 0 &  & 0 & \epsilon_{12} A_{12}
\end{array} \right]
\left[ \begin{array}{c}
T_1^4 \\ T_2^4 \\ \vdots \\ T_{11}^4 \\ T_{12}^4
\end{array} \right] =
-\sigma \left[ ~\epsilon~ \right] \left[ A \right] \left[ T^4 \right]
\end{gather}
$$

各ノード（自分自身も含む）から直接入ってくる輻射熱は次のように表される。

$$
\begin{align}
\left[ R_{\mathrm{in0}} \right] \left[ T^4 \right] &= \sigma
\left[ \begin{array}{ccccc}
\epsilon_1\epsilon_1 A_1 F_{1,1} & \epsilon_1\epsilon_2 A_2 F_{2,1} &  & \epsilon_1\epsilon_{11} A_{11} F_{11,1} & \epsilon_1\epsilon_{12} A_{12} F_{12,1} \\
\epsilon_2\epsilon_1 A_1 F_{1,2} & \epsilon_2\epsilon_2 A_2 F_{2,2} &  & \epsilon_2\epsilon_{11} A_{11} F_{11,2} & \epsilon_2\epsilon_{12} A_{12} F_{12,2} \\
  &   & \ddots &  &  \\
\epsilon_{11}\epsilon_1 A_1 F_{1,11} & \epsilon_{11}\epsilon_2 A_2 F_{2,11} &  & \epsilon_{11}\epsilon_{11} A_{11} F_{11,11} & \epsilon_{11}\epsilon_{12} A_{12} F_{12,11} \\
\epsilon_{12}\epsilon_1 A_1 F_{1,12} & \epsilon_{12}\epsilon_2 A_2 F_{2,12} &  & \epsilon_{12}\epsilon_{11} A_{11} F_{11,12} & \epsilon_{12}\epsilon_{12} A_{12} F_{12,12} \\
\end{array} \right]
\left[ \begin{array}{c}
T_1^4 \\ T_2^4 \\ \vdots \\ T_{11}^4 \\ T_{12}^4
\end{array} \right] \\
&= \sigma \left[ ~\epsilon~ \right] \left[ F \right] \left[ ~\epsilon~ \right] \left[ A \right] \left[ T^4 \right]
\end{align}
$$

各ノードから放出されて、1回反射されたのち入ってくる輻射熱は次のように表される。

$$
\begin{align}
\left[ R_{\mathrm{in1}} \right] \left[ T^4 \right] &= \sigma
\left[ \begin{array}{ccccc}
\epsilon_1 F_{1,1} & \epsilon_1 F_{2,1} &  & \epsilon_1 F_{11,1} & \epsilon_1 F_{12,1} \\
\epsilon_2 F_{1,2} & \epsilon_2 F_{2,2} &  & \epsilon_2 F_{11,2} & \epsilon_1 F_{12,2} \\
  &   & \ddots &  &  \\
\epsilon_{11} F_{1,11} & \epsilon_{11} F_{2,11} &  & \epsilon_{11} F_{11,11} & \epsilon_{11} F_{12,11} \\
\epsilon_{12} F_{1,12} & \epsilon_{12} F_{2,12} &  & \epsilon_{12} F_{11,12} & \epsilon_{12} F_{12,12} \\
\end{array} \right] \\
&\hspace{19pt}\left[ \begin{array}{cccc}
(1-\epsilon_1)\epsilon_1 A_1 F_{1,1} & (1-\epsilon_1)\epsilon_2 A_2 F_{2,1} &  & (1-\epsilon_1)\epsilon_{12} A_{12} F_{12,1} \\
(1-\epsilon_2)\epsilon_1 A_1 F_{1,2} & (1-\epsilon_2)\epsilon_2 A_2 F_{2,2} &  & (1-\epsilon_2)\epsilon_{12} A_{12} F_{12,2} \\
  &   & \ddots &  \\
(1-\epsilon_{11})\epsilon_1 A_1 F_{1,11} & (1-\epsilon_{11})\epsilon_2 A_2 F_{2,11} &  & (1-\epsilon_{11})\epsilon_{12} A_{12} F_{12,11} \\
(1-\epsilon_{12})\epsilon_1 A_1 F_{1,12} & (1-\epsilon_{12})\epsilon_2 A_2 F_{2,12} &  & (1-\epsilon_{12})\epsilon_{12} A_{12} F_{12,12} \\
\end{array} \right]
\left[ \begin{array}{c}
T_1^4 \\ T_2^4 \\ \vdots \\ T_{11}^4 \\ T_{12}^4
\end{array} \right] \\ &=
\sigma \left[ ~\epsilon~ \right] \left[ F \right] \left[ 1-\epsilon \right] \left[ F \right] \left[ ~\epsilon~ \right] \left[ A \right] \left[ T^4 \right]
\end{align}
$$

2回以上の反射に関しても同様に行列形式で表すことができる。

$$
\begin{align}
\left[ R_{\mathrm{in2}} \right] \left[ T^4 \right] &=
\sigma \left[ ~\epsilon~ \right] \left[ F \right] \left[ 1-\epsilon \right] \left[ F \right] \left[ 1-\epsilon \right] \left[ F \right] \left[ ~\epsilon~ \right] \left[ A \right] \left[ T^4 \right] \\
\left[ R_{\mathrm{in3}} \right] \left[ T^4 \right] &=
\sigma \left[ ~\epsilon~ \right] \left[ F \right] \left[ 1-\epsilon \right] \left[ F \right] \left[ 1-\epsilon \right] \left[ F \right] \left[ 1-\epsilon \right] \left[ F \right] \left[ ~\epsilon~ \right] \left[ A \right] \left[ T^4 \right]
\end{align}
$$

ただし、簡略化して表した各行列は以下のとおりである。
ちなみに形態係数の特徴として、$F_{i,j} \neq F_{j,i}$であるが、$A_i F_{i,j} = A_j F_{j,i}$となる。

$$
\begin{gather}
\left[ F \right] =
\left[ \begin{array}{ccccc}
F_{1,1} & F_{2,1} &  & F_{11,1} & F_{12,1} \\
F_{1,2} & F_{2,2} &  & F_{11,2} & F_{12,2} \\
  &   & \ddots &  &  \\
F_{1,11} & F_{2,11} &  & F_{11,11} & F_{12,11} \\
F_{1,12} & F_{2,12} &  & F_{11,12} & F_{12,12} \\
\end{array} \right], ~~~
\left[ A \right] =
\left[ \begin{array}{ccccc}
A_1 & 0 &  & 0 & 0 \\
0 & A_2 &  & 0 & 0 \\
  &   & \ddots &  &  \\
0 & 0 &  & A_{11} & 0 \\
0 & 0 &  & 0 & A_{12}
\end{array} \right]
\\
\left[ ~\epsilon~ \right] =
\left[ \begin{array}{ccccc}
\epsilon_1 & 0 &  & 0 & 0 \\
0 & \epsilon_2 &  & 0 & 0 \\
  &   & \ddots &  &  \\
0 & 0 &  & \epsilon_{11} & 0 \\
0 & 0 &  & 0 & \epsilon_{12}
\end{array} \right], ~~~
\left[ 1-\epsilon \right] =
\left[ \begin{array}{ccccc}
1-\epsilon_1 & 0 &  & 0 & 0 \\
0 & 1-\epsilon_2 &  & 0 & 0 \\
  &   & \ddots &  &  \\
0 & 0 &  & 1-\epsilon_{11} & 0 \\
0 & 0 &  & 0 & 1-\epsilon_{12}
\end{array} \right]
\end{gather}
$$

これで、ハニカムコアの熱モデルを作ることができた。
次に問題になるのは、コアのノードも含めて作った熱モデルをどうやって、フェイスシート間の熱伝導の形に押しつぶすかである。
表現したいのはノード1とノード12の関係なので、実効熱伝導率$k_\mathrm{eff}$を次のように表すことを考えよう。ここで$A$は（何もない部分も含めた）セルの占める面積、$A_\Delta$はコア材料がある面積、$k$はコア材料の熱伝導率を表す。

$$
\begin{equation}
\frac{k_\mathrm{eff} A}{h} (T_1 - T_{12}) = \frac{k A_\Delta}{h} (T_1 - T_{12}) + f(d, h, \epsilon_{1,12}, \epsilon_{2...11}) (T_1^4 - T_{12}^4)
\end{equation}
$$

モデルを表現するのに必要となるパラメタを確認しておこう。

- $d$: (equivalent) diameter of the honeycomb core
- $h$: height of the honeycomb core
- $A_{\Delta}$: (equivalent) core thickness
- $A$: area of the honeycomb core
- $k$: thermal conductivity of the honeycomb core material
- $\epsilon_{1,12}$: emissivity of the face sheet
- $\epsilon_{2...11}$: emissivity of the honeycomb core
- $T_1$: face sheet temperature on one side
- $T_{12}$: face sheet temperature on the other side

Swann, PittmanのTechnical Note[^2]では、$f(\lambda, \epsilon)$がどのような形がよいかをトライアンドエラーで探したところ次の式を使うといい感じだよ、と報告されている。
どのようなモチベーションでこの式の形が出てきたのかは謎だが、実用上では便利に用いられているようで、実際この式が様々な論文[^3] [^4] [^5] [^6]で引用されている。

$$
\begin{equation}
\frac{k_\mathrm{eff} A}{k A_\Delta} = 1 + 0.664(\lambda+0.3)^{-0.69} \epsilon^{1.63(\lambda+1)^{-0.89}} (T_1^2 + T_{12}^2)(T_1 + T_{12})
\end{equation}
$$

[^1]: David G. Gilmore, "Spacecraft Thermal Control Handbook. Vol. 1: Fundamental Technologies", The Aerospace Corporation Press, California, 2002

[^2]: Robert T. Swann, C. M. Pittman, "Analysis of Effective Thermal Conductivities of Honeycomb-Core and Corrugated-Core Sandwich Panels", NASA TN D-714, 1961

[^3]: K. Daryabeigi, "Heat Transfer in Adhesively Bonded Honeycomb Core Panels", JOURNAL OF THERMOPHYSICS AND HEAT TRANSFER, Vol. 16, No. 2, April–June 2002

[^4]: W.H. Yang, H.E. Cheng, A. Cai, "Thermal analysis for folded solar array of spacecraft in orbit", Applied Thermal Engineering 24 (2004) 595–607

[^5]: J. Fatemi, M. H. J. Lemmen, "Effective Thermal/Mechanical Properties of Honeycomb Core Panels for Hot Structure Applications", JOURNAL OF SPACECRAFT AND ROCKETS, Vol. 46, No. 3, May–June 2009

[^6]: Rongnan Yuan, Shouxiang Lu, "Experimental and numerical study for effective thermal conductivity of metallic honeycomb sandwich structures", Journal of Sandwich Structures and Materials, 2021, Vol. 23(8) 3540–3557 -->
