---
title: "Proving Runge-Kutta Method (RK4) is 4th Order"
description: "If you want to calculate the time evolution of ODE, one of the most common and simple methods is the Runge-Kutta method (RK4). In this article, we will confirm that RK4 achieves fourth-order accuracy."
pubDate: 2025-05-29
updatedDate: 2025-05-29
heroImage: ""
tags: ["numerical analysis"]
---

## Update Formula of Runge-Kutta Method

If you want to calculate the time evolution of Eq. (1), one of the most common and simple methods is the Runge-Kutta method (RK4).

$$
\begin{equation}
% \label{eq:differential}
\frac{dx}{dt} = f(x, t)
\end{equation}
$$

Using the Runge-Kutta method, you can update the state $x$ at time $t$ to the next state at time $t+h$ as shown below.

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

The major advantage of the Runge-Kutta method is that it provides a fourth-order approximation while keeping the update formula very simple.
If you just want to achieve fourth-order accuracy, there are various possibilities to create such schemes.
Therefore, the key point of the Runge-Kutta method is its simplicity in procedure.

In this article, we assume that the notation in Eq. (2) is given, and will confirm that it indeed achieves fourth-order accuracy.

## Meaning of Fourth-Order Accuracy

First of all, what does fourth-order accuracy mean?
When approximating $x$ around a point $(x_0, t_0)$ using Eq. (3), it means that Eq. (3) match the Taylor expansion up to the fourth order.
However, Eq. (3) does not match the Taylor expansion in its original form.
If we evaluate the derivative of Eq. (3) with respect to $h$, we find that the coefficients up to the fourth order match.

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

Let's take a look at the derivative of $\tilde{x}$ with respect to $h$.

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

Since we are only interested in the derivatives at $h=0$, we can ignore the terms with $h$ in each formula.
If we prove that these derivatives match $\frac{dx}{dt}$, $\frac{d^2x}{dt^2}$, $\frac{d^3x}{dt^3}$, and $\frac{d^4x}{dt^4}$, then we can confirm that the Runge-Kutta method has fourth-order accuracy (in other words, the error of one step is $O(h^5)$).

## Proving Fourth-Order Accuracy

Now, let us confirm that the Runge-Kutta method indeed achieves fourth-order accuracy.
First, we will derive the derivatives of $x$ based on the original differential equation Eq. (1).

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

Second, we should derive the derivatives of $k_1$, $k_2$, $k_3$, $k_4$ with respect to $h$.
As a preparation, we derive the derivative of $f(X(h), T(h))$ with respect to $h$.

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

$k_1$ is not dependent on $h$, so the following relationship holds.

$$
\begin{equation}
\frac{dk_1}{dh} = 0
\end{equation}
$$

For $k_2$ calculation, we have $X=x+\frac{h}{2} k_1$, and $T=t+\frac{h}{2}$, so the derivatives with respect to $h$ can be expressed as follows.

$$
\begin{equation}
\frac{dX}{dh} = \frac{k_1}{2} + \frac{h}{2}\frac{dk_1}{dh} = \frac{f}{2}, \quad
\frac{dT}{dh} = \frac{1}{2}
\end{equation}
$$

Substituting these relations to $\frac{df}{dh}$, $\frac{d^2f}{dh^2}$, $\frac{d^3f}{dh^3}$, the derivatives of $k_2$ can be obtained.

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

For $k_3$ calculation, we have $X=x+\frac{h}{2} k_2$, $T=t+\frac{h}{2}$, so the derivatives with respect to $h$ can be expressed as follows.

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

As we are only interested in the derivatives at $h=0$, we derive the derivatives of $k_3$ at $h=0$.

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

In the same way, we can derive the derivatives for $k_4$.

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

Derivatives of $k_4$ at $h=0$ are obtained as follows.

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

As we have obtained all the necessary derivatives, we can now compare the coefficients.

First order:

$$
\begin{equation}
\left. \frac{1}{6} (k_1 + 2k_2 + 2k_3 + k_4) \right|_{h=0} = f
\end{equation}
$$

Second order:

$$
\begin{align}
&\frac{1}{3} \left[ \frac{dk_1}{dh} + 2\frac{dk_2}{dh} + 2\frac{dk_3}{dh} + \frac{dk_4}{dh} \right]_{h=0} \notag \\
&= \frac{1}{3} \left[ 2\left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) + 2\left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) + f \frac{\partial f}{\partial x} + \frac{\partial f}{\partial t} \right] \notag \\
&= f \frac{\partial f}{\partial x} +\frac{\partial f}{\partial t}
\end{align}
$$

Third order:

$$
\begin{align}
&\frac{1}{2} \left[ \frac{d^2k_1}{dh^2} + 2\frac{d^2k_2}{dh^2} + 2\frac{d^2k_3}{dh^2} + \frac{d^2k_4}{dh^2}\right] _{h=0} \notag \\

&= \frac{1}{2} \left[ 2\left( \frac{f^2}{4} \frac{\partial^2 f}{\partial x^2} + \frac{f}{2} \frac{\partial^2 f}{\partial x \partial t} + \frac{1}{4} \frac{\partial^2 f}{\partial t^2} \right) \right. \notag \\
&\hspace{20pt}\left. + 2 \left( \frac{f^2}{4} \frac{\partial^2 f}{\partial x^2} + \frac{f}{2} \frac{\partial^2 f}{\partial x \partial t} + \frac{1}{4} \frac{\partial^2 f}{\partial t^2} + \left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) \frac{\partial f}{\partial x} \right) \right. \notag\\
&\hspace{20pt}\left. + \left( f^2 \frac{\partial^2 f}{\partial x^2} + 2f \frac{\partial^2 f}{\partial x \partial t} + \frac{\partial^2 f}{\partial t^2} + 2 \left( \frac{f}{2} \frac{\partial f}{\partial x} + \frac{1}{2} \frac{\partial f}{\partial t} \right) \frac{\partial f}{\partial x} \right) \right] \notag \\

&= \frac{\partial^2 f}{\partial x^2} + 2f \frac{\partial^2 f}{\partial x \partial t} + \frac{\partial^2 f}{\partial t^2} + f \left( \frac{\partial f}{\partial x} \right)^2 + \frac{\partial f}{\partial t} \frac{\partial f}{\partial x}
\end{align}
$$

Fourth order:

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

## Notation Table

| Symbol                              | Definition                                  |
| ----------------------------------- | ------------------------------------------- |
| $f(x,t)$                            | Function of ODE                             |
| $h$                                 | Step size                                   |
| $k_i$                               | Runge-Kutta intermediate values             |
| $\frac{\partial^n f}{\partial x^n}$ | n-th partial derivative with respect to $x$ |
