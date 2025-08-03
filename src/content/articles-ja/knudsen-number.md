---
title: 'クヌッセン数と平均自由行程（Mean Free Path）'
description: '扱っている流体現象が低密度な領域に入ってしまった場合、まだ流体力学が適用可能かを判断する指標となるのがクヌッセン数です。今回はクヌッセン数と、その中で用いられる平均自由行程（mean free path）の評価方法について解説します。'
pubDate: 2025-07-22
updatedDate: 2025-07-30
heroImage: ''
tags: ['fluid dynamics']
---

扱っている流体現象が非常に低密度な領域に入ってしまった場合、まだ通常の流体力学の範囲で考えていいのか、それとも分子動力学や希薄流体の力学と呼ばれるような分野の現象になっているのかは、よくよく注意しないといけない。
このときに指標となるのがクヌッセン数と呼ばれる無次元量だ。今回はクヌッセン数の紹介と、その中で用いられる平均自由行程（mean free path）の評価方法について解説する。

## クヌッセン数（Knudsen Number）の定義

クヌッセン数は(1)のように定義され、$\lambda$は平均自由行程（mean free path）、$L$は代表長さである。

$$
\begin{equation}
\mathrm{Kn} = \frac{\lambda}{L}
\end{equation}
$$

平均自由行程は、ある分子が他の分子と衝突するまでに進む平均距離で、分子が周囲に密にあれば短く、分子が少なくスカスカであれば長くなる。
クヌッセン数はこの値と、流れの代表長さ（例えば円管流れであれば、管の直径）の比として表されており、クヌッセン数が1より十分小さければ、分子の衝突が頻繁に起こり、流れを連続的な流体として扱うことが出来る。

さて、$N$を単位体積あたりの分子数とすると、分子数$N$が半分になれば平均自由行程$\lambda$は倍になるという関係がある。

$$
\begin{equation}
\sigma_\mathrm{T} \lambda \propto \frac{1}{N}
\end{equation}
$$

この式のtotal collision cross-section $\sigma_\mathrm{T}$は衝突に関わる面積で、衝突に関わる分子の種類によって決まる。
直径$d$の同種の分子が衝突する場合は、Figure 1のように2つの分子の距離が$d$以下に入るような場合に衝突となるので、$\sigma_\mathrm{T}$は(3)のように表される。

$$
\begin{equation}
% \label{eq:Bird1994_1.8}
\sigma_\mathrm{T} = \pi d^2
\end{equation}
$$

![knudsen-number-1](../figures/knudsen-number-1.svg)
_Figure 1: Total Collision Cross-Section of Monoatomic Molecules._

つまり(2)の左辺は分子が運動することで掃く領域の体積、右辺は分子1個あたりに割り当てられた体積、というイメージだ。
これらが等しい場合が平均自由行程になるかというと、実はそうではない。なので、この間に成り立つ関係をもう少しきちんと議論していこう。

## 平均自由行程（Mean Free Path）

まずはある分子$t$に注目して、その分子が他の分子に衝突する平均頻度（mean collision rate）を求めていく。
速度$\boldsymbol{v}_i$を持つような分子たちを分子$i$と表すことにしよう。
分子$i$に対する分子$t$の相対速度を$\boldsymbol{v}_\mathrm{r} = \boldsymbol{v}_\mathrm{t} - \boldsymbol{v}_\mathrm{i}$と表して、
止まった分子$i$たちの中を分子$t$が速度$\boldsymbol{v}_\mathrm{r}$で飛んでいる状況を考えると、単位時間に分子$t$が掃く領域は$\sigma_\mathrm{T} |\bm{v}_\mathrm{r}|$と表される。
いま速度$\boldsymbol{v}_i$を持つ分子が単位体積内にある数は$\Delta N_i$個と表されるとする。
これを可能な速度分布全体に関して足し合わせてやると、単位時間当たりに衝突する分子の個数を求められる。

$$
\begin{equation}
\nu = N \overline{\sigma_\mathrm{T} v_\mathrm{r}}, \quad \mathrm{where} \quad \overline{\sigma_\mathrm{T} v_\mathrm{r}} = \frac{1}{N} \sum_i \Delta N_i \sigma_\mathrm{T} |\bm{v}_\mathrm{r}|
\end{equation}
$$

いま分子が1種類の剛体球であれば、平均衝突頻度（mean collision rate）は次のように書いてしまってよい。

$$
\begin{equation}
\nu = N \sigma_\mathrm{T} \overline{v_\mathrm{r}} = N \pi d^2 \overline{v_\mathrm{r}}
\end{equation}
$$

平均自由行程（mean free path）$\lambda [\mathrm{m}]$は分子の平均速度$\overline{\bm{v}'}  [\mathrm{m} \mathrm{s}^{-1}]$を平均衝突頻度（mean collision rate）$\nu [\mathrm{s}^{-1}]$でわってやればよい。

$$
\begin{equation}
\lambda = \frac{\overline{v'}}{\nu} = \frac{1}{N} \frac{\overline{v'}}{\overline{\sigma_\mathrm{T} v_\mathrm{r}}}
\end{equation}
$$

分子が1種類の剛体球の場合は次のように表される。

$$
\begin{equation}
\lambda = \frac{1}{N} \frac{\overline{v'}}{\pi d^2 \overline{v_\mathrm{r}}}
\end{equation}
$$

## 平衡状態での平均自由行程（Mean Free Path）

分子が1種類の剛体球の場合について、平均自由行程の具体的な形を求めよう。
(7)で未知なのは、分子の平均速度$\overline{{v}'}$と平均相対速度$\overline{v_\mathrm{r}}$の比である。
平衡状態の分子の速度分布であるマクスウェル・ボルツマン分布(8)をもとに、これを求めていく。

$$
\begin{equation}
% \label{eq:Bird1994_4.1}
d\varrho_v = \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m(u^2+v^2+w^2)}{2k_\mathrm{B}T} \right] du dv dw
\end{equation}
$$

### 平均相対速度（Mean Relative Velocity）, $\overline{v_\mathrm{r}}$

分子1と分子2に関して、平均速度$\boldsymbol{v}_\mathrm{m}$と相対速度$\boldsymbol{v}_\mathrm{r}$を次のように定義しておく。

$$
\begin{align}
m_1 \bm{v}_1 + m_2 \bm{v}_2 = (m_1 + m_2) \bm{v}_\mathrm{m}
\end{align}
$$

$$
\begin{align}
\bm{v}_\mathrm{r} = \bm{v}_1 - \bm{v}_2
\end{align}
$$

ある相対速度$\boldsymbol{v}_\mathrm{r}$が現れる確率は、$\boldsymbol{v}_1$の現れる確率と$\boldsymbol{v}_2$の現れる確率の積で現される。
これに相対速度の大きさ$|\bm{v}_\mathrm{r}|$をかけて、分子1と分子2の速度分布全体で積分してやれば、相対速度（の大きさ）の平均が得られる。

$$
\begin{align}
\overline{v_\mathrm{r}}
&= \int^\infty_{-\infty} \int^\infty_{-\infty} |\bm{v}_\mathrm{r}| \left\{ \left( \frac{m_1}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m_1 |\bm{v}_1|^2}{2k_\mathrm{B}T} \right] \right\} \notag \\
&\hspace{58pt}\times \left\{ \left( \frac{m_2}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m_2 |\bm{v}_2|^2}{2k_\mathrm{B}T} \right] \right\} d\boldsymbol{v}_1 d\boldsymbol{v}_2 \notag \\
&= \frac{(m_1 m_2)^{\frac{3}{2}}}{(2\pi k_\mathrm{B} T)^3} \int^\infty_{-\infty} \int^\infty_{-\infty} |\bm{v}_\mathrm{r}| \exp\left[ \frac{-(m_1 |\bm{v}_1|^2 + m_2 |\bm{v}_2|^2)}{2k_\mathrm{B} T} \right] d\boldsymbol{v}_1 d\boldsymbol{v}_2
\end{align}
$$

これを計算するために、積分変数を$\boldsymbol{v}_1=(u_1, v_1, w_1), \boldsymbol{v}_2=(u_2, v_2, w_2)$から、$\boldsymbol{v}_\mathrm{r}=(u_\mathrm{r}, v_\mathrm{r}, w_\mathrm{r}), \boldsymbol{v}_\mathrm{m}=(u_\mathrm{m}, v_\mathrm{m}, w_\mathrm{m})$に変換したい。
そのためには変換のためのヤコビアンを知る必要がある。
まず(9), (10)をもとに、$\boldsymbol{v}_1, \boldsymbol{v}_2$を$\boldsymbol{v}_\mathrm{r}, \boldsymbol{v}_\mathrm{m}$で書き表しておく。

$$
\begin{gather}
% \label{eq:Bird1994_2.4a}
\bm{v}_1 = \frac{m_2}{m_1 + m_2} \bm{v}_\mathrm{r} + \bm{v}_\mathrm{m}
\end{gather}
$$

$$
\begin{gather}
% \label{eq:Bird1994_2.4b}
\boldsymbol{v}_2
= - \frac{m_1}{m_1 + m_2} \bm{v}_\mathrm{r} + \bm{v}_\mathrm{m}
\end{gather}
$$

以下ちまちま余因子展開しているが、結果ヤコビアンは$1$である。

$$
\begin{align*}

&\left| \frac{\partial (u_1, v_1, w_1, u_2, v_2, w_2)}{\partial (u_\mathrm{r}, v_\mathrm{r}, w_\mathrm{r}, u_\mathrm{m}, v_\mathrm{m}, w_\mathrm{m})} \right| =
\left| \begin{array}{ccc}
\frac{\partial u_1}{\partial u_\mathrm{r}} & \cdots & \frac{\partial u_1}{\partial w_\mathrm{m}} \\
\vdots & ~ & \vdots \\
\frac{\partial w_2}{\partial u_\mathrm{r}} & \cdots & \frac{\partial w_2}{\partial w_\mathrm{m}}
\end{array} \right|

= \left| \begin{array}{cccccc}
\frac{m_2}{m_1+m_2} & 0 & 0 & 1 & 0 & 0\\
0 & \frac{m_2}{m_1+m_2} & 0 & 0 & 1 & 0\\
0 & 0 & \frac{m_2}{m_1+m_2} & 0 & 0 & 1\\
-\frac{m_1}{m_1+m_2} & 0 & 0 & 1 & 0 & 0\\
0 & -\frac{m_1}{m_1+m_2} & 0 & 0 & 1 & 0\\
0 & 0 & -\frac{m_1}{m_1+m_2} & 0 & 0 & 1
\end{array} \right|\\

&= \frac{m_2}{m_1+m_2}
\left| \begin{array}{ccccc}
\frac{m_2}{m_1+m_2} & 0 & 0 & 1 & 0\\
0 & \frac{m_2}{m_1+m_2} & 0 & 0 & 1\\
0 & 0 & 1 & 0 & 0\\
-\frac{m_1}{m_1+m_2} & 0 & 0 & 1 & 0\\
0 & -\frac{m_1}{m_1+m_2} & 0 & 0 & 1
\end{array} \right|
+\frac{m_1}{m_1+m_2}
\left| \begin{array}{ccccc}
0 & 0 & 1 & 0 & 0\\
\frac{m_2}{m_1+m_2} & 0 & 0 & 1 & 0\\
0 & \frac{m_2}{m_1+m_2} & 0 & 0 & 1\\
-\frac{m_1}{m_1+m_2} & 0 & 0 & 1 & 0\\
0 & -\frac{m_1}{m_1+m_2} & 0 & 0 & 1
\end{array} \right| \\

&= \left( \frac{m_2}{m_1+m_2} \right)^2
\left| \begin{array}{cccc}
\frac{m_2}{m_1+m_2} & 0 & 0 & 1\\
0 & 1 & 0 & 0\\
0 & 0 & 1 & 0\\
-\frac{m_1}{m_1+m_2} & 0 & 0 & 1
\end{array} \right|
+ \frac{m_1 m_2}{(m_1+m_2)^2}
\left| \begin{array}{cccc}
0 & 0 & 1 & 0\\
\frac{m_2}{m_1+m_2} & 0 & 0 & 1\\
0 & 1 & 0 & 0\\
-\frac{m_1}{m_1+m_2} & 0 & 0 & 1
\end{array} \right|\\
&\hspace{30pt}- \frac{m_1 m_2}{(m_1+m_2)^2}
\left| \begin{array}{cccc}
0 & 1 & 0 & 0\\
\frac{m_2}{m_1+m_2} & 0 & 0 & 1\\
0 & 0 & 1 & 0\\
-\frac{m_1}{m_1+m_2} & 0 & 0 & 1
\end{array} \right|
+ \left( \frac{m_1}{m_1+m_2} \right)^2
\left| \begin{array}{ccccc}
0 & 1 & 0 & 0\\
0 & 0 & 1 & 0\\
\frac{m_2}{m_1+m_2} & 0 & 0 & 1\\
-\frac{m_1}{m_1+m_2} & 0 & 0 & 1
\end{array} \right|\\

&= \left( \frac{m_2}{m_1+m_2} \right)^3
\left| \begin{array}{ccc}
1 & 0 & 0\\
0 & 1 & 0\\
0 & 0 & 1
\end{array} \right|
+ \frac{m_1 m_2^2}{(m_1+m_2)^3}
\left| \begin{array}{ccc}
0 & 0 & 1\\
1 & 0 & 0\\
0 & 1 & 0
\end{array} \right| \\
&\hspace{30pt}- \frac{m_1 m_2^2}{(m_1+m_2)^3}
\left| \begin{array}{ccc}
0 & 1 & 0\\
1 & 0 & 0\\
0 & 0 & 1
\end{array} \right|
+ \frac{m_1^2 m_2}{(m_1+m_2)^3}
\left| \begin{array}{ccc}
0 & 1 & 0\\
0 & 0 & 1\\
1 & 0 & 0
\end{array} \right|\\

&\hspace{30pt}+ \frac{m_1 m_2^2}{(m_1+m_2)^3}
\left| \begin{array}{ccc}
1 & 0 & 0\\
0 & 1 & 0\\
0 & 0 & 1
\end{array} \right|
- \frac{m_1^2 m_2}{(m_1+m_2)^3}
\left| \begin{array}{ccc}
1 & 0 & 0\\
0 & 0 & 1\\
0 & 1 & 0
\end{array} \right| \\
&\hspace{30pt}+\frac{m_1^2 m_2}{(m_1+m_2)^3}
\left| \begin{array}{ccc}
1 & 0 & 0\\
0 & 1 & 0\\
0 & 0 & 1
\end{array} \right|
+ \left( \frac{m_1}{m_1+m_2} \right)^3
\left| \begin{array}{ccc}
1 & 0 & 0\\
0 & 1 & 0\\
0 & 0 & 1
\end{array} \right|\\

&= \frac{m_2^3}{(m_1+m_2)^3} + \frac{m_1 m_2^2}{(m_1+m_2)^3} + \frac{m_1 m_2^2}{(m_1+m_2)^3} + \frac{m_1^2 m_2}{(m_1+m_2)^3} \\
&\hspace{30pt}+ \frac{m_1 m_2^2}{(m_1+m_2)^3} + \frac{m_1^2 m_2}{(m_1+m_2)^3} + \frac{m_1^2 m_2}{(m_1+m_2)^3} + \frac{m_1^3}{(m_1+m_2)^3} \\

&=1

\end{align*}
$$

もうひとつ、以下の変換も確認しておこう。ちなみに$m_\mathrm{r}$は古典力学で出てくる換算質量（reduced mass）と呼ばれる量である。

$$
\begin{align}
&m_1 v_1^2 + m_2 v_2^2 \notag \\
&= \frac{m_1 m_2^2}{(m_1 + m_2)^2} v_\mathrm{r}^2 + \frac{m_1 m_2}{m_1 + m_2} \boldsymbol{v}_\mathrm{r} \cdot \boldsymbol{v}_\mathrm{m} + m_1 v_\mathrm{m}^2 \notag \\
&\hspace{11pt}+ \frac{m_1^2 m_2}{(m_1 + m_2)^2} v_\mathrm{r}^2 - \frac{m_1 m_2}{m_1 + m_2} \boldsymbol{v}_\mathrm{r} \cdot \boldsymbol{v}_\mathrm{m} + m_2 v_\mathrm{m}^2 \notag \\
&= m_\mathrm{r} v_\mathrm{r}^2 + (m_1 + m_2) v_\mathrm{m}^2, \quad \mathrm{where} \quad m_\mathrm{r} = \frac{m_1 m_2}{m_1 + m_2}
\end{align}
$$

これで、(11)は次のように書き換えられる。

$$
\begin{align}
\overline{v_\mathrm{r}}
= \frac{(m_1 m_2)^{\frac{3}{2}}}{(2\pi k_\mathrm{B} T)^3} \int^\infty_{-\infty} \int^\infty_{-\infty} |\bm{v}_\mathrm{r}| \exp\left[ \frac{-(m_\mathrm{r} |\bm{v}_\mathrm{r}|^2 + (m_1 + m_2) |\bm{v}_\mathrm{m}|^2)}{2k_\mathrm{B} T} \right] d\boldsymbol{v}_\mathrm{r} d\boldsymbol{v}_\mathrm{m}
\end{align}
$$

積分は $(u_\mathrm{r}, v_\mathrm{r}, w_\mathrm{r})$ および $(u_\mathrm{m}, v_\mathrm{m}, w_\mathrm{m})$に関する速度分布全体にわたって行う必要がある。
いま、被積分関数は $|\bm{v}_\mathrm{r}|$ および $|\bm{v}_\mathrm{m}|$ の関数であるため、極座標系を用いることで計算を簡略化できる。

$$
\begin{align}
&\left[ \begin{array}{c} u_\mathrm{r} \\ v_\mathrm{r} \\ w_\mathrm{r} \end{array} \right] =
\left[ \begin{array}{c}
|\bm{v}_\mathrm{r}| \sin\theta \cos\varphi \\
|\bm{v}_\mathrm{r}| \sin\theta \sin\varphi \\
|\bm{v}_\mathrm{r}| \cos\theta \end{array} \right] \\
&du_\mathrm{r} dv_\mathrm{r} dw_\mathrm{r} = |\bm{v}_\mathrm{r}|^2 \sin\theta ~d|\bm{v}_\mathrm{r}| d\theta d\varphi
\end{align}
$$

$$
\begin{align}
&\left[ \begin{array}{c} u_\mathrm{m} \\ v_\mathrm{m} \\ w_\mathrm{m} \end{array} \right] =
\left[ \begin{array}{c}
|\bm{v}_\mathrm{m}| \sin\theta \cos\varphi \\
|\bm{v}_\mathrm{m}| \sin\theta \sin\varphi \\
|\bm{v}_\mathrm{m}| \cos\theta \end{array} \right] \\
&du_\mathrm{m} dv_\mathrm{m} dw_\mathrm{m} = |\bm{v}_\mathrm{m}|^2 \sin\theta ~d|\bm{v}_\mathrm{m}| d\theta d\varphi
\end{align}
$$

$$
\begin{align}
% \label{eq:Bird1994_4.41}
\overline{v_\mathrm{r}}
&= \frac{2(m_1 m_2)^{\frac{3}{2}}}{\pi (k_\mathrm{B} T)^3} \int^\infty_{0} \int^\infty_{0} |\bm{v}_\mathrm{r}|^3 |\bm{v}_\mathrm{m}|^2
\exp\left[ \frac{-(m_\mathrm{r} |\bm{v}_\mathrm{r}|^2 + (m_1 + m_2) |\bm{v}_\mathrm{m}|^2)}{2k_\mathrm{B} T} \right] d|\bm{v}_\mathrm{r}| d|\bm{v}_\mathrm{m}| \notag \\
&= \frac{2(m_1 m_2)^{\frac{3}{2}}}{\pi (k_\mathrm{B} T)^3}
\int^\infty_{0} |\bm{v}_\mathrm{r}|^3 \exp\left[ \frac{-m_\mathrm{r} |\bm{v}_\mathrm{r}|^2}{2k_\mathrm{B} T} \right] d|\bm{v}_\mathrm{r}| \notag \\
&\hspace{50pt}\times \int^\infty_{0} |\bm{v}_\mathrm{m}|^2 \exp\left[ \frac{-(m_1 + m_2) |\bm{v}_\mathrm{m}|^2}{2k_\mathrm{B} T} \right] d|\bm{v}_\mathrm{m}|
\end{align}
$$

(20)の$v_\mathrm{r}, v_\mathrm{m}$それぞれの積分は、ガンマ関数を使って評価することができる。

$$
\begin{equation}
% \label{eq:Tasaki2008_A.1.7}
\Gamma(x) = \int^\infty_0 e^{-t} t^{x-1} dt
\end{equation}
$$

$$
\begin{align}
&\int^\infty_0 x^3 \exp(-\alpha^2 x^2) dx
= \int^\infty_0 \frac{X}{\alpha^2} \exp(-X) \frac{dX}{2\alpha^2}
= \frac{\Gamma(2)}{2\alpha^4} = \frac{1}{2\alpha^4}\\
&\mathrm{where}\quad X = \alpha^2 x^2, \quad dX = 2\alpha^2 x dx \notag
\end{align}
$$

$$
\begin{align}
&\int^\infty_0 x^2 \exp(-\alpha^2 x^2) dx
= \int^\infty_0 \frac{X^{1/2}}{\alpha} \exp(-X) \frac{dX}{2\alpha^2}
= \frac{\Gamma(\frac{3}{2})}{2\alpha^3} = \frac{\sqrt{\pi}}{4 \alpha^3}\\
&\mathrm{where}\quad X = \alpha^2 x^2, \quad dX = 2\alpha^2 x dx \notag
\end{align}
$$

(22)および(23)を用いると、(20)の結果は次のように表される。

$$
\begin{equation}
\overline{v_\mathrm{r}} = \frac{2(m_1 m_2)^{\frac{3}{2}}}{\pi (k_\mathrm{B} T)^3}
\frac{2 (k_\mathrm{B} T)^2}{m_\mathrm{r}^2}
\frac{\sqrt{\pi}}{4} \frac{(2 k_\mathrm{B} T)^{\frac{3}{2}}}{(m_1 + m_2)^{\frac{3}{2}}}
= \frac{2}{\sqrt{\pi}} \left( \frac{2k_\mathrm{B} T}{m_\mathrm{r}} \right)^\frac{1}{2}
\end{equation}
$$

いま、分子1と分子2が同じ質量$m$を持つ場合を考えると、換算質量（reduced mass）は$m_\mathrm{r} = m/2$と表される。
このとき、平均相対速度は次のように表される。

$$
\begin{equation}
\overline{v_\mathrm{r}}
= \sqrt{\frac{16 k_\mathrm{B} T}{\pi m}}
\end{equation}
$$

### 平均速度（Mean Velocity）, $\overline{v'}$

分子の平均速度も同様の手順で評価できる。

$$
\begin{align}
\overline{v'} &= \int_{-\infty}^{\infty} |\bm{v}| \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m|\bm{v}|^2}{2k_\mathrm{B}T} \right] du dv dw \notag \\
&= \int_{0}^{\infty} |\bm{v}|^3 \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m|\bm{v}|^2}{2k_\mathrm{B}T} \right] d|\bm{v}| \int_0^{2\pi} d\varphi \int_0^{\pi} \sin\theta d\theta \notag \\
&= 4\pi \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \int_{0}^{\infty} |\bm{v}|^3 \exp \left[ -\frac{m|\bm{v}|^2}{2k_\mathrm{B}T} \right] d|\bm{v}|
\end{align}
$$

(22)を用いると、分子の平均速度は次のように表される。

$$
\begin{equation}
\overline{v'} = 4\pi \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \frac{2k_\mathrm{B}^2 T^2}{m^2} = \sqrt{\frac{8 k_\mathrm{B} T}{\pi m}}
\end{equation}
$$

## まとめ

かなり長い手順となったが、平均相対速度$\overline{v_\mathrm{r}}$および平均速度$\overline{v'}$を(25), (27)のように評価することができた。
これらの結果(7)に代入すると、平均自由行程は次のように表される。

$$
\begin{equation}
% \label{eq:Bird1994_4.55}
\lambda = \frac{1}{\sqrt{2} \sigma_\mathrm{T} N} = \frac{1}{\sqrt{2} \pi d^2 N}
\end{equation}
$$

得られた結果をクヌッセン数の定義(1)に代入してやれば、クヌッセン数の具体的な求め方が分かる。
変形には理想気体の状態方程式$PV = Nk_\mathrm{B}T$（$N$は分子の個数）を用いている。

$$
\begin{equation}
\mathrm{Kn} = \frac{\lambda}{L} = \frac{1}{\sqrt{2} \pi d^2 NL} = \frac{k_\mathrm{B}T}{\sqrt{2} \pi d^2 PL}
\end{equation}
$$

## Reference

1. G.A. Bird “Molecular Gas Dynamics and the Direct Simulation of Gas Flows”, Oxford University Press, 1994.
