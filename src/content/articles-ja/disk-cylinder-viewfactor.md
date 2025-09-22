---
title: '円板と円筒に関する形態係数（View Factor）'
description: '微小表面から円板への形態係数（View Factor）の解析解をもとに、有限の面積を持った円板と円筒に関する様々な形態係数を導出します。'
pubDate: 2025-08-10
updatedDate: 2025-09-22
heroImage: ''
tags: ['thermal']
---

以前の記事で、[微小表面から円板への形態係数（View Factor）の解析解](https://thermocraft.space/ja/articles/disk-viewfactor/)を導出した。
この結果を用いると、有限の面積を持った円板と円筒に関連する様々なView Factorを求めることができる。
この記事では、円板と円筒に関するView Factorを実際に導出してみよう。

## 半径$R_1$の円板から半径$R_2$の円板へのView Factor

Figure 1のように、半径$R_1$の円板1と半径$R_2$の円板2が、距離$h$だけ離れて平行に配置されているとする。
円板1から円板2へのView Factorは、微小表面から円板2へのView Factorを円板1上で積分して、円板1の面積で割ってやれば求めることができる。

![disk-viewfactor-1](../figures/disk-cylinder-viewfactor-1.svg)
_Figure 1: Geometrical Configuration of two Parallel Disks for View Factor Evaluation._

微小表面から円板へのView Factorは[以前の記事](https://thermocraft.space/ja/articles/disk-viewfactor/)で導出したので、関連する結果だけ確認しておこう。
ここで、パラメタ$a$は円板の中心から微小表面までの距離を表す。

$$
\begin{align}
F_\mathrm{offset} &= \frac{1}{2} - \frac{1}{2}\frac{a^2 + h^2 - R_2^2}{\sqrt{(R_2^2 + a^2 + h^2)^2 - 4a^2R_2^2}}
\end{align}
$$

微小表面からのView Factorを、円板1上で積分すると次のように表される。

$$
\begin{align}
&\int_0^{2\pi} \int_0^{R_1} \left(\frac{1}{2} - \frac{1}{2}\frac{a^2 + h^2 - R_2^2}{\sqrt{(R_2^2 + a^2 + h^2)^2 - 4a^2R_2^2}}\right) a~ da d\theta \notag \\
&=  \pi \int_0^{R_1} a~ da - \pi \int_0^{R_1} \frac{(a^2 + h^2 - R_2^2)}{\sqrt{(R_2^2 + a^2 + h^2)^2 - 4a^2R_2^2}} a~ da \notag \\
&= \frac{\pi}{2} R_1^2 - \pi \int_0^{R_1} \frac{(a^2 + h^2 - R_2^2)}{\sqrt{(a^2 + h^2 - R_2^2)^2 + 4h^2R_2^2}} a~ da \notag \\
&= \frac{\pi}{2} R_1^2 - \frac{\pi}{2} \left[ \sqrt{(a^2 + h^2 -R_2^2)^2 + 4h^2R_2^2} \right]_0^{R_1} \notag \\
&= \frac{\pi}{2} R_1^2 - \frac{\pi}{2} \left\{  \sqrt{(R_1^2 + h^2 -R_2^2)^2 + 4h^2R_2^2} - (h^2 +R_2^2) \right\}
\end{align}
$$

積分結果を円板1の面積で割ると、円板1から円板2へのView Factorは次のように表される。

$$
\begin{align}
F_{12} &= \frac{1}{2} - \frac{1}{2} \left\{  \sqrt{\left(1 + \frac{h^2}{R_1^2} -\frac{R_2^2}{R_1^2}\right)^2 + \frac{4h^2R_2^2}{R_1^4}} - \left(\frac{h^2}{R_1^2} +\frac{R_2^2}{R_1^2}\right) \right\} \notag \\
&=\frac{1}{2}\left(1 + \frac{h^2}{R_1^2} +\frac{R_2^2}{R_1^2}\right) - \frac{1}{2}  \sqrt{\left(1 + \frac{h^2}{R_1^2} -\frac{R_2^2}{R_1^2}\right)^2 + \frac{4h^2R_2^2}{R_1^4}}
\end{align}
$$

特に、$R_1 = R_2 = R$のときは、次のように簡略化される。

$$
\begin{equation}
F_{12} = \frac{1}{2} \left\{ 2 + \frac{h^2}{R^2} - \sqrt{\frac{h^4}{R^4} + 4\frac{h^2}{R^2}} \right\}
\end{equation}
$$

## 円筒型容器の内側の面同士のView Factor

次にFigure 2のような、円筒型の容器の内側の面同士のView Factorを求めてみよう。

![disk-viewfactor-2](../figures/disk-cylinder-viewfactor-2.svg)
_Figure 2: View Factor Evaluation for Cylinder Inner Walls._

まず、底面から上面へのView Factorはすでに求めたように、(4)の式で表される。
底面からは、上面あるいは円筒の側面のみが見えるので、底面から側面へのView Factorは次のように表される。

$$
\begin{align}
F_{13} &= 1 - F_{12}
= \frac{1}{2} \left\{  - \frac{h^2}{R^2} + \sqrt{\frac{h^4}{R^4} + 4\frac{h^2}{R^2}} \right\}
\end{align}
$$

逆に、側面から底面へのView Factorは、Reciprocal Relationを用いて次のように表される。
側面から上面へのView Factorも同様に表される。

$$
\begin{align}
F_{31} &= \frac{A_1}{A_3} F_{13}
= \frac{R}{4h} \left\{  - \frac{h^2}{R^2} + \sqrt{\frac{h^4}{R^4} + 4\frac{h^2}{R^2}} \right\} \notag \\
&= - \frac{h}{4R} + \frac{1}{4} \sqrt{\frac{h^2}{R^2} + 4}
\end{align}
$$

円筒の側面からは、底面と上面、および側面自身が見えるので、自分自身へのView Factorは次のように表される。

$$
\begin{align}
F_{33} = 1 - 2 F_{31}
= 1 + \frac{h}{2R} - \sqrt{\frac{h^2}{4R^2} + 1}
\end{align}
$$

## 円筒の内側面を分割する

さらに、円筒の内側面を分割して、円筒側面の一部分のView Factorを求めてみよう。
まず、Figure 3のように、円筒の内側面を上下に分割して、上側の面を3A、下側の面を3Bとする。

![disk-viewfactor-3](../figures/disk-cylinder-viewfactor-3.svg)
_Figure 3: View Factor Evaluation for a Part of Cylinder Inner Wall._

先ほどの結果(5)を用いると、底面から、少し離れた円筒側面へのView Factorを計算することができる。

$$
\begin{align}
&F_{1-3A} = F_{1-3A3B} - F_{1-3B} \notag \\
&= \frac{1}{2} \left\{  - \frac{h_A^2 + 2h_A h_B}{R^2} + \sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\}
\end{align}
$$

この結果についても、Reciprocal Relationを用いて、側面から底面へのView Factorを求めることができる。

$$
\begin{align}
&F_{3A-1} = \frac{A_1}{A_{3A}} F_{1-3A} \notag \\
&= \frac{R}{4h_A} \left\{  - \frac{h_A^2 + 2h_A h_B}{R^2} + \sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\}
\end{align}
$$

この結果を用いて、側面から隣の側面へのView Factorを求めることができる。

$$
\begin{align}
&F_{3A-3B} = F_{3A-2} - F_{3A-1} \notag \\
&=-\frac{h_A}{4R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} - \frac{R}{4h_A} \notag \\
&\quad \times \left\{  - \frac{h_A^2 + 2h_A h_B}{R^2}+ \sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\} \notag \\
&=\frac{h_B}{2R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} - \frac{R}{4h_A} \left\{\sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\} \notag \\
&=\frac{h_B}{2R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} + \frac{h_B}{4h_A} \sqrt{\frac{h_B^2}{R^2} + 4} - \frac{h_A + h_B}{4h_A} \sqrt{\frac{(h_A + h_B)^2}{R^2} + 4}
\end{align}
$$

最後に、円筒の側面同士のView Factorで、より一般的なケースについて考えてみよう。
Figure 4に示すように、円筒の側面を3つに分割して、上側の面を3A、中央の面を3B、下側の面を3Cとする。

![disk-viewfactor-4](../figures/disk-cylinder-viewfactor-4.svg)
_Figure 4: View Factor between Cylinder Inner Walls._

隣同士の側面のView Factorはすでに導出したので、これをもとに3Aから3CへのView Factorを求めることができる。

$$
\begin{align}
&F_{3A-3C} = F_{3A-3B3C} - F_{3A-3B} \notag \\
&= \frac{(h_B+h_C)}{2R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} + \frac{h_B+h_C}{4h_A} \sqrt{\frac{(h_B+h_C)^2}{R^2} + 4} \notag \\
&\quad- \frac{h_A + h_B+h_C}{4h_A} \sqrt{\frac{(h_A + h_B+h_C)^2}{R^2} + 4} \notag \\
&\quad- \left\{ \frac{h_B}{2R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} + \frac{h_B}{4h_A} \sqrt{\frac{h_B^2}{R^2} + 4} - \frac{h_A + h_B}{4h_A} \sqrt{\frac{(h_A + h_B)^2}{R^2} + 4} \right\} \notag \\
&=\frac{h_C}{2R} + \frac{h_B+h_C}{4h_A} \sqrt{\frac{(h_B+h_C)^2}{R^2} + 4} - \frac{h_B}{4h_A} \sqrt{\frac{h_B^2}{R^2} + 4} \notag \\
&\quad- \frac{h_A + h_B+h_C}{4h_A} \sqrt{\frac{(h_A + h_B+h_C)^2}{R^2} + 4} + \frac{h_A + h_B}{4h_A} \sqrt{\frac{(h_A + h_B)^2}{R^2} + 4}
\end{align}
$$

$h_A, h_B, h_C$の値は自由に設定できるので、任意の間隔、幅をもった側面同士のView Factorを求めることができた。

## References

1. John R. Howell, M. Pinar Mengüç, "Radiative transfer configuration factor catalog: A listing of relations for common geometries", Journal of Quantitative Spectroscopy and Radiative Transfer, Volume 112, Issue 5, 2011, Pages 910-912, doi: [10.1016/j.jqsrt.2010.10.002](https://doi.org/10.1016/j.jqsrt.2010.10.002)
2. A Catalog of Configuration Factors, 3rd Edition, [https://www.thermalradiation.net/indexCat.html](https://www.thermalradiation.net/indexCat.html)
3. View Factor Calculator, [https://sterad.net](https://sterad.net)
