---
title: '微小表面から円板への形態係数(View Factor)'
description: '微小表面から円板への形態係数(View Factor)の解析解を、面積分による方法と線積分による方法の、２つのアプローチで導出する。'
pubDate: 2025-07-21
updatedDate: 2025-07-21
heroImage: ''
tags: ['thermal']
---

## Introduction

今回は、微小表面から円板へのView Factor例に、View Factorの解析解を導出する方法について解説する。
微小表面と円板の形状、および位置関係は以下のFigure 1のように、$(R,~h,~\omega)$によって表されるものとする。
実際のView Factorの値は、オンラインツール[View Factor Calculator](https://sterad.net/)を使用して計算することができる。

![disk-viewfactor-1](../figures/disk-viewfactor-1.svg)
_Figure 1: Geometrical Configuration of a Disk and a Plate Element for View Factor Evaluation._

## 面積分によるView Factorの計算

定義どおり真面目に積分してやればView Factorを計算することが出来る。

$$
\begin{align}
F &= \int_A \frac{\cos \Omega \cos \Lambda}{\pi S^2} dA
= \int_0^R \int_0^{2\pi} \frac{\cos \Omega \cos \Lambda}{\pi (r^2+h^2)}\ r d \beta dr \notag \\
&= \int_0^R \int_0^{2\pi} \frac{r^2h \sin \omega \cos \beta + rh^2 \cos \omega}{\pi (r^2+h^2)^2} d\beta dr
= \int_0^R \frac{2 rh^2 \cos \omega}{(r^2+h^2)^2} dr \notag \\
&= \left[- \frac{h^2 \cos \omega}{r^2 + h^2} \right]^R_0
= \frac{R^2}{R^2 + h^2} \cos \omega = \frac{1}{1 + (\frac{h}{R})^2} \cos \omega
\end{align}
$$

ただし、$\cos \Omega$と$\cos \Lambda$の変形はEqs. (2)および(3)のように行っている。

$$
\begin{align}
&\cos \Omega = \frac{\boldsymbol{\omega} \cdot \boldsymbol{r}}{\|\boldsymbol{\omega}\| \|\boldsymbol{r}\|}
= \frac{r \sin \omega \cos \beta + h \cos \omega}{\sqrt{r^2 + h^2}} \\
&\mathrm{where}\quad\boldsymbol{\omega} = \left( \begin{array}{c} \sin\omega \\ 0 \\ \cos \omega \end{array} \right), \hspace{10pt}
\boldsymbol{r} = \left( \begin{array}{c} r \cos \alpha \\ r \sin \alpha \\ h \end{array} \right) \notag
\end{align}
$$

$$
\begin{equation}
h \tan \Lambda = r, \quad
\cos \Lambda = \frac{h}{\sqrt{r^2 + h^2}}
\end{equation}
$$

ここまでに計算したのは、円板全体が見えている場合だ。もし微小表面の方向がもう少し倒れてくる（$\omega > \arctan \frac{h}{R}$）と、円板の一部が視野の水平線の下に落ち込んで見えなくなる。
このような場合、円板の見えている部分のみについて面積分しないといけないので、計算が一気に煩雑になる。

![disk-viewfactor-2](../figures/disk-viewfactor-2.svg)
_Figure 2: Disk View Factor Calculation by Area Integration._

積分を実行するために、積分領域を2つに分割する。1つ目は円弧状の領域（$0 \le \beta \le \beta_0$）。2つ目は三角形状の領域である。

$$
\begin{align}
&F = \int_A \frac{\cos \Omega \cos \Lambda}{\pi S^2} dA \notag \\
&= \int_0^R \int_{0}^{\beta_0} \frac{2 \cos \Omega \cos \Lambda}{\pi (r^2+h^2)} r d \beta dr + \int^0_{R \cos \beta_0} \int_0^{x \tan \beta_0} \frac{2 \cos \Omega \cos \Lambda}{\pi (x^2 + y^2 + h^2)} dy dx \notag \\
&= \int^R_0 \int_{0}^{\beta_0} \frac{2r^2h \sin \omega \cos \beta + 2rh^2 \cos \omega}{\pi (r^2 + h^2)^2} d\beta dr + \int^0_{R \cos \beta_0} \int_0^{x \tan \beta_0} \frac{2xh \sin \omega + 2h^2 \cos \omega}{\pi (x^2 + y^2 + h^2)^2} dy dx \notag \\
&= \int^R_0 \frac{2r^2h \sin \omega \sin \beta_0}{\pi (r^2 + h^2)^2}dr + \int^R_0 \frac{2\beta_0 rh^2 \cos \omega}{\pi (r^2 + h^2)^2}dr + \int_0^{R \sin \beta_0} \int^{\frac{y}{\tan \beta_0}}_{R \cos \beta_0} \frac{2xh \sin \omega}{\pi (x^2 + y^2 + h^2)^2} dx dy \notag \\
&\hspace{11pt}+ \int^0_{R \cos \beta_0} \int_0^{x \tan \beta_0} \frac{2h^2 \cos \omega}{\pi (x^2 + y^2 + h^2)^2} dy dx
\end{align}
$$

三角形部分での、$\cos \Omega$と$\cos \Lambda$の変形には次の関係を用いた。

$$
\begin{align}
&\cos \Omega = \frac{\boldsymbol{\omega} \cdot \boldsymbol{r}}{\|\boldsymbol{\omega}\| \|\boldsymbol{r}\|} = \frac{x \sin \omega + h \cos \omega}{\sqrt{x^2 + y^2 + h^2}} \\
&\mathrm{where}\quad \boldsymbol{\omega} = \left( \begin{array}{c} \sin\omega \\ 0 \\ \cos \omega \end{array} \right), \hspace{5pt}
\boldsymbol{r} = \left( \begin{array}{c} x \\ y \\ h \end{array} \right) \notag
\end{align}
$$

$$
\begin{gather}
h \tan \Lambda = \sqrt{x^2 + y^2}, \quad
\cos \Lambda = \frac{h}{\sqrt{x^2 + y^2 + h^2}}
\end{gather}
$$

ここから順番に各項の積分を求めていく。

1項目：

$$
\begin{align}
&\int^R_0 \frac{2r^2h \sin \omega \sin \beta_0}{\pi (r^2 + h^2)^2}dr
= \frac{2 \sin \omega \sin \beta_0}{\pi} \int_0^{\frac{R}{h}} \frac{u^2}{(u^2 + 1)^2} du,
\hspace{10pt} \mathrm{where}\ u=\frac{r}{h} \notag \\
&= \frac{2 \sin \omega \sin \beta_0}{\pi} \int_0^{\arctan \frac{R}{h}} \frac{\tan^2 t}{(\tan^2 t + 1)^2} \frac{dt}{\cos^2 t},
\hspace{10pt} \mathrm{where}\ \tan t=u \notag \\
&= \frac{2 \sin \omega \sin \beta_0}{\pi} \int_0^{\arctan \frac{R}{h}} \sin^2 t dt
= \frac{\sin \omega \sin \beta_0}{\pi} \int_0^{\arctan \frac{R}{h}} 1 - \cos 2t dt \notag \\
&= \frac{\sin \omega \sin \beta_0}{\pi} \left[t - \frac{1}{2} \sin 2t \right]^{\arctan \frac{R}{h}}_0
= \frac{\sin \omega \sin \beta_0}{\pi} \left[t - \frac{\tan t}{1+ \tan^2 t} \right]^{\arctan \frac{R}{h}}_0 \notag \\
&= \frac{\sin \omega \sin \beta_0}{\pi} \left( \arctan \frac{R}{h} - \frac{Rh}{R^2 + h^2} \right)
\end{align}
$$

2項目：

$$
\begin{align}
&\int^R_0 \frac{2\beta_0 rh^2 \cos \omega}{\pi (r^2 + h^2)^2}dr
= \left[ - \frac{\beta_0 h^2 \cos \omega}{\pi (r^2 + h^2)} \right]^R_0
= \frac{\beta_0 \cos \omega}{\pi} \frac{R^2}{R^2 + h^2} \notag \\
&= \frac{\beta_0 \cos \omega}{\pi} \frac{1}{1 + (h/R)^2}
\end{align}
$$

3項目：

$$
\begin{align}
&\int_0^{R \sin \beta_0} \int^{\frac{y}{\tan \beta_0}}_{R \cos \beta_0} \frac{2xh \sin \omega}{\pi (x^2 + y^2 + h^2)^2} dx dy
= \int_0^{R \sin \beta_0} \left[ - \frac{h \sin \omega}{\pi (x^2 + y^2 + h^2)} \right]^{\frac{y}{\tan \beta_0}}_{R \cos \beta_0} dy \notag \\
&= \frac{h \sin \omega}{\pi}\int_0^{R \sin \beta_0} \left( \frac{1}{y^2 + h^2 + R^2 \cos^2 \beta_0} - \frac{1}{\frac{y^2}{\sin^2 \beta_0} + h^2} \right) dy \notag \\
&= \frac{h \sin \omega}{\pi}\int_0^{R \sin \beta_0} \frac{1}{\frac{y^2}{h^2 + R^2 \cos^2 \beta_0} + 1} \frac{dy}{h^2 + R^2 \cos^2 \beta_0} - \frac{h \sin \omega}{\pi} \int_0^{R \sin \beta_0} \frac{1}{\frac{y^2}{h^2 \sin^2 \beta_0} + 1} \frac{dy}{h^2} \notag \\
&= \frac{h \sin \omega}{\pi} \int_0^{\arctan \frac{R\sin \beta_0}{\sqrt{h^2 + R^2 \cos^2 \beta_0}}} \frac{du}{\sqrt{h^2 + R^2 \cos^2 \beta_0}} - \frac{h \sin \omega}{\pi} \int_0^{\arctan \frac{R}{h}} \frac{\sin \beta_0}{h} dv \notag \\
&= \frac{h \sin \omega}{\pi \sqrt{h^2 + R^2 \cos^2 \beta_0}} \arctan \frac{R\sin \beta_0}{\sqrt{h^2 + R^2 \cos^2 \beta_0}} - \frac{\sin \omega \sin \beta_0}{\pi} \arctan \frac{R}{h}
\end{align}
$$

4項目：

まずY方向に積分する。

$$
\begin{align}
&\int_0^{x \tan \beta_0} \frac{1}{(y^2 + x^2 + h^2)^2} dy \notag \\
&= \int_0^{x \tan \beta_0} \frac{\frac{1}{(x^2 + h^2)^2}}{(\frac{y^2}{x^2 + h^2} + 1)^2} dy
= \frac{\sqrt{x^2 + h^2}}{(x^2 + h^2)^2} \int_0^{\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}}} \frac{1}{(u^2 + 1)^2} du \notag \\
&= \frac{\sqrt{x^2 + h^2}}{(x^2 + h^2)^2} \int_0^{\arctan(\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}})} \frac{1}{(\tan^2 t + 1)^2} \frac{1}{\cos^2 t}dt \notag \\
&= \frac{\sqrt{x^2 + h^2}}{(x^2 + h^2)^2} \int_0^{\arctan(\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}})} \cos^2 t dt
= \frac{\sqrt{x^2 + h^2}}{2(x^2 + h^2)^2} \int_0^{\arctan(\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}})} \cos 2t + 1 dt \notag \\
&= \frac{\sqrt{x^2 + h^2}}{2(x^2 + h^2)^2} \left[ \frac{1}{2}\sin 2t + t\right]_0^{\arctan(\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}})}
= \frac{\sqrt{x^2 + h^2}}{2(x^2 + h^2)^2} \left[ \frac{\tan t}{\tan^2 t + 1} + t\right]_0^{\arctan(\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}})} \notag \\
&= \frac{\sqrt{x^2 + h^2}}{2(x^2 + h^2)^2} \left\{ \frac{\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}}}{\frac{x^2 \tan^2 \beta_0}{x^2 + h^2} + 1} + \arctan \left(\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}}\right)\right\} \notag \\
&= \frac{1}{2} (x^2 + h^2)^{-1} \frac{x \tan \beta_0}{x^2 (\tan^2 \beta_0 + 1) + h^2} + \frac{1}{2} (x^2 + h^2)^{-\frac{3}{2}} \arctan \frac{x \tan \beta_0}{\sqrt{x^2 + h^2}}
\end{align}
$$

置換積分は以下のように行った。

$$
\begin{align}
&\frac{x \tan \beta_0}{\sqrt{x^2 + h^2}} = \tan u \\
&h^2 \tan \beta_0 (x^2 + h^2)^{-\frac{3}{2}} dx = \frac{du}{\cos^2 u}
\end{align}
$$

積分結果を今度はX方向に積分する。

$$
\begin{align}
&\int^0_{R \cos \beta_0} \frac{1}{2} (x^2 + h^2)^{-\frac{3}{2}} \arctan \frac{x \tan \beta_0}{\sqrt{x^2 + h^2}} dx \notag\\
&= \int^0_{\arctan \left( \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \right)} \frac{u du}{2 h^2 \tan \beta_0 \cos^2 u} \notag \\
&= \frac{1}{2h^2 \tan \beta_0}\left[ u \tan u + \log (\cos u) \right]^0_{\arctan \left( \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \right)} \notag \\
&= -\frac{1}{2h^2 \tan \beta_0} \left[ \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \arctan \left( \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \right)\right. \notag \\
&\hspace{11pt}\times \left.\log \left\{ \cos \left( \arctan \left( \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \right) \right) \right\} \right]
\end{align}
$$

$$
\begin{align}
&\int^0_{R \cos \beta_0} \frac{1}{2} (x^2 + h^2)^{-1} \frac{x \tan \beta_0}{x^2 (\tan^2 \beta_0 + 1) + h^2} dx \notag \\
&= \int^0_{R \cos \beta_0} \frac{1}{4 h^2 \tan \beta_0} \left\{ \frac{2x(\tan^2 \beta_0 + 1)}{(\tan^2 \beta_0 + 1) x^2 + h^2} - \frac{2x}{x^2 + h^2} \right\} dx \notag \\
&= \frac{1}{4 h^2 \tan \beta_0} \left[ \log \left( 1 + \frac{x^2 \tan^2 \beta_0}{x^2 + h^2} \right) \right]^0_{R \cos \beta_0} \notag \\
&= - \frac{1}{4 h^2 \tan \beta_0} \log \left( 1 + \frac{R^2 \sin^2 \beta_0}{R^2 \cos^2 \beta_0 + h^2} \right)
\end{align}
$$

結局4項目の積分結果は次のようになった。

$$
\begin{align}
&\int^0_{R \cos \beta_0} \int_0^{x \tan \beta_0} \frac{2h^2 \cos \omega}{\pi (x^2 + y^2 + h^2)^2} dy dx \notag \\
&= -\frac{\cos \omega}{\pi \tan \beta_0} \left[ \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \arctan \left( \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \right)
\log \left\{ \cos \left( \arctan \left( \frac{R \sin \beta_0}{\sqrt{R^2 \cos^2 \beta_0 + h^2}} \right) \right) \right\} \right] \notag \\
&\hspace{11pt}- \frac{\cos \omega}{2 \pi \tan \beta_0} \log \left( 1 + \frac{R^2 \sin^2 \beta_0}{R^2 \cos^2 \beta_0 + h^2} \right)
\end{align}
$$

In this case, we managed to complete the integration, thanks to the shape simplicity of the disk.
However, performing the double integration is highly complex, and it would be very difficult to obtain a closed-form solution for more complicated geometries.

この例では、対象が円板といういい形をしているので、なんとか計算できたものの、2重積分しなければいけないというのは非常に厄介だ。
対象がもうちょっと変な形をしていたら、初頭関数を用いた解析的な表現を見つけることはほとんど不可能になってくる。

## 線積分によるView Factorの計算

面積分はストークスの定理を用ると、対象の面を取り囲むような線積分（Contour Integration）の形に置き換えられることがある。
これはいつでもできるわけではないが、View Factorの計算にはこの手法を適用できる。まずストークスの定理を確認しておこう。

$$
\begin{equation}
\iint_{\Omega} \nabla \times \boldsymbol{F} \cdot d\boldsymbol{s} = \int_{\partial \Omega} \boldsymbol{F} \cdot d \boldsymbol{l}
\end{equation}
$$

各パラメタを明示的に書くと次のようになる。

$$
\begin{align}
\iint_{\Omega} \left[ l \left( \frac{\partial R}{\partial y} - \frac{\partial Q}{\partial z} \right)
+ m \left( \frac{\partial P}{\partial z} - \frac{\partial R}{\partial x} \right)
+ n \left( \frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y} \right) \right] ds \notag \\
= \int_{\partial \Omega} \left( P \frac{dx}{dt} + Q \frac{dy}{dt} + R \frac{dz}{dt} \right) dt
\end{align}
$$

さて、今求めたいView Factorは次のように表される。

$$
\begin{align}
F &= \int_A \frac{\cos \psi \cos \lambda}{\pi S^2} dA
= \int \frac{(\boldsymbol{x} \cdot \boldsymbol{n}_1) \times (-\boldsymbol{x} \cdot \boldsymbol{n}_2)}{\pi S^4} dA \notag \\
&= \int (-l_2 x f - m_2 y f - n_2 z f)\ dA, \quad
\mathrm{where}\quad f = \frac{1}{\pi S^4} (l_1 x + m_1 y + n_1 z)
\end{align}
$$

これらの式を見比べると、ストークスの定理を用いるには、$P,Q,R$が次の関係を満たせばよいことが分かる。

$$
\begin{align}
\frac{\partial R}{\partial y} - \frac{\partial Q}{\partial z} = - x f, \quad
\frac{\partial P}{\partial z} - \frac{\partial R}{\partial x} = - y f, \quad
\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y} = - z f
\end{align}
$$

天下り的だが、このように$P,Q,R$をとるとうまいこと条件が満たされる。

$$
\begin{align*}
P = \frac{-m_1 z + n_1 y}{2 \pi S^2}, \hspace{10pt}
Q = \frac{-n_1 x + l_1 z}{2 \pi S^2}, \hspace{10pt}
R = \frac{-l_1 y + m_1 x}{2 \pi S^2}
\end{align*}
$$

本当に満たされているか、ひとつ確認してみよう。

$$
\begin{align}
\frac{\partial R}{\partial y} &= \frac{\partial}{\partial y} \left(\frac{-l_1 y + m_1 x}{2 \pi S^2} \right) = \frac{-l_1}{2 \pi S^2} - \frac{(-l_1y + m_1x)y}{\pi S^4} \\
\frac{\partial Q}{\partial z} &= \frac{\partial}{\partial z} \left(\frac{-n_1 x + l_1 z}{2 \pi S^2} \right) = \frac{l_1}{2 \pi S^2} - \frac{(-n_1x + l_1z)z}{\pi S^4}
\end{align}
$$

$$
\begin{align}
\frac{\partial R}{\partial y} - \frac{\partial Q}{\partial z}
&= \frac{-l_1(x^2 + y^2 + z^2)}{\pi S^4} + \frac{l_1y^2 - m_1xy - n_1xz + l_1z^2}{\pi S^4} \notag \\
&= \frac{-x (l_1 x + m_1 y + n_1 z)}{\pi S^4} = -xf
\end{align}
$$

適切な$P,Q,R$が見つかったので、View Factorを線積分で表すことができる。

$$
\begin{align}
F &= \frac{1}{2 \pi} \int \left( \frac{-m_1 z + n_1 y}{S^2} dx + \frac{-n_1 x + l_1 z}{S^2} dy + \frac{-l_1 y + m_1 x}{S^2} dz \right) \notag \\
&= l_1 \int \frac{z dy - y dz}{2 \pi S^2} + m_1 \int \frac{x dz - z dx}{2 \pi S^2} + n_1 \int \frac{y dx - x dy}{2 \pi S^2}
\end{align}
$$

$$
\begin{align*}
&\left[ \begin{array}{c}l_1 \\ m_1 \\ n_1\end{array} \right]
=\left[ \begin{array}{c}\sin \omega \\ 0 \\ \cos \omega\end{array} \right]
&&:\quad \text{Infinitesimal surface direction} \\
&\left[ \begin{array}{c}x \\ y \\ z\end{array} \right]
=\left[ \begin{array}{c}R \cos \alpha \\ R \sin \alpha \\ h\end{array} \right]
&&:\quad \text{Line on the disk edge} \\
&\left[ \begin{array}{c}x \\ y \\ z\end{array} \right]
=\left[ \begin{array}{c}R \cos \alpha_0 \\ y \\ h\end{array} \right]
&&:\quad \text{Line of the view edge}
\end{align*}
$$

![disk-viewfactor-3](../figures/disk-viewfactor-3.svg)
_Figure 3: Disk View Factor Calculation by Line Integration._

これで、View Factorを実際に計算する準備が整った。
ひとつ注意点として、円板の向きを表すベクトル$(l_2, m_2, n_2)$は下向きで、このベクトルに関して右回りになるように積分する。

$$
\begin{align}
&F = l_1 \int \frac{z dy - y dz}{2 \pi S^2} + n_1 \int \frac{y dx - x dy}{2 \pi S^2} \notag \\
&= \sin \omega \left( \int_{\alpha_0}^{-\alpha_0} \frac{h (R \cos \alpha)}{2 \pi (R^2 + h^2)} d\alpha
+ \int_{-R \sin \alpha_0}^{R \sin \alpha_0} \frac{h}{2 \pi (R^2 \cos^2 \alpha_0 + y^2 + h^2)} dy \right) \notag \\
&\hspace{11pt}+ \cos \omega \left( \int_{\alpha_0}^{-\alpha_0} \frac{R \sin \alpha (-R \sin \alpha) - R \cos \alpha (R \cos \alpha)}{2 \pi (R^2 + h^2)} d\alpha
+ \int_{-R \sin \alpha_0}^{R \sin \alpha_0} \frac{- R \cos \alpha_0}{2 \pi (R^2 \cos^2 \alpha_0 + y^2 + h^2)} dy \right) \notag \\
&= \frac{Rh \sin \omega}{2 \pi (R^2 + h^2)} \int^{-\alpha_0}_{\alpha_0} \cos \alpha d\alpha
+ \frac{h \sin \omega}{2 \pi} \int_{\arctan(-\frac{R \sin \alpha_0}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}})}^{\arctan(\frac{R \sin \alpha_0}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}})} \frac{dt}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}} \notag \\
&\hspace{11pt}- \frac{R^2}{2 \pi (R^2 + h^2)} \int_{\alpha_0}^{-\alpha_0} d\alpha
- \frac{R \cos \omega \cos \alpha_0}{2 \pi} \int_{\arctan(-\frac{R \sin \alpha_0}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}})}^{\arctan(\frac{R \sin \alpha_0}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}})} \frac{dt}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}} \notag \\
&= - \frac{Rh \sin \omega \sin \alpha_0}{\pi (R^2 + h^2)} + \frac{h \sin \omega}{\pi \sqrt{R^2 \cos^2 \alpha_0 + h^2}} \arctan \left( \frac{R \sin \alpha_0}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}} \right) \notag \\
&\hspace{11pt}+ \frac{R^2 \alpha_0 \cos\omega}{\pi (R^2 + h^2)} - \frac{R \cos \omega \cos \alpha_0}{\pi \sqrt{R^2 \cos^2 \alpha_0 + h^2}} \arctan \left( \frac{R \sin \alpha_0}{\sqrt{R^2 \cos^2 \alpha_0 + h^2}} \right)
\end{align}
$$

非常に煩雑だった面積分の計算が、線積分に置き換えることでかなり簡略化されたのが分かる。

## Reference

1. A Catalog of Configuration Factors, 3rd Edition, [https://www.thermalradiation.net/indexCat.html](https://www.thermalradiation.net/indexCat.html)
2. View Factor Calculator, [https://sterad.net](https://sterad.net)
