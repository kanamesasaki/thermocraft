---
title: '非定常な一次元流れの基礎式'
description: '非定常な一次元流れに関する、連続の式、運動方程式（運動量の式）、エネルギーの式を、壁面からの流入や壁面でのせん断力の影響も考慮して導出します。'
pubDate: 2025-09-19
updatedDate: 2025-09-19
heroImage: ''
tags: ['fluid dynamics']
---

一次元流れの仮定とは「流体のあらゆるパラメタが断面の任意の点において等しい」とすることに相当する。
以下のような条件下において、一次元流れの仮定をおくことができる。

- 断面積の変化が十分小さい
- 断面直径に対して流線の曲率半径が十分大きい
- 断面内の速度分布および温度分布の形状がほぼ一定である

今回は、このような一次元流れの仮定の下で、流体力学の基礎式がどのように表されるかを考える。
具体的には一次元流れの連続の式、運動方程式（運動量の式）、エネルギーの式の導出を行おう。
いずれの式も、微小な領域に注目して、質量・運動量・エネルギーの保存則を適用することで導出される。
ただし、運動方程式（運動量の式）に関しては、微小な領域に注目したオイラー的な議論だけでなく、物質に注目したラグランジュ的な議論からも導出を行う。

## 連続の式

管内の微小部分$dz$を占める流体の質量は$\rho A dz$と表せて、同領域を占める質量の時間変化と、流入出量の等式を立てることが出来る。
$\dot{m}_\mathrm{in} = \rho_\mathrm{in} \pi D v_\mathrm{in} [\mathrm{kg} \mathrm{m}^{-1} \mathrm{s}]$は壁面等から流入出がある場合の単位長さあたりの流入量を表す。
流入する流体の密度は$\rho_\mathrm{in}$、流入する流体の圧力を$p_\mathrm{in}$と表すが、圧力に関しては、管内の流体と同じ$p_\mathrm{in} = p$であることを仮定する。

![one-dimensional-flow-1](../figures/one-dimensional-flow-1.svg)
_Figure 1: Mass Balance in One-Dimensional Flow._

$$
\begin{equation}
\frac{\partial}{\partial t} (\rho A dz) = - \frac{\partial}{\partial z} (\rho A v_z) dz + \dot{m}_\mathrm{in} dz
\end{equation}
$$

この式から$dz$をはらってしまえば、一次元流れの連続の式が得られる。

$$
\begin{equation}
% \label{eq:Matsuo_3.1}
\frac{\partial}{\partial t} (\rho A) = - \frac{\partial}{\partial z} (\rho A v_z) + \dot{m}_\mathrm{in}
\end{equation}
$$

<!-- 定常流れでは以下の式が成り立つ。

$$
\begin{equation}
% \label{eq:Matsuo_3.3}
\frac{d}{dz} (\rho v_z A) = \dot{m}_\mathrm{in}
\end{equation}
$$

質量流量(mass flow rate)を$\dot{m}(z) = \rho v_z A [kg/s]$と表せば、$d\dot{m} = \dot{m}_\mathrm{in} dz$で、定常流れでの連続の式は次のように表される。

$$
\begin{equation}
% \label{eq:Matsuo_3.5}
\frac{d \rho}{\rho} + \frac{d v_z}{v_z} + \frac{dA}{A} = \frac{d\dot{m}}{\dot{m}}
\end{equation}
$$

さらに、壁面からの流入出がない場合であれば以下のように表される。

$$
\begin{equation}
\rho v_z A = \dot{m} = \mathrm{const\ [kg/s]}
\end{equation}
$$

$$
\begin{equation}
\frac{d \rho}{\rho} + \frac{d v_z}{v_z} + \frac{dA}{A} = 0
\end{equation}
$$ -->

## 運動方程式（運動量の式）

### ラグランジュ的な議論

運動方程式と運動量の式は基本的に同じもので、ラグランジュ的な議論とオイラー的な議論の両方が可能であるが、まずはラグランジュ的な見方から導出を行ってみよう。
長さ$dz$の微小な領域にある流体に注目し、その流体にかかる力と、その流体の位置の変化を運動方程式として表現する。
このとき、注目する流体は移動や変形はするものの、同じものなので質量は一定である。

![one-dimensional-flow-2](../figures/one-dimensional-flow-2.svg)
_Figure 2: Forces in One-Dimensional Flow._

長さ$dz$の領域の、入口側および出口側の面にかかる圧力は次のように表される。

$$
\begin{equation}
\text{Inlet}:~ pA, \quad \text{Outlet}:~ pA + \frac{\partial (pA)}{\partial z} dz
\end{equation}
$$

次に、管側面にかかる圧力は次のように表される。

$$
\begin{align}
&2\pi \left(r + \frac{1}{2}\frac{\partial r}{\partial z} dz \right) \frac{1}{\cos \theta} \times \left( p + \frac{1}{2} \frac{\partial p}{\partial z} dz \right) \sin \theta \notag \\
&= 2 \pi rp \tan \theta + \pi r \frac{\partial p}{\partial z} dz \tan \theta + \pi p \frac{\partial r}{\partial z} dz \tan \theta \notag \\
&= p \frac{\partial A}{\partial z} + \pi \frac{\partial (pr)}{\partial z} dz \tan \theta
\end{align}
$$

$$
\begin{equation*}
\mathrm{where}\quad \frac{\partial A}{\partial z} = 2 \pi r \frac{\partial r}{\partial z} = 2 \pi r \tan \theta
\end{equation*}
$$

これより、流体に働く圧力の合計は次のように表される。

$$
\begin{align}
dF_p &= pA - \left( pA + \frac{\partial (pA)}{\partial z} dz \right)+ p \frac{\partial A}{\partial z} dz + \pi \frac{\partial (pr)}{\partial z} {dz}^2 \tan \theta \notag \\
&\simeq -\frac{\partial (pA)}{\partial z} dz + p \frac{\partial A}{\partial z} dz= - A \frac{\partial p}{\partial z} dz
\end{align}
$$

せん断応力による力の合計は次のように表される。
ただし、せん断応力$\tau[N/m^2]$を単位体積あたりの運動エネルギー（動圧）$\frac{1}{2}\rho v^2 [N/m^2]$でわった無次元数を、Fanning摩擦係数$f$と呼ぶ。

$$
\begin{align}
dF_w &= - 2\pi \left(r + \frac{1}{2}\frac{\partial r}{\partial z} dz \right) \frac{ \tau dz}{\cos \theta} \times \cos \theta \quad \mathrm{where}\quad D = 2r  \notag \\
&\simeq - \pi D \tau dz \notag \\
&= -\frac{1}{2} \rho v_z^2 \pi Df dz = -\frac{1}{2} v_z^2 \frac{4f}{D} \hspace{32pt} \mathrm{where}\quad f = \frac{\tau}{\rho v_z^2 / 2}
\end{align}
$$

これより一次元流れの運動方程式は次のように表される。

$$
\begin{equation}
\rho A dz \frac{D v_z}{D t} = - A \frac{\partial p}{\partial z} dz - \frac{1}{2} v_z^2 \frac{4f}{D} dz
\end{equation}
$$

注目した流体に関する運動方程式としては、これで問題ないのだが、
これをオイラー的な見方に変換するために、ラグランジュ微分とオイラー微分の関係式を用いて、以下のように変形する。

$$
\begin{equation}
% \label{Matsuo_3.13}
\rho A dz \left( \frac{\partial v_z}{\partial t} + v_z \frac{\partial v_z}{\partial z} \right) = - A \frac{\partial p}{\partial z} dz - \frac{1}{2} v_z^2 \frac{4f}{D} dz \\
\end{equation}
$$

全体を$\rho A dz$で割れば、一次元流れの運動方程式が得られる。

$$
\begin{equation}
\frac{\partial v_z}{\partial t} + v_z \frac{\partial v_z}{\partial z} = - \frac{1}{\rho} \frac{\partial p}{\partial z} - \frac{1}{2} v_z^2 \frac{4f}{D}
\end{equation}
$$

### オイラー的な議論

一方で、長さ$dz$の微小な領域に注目し、その領域を出入りする流体の運動量と、その領域にかかる力を考慮して、運動量の式を立てることもできる。
微小領域の入口側から入ってくる運動量、出口側から出ていく運動量はそれぞれ次のように表される。

![one-dimensional-flow-3](../figures/one-dimensional-flow-3.svg)
_Figure 3: In-coming and Out-going Momentum in One-Dimensional Flow._

$$
\begin{align}
&\text{Inlet}:~ \rho A v_z \times v_z = \rho A v_z^2
\end{align}
$$

$$
\begin{align}
\text{Outlet}:~ &\left( \rho A v_z + \frac{\partial (\rho A v_z)}{\partial z} dz \right) \times \left( v_z + \frac{\partial v_z}{\partial z} dz \right) \notag \\
&\simeq \rho A v_z^2 + \rho A v_z \frac{\partial v_z}{\partial z} dz + v_z \frac{\partial (\rho A v_z)}{\partial z} dz \notag \\
&=\rho A v_z^2 + \frac{\partial (\rho A v_z^2)}{\partial z} dz
\end{align}
$$

壁面からの流入出がある場合、対応する運動量変化を書き加える必要があるが、ここでは流入出は軸に対して垂直であると仮定し、運動量変化はゼロと考える。
微小領域にかかる圧力およびせん断力は、先ほどの議論と同じ形で表されるので、運動量の式は次のように表される。

$$
\begin{equation}
\frac{\partial}{\partial t} (\rho A v_z dz) = - \frac{\partial}{\partial z} (\rho A v_z^2) dz - A \frac{\partial p}{\partial z} dz - \frac{1}{2} \rho v_z^2 \pi Df dz
\end{equation}
$$

$$
\begin{align}
v_z \frac{\partial}{\partial t} (\rho A) dz + \rho A \frac{\partial v_z}{\partial t} dz + v_z &\frac{\partial}{\partial z} (\rho A v_z) dz + \rho A v_z \frac{\partial v_z}{\partial z} dz \notag \\
&= - A \frac{\partial p}{\partial z} dz - \frac{1}{2} \rho v_z^2 \pi Df dz
\end{align}
$$

この式に連続の式を代入すれば、以下の関係が得られる。

$$
\begin{equation}
\rho A\frac{\partial v_z}{\partial t}dz + \rho A v_z \frac{\partial v_z}{\partial z}dz + v_z \dot{m}_\mathrm{in} dz = - A \frac{\partial p}{\partial z} dz - \frac{1}{2} \rho v_z^2 \pi Df dz
\end{equation}
$$

特に、壁面からの流入出がない場合、$\dot{m}_\mathrm{in}=0$であるので、$\rho A dz$を払ってしまえば、先ほど導出した１次元流れの運動方程式(13)と同じ式が得られる。

<!-- 一方、微小でない領域について運動方程式を足し合わせると運動量の式が得られる。
ある領域の持つ運動量は$M=\int \rho v_z A dz$と表されて、運動量の微小変化はその領域の受ける外力の総和に等しい。

$$
\begin{equation}
\frac{\partial }{\partial t} \left(\int \rho A v_z dz \right) + v_z \frac{\partial}{\partial z} \left( \int \rho A v_z dz \right) = - \int A \frac{\partial p}{\partial z}dz - \int \frac{1}{2} \rho v_z^2 \pi Df dz
\end{equation}
$$

定常状態を仮定し、両辺をzについて微分すると以下の式が得られる。

$$
\begin{equation}
v_z \frac{d}{d z} \left( \int \rho A v_z dz \right) = - \int A \frac{d p}{d z}dz - \int \frac{1}{2} \rho v_z^2 \pi Df dz
\end{equation}
$$

$$
\begin{equation}
% \label{eq:Matsuo_3.26}
\frac{d}{dz} \left( \rho A v_z^2 \right) = - A \frac{dp}{dz} - \frac{1}{2} \rho v_z^2 \pi D f
\end{equation}
$$

さらに断面一定かつ壁面のせん断力ゼロを仮定すると、以下の式が得られる。

$$
\begin{equation}
% \label{eq:Matsuo_3.27}
\rho v_z^2 + p = \mathrm{const}
\end{equation}
$$ -->

## エネルギーの式

エネルギーの式は、微小部分のエネルギーの時間変化が、流入出によるエネルギー変化、圧力による仕事、外部からの熱流入、摩擦力による仕事の和に等しいことを表す式である。
微小部分のエネルギーは内部エネルギーと運動エネルギーの和として$\rho A(e + \frac{v_z^2}{2})dz$と表されるので、微小部分のエネルギーの時間変化は次のように表される。

$$
\begin{equation}
\frac{\partial}{\partial t} \left[ \rho A \left( e + \frac{v_z^2}{2} \right) \right] dz
\end{equation}
$$

微小領域の入口側、出口側それぞれにおける、流体の流入出に伴うエネルギーの変化は、次のように表される。

$$
\begin{equation}
\text{Inlet}:~ \rho A v_z \left( e + \frac{v_z^2}{2} \right)
\end{equation}
$$

$$
\begin{align}
\text{Outlet}:~ \rho A v_z \left( e + \frac{v_z^2}{2} \right) + \frac{\partial}{\partial z} \left[ \rho A v_z \left( e + \frac{v_z^2}{2} \right) \right] dz
\end{align}
$$

壁面からの流入がある場合、流入してくる流体の密度を$\rho_\mathrm{in}$、内部エネルギーを$e_\mathrm{in}$、流入速度を$v_\mathrm{in}$と表せば、流入出に伴うエネルギーの変化は次のように表される。

$$
\begin{equation}
\text{Wall}:~ \rho_\mathrm{in} \pi D v_\mathrm{in} dz \left( e_\mathrm{in} + \frac{v_\mathrm{in}^2}{2} \right)
\end{equation}
$$

![one-dimensional-flow-4](../figures/one-dimensional-flow-4.svg)
_Figure 4: In-coming and Out-going Energy in One-Dimensional Flow._

次に、圧力とせん断力による仕事を考える。
管入口側、出口側の面にかかる外部からかかる圧力のする仕事は次のように表される。

$$
\begin{equation}
\text{Inlet}:~ pA v_z
\end{equation}
$$

$$
\begin{equation}
\text{Outlet}:~ \left( pA + \frac{\partial (pA)}{\partial z} dz \right) \left( v_z + \frac{\partial v_z}{\partial z} dz \right) \simeq pA v_z + \frac{\partial (pA v_z)}{\partial z} dz
\end{equation}
$$

壁面に関しては、流体の流入がなければ圧力による仕事はゼロであるが、流入がある場合は次のように表される。

$$
\begin{equation}
\text{Wall}:~ p \pi D v_\mathrm{in} dz
\end{equation}
$$

せん断力による仕事は次のように表される。

$$
\begin{equation}
% \label{eq:friction}
- \rho A v_z \frac{4f}{D} \frac{v_z^2}{2}dz
\end{equation}
$$

![one-dimensional-flow-5](../figures/one-dimensional-flow-5.svg)
_Figure 5: Work Done by Pressure and Friction in One-Dimensional Flow._

最後に、単位質量に対する熱流入を$\dot{q}$と表せば、エネルギーのつりあい式は以下のように表される。

$$
\begin{align}
% \label{eq:Matsuo_3.36}
\frac{\partial}{\partial t} \left[ \rho A \left( e + \frac{v_z^2}{2} \right) \right] = &\rho A \dot{q} - \frac{\partial}{\partial z} \left[ \rho A v_z \left( e + \frac{v_z^2}{2} + \frac{p}{\rho} \right) \right] \notag \\
&- \rho A v_z \frac{4f}{D} \frac{v_z^2}{2} + \rho_\mathrm{in} \pi D v_\mathrm{in} \left( e_\mathrm{in} + \frac{v_\mathrm{in}^2}{2} + \frac{p}{\rho_\mathrm{in}} \right)
\end{align}
$$

ここで、管を流れる流体のエンタルピーを$h$、壁面から流れ込む流体のエンタルピーを$h_\mathrm{in}$のように表せば、エンタルピーを用いてエネルギーの式を次のように表すことができる。

$$
\begin{equation}
h = e + \frac{p}{\rho}, \quad h_\mathrm{in} = e_\mathrm{in} + \frac{p_\mathrm{in}}{\rho_\mathrm{in}} = e_\mathrm{in} + \frac{p}{\rho_\mathrm{in}}
\end{equation}
$$

$$
\begin{align}
\frac{\partial}{\partial t} \left[ \rho A \left( e + \frac{v_z^2}{2} \right) \right] = &\rho A \dot{q} - \frac{\partial}{\partial z} \left[ \rho A v_z \left( h + \frac{v_z^2}{2} \right) \right] \notag \\
&- \rho A v_z \frac{4f}{D} \frac{v_z^2}{2}+ \rho_\mathrm{in} \pi D v_\mathrm{in} \left( h_\mathrm{in} + \frac{v_\mathrm{in}^2}{2} \right)
\end{align}
$$

<!-- 定常かつ断熱な流れでは、$\dot{q}=0$かつ$\rho A v_z = \mathrm{const}$より以下の式が成り立つ。この関係は、粘性によるせん断力が働く場合でも成立する。

$$
\begin{equation}
% \label{eq:Matsuo_3.39}
h + \frac{v_z^2}{2} = \mathrm{const} \quad \mathrm{where}\quad h = e + \frac{p}{\rho}
\end{equation}
$$ -->

## References

1. 松尾一泰，"圧縮性流体力学，内部流れの理論と解析"，理工学社，1994
2. Ascher H. Shapiro, "The Dynamics and Thermodynamics of Compressible Fluid Flow, Volume I", The Ronald Press Company, 1953
