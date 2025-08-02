---
title: 'バイメタル変形の解析解'
description: '材質の異なる薄板を張り合わせると、常温では平らだったものが、温度を変化させると“そり”が生じてしまうことがあります。これは各材質が異なる熱膨張率を持っていることに起因する変形で、このような材料をバイメタルと呼びます。'
pubDate: 2025-08-02
updatedDate: 2025-08-02
heroImage: ''
tags: ['structural mechanics', 'thermal']
---

材質の異なる薄板を張り合わせると、常温では平らだったものが、温度を変化させると“そり”が生じてしまうことがある。
これは、それぞれの材質が異なる熱膨張率を持っていることに起因する変形で、このような材料をバイメタルと呼ぶ。

この特性を積極的に利用した製品として、バイメタル温度計、サーモスタットなどが挙げられる。
また宇宙での使用例としては、宇宙機の熱制御に用いられるサーマルルーバがある。
これは、ブラインドの開閉によって宇宙機からの放熱を調節する機器で、ブラインドを回転させるアクチュエータにバイメタルが用いられる。
アクチュエータそのものの温度によって自動的に開閉され、かつモーターやベアリングといった摺動部を持った部品を使わずに駆動できるのが利点である。
一方で、ブラインドのブレード、アクチュエータ、それらを保持するフレームなどが必要になるので、ある程度の重量となることは避けられない。
あまり積極的に採用されるデバイスではないが、ミッションフェーズによって熱環境が大きく変化する探査機などでは近年でも利用されることがある。

_Table 1: Application Examples of Spacecraft Thermal Louver_

| Spacecraft Name      | COSPAR ID            | Reference          |
| -------------------- | -------------------- | ------------------ |
| Mariner 4            | 1964-077A            | [[3]](#reference)  |
| Pioneer 6            | 1965-105A            | [[4]](#reference)  |
| Voyager 1, Voyager 2 | 1977-084A, 1977-076A | [[5]](#reference)  |
| Ohzora               | 1984-015A            | [[6]](#reference)  |
| Sakigake             | 1985-001A            | [[6]](#reference)  |
| Suisei               | 1985-073A            | [[6]](#reference)  |
| Akebono              | 1989-016A            | [[6]](#reference)  |
| Magellan             | 1989-033B            | [[7]](#reference)  |
| HALCA                | 1997-005A            | [[6]](#reference)  |
| Landsat 7            | 1999-020A            | [[8]](#reference)  |
| Rosetta              | 2004-006A            | [[9]](#reference)  |
| Kaguya               | 2007-001A            | [[10]](#reference) |
| Dawn                 | 2007-043A            | [[11]](#reference) |
| Parker Solar Probe   | 2018-065A            | [[12]](#reference) |
| Psyche               | 2023-157A            | [[13]](#reference) |
| Europa Clipper       | 2024-182A            | [[14]](#reference) |

## 曲率半径の解析解

外力・モーメントがなく均一な温度変化の下で、バイメタルの変形（曲率半径$\rho$）は解析的に表すことができるので、これを導出してみよう。
Figure 1に示すように、ひずみの生じていない中立軸の位置をZ方向原点、物体底面の位置を$R$とする。

![bimetal-1](../figures/bimetal-1.svg)
_Figure 1: Deformation of Bi-metal Beam._

このとき、軸力およびモーメントは次のように表される。

$$
\begin{align*}
N_x &= \int^{R+h_1}_{R} \sigma_1 b\ dz + \int^{R+h_1+h_2}_{R+h_1} \sigma_2 b\ dz \\
&= \int^{R+h_1}_{R} E_1 \left( \frac{z}{\rho} - \alpha_1 \Delta T \right) b\ dz + \int^{R+h_1+h_2}_{R+h_1} E_2 \left( \frac{z}{\rho} - \alpha_2 \Delta T \right) b\ dz \\
&= E_1 b \left[ \frac{z^2}{2\rho} - \alpha_1 \Delta T z \right]^{R+h_1}_{R} + E_2 b \left[ \frac{z^2}{2\rho} - \alpha_2 \Delta T z \right]^{R+h_1+h_2}_{R+h_1} \\
&= E_1 b \left( \frac{2Rh_1 + h_1^2}{2\rho} - \alpha_1 \Delta T h_1 \right) + E_2 b \left( \frac{2(R+h_1)h_2 + h_2^2}{2\rho} - \alpha_2 \Delta T h_2 \right)
\end{align*}
$$

$$
\begin{align*}
M_y &= \int^{R+h_1}_{R} \sigma_1 zb\ dz + \int^{R+h_1+h_2}_{R+h_1} \sigma_2 zb\ dz \\
&= \int^{R+h_1}_{R} E_1 \left( \frac{z}{\rho} - \alpha_1 \Delta T \right) zb\ dz + \int^{R+h_1+h_2}_{R+h_1} E_2 \left( \frac{z}{\rho} - \alpha_2 \Delta T \right) zb\ dz \\
&= E_1 b \left[ \frac{z^3}{3\rho} - \frac{\alpha_1 \Delta T z^2}{2} \right]^{R+h_1}_{R} + E_2 b \left[ \frac{z^3}{3\rho} - \frac{\alpha_2 \Delta T z^2}{2} \right]^{R+h_1+h_2}_{R+h_1} \\
&= E_1 b \left( \frac{3R^2h_1 + 3Rh_1^2 + h_1^3}{3\rho} - \frac{\alpha_1 \Delta T (2Rh_1 + h_1^2)}{2} \right) \\
&+ E_2 b \left( \frac{3(R+h_1)^2h_2 + 3(R+h_1)h_2^2 + h_2^3}{3\rho} - \frac{\alpha_2 \Delta T (2(R+h_1)h_2 + h_2^2)}{2} \right)
\end{align*}
$$

ただし式変形には、ひずみと曲率半径の関係(1)を用いた。

$$
\begin{equation}
% \label{eq:curvature}
\epsilon(z) = \frac{(\rho - z)d\theta - \rho d\theta}{\rho d\theta} = \frac{-z}{\rho}
\end{equation}
$$

いま外力なしの状態を考えているので、断面に生じる応力の合計（軸力）はゼロである。このことから(2)の関係が得られる。

$$
\begin{equation}
% \label{eq:force}
R (E_1 h_1 + E_2 h_2) + E_2 h_1 h_2 + \frac{1}{2} (E_1 h_1^2 + E_2 h_2^2)= \rho \Delta T (E_1 \alpha_1 h_1 + E_2 \alpha_2 h_2)
\end{equation}
$$

同様に、モーメントも断面全体でゼロとなるので以下の関係が得られる。

$$
\begin{align*}
% \label{eq:moment}
&R^2 (E_1 h_1 + E_2 h_2) + R (E_1 h_1^2 + 2 E_2 h_1 h_2 + E_2 h_2^2) + \frac{1}{3} E_1 h_1^3 + E_2 h_1^2 h_2 + E_2 h_1 h_2^2 + \frac{1}{3} E_2 h_2^3 \\
&= E_1 \alpha_1 \Delta T \rho R h_1 + E_2 \alpha_2 \Delta T \rho (R + h_1)h_2 + \frac{1}{2} E_1 \alpha_1 \Delta T \rho h_1^2 + \frac{1}{2} E_2 \alpha_2 \Delta T \rho h_2^2
\end{align*}
$$

軸力の式(2)を用いて、モーメントの式から$R$の次数を下げる。

$$
\begin{align*}
& \frac{1}{2} R (E_1 h_1^2 + 2 E_2 h_1 h_2 + E_2 h_2^2) + \frac{1}{3} E_1 h_1^3 + E_2 h_1^2 h_2 + E_2 h_1 h_2^2 + \frac{1}{3} E_2 h_2^3 \\
&= \rho E_2 \alpha_2 \Delta T h_1 h_2 + \frac{1}{2} \rho E_1 \alpha_1 \Delta T h_1^2 + \frac{1}{2} \rho E_2 \alpha_2 \Delta T h_2^2
\end{align*}
$$

以下の2式から$R$を消去すれば、曲率半径$\rho$が求められる。

$$
\begin{align*}
&R (E_1 h_1 + E_2 h_2) (E_1 {h_1}^2 + 2 E_2 h_1 h_2 + E_2 {h_2}^2) \notag \\
&+ \frac{2}{3} (E_1 h_1 + E_2 h_2) ( E_1 {h_1}^3 + 3 E_2 {h_1}^2 h_2 + 3 E_2 h_1 {h_2}^2 + E_2 {h_2}^3 ) \\
&~~= \rho \Delta T (E_1 h_1 + E_2 h_2) (2 E_2 \alpha_2 h_1 h_2 + E_1 \alpha_1 {h_1}^2 + E_2 \alpha_2 {h_2}^2)
\end{align*}
$$

$$
\begin{align*}
&R (E_1 h_1 + E_2 h_2) (E_1 {h_1}^2 + 2 E_2 h_1 h_2 + E_2 {h_2}^2) + \frac{1}{2} (E_1 {h_1}^2 + 2 E_2 h_1 h_2 +E_2 {h_2}^2)^2 \\
&~~= \rho \Delta T (E_1 \alpha_1 h_1 + E_2 \alpha_2 h_2)(E_1 {h_1}^2 + 2 E_2 h_1 h_2 + {E_2}^2)
\end{align*}
$$

かなり込み入ってはいるが、以下のように変形できる。

$$
\begin{align*}
&\rho \Delta T (E_1 h_1 + E_2 h_2) (2 E_2 \alpha_2 h_1 h_2 + E_1 \alpha_1 {h_1}^2 + E_2 \alpha_2 {h_2}^2) \notag \\
&- \frac{2}{3} (E_1 h_1 + E_2 h_2) ( E_1 {h_1}^3 + 3 E_2 {h_1}^2 h_2 + 3 E_2 h_1 {h_2}^2 + E_2 {h_2}^3 ) \\
&= \rho \Delta T (E_1 \alpha_1 h_1 + E_2 \alpha_2 h_2)(E_1 {h_1}^2 + 2 E_2 h_1 h_2 + {E_2}^2) -
\frac{1}{2} (E_1 {h_1}^2 + 2 E_2 h_1 h_2 +E_2 {h_2}^2)^2 \\
\end{align*}
$$

$$
\begin{align*}
&\rho \Delta T E_1 E_2 (\alpha_2 {h_1}^2 h_2 - \alpha_1 {h_1}^2 h_2 + \alpha_2 h_1 {h_2}^2 - \alpha_1 h_1 {h_2}^2) \notag \\
&= \frac{1}{6} ({E_1}^2 {h_1}^4 + 4 E_1 E_2 {h_1}^3 h_2 + 6 E_1 E_2 {h_1}^2 {h_2}^2 + 4 E_1 E_2 h_1 {h_2}^3 + {E_2}^2 {h_2}^4 )
\end{align*}
$$

最終的に曲率半径$\rho$は以下のように表される。

$$
\begin{equation}
\frac{1}{\rho} = \frac{6(\alpha_2 - \alpha_1) \Delta T E_1 E_2 h_1 h_2 (h_1 + h_2)}{{E_1}^2 {h_2}^4 + 2 E_1 E_2 h_1 h_2 (2 {h_1}^2 + 3 h_1 h_2 + 2{h_2}^2) + {E_2}^2 {h_2}^4}
\end{equation}
$$

## Reference

熱構造について日本語で書かれた教科書は少ないが、[[1]](#reference)は構造に関わる幅広いトピックを扱っており、例題・コード例も豊富なので非常に参考になる。Timoshenkoの論文[[2]](#reference)はバイメタルの変形について解析的に扱っている（おそらく）最初の論文だ。

1. 小松 敬治, "機械構造弾性力学-弾性力学の基礎とMATLABによる有限要素解析-", 森北出版, 2013
2. S. Timoshenko, "Analysis of Bi-Metal Thermostats," J. Opt. Soc. Am. 11, 233-255 (1925), doi: [10.1364/JOSA.11.000233](https://doi.org/10.1364/JOSA.11.000233)
3. F. GABRON, R. W. JOHNSON, J. M. F. VICKERS, J. W. LUCAS, "Thermal scale modeling of the Mariner IV SPACECRAFT", AIAA 3rd Aerospace Sciences Meeting, 1966, doi: [10.2514/6.1966-23](https://doi.org/10.2514/6.1966-23)
4. O. W. Clausen, J, P. Kirkpatrick, "Thermal tests of an improved louver system for spacecraft thermal control", AIAA 4th Aerospace Sciences Meeting, 1969, doi: [10.2514/6.1969-627](https://doi.org/10.2514/6.1969-627)
5. Heacock RL. The Voyager Spacecraft. Proceedings of the Institution of Mechanical Engineers. 1980;194(1):211-224. doi: [10.1243/PIME_PROC_1980_194_026_02](https://doi.org/10.1243/PIME_PROC_1980_194_026_02)
6. 大西 晃, 科学衛星の熱設計の歩みと熱物性研究について, 2012
7. James C. Neuman, Joseph A. Buescher, Gregory J. Esterl, "Magellan Spacecraft Thermal Control System Design and Performance," AIAA 28th Thermophysics Conference, 1993, doi: [10.2514/6.1993-2844](https://doi.org/10.2514/6.1993-2844)
8. Choi, M., "Validation of Landsat-7 ETM+ MEM Thermal Improvement in Thermal Vacuum Tests and in Flight Due to Lower Louver Set Points," SAE Technical Paper 1999-01-2629, 1999, doi: [10.4271/1999-01-2629](https://doi.org/10.4271/1999-01-2629).
9. Härtel, K., Morgenroth, L., Reichenberger, K., Domingo, M. et al., "Thermal Design and Test of ROSETTA Platform Louvres," SAE Technical Paper 2000-01-2276, 2000, doi: [10.4271/2000-01-2276](https://doi.org/10.4271/2000-01-2276)
10. Hiroyuki MINAMINO, Michio TAKAHASHI, Satoshi TAYAMA, Yutaka TAKANO, Takeshi SASAKI, Shuichi MATSUMOTO, Shingo IKEGAMI, 月周回衛星「かぐや」衛星システムの開発, Aeronautical and Space Sciences Japan, 2008, Volume 56, Issue 656, Pages 229-235, doi: [10.14822/kjsass.56.656_229](https://doi.org/10.14822/kjsass.56.656_229)
11. Thomas, V.C., Makowski, J.M., Brown, G.M. et al. The Dawn Spacecraft. Space Sci Rev 163, 175–249 (2011). doi: [10.1007/s11214-011-9852-2](https://doi.org/10.1007/s11214-011-9852-2)
12. Carl J. Ercol, G. Allan Holtzman, "Post-Launch and Early Mission Thermal Performance of Parker Solar Probe", 49th International Conference on Environmental Systems, 2019.
13. Isabel SOTO ARMAÑANZAS, Jose Javier VIÑALS ABELAN, Ben KWONG, Paul LINGGI, "Passive Thermal Control Louvers Mechanical Reliability", 50th International Conference on Environmental Systems, 2020.
14. Pradeep Bhandari, A. J. Mastropietro, Razmig Kandilian, Jenny Hua, Sean Reilly, Paul Woodmansee, Tyler Schmidt, Mark Duran, "Thermal Control Technologies for Europa Clipper Mission", 49th International Conference on Environmental Systems, 2019.
