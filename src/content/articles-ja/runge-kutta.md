---
title: 'ルンゲクッタ法（RK4）の精度確認'
description: '常微分方程式の時間発展を計算する際に、手軽に用いられる手法にルンゲクッタ法（RK4）があります。この記事では、ルンゲクッタ法が4次精度を達成していることを確認します。'
pubDate: 2025-05-29
updatedDate: 2025-05-29
heroImage: ''
tags: ['numerical analysis']
---

## ルンゲクッタ法の更新式

常微分方程式(1)の時間発展を計算したい場合、手軽に使用できるのが古典的なルンゲクッタ法だ。

$$
\begin{equation}
% \label{eq:differential}
\frac{dx}{dt} = f(x, t)
\end{equation}
$$

これを用いると、$h$を時間ステップ幅として次のように計算を進めることができる。

$$
\begin{align}
% \label{eq:runge_kutta}
&k_1 = f(x, t) \\
&k_2 = f(x + \frac{h}{2} k_1,~ t + \frac{h}{2}) \\
&k_3 = f(x + \frac{h}{2} k_2,~ t + \frac{h}{2}) \\
&k_4 = f(x + h k_3,~ t + h) \\
\end{align}
$$

$$
\begin{equation}
\Delta x = \frac{h}{6} (k_1 + 2k_2 + 2k_3 + k_4)
\end{equation}
$$

ルンゲクッタ法の利点は、4次精度の近似を行いつつも更新式が非常にシンプルに表されている点だ。4次精度を得たいだけであれば、それを実現するようなスキームはいくらでも作り出すことが出来るので、ルンゲクッタ法の重要なポイントは手順のシンプルさにあると言ってもいいだろう。
今回は、(2)の表記は与えられているものとして、これが4次精度を実現出来ているということを確認してみたい。

## 4次精度の意味

まず4次精度ということの意味はなんだろうか。ある点$(x_0, t_0)$の周りで(3)と近似したとき、そのままの形で、点$(x_0, t_0)$でのテイラー展開(4)と4次の項まで一致すれば文句なしだが、そんなことはない。
(3)の$h$についての微係数が4次まで一致するのである。（あるいは(3)を$h$についてテイラー展開し直したものが(4)と4次の項まで一致するといってもいい。）

$$
\begin{equation}
% \label{eq:x0}
\tilde{x} = x_0 + \frac{h}{6} (k_1 + 2k_2 + 2k_3 + k_4)
\end{equation}
$$

$$
\begin{equation}
% \label{eq:taylor}
x = x_0 + \frac{dx}{dt} h + \frac{1}{2} \frac{d^2x}{dt^2} h^2 + \frac{1}{6} \frac{d^3x}{dt^3} h^3 + \frac{1}{24} \frac{d^4x}{dt^4} h^4 + O(h^5)
\end{equation}
$$

$\tilde{x}$の$h$に関する微分を見てみよう。

$$
\begin{align}
\frac{d\tilde{x}}{dh} &= \frac{1}{6} (k_1 + 2k_2 + 2k_3 + k_4)
+ \frac{h}{6} \left[ \frac{dk_1}{dh} + 2\frac{dk_2}{dh} + 2\frac{dk_3}{dh} + \frac{dk_4}{dh}\right] \\
\frac{d^2\tilde{x}}{dh^2} &= \frac{1}{3} \left[ \frac{dk_1}{dh} + 2\frac{dk_2}{dh} + 2\frac{dk_3}{dh} + \frac{dk_4}{dh}\right]
+ \frac{h}{6} \left[ \frac{d^2k_1}{dh^2} + 2\frac{d^2k_2}{dh^2} + 2\frac{d^2k_3}{dh^2} + \frac{d^2k_4}{dh^2}\right] \\
\frac{d^3\tilde{x}}{dh^3} &= \frac{1}{2} \left[ \frac{d^2k_1}{dh^2} + 2\frac{d^2k_2}{dh^2} + 2\frac{d^2k_3}{dh^2} + \frac{d^2k_4}{dh^2}\right]
+ \frac{h}{6} \left[ \frac{d^3k_1}{dh^3} + 2\frac{d^3k_2}{dh^3} + 2\frac{d^3k_3}{dh^3} + \frac{d^3k_4}{dh^3}\right] \\
\frac{d^4\tilde{x}}{dh^4} &= \frac{2}{3} \left[ \frac{d^3k_1}{dh^3} + 2\frac{d^3k_2}{dh^3} + 2\frac{d^3k_3}{dh^3} + \frac{d^3k_4}{dh^3}\right]
+ \frac{h}{6} \left[ \frac{d^4k_1}{dh^4} + 2\frac{d^4k_2}{dh^4} + 2\frac{d^4k_3}{dh^4} + \frac{d^4k_4}{dh^4}\right]
\end{align}
$$

$h=0$での微係数についてのみ考えたいので、いずれの場合も$h$の1次の項になっている部分は無視してしまってよい。
これらが、$\frac{dx}{dt}, \frac{d^2x}{dt^2}, \frac{d^3x}{dt^3}, \frac{d^4x}{dt^4}$と一致することを示せば、4次精度（ルンゲクッタ法1ステップの誤差が$O(h^5)$オーダーであるということ）が確認できたことになる。

## ルンゲクッタ法の精度確認

ここからは、地道に4次精度となっていることを確認していこう。
まず、実現するべき1~4次の微係数を求めておく。

$$
\begin{equation}
\frac{dx}{dt} = f
\end{equation}
$$

$$
\begin{equation}
\frac{d^2x}{dt^2} = \frac{\partial f}{\partial t} + \frac{\partial f}{\partial x} \frac{dx}{dt} = \frac{\partial f}{\partial t} + \frac{\partial f}{\partial x} f
\end{equation}
$$

$$
\begin{align}
\frac{d^3x}{dt^3} &= \frac{\partial}{\partial t} \left( \frac{\partial f}{\partial t} + \frac{\partial f}{\partial x} f \right)+ \frac{\partial}{\partial x} \left( \frac{\partial f}{\partial t} + \frac{\partial f}{\partial x} f \right) f \notag \\
&= \frac{\partial^2 f}{\partial t^2} + \frac{\partial^2 f}{\partial t \partial x} f + \frac{\partial f}{\partial x} \frac{\partial f}{\partial t} + \frac{\partial^2 f}{\partial x \partial t} f + \frac{\partial^2 f}{\partial x^2} f^2 + \left( \frac{\partial f}{\partial x} \right)^2 f \notag \\
&= \frac{\partial^2 f}{\partial t^2} + 2\frac{\partial^2 f}{\partial t \partial x} f + \frac{\partial f}{\partial x} \frac{\partial f}{\partial t} + \frac{\partial^2 f}{\partial x^2} f^2 + \left( \frac{\partial f}{\partial x} \right)^2 f
\end{align}
$$

$$
\begin{align}
\frac{d^4x}{dt^4} &= \frac{\partial}{\partial t} \left( \frac{d^3x}{dt^3} \right)+ \frac{\partial}{\partial x} \left( \frac{d^3x}{dt^3} \right) f \notag \\
&= \frac{\partial^3 f}{\partial t^3} + 2 \frac{\partial^3 f}{\partial t^2 \partial x} f + 2 \frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial t} + \frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial t} + \frac{\partial f}{\partial x} \frac{\partial^2 f}{\partial t^2} \notag \\
&~~~+ \frac{\partial^3 f}{\partial t \partial x^2} f^2 + 2 \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial t} f + 2 \frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial x} f + \left( \frac{\partial f}{\partial x} \right)^2 \frac{\partial f}{\partial t} \notag \\
&~~~+ \frac{\partial^3 f}{\partial x \partial t^2} f + 2 \frac{\partial^3 f}{\partial t \partial x^2} f^2 + 2 \frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial x} f + \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial t} f + \frac{\partial f}{\partial x} \frac{\partial^2 f}{\partial x \partial t} f \notag \\
&~~~+ \frac{\partial^3 f}{\partial x^3} f^3 + 2 \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial x} f^2 + 2 \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial x} f^2 + \left( \frac{\partial f}{\partial x} \right)^3 f \notag \\
&= \frac{\partial^3 f}{\partial t^3} + 3 \frac{\partial^3 f}{\partial t^2 \partial x} f + 3 \frac{\partial^3 f}{\partial t \partial x^2} f^2 + \frac{\partial^3 f}{\partial x^3} f^3 \notag \\
&~~~+ 3\frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial t} + 5 \frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial x}f + 3 \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial t} f + \frac{\partial f}{\partial x} \frac{\partial^2 f}{\partial t^2} + 4 \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial x} f^2 \notag \\
&~~~+ \left( \frac{\partial f}{\partial x} \right)^2 \frac{\partial f}{\partial t} + \left( \frac{\partial f}{\partial x} \right)^3 f
\end{align}
$$

次に、$k_1, k_2, k_3, k_4$の$h$に関する微分を求めていきたいのだが、まず準備として$f(X(h), T(h))$の微分を求めておく。

$$
\begin{equation}
% \label{eq:f'}
\frac{df}{dh} = \left( \frac{dX}{dh} \frac{\partial}{\partial X} + \frac{dT}{dh} \frac{\partial}{\partial T} + \frac{\partial}{\partial h} \right) f
= \frac{\partial f}{\partial X} \frac{dX}{dh} + \frac{\partial f}{\partial T} \frac{dT}{dh}
\end{equation}
$$

$$
\begin{align}
% \label{eq:f''}
\frac{d^2f}{dh^2} &= \left( \frac{dX}{dh} \frac{\partial}{\partial X} + \frac{dT}{dh} \frac{\partial}{\partial T} + \frac{\partial}{\partial h} \right)^2 f \notag \\
&= \left[ \left( \frac{dX}{dh} \right)^2 \frac{\partial^2}{\partial X^2} + 2 \frac{dX}{dh} \frac{dT}{dh} \frac{\partial^2}{\partial X \partial T} + \left( \frac{dT}{dh} \right)^2 \frac{\partial^2}{\partial T^2}
+ \frac{\partial}{\partial h} \left( \frac{dX}{dh} \frac{\partial}{\partial X} \right) +\frac{\partial}{\partial h} \left( \frac{dT}{dh} \frac{\partial}{\partial T} \right) \right] f \notag \\
&= \left[ \left( \frac{dX}{dh} \right)^2 \frac{\partial^2}{\partial X^2} + 2 \frac{dX}{dh} \frac{dT}{dh} \frac{\partial^2}{\partial X \partial T} + \left( \frac{dT}{dh} \right)^2 \frac{\partial^2}{\partial T^2} + \frac{d^2X}{dh^2} \frac{\partial}{\partial X} + \frac{d^2T}{dh^2} \frac{\partial}{\partial T} \right] f
\end{align}
$$

$$
\begin{align}
% \label{eq:f'''}
\frac{d^3f}{dh^3} &= \left( \frac{dX}{dh} \frac{\partial}{\partial X} + \frac{dT}{dh} \frac{\partial}{\partial T} + \frac{\partial}{\partial h} \right)^3 f \notag \\
&= \left[ \left( \frac{dX}{dh} \right)^3 \frac{\partial^3}{\partial X^3} + 3 \left( \frac{dX}{dh} \right)^2 \frac{dT}{dh} \frac{\partial^3}{\partial X^2 \partial T}+ 3 \frac{dX}{dh} \left( \frac{dT}{dh} \right)^2 \frac{\partial^3}{\partial X \partial T^2} + \left( \frac{dT}{dh} \right)^3 \frac{\partial^3}{\partial T^3} \right. \notag \\
&\hspace{12pt}+ 3 \frac{dX}{dh} \frac{d^2X}{dh^2} \frac{\partial^2}{\partial X^2} + 3\left( \frac{dX}{dh} \frac{d^2T}{dh^2} + \frac{dT}{dh} \frac{d^2X}{dh^2} \right) \frac{\partial^2}{\partial X \partial T} + 3 \frac{dT}{dh} \frac{d^2T}{dh^2} \frac{\partial^2}{\partial T^2} \notag \\
&\hspace{12pt} \left.+ \frac{d^3X}{dh^3} \frac{\partial}{\partial X} + \frac{d^3T}{dh^3} \frac{\partial}{\partial T} \right] f
\end{align}
$$

$k_1$は$h$に依らないので、次の関係が成り立つ。

$$
\begin{equation}
\frac{dk_1}{dh} = 0
\end{equation}
$$

$k_2$に関しては、$X=x+\frac{h}{2} k_1,~ T=t+\frac{h}{2}$より、$\frac{dX}{dh}, \frac{dT}{dh}$は次のように表される。

$$
\begin{equation}
\frac{dX}{dh} = \frac{k_1}{2} + \frac{h}{2}\frac{dk_1}{dh} = \frac{f}{2}, \quad
\frac{dT}{dh} = \frac{1}{2}
\end{equation}
$$

これを$\frac{df}{dh}$, $\frac{d^2f}{dh^2}$, $\frac{d^3f}{dh^3}$に代入すれば、$k_2$の微分が得られる。

$$
\begin{equation}
\frac{dk_2}{dh} = \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t}
\end{equation}
$$

$$
\begin{equation}
\frac{d^2 k_2}{dh^2} = \frac{f^2}{4} \frac{\partial^2 f}{\partial x^2} + \frac{f}{2} \frac{\partial^2 f}{\partial x \partial t} + \frac{1}{4} \frac{\partial^2 f}{\partial t^2}
\end{equation}
$$

$$
\begin{equation}
\frac{d^3 k_2}{dh^3} = \frac{f^3}{8} \frac{\partial^3 f}{\partial x^3} + \frac{3f^2}{8} \frac{\partial^3 f}{\partial x^2 \partial t} + \frac{3f}{8} \frac{\partial^3 f}{\partial x \partial t^2} + \frac{1}{8} \frac{\partial^3 f}{\partial t^3}
\end{equation}
$$

次に$k_3$に関しては、$X=x+\frac{h}{2} k_2,~ T=t+\frac{h}{2}$より、それぞれの微分は次のように表される。

$$
\begin{equation}
\frac{dX}{dh} = \frac{k_2}{2} + \frac{h}{2}\frac{dk_2}{dh}, \quad
\frac{dT}{dh} = \frac{1}{2}
\end{equation}
$$

$$
\begin{equation}
\frac{d^2X}{dh^2} = \frac{dk_2}{dh} + \frac{h}{2}\frac{d^2k_2}{dh^2}, \quad
\frac{d^2T}{dh^2} = 0
\end{equation}
$$

$$
\begin{equation}
\frac{d^3X}{dh^3} = \frac{3}{2} \frac{d^2k_2}{dh^2} + \frac{h}{2}\frac{d^3k_2}{dh^3}
\end{equation}
$$

最終的に必要なのは$h=0$の時の値なので、その場合に限ってのみ$k_3$の微分を求める。

$$
\begin{equation}
\left. \frac{dk_3}{dh} \right|_{h=0} = \frac{k_2}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t}
\end{equation}
$$

$$
\begin{align}
\left. \frac{d^2k_3}{dh^2} \right|_{h=0} = \frac{k_2^2}{4} \frac{\partial^2 f}{\partial x^2} + \frac{k_2}{2} \frac{\partial^2 f}{\partial x \partial t}+ \frac{1}{4} \frac{\partial^2 f}{\partial t^2} + \frac{dk_2}{dh} \frac{\partial f}{\partial x}
\end{align}
$$

$$
\begin{align}
\left. \frac{d^3k_3}{dh^3} \right|_{h=0}
=&\frac{k_2^3}{8} \frac{\partial^3 f}{\partial x^3} + \frac{3k_2^2}{8} \frac{\partial^3 f}{\partial x^2 \partial t} + \frac{3 k_2}{8} \frac{\partial^3 f}{\partial x \partial t^2} \notag \\
&+ \frac{1}{8} \frac{\partial^3 f}{\partial t^3} + \frac{3 k_2}{2} \frac{dk_2}{dh} \frac{\partial^2 f}{\partial x^2} + \frac{3}{2} \frac{dk_2}{dh} \frac{\partial^2 f}{\partial x \partial t} + \frac{3}{2} \frac{d^2 k_2}{dh^2} \frac{\partial f}{\partial x}
\end{align}
$$

同様に$k_4$に関しては、次のように求めることが出来る。

$$
\begin{equation}
\frac{dX}{dh} = k_3 + h \frac{dk_3}{dh}, \quad
\frac{dT}{dh} = 1
\end{equation}
$$

$$
\begin{equation}
\frac{d^2X}{dh^2} = 2 \frac{dk_3}{dh} +h \frac{d^2k_3}{dh^2}, ~~~
\frac{d^2T}{dh^2} = 0
\end{equation}
$$

$$
\begin{equation}
\frac{d^3X}{dh^3} = 3 \frac{d^2k_3}{dh^2} +h \frac{d^3k_3}{dh^3}
\end{equation}
$$

$h=0$での$k_4$の微分は次のように表される。

$$
\begin{equation}
\left. \frac{dk_4}{dh} \right|_{h=0} = k_3 \frac{\partial f}{\partial x} + \frac{\partial f}{\partial t}
\end{equation}
$$

$$
\begin{align}
\left.\frac{d^2k_4}{dh^2} \right|_{h=0} = k_3^2 \frac{\partial^2 f}{\partial x^2} + 2 k_3 \frac{\partial^2 f}{\partial x \partial t}
+ \frac{\partial^2 f}{\partial t^2} + 2\frac{dk_3}{dh} \frac{\partial f}{\partial x}
\end{align}
$$

$$
\begin{align}
\left.\frac{d^3k_4}{dh^3} \right|_{h=0} =& k_3^3 \frac{\partial^3 f}{\partial x^3} + 3k_3^2 \frac{\partial^3 f}{\partial x^2 \partial t} + 3k_3 \frac{\partial^3 f}{\partial x \partial t^2} + \frac{\partial^3 f}{\partial t^3} \notag \\
&+ 6 k_3 \frac{dk_3}{dh} \frac{\partial^2 f}{\partial x^2} + 6 \frac{dk_3}{dh} \frac{\partial^2 f}{\partial x \partial t} + 3 \frac{d^2 k_3}{dh^2} \frac{\partial f}{\partial x}
\end{align}
$$

これで（やっと）準備が整ったので、係数比較を行っていこう。

1次：

$$
\begin{equation}
\left. \frac{1}{6} (k_1 + 2k_2 + 2k_3 + k_4) \right|_{h=0} = f
\end{equation}
$$

2次：

$$
\begin{align}
&\frac{1}{3} \left[ \frac{dk_1}{dh} + 2\frac{dk_2}{dh} + 2\frac{dk_3}{dh} + \frac{dk_4}{dh} \right]_{h=0} \notag \\
&= \frac{1}{3} \left[ 2\left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) + 2\left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) + f \frac{\partial f}{\partial x} + \frac{\partial f}{\partial t} \right] \notag \\
&= f \frac{\partial f}{\partial x} +\frac{\partial f}{\partial t}
\end{align}
$$

3次：

$$
\begin{align}
&\frac{1}{2} \left[ \frac{d^2k_1}{dh^2} + 2\frac{d^2k_2}{dh^2} + 2\frac{d^2k_3}{dh^2} + \frac{d^2k_4}{dh^2}\right] _{h=0} \notag \\

&= \frac{1}{2} \left[ 2\left( \frac{f^2}{4} \frac{\partial^2 f}{\partial x^2} + \frac{f}{2} \frac{\partial^2 f}{\partial x \partial t} + \frac{1}{4} \frac{\partial^2 f}{\partial t^2} \right) \right. \notag \\
&\hspace{20pt}\left. + 2 \left( \frac{f^2}{4} \frac{\partial^2 f}{\partial x^2} + \frac{f}{2} \frac{\partial^2 f}{\partial x \partial t} + \frac{1}{4} \frac{\partial^2 f}{\partial t^2} + \left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) \frac{\partial f}{\partial x} \right) \right. \notag\\
&\hspace{20pt}\left. + \left( f^2 \frac{\partial^2 f}{\partial x^2} + 2f \frac{\partial^2 f}{\partial x \partial t} + \frac{\partial^2 f}{\partial t^2} + 2 \left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) \frac{\partial f}{\partial x} \right) \right] \notag \\

&= \frac{\partial^2 f}{\partial x^2} + 2f \frac{\partial^2 f}{\partial x \partial t} + \frac{\partial^2 f}{\partial t^2} + f \left( \frac{\partial f}{\partial x} \right)^2 + \frac{\partial f}{\partial t} \frac{\partial f}{\partial x}
\end{align}
$$

4次：

$$
\begin{align}

&\frac{2}{3} \left[ \frac{d^3k_1}{dh^3} + 2\frac{d^3k_2}{dh^3} + 2\frac{d^3k_3}{dh^3} + \frac{d^3k_4}{dh^3}\right]_{h=0} \notag \\

&= \frac{2}{3} \left[ 2\left( \frac{f^3}{8} \frac{\partial^3 f}{\partial x^3} + \frac{3f^2}{8} \frac{\partial^3 f}{\partial x^2 \partial t} + \frac{3f}{8} \frac{\partial^3 f}{\partial x \partial t^2} + \frac{1}{8} \frac{\partial^3 f}{\partial t^3} \right) \right. \notag \\
&\hspace{11pt}+ 2 \left( \frac{k_2^3}{8} \frac{\partial^3 f}{\partial x^3} + \frac{3k_2^2}{8} \frac{\partial^3 f}{\partial x^2 \partial t} + \frac{3 k_2}{8} \frac{\partial^3 f}{\partial x \partial t^2} + \frac{1}{8} \frac{\partial^3 f}{\partial t^3}
+ \frac{3 k_2}{2} \frac{dk_2}{dh} \frac{\partial^2 f}{\partial x^2} + \frac{3}{2} \frac{dk_2}{dh} \frac{\partial^2 f}{\partial x \partial t} + \frac{3}{2} \frac{d^2 k_2}{dh^2} \frac{\partial f}{\partial x} \right) \notag \\
&\hspace{11pt}+ \left( k_3^3 \frac{\partial^3 f}{\partial x^3} + 3k_3^2 \frac{\partial^3 f}{\partial x^2 \partial t} + 3k_3 \frac{\partial^3 f}{\partial x \partial t^2} + \frac{\partial^3 f}{\partial t^3}
\left.+ 6 k_3 \frac{dk_3}{dh} \frac{\partial^2 f}{\partial x^2} + 6 \frac{dk_3}{dh} \frac{\partial^2 f}{\partial x \partial t} + 3 \frac{d^2 k_3}{dh^2} \frac{\partial f}{\partial x} \right) \right] \notag \\

&= \frac{\partial^3 f}{\partial t^3} + 3 \frac{\partial^3 f}{\partial t^2 \partial x} f + 3 \frac{\partial^3 f}{\partial t \partial x^2} f^2 + \frac{\partial^3 f}{\partial x^3} f^3 \notag \\
&\hspace{11pt}+ 3\frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial t} + 5 \frac{\partial^2 f}{\partial t \partial x} \frac{\partial f}{\partial x}f + 3 \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial t} f + \frac{\partial f}{\partial x} \frac{\partial^2 f}{\partial t^2} + 4 \frac{\partial^2 f}{\partial x^2} \frac{\partial f}{\partial x} f^2 \notag \\
&\hspace{11pt}+ \left( \frac{\partial f}{\partial x} \right)^2 \frac{\partial f}{\partial t} + \left( \frac{\partial f}{\partial x} \right)^3 f

\end{align}
$$

以上で各微係数が一致することが確認できた。途中の式変形をほぼ省略せずに書いているので、何かの参考になれば幸いだ。

## Notation Table

| Symbol                              | Definition                                  |
| ----------------------------------- | ------------------------------------------- |
| $f(x,t)$                            | Function of ODE                             |
| $h$                                 | Step size                                   |
| $k_i$                               | Runge-Kutta intermediate values             |
| $\frac{\partial^n f}{\partial x^n}$ | n-th partial derivative with respect to $x$ |
