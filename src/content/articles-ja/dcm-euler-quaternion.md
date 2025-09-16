---
title: DCM・オイラー角・クォータニオン
description: '物体の姿勢を表現する手法として一般的なものに、DCM（Direct Cosine Matrix）、オイラー角、クォータニオンがあります。これらの表現方法の概要と、姿勢の更新式について解説します。'
pubDate: 2025-08-03
updatedDate: 2025-08-03
heroImage: ''
tags: ['astrodynamics']
---

## 物体の姿勢表現

人工衛星でも、ロボットアームでも、3Dモデルでも、ある物体の姿勢を表したいというのは、基準となる座標系に対してその物体に固定された座標系を何らかの形で表現したい、ということだ。
つまり、異なる（ただし正規直交な）座標系の関係をどのようなパラメタと式を使って表現すると、分かりやすく効率的かという話である。
一般的によく知られている方法として、以下の3つが挙げられる。

- Direct Cosine Matrix (DCM)
- Euler Angle (オイラー角)
- Quaternion (クオータニオン)

それぞれについて、もう少し詳しく説明していこう。

## Direct Cosine Matrix（DCM）

DCMのアイデアは「座標基底ベクトルを並べたものを行列で変換して別の座標の基底を書き表そう。
このときに使う変換行列（DCM）がまさに座標間の関係そのものだ」というものである。
DCMまたは座標基底そのままだとパラメタが必要以上ある（正規直交な座標しか考えないので座標パラメタ全てをいつでも使えるように用意しておく必要がない）ので、数値計算する場合もこのまま使うとメモリや計算量が多くなってしまうのが難点である。
一方で個人的には、座標基底をはっきりした形で操作するので、最も分かりやすい姿勢表現の方法だと思っている。

さて、DCMを用いると$\mathcal{F}_\mathrm{a}$から$\mathcal{F}_\mathrm{b}$への座標回転は次のように表される。
今回もベクトリクスを使って座標を表すので、この表記方法については[剛体の運動方程式](https://thermocraft.space/ja/articles/rigid-body/)または参考文献[[1]](#reference)を見てもらいたい。

$$
\begin{equation*}
% \label{Hughes2004_2.1.7}
\mathcal{F}_\mathrm{b} = \boldsymbol{C}_\mathrm{ba} \mathcal{F}_\mathrm{a}
\end{equation*}
$$

$$
\begin{equation}
\left[ \begin{array}{c} \vec{b}_1 \\ \vec{b}_2 \\ \vec{b}_3 \end{array} \right] =
\left[ \begin{array}{ccc} c_{11} & c_{12} & c_{13} \\ c_{21} & c_{22} & c_{23} \\ c_{31} & c_{32} & c_{33} \end{array} \right]
\left[ \begin{array}{c} \vec{a}_1 \\ \vec{a}_2 \\ \vec{a}_3 \end{array} \right]
\end{equation}
$$

転置すれば次のように表すこともできる。

$$
\begin{equation*}
% \label{Hughes2004_2.3.1}
\mathcal{F}_\mathrm{b}^T = \mathcal{F}_\mathrm{a}^T \boldsymbol{C}_\mathrm{ba}^T
\end{equation*}
$$

$$
\begin{equation}
[ \begin{array}{ccc} \vec{b}_1 & \vec{b}_2 & \vec{b}_3 \end{array} ] =
[ \begin{array}{ccc} \vec{a}_1 & \vec{a}_2 & \vec{a}_3 \end{array} ]
\left[ \begin{array}{ccc} c_{11} & c_{21} & c_{31} \\ c_{12} & c_{22} & c_{32} \\ c_{13} & c_{23} & c_{33} \end{array} \right]
\end{equation}
$$

このとき、DCMを(3)のように表すことにすると整合性が取れる。

$$
\begin{equation}
% \label{Hughes2004_B.3.1}
\boldsymbol{C}_\mathrm{ba} = \mathcal{F}_\mathrm{b} \cdot \mathcal{F}_\mathrm{a}^T
= \left[ \begin{array}{ccc} \vec{b}_1 \cdot \vec{a}_1  & \vec{b}_1 \cdot \vec{a}_2 & \vec{b}_1 \cdot \vec{a}_3 \\ \vec{b}_2 \cdot \vec{a}_1 & \vec{b}_2 \cdot \vec{a}_2 & \vec{b}_2 \cdot \vec{a}_3 \\ \vec{b}_3 \cdot \vec{a}_1 & \vec{b}_3 \cdot \vec{a}_2 & \vec{b}_3 \cdot \vec{a}_3 \\ \end{array} \right]
\end{equation}
$$

では、ある座標が角速度ベクトル$\vec{\omega}$で回転しているとき、どのように座標を更新すればよいかを確認しておこう。
あるベクトル$\vec{r}$を単位ベクトル$\vec{n}$の周りで微小な角度$d\varphi$だけ回転させると、変化分は$d\vec{r} = (\vec{n} \times \vec{r}) d\varphi$となる。
角速度ベクトルを用いて書けば次のようになる。

$$
\begin{equation}
d\vec{r} = \left(\frac{\vec{\omega}}{|\vec{\omega}|} \times \vec{r} \right) |\vec{\omega}| dt = (\vec{\omega} \times \vec{r}) dt
\end{equation}
$$

このことを、座標系$\mathcal{F}_\mathrm{b}^T = [\vec{b}_1~\vec{b}_2~\vec{b}_3]$に適用してやれば座標の更新式が得られる。

$$
\begin{equation}
% \label{Hughes2004_B.4.3}
\frac{d}{dt} \mathcal{F}_\mathrm{b}^T = \vec{\omega} \times \mathcal{F}_\mathrm{b}^T = (\mathcal{F}_\mathrm{b}^T \boldsymbol{\omega}) \times \mathcal{F}_\mathrm{b}^T
\end{equation}
$$

DCMの更新式を得るために、$\mathcal{F}_\mathrm{a}^T = \mathcal{F}_\mathrm{b}^T \boldsymbol{C}_\mathrm{ba}$を時間微分する。

$$
\begin{align}
\vec{0} &= \frac{d\mathcal{F}_\mathrm{b}^T}{dt}  \boldsymbol{C}_\mathrm{ba} + \mathcal{F}_\mathrm{b}^T \frac{d\boldsymbol{C}_\mathrm{ba}}{dt}  \\
\vec{0} &= (\omega^T \mathcal{F}_\mathrm{b}) \times \mathcal{F}_\mathrm{b}^T \boldsymbol{C}_\mathrm{ba} +  \mathcal{F}_\mathrm{b}^T \frac{d\boldsymbol{C}_\mathrm{ba}}{dt}
\end{align}
$$

右辺の1項目は$\omega$と$\boldsymbol{C}_\mathrm{ba}$の各縦ベクトルを$\mathcal{F}_\mathrm{b}$で外積を取ったものと考えれば次のように変形できる。

$$
\begin{equation}
\vec{0} = \mathcal{F}_\mathrm{b}^T (\omega^\times \boldsymbol{C}_\mathrm{ba} + \frac{d\boldsymbol{C}_\mathrm{ba}}{dt})
\end{equation}
$$

よってDCMの微分方程式が次のように得られる。

$$
\begin{equation}
% \label{Hughes2004_2.3.5}
\frac{d\boldsymbol{C}_\mathrm{ba}}{dt} + \omega^\times \boldsymbol{C}_\mathrm{ba} = 0
\end{equation}
$$

これで、角速度ベクトルに従って座標系またはDCMを更新できるようになった。
角速度ベクトルがどのように変化していくかを予測するのは運動方程式の仕事なので、[以前の記事](https://thermocraft.space/ja/articles/rigid-body/)とあわせて剛体の運動を予測し記述する準備が整ったことになる。
これと同じ機能を持つ微分方程式を、オイラー角・クォータニオンを用いても書くことが出来るのでこちらも確認しておこう。

## オイラー角 / Euler Angle

オイラー角は回転する物に固定された軸の周りに3回回転させることで、任意の姿勢を表すものだ。
パラメタが少なく、比較的イメージもしやすいので、ものの姿勢を表すのに使いやすい。
一方で、後で述べるように特異点があるので、オイラー角の更新には注意を払う必要がある。
あと個人的には、話しているうちにものを回しているんだか座標を回しているんだか、よく分からなくなる気がする。

回転軸の選び方・順序によってオイラー角はいくつも表現方法があるが、今回はYaw(Z) $\psi$, Pitch(Y) $\theta$, Roll(X) $\varphi$の順で回転させる、3-2-1（ZYX）オイラー角について紹介する。3-2-1オイラー角による回転行列は次のように表される。

$$
\begin{align}
&\boldsymbol{C} = \boldsymbol{C}_1 (\varphi) \boldsymbol{C}_2 (\theta) \boldsymbol{C}_3 (\psi) \\
&= \left[ \begin{array}{ccc} 1 & 0 & 0 \\ 0 & \cos \varphi & \sin \varphi \\ 0 & - \sin \varphi & \cos \varphi \end{array} \right]
\left[ \begin{array}{ccc} \cos \theta & 0 & -\sin \theta \\ 0 & 1 & 0 \\ \sin \theta & 0 & \cos \theta \end{array} \right]
\left[ \begin{array}{ccc} \cos \psi & \sin \psi & 0 \\ -\sin \psi & \cos \psi & 0 \\ 0 & 0 & 1 \end{array} \right] \notag \\
&= \left[ \begin{array}{ccc}
\cos \theta \cos \psi & \cos \theta \sin \psi & - \sin \theta \\
-\cos \varphi \sin \psi + \sin \varphi \sin \theta \cos \psi & \cos \varphi \cos \psi + \sin \varphi \sin \theta \sin \psi & \sin \varphi \cos \theta \\
\sin \varphi \sin \psi + \cos \varphi \sin \theta \cos \psi & - \sin \varphi \cos \psi + \cos \varphi \sin \theta \sin \psi & \cos \varphi \cos \theta
\end{array} \right] \notag
\end{align}
$$

$\cos \theta \neq 0$であれば、DCMからオイラー角への変換は次のように行うことが出来る。

$$
\begin{equation}
\begin{array}{ll}
\theta = \arcsin (-c_{13}) & -\frac{\pi}{2} \le \theta \le \frac{\pi}{2} \\
\varphi = \arctan(c_{23}, c_{33}) & -\pi \le \varphi \le \pi \\
\psi = \arctan(c_{12}, c_{11}) & -\pi \le \psi \le \pi
\end{array}
\end{equation}
$$

$\cos \theta = 0$の場合はこの方法は使えない。仮に$\psi=0$とすれば、$\sin \theta = \pm1$それぞれの場合について、オイラー角は次のように求められる。

$$
\begin{align}
\left[ \begin{array}{cc} c_{21} & c_{22} \\ c_{31} & c_{32} \end{array} \right] &=
\left[ \begin{array}{cc}
-\cos \varphi \sin \psi + \sin \varphi \cos \psi & \cos \varphi \cos \psi + \sin \varphi \sin \psi \\
\sin \varphi \sin \psi + \cos \varphi \cos \psi & -\sin \varphi \cos \varphi + \cos \varphi \sin \psi
\end{array} \right] \notag \\
&= \left[
  \begin{array}{cc} \sin (\varphi - \psi) & \cos (\varphi - \psi) \\
  \cos (\varphi -\psi) & -\sin (\varphi - \psi) \end{array}
  \right]
\end{align}
$$

$$
\begin{equation*}
\text{where} \quad \varphi = \arcsin (-c_{32}), ~~~ \theta = \frac{\pi}{2}, \quad \psi = 0
\end{equation*}
$$

$$
\begin{align}
\left[ \begin{array}{cc} c_{21} & c_{22} \\ c_{31} & c_{32} \end{array} \right] &=
\left[ \begin{array}{cc}
-\cos \varphi \sin \psi - \sin \varphi \cos \psi & \cos \varphi \cos \psi - \sin \varphi \sin \psi \\
\sin \varphi \sin \psi - \cos \varphi \cos \psi & -\sin \varphi \cos \varphi - \cos \varphi \sin \psi
\end{array} \right] \notag \\
&= \left[ \begin{array}{cc} -\sin (\varphi + \psi) & \cos (\varphi + \psi) \\
\cos (\varphi +\psi) & -\sin (\varphi + \psi) \end{array} \right]
\end{align}
$$

$$
\begin{equation*}
\text{where} \quad \varphi = \arcsin (-c_{32}), ~~~ \theta = - \frac{\pi}{2}, ~~~ \psi = 0
\end{equation*}
$$

このように、DCMの形で与えられたものをオイラー角で表す場合は、適当に変換すればよいのだが、オイラー角で姿勢を表していて$\cos \theta = 0$に近づいてしまうと問題が生じる。これを確認するために、オイラー角の更新式を見てみよう。
DCMが3-2-1オイラー角による回転行列からなる時、(9)は次のように変形できる。

<!-- DCMが3-2-1オイラー角による回転行列からなる時、(\label{Hughes2004_2.3.5})は次のように変形できる。 -->

$$
\begin{align}
\boldsymbol{\omega}^\times &= - \dot{\boldsymbol{C}} \boldsymbol{C}^T \notag \\
&= -(\dot{\boldsymbol{C}}_1 \boldsymbol{C}_2 \boldsymbol{C}_3 + \boldsymbol{C}_1 \dot{\boldsymbol{C}}_2 \boldsymbol{C}_3 + \boldsymbol{C}_1 \boldsymbol{C}_2 \dot{\boldsymbol{C}}_3)~\boldsymbol{C}_3^T \boldsymbol{C}_2^T \boldsymbol{C}_1^T \notag \\
&= - \dot{\boldsymbol{C}}_1 \boldsymbol{C}_1^T - \boldsymbol{C}_1 \dot{\boldsymbol{C}}_2 \boldsymbol{C}_2^T \boldsymbol{C}_1^T - \boldsymbol{C}_1 \boldsymbol{C}_2 \dot{\boldsymbol{C}}_3 \boldsymbol{C}_3^T \boldsymbol{C}_2^T \boldsymbol{C}_1^T
\end{align}
$$

1軸周りの回転を考えると(9)から以下の関係が得られる。

<!-- 1軸周りの回転を考えると(\label{Hughes2004_2.3.5})から以下の関係が得られる。 -->

$$
\begin{align}
&- \dot{\boldsymbol{C}}_1 \boldsymbol{C}_1^T = (\dot{\psi} \boldsymbol{i}_1)^\times, \quad
- \dot{\boldsymbol{C}}_2 \boldsymbol{C}_2^T = (\dot{\theta} \boldsymbol{i}_2)^\times, \quad
- \dot{\boldsymbol{C}}_3 \boldsymbol{C}_3^T = (\dot{\varphi} \boldsymbol{i}_3)^\times \\
&\text{where} \quad
\boldsymbol{i}_1 = \left[ \begin{array}{c} 1 \\ 0 \\ 0 \end{array} \right], \quad
\boldsymbol{i}_2 = \left[ \begin{array}{c} 0 \\ 1 \\ 0 \end{array} \right], \quad
\boldsymbol{i}_3 = \left[ \begin{array}{c} 0 \\ 0 \\ 1 \end{array} \right] \notag
\end{align}
$$

これより次の関係が得られる。

$$
\begin{align}
\boldsymbol{\omega}^\times &= - \dot{\boldsymbol{C}} \boldsymbol{C}^T \notag \\
&= (\dot{\psi} \boldsymbol{i}_1)^\times + \boldsymbol{C}_1 (\dot{\theta} \boldsymbol{i}_2)^\times \boldsymbol{C}_1^T + \boldsymbol{C}_1 \boldsymbol{C}_2 (\dot{\varphi} \boldsymbol{i}_3)^\times (\boldsymbol{C}_1 \boldsymbol{C}_2)^T \boldsymbol{C}_1 \notag \\
&= (\dot{\psi} \boldsymbol{i}_1)^\times + (\boldsymbol{C}_1 \dot{\theta} \boldsymbol{i}_2)^\times + (\boldsymbol{C}_1 \boldsymbol{C}_2 \dot{\varphi} \boldsymbol{i}_3)^\times
\end{align}
$$

ちなみに最後の変形は次のように確認することが出来る。まず、基準座標系での外積を2つの形で表す。

$$
\begin{align}
&\vec{u} \times \vec{v} = \mathcal{F}_\mathrm{b}^T \boldsymbol{C}_{ba} (\boldsymbol{u}_\mathrm{a}^\times \boldsymbol{C}_\mathrm{ab} \boldsymbol{v}_\mathrm{b}) \\
&\vec{u} \times \vec{v} = \boldsymbol{u}_\mathrm{a}^T \boldsymbol{C}_\mathrm{ab} \mathcal{F}_\mathrm{b} \times \mathcal{F}_\mathrm{b}^T \boldsymbol{v}_\mathrm{b} = \mathcal{F}_\mathrm{b}^T (\boldsymbol{C}_\mathrm{ba} \boldsymbol{u}_\mathrm{a})^\times \boldsymbol{v}_\mathrm{b}
\end{align}
$$

これらを比較すると、次の関係が成り立つことが分かる。

$$
\begin{equation}
\boldsymbol{C}_{ba} \boldsymbol{u}_\mathrm{a}^\times \boldsymbol{C}_\mathrm{ab} = (\boldsymbol{C}_\mathrm{ba} \boldsymbol{u}_\mathrm{a})^\times
\end{equation}
$$

ベクトル部分が一致するはずなので、

$$
\begin{equation}
% \label{Hughes2004_2.3.24}
\boldsymbol{\omega} = \dot{\psi} \boldsymbol{i}_1 + \boldsymbol{C}_1 \dot{\theta} \boldsymbol{i}_2 + \boldsymbol{C}_1 \boldsymbol{C}_2 \dot{\varphi} \boldsymbol{i}_3
= \boldsymbol{S} (\psi, \theta) \dot{\boldsymbol{\theta}}
\end{equation}
$$

$$
\begin{equation*}
\text{where} \quad
\boldsymbol{S}(\psi, \theta) = \left[ \begin{array}{ccc}
1 & 0 & - \sin \theta \\
0 & \cos \psi & \sin \psi \cos \theta \\
0 & -\sin \psi & \cos \psi \cos \theta
\end{array} \right], ~~
\boldsymbol{\theta} = \left[ \begin{array}{c} \psi \\ \theta \\ \varphi \end{array} \right]
\end{equation*}
$$

(20)に逆行列$\boldsymbol{S}^{-1}$をかけてやれば、オイラー角の更新式が得られる。

$$
\begin{equation}
% \label{Hughes2004_2.3.26}
\dot{\boldsymbol{\theta}} = \boldsymbol{S}^{-1} (\psi, \theta) \boldsymbol{\omega}
\end{equation}
$$

$$
\begin{equation*}
\text{where} \quad
\boldsymbol{S}^{-1} = \left[ \begin{array}{ccc}
1 & \sin \psi \tan \theta & \cos \psi \tan \theta \\
0 & \cos \psi & - \sin \psi \\
0 & \frac{\sin \psi}{\cos \theta}  & \frac{\cos \psi}{\cos \theta}
\end{array} \right], ~~
\boldsymbol{\theta} = \left[ \begin{array}{c} \psi \\ \theta \\ \varphi \end{array} \right]
\end{equation*}
$$

この式を見ると明らかなように、$\cos \theta = 0$となると更新式が発散する。そのため、オイラー角を用いて姿勢の更新を行う場合は、物理的にシステムが特異点に陥らないことを保証する、計算上特異点を回避する、などの対策を考える必要がある。

## クォータニオン / Quaternion

クォータニオンはDCMほどパラメタ数が多くなく、オイラー角のような特異点もないので、実用上便利に用いられる。ちなみに、本によって書き方の流儀（パラメタの順番）が違っていたりするのでよくよく確認したほうがよいのと、Euler Parameterと呼ばれることもあるので、混乱しないように気をつけよう。

まずクォータニオンの書き表し方と計算のルールについて確認する。

$$
\begin{equation}
% \label{eq:Werts1978_D-1}
\boldsymbol{q} \equiv q_4 + i q_1 + j q_2 + k q_3
\end{equation}
$$

$q_4$を実部またはスカラー部、残りの部分を虚部またはベクトル部と呼ぶことにする。
ベクトル部に対応するように、$\boldsymbol{q}' = i q_1 + j q_2 + k q_3$と定めれば、(22)は次のように表すこともできる。

<!-- ベクトル部に対応するように、$\boldsymbol{q}' = i q_1 + j q_2 + k q_3$と定めれば、(\label{eq:Werts1978_D-1})は次のように表すこともできる。 -->

$$
\begin{equation}
% \label{eq:Werts1978_D-5}
\boldsymbol{q} = q_4 + \boldsymbol{q}'
\end{equation}
$$

$i,j,k$の間には次の関係が成り立つ。

$$
\begin{align}
&i^2 = j^2 = k^2 = -1 \\
&ij = -ji = k, ~~ jk = -kj = i, ~~ ki = -ik = j
\end{align}
$$

このルールに基づくとクォータニオン同士の掛け算は次のように表される。

$$
\begin{align}
\boldsymbol{q} \boldsymbol{p} &= (q_4 + i q_1 + j q_2 + k q_3) (p_4 + i p_1 + j p_2 + k p_3) \notag \\
&= (q_4 p_4 - q_1 p_1 - q_2 p_2 - q_3 p_3) + i (q_1 p_4 + q_4 p_1 - q_3 p_2 + q_2 p_3) \notag \\
&+ j (q_2 p_4 + q_3 p_1 - q_4 p_2 - q_1 p_3) + k (q_3 p_4 - q_2 p_1 + q_1 p_2 - q_4 p_3)
\end{align}
$$

<!-- (\ref{eq:Werts1978_D-5})の形を用いて整理すれば、次のように表すことができる。 -->

(23)の形を用いて整理すれば、次のように表すことができる。

$$
\begin{equation}
% \label{eq:Werts1978_D-8}
\boldsymbol{q} \boldsymbol{p} = (q_4 + \boldsymbol{q}')(p_4 + \boldsymbol{p}') = q_4 p_4 + q_4 \boldsymbol{p}' + p_4 \boldsymbol{q}' - \boldsymbol{q}' \cdot \boldsymbol{p}' + \boldsymbol{q}' \times \boldsymbol{p}'
\end{equation}
$$

ここまでは単純にクォータニオンのルールなので「そういうものです」ということにしておこう。ここからが「そんなもの考えて何がうれしいの？」と言う部分だ。
このクォータニオンのルールに従うと、ベクトル$\boldsymbol{r}$を単位ベクトル$\boldsymbol{n}$の周りで$\theta$だけ回転させるようなクォータニオンというものを作ることが出来る。

$$
\begin{equation}
% \label{eq:Werts1978_12-11}
\boldsymbol{q} = \cos \frac{\theta}{2} + \boldsymbol{n} \sin \frac{\theta}{2}
\end{equation}
$$

計算方法は以下のとおりだ。回転させたいのはベクトル$\boldsymbol{r}$だが、計算上はスカラー成分ゼロのクォータニオンを考える。

$$
\begin{align}

&\boldsymbol{r}' = \boldsymbol{q} \boldsymbol{r} \boldsymbol{q}^* = \left( \cos \frac{\theta}{2} + \boldsymbol{n} \sin \frac{\theta}{2} \right) \boldsymbol{r} \left( \cos \frac{\theta}{2} - \boldsymbol{n} \sin \frac{\theta}{2} \right) \notag \\
&= \left( -(\boldsymbol{n}\cdot \boldsymbol{r}) \sin \frac{\theta}{2} + \boldsymbol{r} \cos \frac{\theta}{2} + (\boldsymbol{n} \times \boldsymbol{r}) \sin \frac{\theta}{2} \right) \left( \cos \frac{\theta}{2} - \boldsymbol{n} \sin \frac{\theta}{2} \right) \notag \\

&= \sin^2 \frac{\theta}{2} (\boldsymbol{n} \cdot \boldsymbol{r}) \boldsymbol{n} + \cos^2 \frac{\theta}{2} \boldsymbol{r} + \sin \frac{\theta}{2} \cos \frac{\theta}{2} (\boldsymbol{n} \times \boldsymbol{r})
- \cos \frac{\theta}{2} \sin \frac{\theta}{2} (\boldsymbol{r} \times \boldsymbol{n}) - \sin^2 \frac{\theta}{2} (\boldsymbol{n} \times  \boldsymbol{r}) \times \boldsymbol{n} \notag \\
&= \sin^2 \frac{\theta}{2} (\boldsymbol{n} \cdot \boldsymbol{r}) \boldsymbol{n} + \cos^2 \frac{\theta}{2} \boldsymbol{r} + 2 \sin \frac{\theta}{2} \cos \frac{\theta}{2} (\boldsymbol{n} \times \boldsymbol{r}) - \sin^2 \frac{\theta}{2} ((\boldsymbol{n} \cdot \boldsymbol{n}) \boldsymbol{r} - (\boldsymbol{r} \cdot \boldsymbol{n}) \boldsymbol{n}) \notag \\
&= 2 \sin^2 \frac{\theta}{2} (\boldsymbol{n} \cdot \boldsymbol{r}) \boldsymbol{n} + \left( \cos^2 \frac{\theta}{2} - \sin^2 \frac{\theta}{2} \right) \boldsymbol{r} + \sin \theta (\boldsymbol{n} \times  \boldsymbol{r}) \notag \\
&=(\boldsymbol{n} \cdot \boldsymbol{r}) \boldsymbol{n} + \cos \theta (\boldsymbol{r} - (\boldsymbol{n} \cdot \boldsymbol{r}) \boldsymbol{n}) + \sin \theta (\boldsymbol{n} \times \boldsymbol{r})

\end{align}
$$

式変形の結果を下図と見比べると、確かにベクトルの回転になっていることが分かると思う。

![dcm-euler-quaternion-1](../figures/dcm-euler-quaternion-1.svg)
_Figure 1: Vector Rotation by Quaternion._

ベクトルをある方向に回転させるような変換は、基底を逆回転させることに対応するのでDCMは$\boldsymbol{q}^* \boldsymbol{I} \boldsymbol{q}$のように表すことができる。

$$
\begin{equation}
\boldsymbol{C} = \left[ \begin{array}{ccc}
q_1^2 - q_2^2 - q_3^2 + q_4^2 & 2(q_1 q_2 + q_3 q_4) & 2(q_1 q_3 - q_2 q_4) \\
2(q_1 q_2 - q_3 q_4) & - q_1^2 + q_2^2 - q_3^2 + q_4^2 & 2(q_2 q_3 - q_1 q_4) \\
2(q_1 q_3 + q_2 q_4) & 2(q_2 q_3 - q_1 q_4) & - q_1^2 - q_2^2 + q_3^2 + q_4^2
\end{array} \right]
\end{equation}
$$

DCMからクォータニオンは例えば次のように決めることが出来る。

$$
\begin{align}
&q_4 = \frac{1}{2} \sqrt{1 + c_{11} + c_{22} + c_{33}} \\
&q_1 = \frac{1}{4q_4} (c_{23} - c_{32}), ~~
q_2 = \frac{1}{4q_4} (c_{31} - c_{13}), ~~
q_3 = \frac{1}{4q_4} (c_{12} - c_{21})
\end{align}
$$

最後に角速度ベクトル$\boldsymbol{\omega}$によるクォータニオンの更新式を導出しよう。ここで、座標系をクォータニオン$\boldsymbol{p}$によって変換し、その後クォータニオン$\boldsymbol{q}$によって変換するという手順を考える。

$$
\begin{equation}
\mathcal{F}_\mathrm{c} \xleftarrow{\boldsymbol{q}} \mathcal{F}_\mathrm{b} \xleftarrow{\boldsymbol{p}} \mathcal{F}_\mathrm{a}
\end{equation}
$$

あるベクトル$\boldsymbol{r}$が基準座標系に固定されているものとして、各座標系においてどのようにパラメタ表示されるかというと

$$
\begin{equation}
\mathcal{F}_\mathrm{c}^T (\boldsymbol{q}^* \boldsymbol{p}^* \boldsymbol{r} \boldsymbol{p} \boldsymbol{q}) = \mathcal{F}_\mathrm{b}^T (\boldsymbol{p}^* \boldsymbol{r} \boldsymbol{p}) =
\mathcal{F}_\mathrm{a}^T \boldsymbol{r}
\end{equation}
$$

式を見ると、$\boldsymbol{p} \boldsymbol{q}$というクォータニオンが2つの変換を合わせたクォータニオンになっていることが分かる。このとき、2つ目の変換$\boldsymbol{q}$を軸$\boldsymbol{n}$、角度$d\theta$の回転として、最終的な変換がどのように表されるかを考よう。

$$
\begin{align}
&\boldsymbol{q} = \cos \frac{d\theta}{2} + i~n_1\sin \frac{d\theta}{2} + j~n_2\sin \frac{d\theta}{2} + k~n_3 \sin \frac{d\theta}{2}, \\
&\mathrm{where} \quad \boldsymbol{n} = \frac{\omega}{|\omega|}, ~ |\omega| dt = d\theta \notag
\end{align}
$$

地道に掛け算して、一次近似してやれば次のようになる。

$$
\begin{align}
\boldsymbol{p} \boldsymbol{q} &\simeq \left( p_4 - p_1 n_1 \frac{d\theta}{2} - p_2 n_2 \frac{d\theta}{2} - p_3 n_3 \frac{d\theta}{2} \right) + i \left( p_1 + p_4 n_1 \frac{d\theta}{2} - p_3 n_2 \frac{d\theta}{2} + p_2 n_3 \frac{d\theta}{2} \right) \notag \\
&+ j \left( p_2 + p_3 n_1 \frac{d\theta}{2} + p_4 n_2 \frac{d\theta}{2} - p_1 n_3 \frac{d\theta}{2} \right) + k \left( p_3 - p_2 n_1 \frac{d\theta}{2} + p_1 n_2 \frac{d\theta}{2} + p_4 n_3 \frac{d\theta}{2} \right) \notag \\
&= \boldsymbol{p} + \frac{1}{2} (- p_1 \omega_1 - p_2 \omega_2 - p_3 \omega_3) + \frac{i}{2} (p_4 \omega_1 - p_3 \omega_2 + p_2 \omega_3) \notag \\
&+ \frac{j}{2} (p_3 \omega_1 + p_4 \omega_2 - p_1 \omega_3) + \frac{k}{2} (-p_2 \omega_1 + p_1 \omega_2 + p_4 \omega_3)
\end{align}
$$

行列の形で表せばクォータニオンの更新式は次のように表される。

$$
\begin{equation}
\frac{d}{dt}\left[ \begin{array}{c} q_1 \\ q_2 \\ q_3 \\ q_4 \end{array} \right] =\frac{1}{2}
\left[ \begin{array}{cccc}
0 & \omega_3 & - \omega_2 & \omega_1\\ -\omega_3 & 0 & \omega_1 & \omega_2 \\
\omega_2 & -\omega_1 & 0 & \omega_3 \\ -\omega_1 & -\omega_2 & -\omega_3 & 0
\end{array} \right]
\left[ \begin{array}{c} q_1 \\ q_2 \\ q_3 \\ q_4 \end{array} \right]
\end{equation}
$$

## Reference

DCM、オイラー角、クォータニオンによる姿勢の表現と更新式について一通り確認した。
宇宙機の姿勢表現や3Dモデリングなどの分野ではクォータニオンがよく用いられるようだが、「姿勢表現はクォータニオンでないといけない」と言うものでもない。
それぞれの方法を理解して、目的や状況に合わせて適切な方法を使えるのが一番だ。

この記事のDCMとオイラー角に関する部分は[[1]](#reference)を参考に、クォータニオンに関する部分は[[2]](#reference)を参考にしつつ書いている。どちらも宇宙機の姿勢運動に関する名著と呼ばれるものなので、より詳しく理解したい場合には参照してもらいたい。

1. Peter C. Hughes, "Spacecraft Attitude Dynamics", Dover Publications, 2004
2. James R. Wertz, "Spacecraft Attitude Determination and Control", Springer, 1978
