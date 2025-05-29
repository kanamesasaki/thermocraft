---
title: "Knudsen Number"
description: "When the interested fluid phenomenon reaches very low density region, we have to be careful whether it is still within the scope of ordinary fluid dynamics. One of the useful indicators to characterize such physical phenomena is a dimensionless quantity called Knudsen number."
pubDate: 2025-05-20
updatedDate: 2025-05-29
heroImage: ""
tags: ["fluid dynamics"]
---

When the interested fluid phenomenon reaches very low density region, we have to be careful whether it is still within the scope of ordinary fluid dynamics, or it is rather in the field of molecular dynamics or rarefied gas dynamics.
One of the useful indicators to characterize such physical phenomena is a dimensionless quantity called Knudsen number.
In this article, we will look into the definition of the Knudsen number, and evaluate the mean free path, which is an essential quantity to understand the Knudsen number.
The evaluation procedure is mainly based on [[1]](#reference).

## Definition of Knudsen Number

The Knudsen number is defined as shown in Eq. (1), where $\lambda$ [m] is a mean free path, and $L$ [m] is a representative length.

$$
\begin{equation}
\mathrm{Kn} = \frac{\lambda}{L}
\end{equation}
$$

Mean free path indicates the average distance, where a molecule travels before colliding with another molecule.
Thus, if there are a lot of molecules close to each other, the mean free path becomes smaller.
The Knudsen number is defined as the ratio of the mean free path and the representative length (ex. pipe diameter for pipe flow).
When the Knudsen number is well below 1, typically less than 0.01, the molecules collide frequently enough and the flow can be considered continuum.

Now, assuming that $N$ is the number of molecules in unit volume, there is a following relation between the number of molecules $N$ and the mean free path $\lambda$.

$$
\begin{equation}
\sigma_\mathrm{T} \lambda \propto \frac{1}{N}
\end{equation}
$$

In this equation, $\sigma_\mathrm{T}$ is a total collision cross-section, which represents the area involved in the collision and varies depending on the molecule type.
When molecules of the same type with diameter $d$ collide, a collision occurs when the distance between the two molecules is equal to or less than $d$.
Therefore, $\sigma_\mathrm{T}$ is described by Eq. (3).

$$
\begin{equation}
% \label{eq:Bird1994_1.8}
\sigma_\mathrm{T} = \pi d^2
\end{equation}
$$

![knudsen-number-1](./knudsen-number-1.svg)
_Figure 1: Total Collision Cross-Section of Monoatomic Molecules._

In other words, the left side of Eq. (2) indicates the volume of the region swept by the motion of a molecule, and the right side indicates the volume allocated to a molecule.
However, a mean free path $\lambda$ cannot be specified by simply equating the left and right side of Eq. (2).
Thus, we have to discuss the underlying relations between the parameters more in detail.

## Definition of Mean Free Path

First, let us focus on a molecule $t$, and determine the mean collision rate (average collision frequency of a molecule with other molecules).
Let us describe molecules with a velocity vector $\bm{v}_i$ as $i$.
Now, we describe the relative velocity of $t$ with respect to $i$ as $\bm{v}_\mathrm{r} = \bm{v}_t - \bm{v}_i$, and think about the molecule $t$ flying with $\bm{v}_\mathrm{r}$ in the stable molecules $i$.
In this case, the volume swept by the molecule $t$ in unit time is described by $\sigma_\mathrm{T} |\bm{v}_\mathrm{r}|$.
Let us assume that the number of molecules with a velocity vector $\bm{v}_i$ is $\Delta N_i$ within a unit volume.
Integrating $\Delta N_i$ for all possible velocity vectors, i.e. over the velocity distribution, the number of molecules in collision per unit time can be estimated.

$$
\begin{equation}
\nu = N \overline{\sigma_\mathrm{T} v_\mathrm{r}}, \quad \mathrm{where} \quad \overline{\sigma_\mathrm{T} v_\mathrm{r}} = \frac{1}{N} \sum_i \Delta N_i \sigma_\mathrm{T} |\bm{v}_\mathrm{r}|
\end{equation}
$$

If the molecules are only one type, sphere shape, and rigid, the mean collision rate can be written in the following form.

$$
\begin{equation}
\nu = N \sigma_\mathrm{T} \overline{v_\mathrm{r}} = N \pi d^2 \overline{v_\mathrm{r}}
\end{equation}
$$

The mean free path $\lambda$ [m] can be estimated by molecules' average velocity $\overline{v'}$ [$\mathrm{ms^{-1}}$] divided by the mean collision rate $\nu$ [$\mathrm{s^{-1}}$].

$$
\begin{equation}
\lambda = \frac{\overline{v'}}{\nu} = \frac{1}{N} \frac{\overline{v'}}{\overline{\sigma_\mathrm{T} v_\mathrm{r}}}
\end{equation}
$$

If the molecules are single type rigid sphere, the mean free path can be estimated by

$$
\begin{equation}
\lambda = \frac{1}{N} \frac{\overline{v'}}{\pi d^2 \overline{v_\mathrm{r}}}
\end{equation}
$$

## Mean Free Path under Equilibrium

In this section, we look into the concrete description of the mean free path for single type and rigid sphere molecules.
In Eq. (7), the unknowns are the mean velocity of the molecules $\overline{v'}$ and the mean relative velocity of the molecules $\overline{v_\mathrm{r}}$.
Using the Maxwell-Boltzmann distribution Eq. (8), which is a velocity distribution under equilibrium, we try to determine the form of these parameters.

$$
\begin{equation}
% \label{eq:Bird1994_4.1}
d\varrho_v = \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m(u^2+v^2+w^2)}{2k_\mathrm{B}T} \right] du dv dw
\end{equation}
$$

### Mean Relative Velocity, $\overline{v_\mathrm{r}}$

For molecule 1 and molecule 2, we define the average velocity vector and relative velocity vector as show below.

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

A probability of having $\bm{v}_\mathrm{r}$ is calculated as a product of a probability of having $\bm{v}_1$ and a probability of having $\bm{v}_2$.
Multiplying the probability of $\bm{v}_\mathrm{r}$ and the relative velocity $v_\mathrm{r}$, and integrating it over the entire $\bm{v}_1$ and $\bm{v}_2$ distributions, gives the average relative velocity.

$$
\begin{align}
\overline{v_\mathrm{r}}
&= \int^\infty_{-\infty} \int^\infty_{-\infty} |\bm{v}_\mathrm{r}| \left\{ \left( \frac{m_1}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m_1 |\bm{v}_1|^2}{2k_\mathrm{B}T} \right] \right\} \notag \\
&\hspace{58pt}\times \left\{ \left( \frac{m_2}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m_2 |\bm{v}_2|^2}{2k_\mathrm{B}T} \right] \right\} d\boldsymbol{v}_1 d\boldsymbol{v}_2 \notag \\
&= \frac{(m_1 m_2)^{\frac{3}{2}}}{(2\pi k_\mathrm{B} T)^3} \int^\infty_{-\infty} \int^\infty_{-\infty} |\bm{v}_\mathrm{r}| \exp\left[ \frac{-(m_1 |\bm{v}_1|^2 + m_2 |\bm{v}_2|^2)}{2k_\mathrm{B} T} \right] d\boldsymbol{v}_1 d\boldsymbol{v}_2
\end{align}
$$

To calculate this integral, we should convert the parameters from $\bm{v}_1 = (u_1, v_1, w_1)$ and $\bm{v}_2 = (u_2, v_2, w_2)$ to $\bm{v}_\mathrm{r} = (u_\mathrm{r}, v_\mathrm{r}, w_\mathrm{r})$ and $\bm{v}_\mathrm{m} = (u_\mathrm{m}, v_\mathrm{m}, w_\mathrm{m})$.
To do this parameter conversion, the corresponding Jacobian has to be specified.
First, we describe $\bm{v}_1, \bm{v}_2$ by using $\bm{v}_\mathrm{r}, \bm{v}_\mathrm{m}$ based on Eq. (9) and (10).

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

Then, we can calculate the Jacobian by performing the Laplace expansion (cofactor expansion).
The process is quite lengthy, but the Jacobian is simply 1 in the end.

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

In the meantime, we should review the following transformation as well.
The parameter $m_\mathrm{r}$ corresponds to the reduced mass, which is commontly known in the classical mechanics.

$$
\begin{align}
&m_1 v_1^2 + m_2 v_2^2 \notag \\
&= \frac{m_1 m_2^2}{(m_1 + m_2)^2} v_\mathrm{r}^2 + \frac{m_1 m_2}{m_1 + m_2} \boldsymbol{v}_\mathrm{r} \cdot \boldsymbol{v}_\mathrm{m} + m_1 v_\mathrm{m}^2 \notag \\
&\hspace{11pt}+ \frac{m_1^2 m_2}{(m_1 + m_2)^2} v_\mathrm{r}^2 - \frac{m_1 m_2}{m_1 + m_2} \boldsymbol{v}_\mathrm{r} \cdot \boldsymbol{v}_\mathrm{m} + m_2 v_\mathrm{m}^2 \notag \\
&= m_\mathrm{r} v_\mathrm{r}^2 + (m_1 + m_2) v_\mathrm{m}^2, \quad \mathrm{where} \quad m_\mathrm{r} = \frac{m_1 m_2}{m_1 + m_2}
\end{align}
$$

Based on the Jacobian result and Eq. (14), Eq. (11) can be re-written in the following form.

$$
\begin{align}
\overline{v_\mathrm{r}}
= \frac{(m_1 m_2)^{\frac{3}{2}}}{(2\pi k_\mathrm{B} T)^3} \int^\infty_{-\infty} \int^\infty_{-\infty} |\bm{v}_\mathrm{r}| \exp\left[ \frac{-(m_\mathrm{r} |\bm{v}_\mathrm{r}|^2 + (m_1 + m_2) |\bm{v}_\mathrm{m}|^2)}{2k_\mathrm{B} T} \right] d\boldsymbol{v}_\mathrm{r} d\boldsymbol{v}_\mathrm{m}
\end{align}
$$

The integration should be performed over the entire velocity space for $(u_\mathrm{r}, v_\mathrm{r}, w_\mathrm{r})$ and $(u_\mathrm{m}, v_\mathrm{m}, w_\mathrm{m})$.
As the integrand is only a function of $|\bm{v}_\mathrm{r}|$ and $|\bm{v}_\mathrm{m}|$, the calculation can be simplified by using the polar coordinate system.

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

Each integration in Eq. (20) is calculated, utilizing the gamma function.

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

Based on the above relations, Eq. (20) is concretely described by

$$
\begin{equation}
\overline{v_\mathrm{r}} = \frac{2(m_1 m_2)^{\frac{3}{2}}}{\pi (k_\mathrm{B} T)^3} 
\frac{2 (k_\mathrm{B} T)^2}{m_\mathrm{r}^2} 
\frac{\sqrt{\pi}}{4} \frac{(2 k_\mathrm{B} T)^{\frac{3}{2}}}{(m_1 + m_2)^{\frac{3}{2}}}
= \frac{2}{\sqrt{\pi}} \left( \frac{2k_\mathrm{B} T}{m_\mathrm{r}} \right)^\frac{1}{2}
\end{equation}
$$

If molecule 1 and molecule 2 have the same mass of $m$, the reduced mass is described by $m_\mathrm{r} = m/2$.
In this case, the mean relative velocity is described by

$$
\begin{equation}
\overline{v_\mathrm{r}} 
= \sqrt{\frac{16 k_\mathrm{B} T}{\pi m}}
\end{equation}
$$

### Mean Velocity, $\overline{v'}$

The mean velocity of molecules can be evaluated in the similar procedure.

$$
\begin{align}
\overline{v'} &= \int_{-\infty}^{\infty} |\bm{v}| \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m|\bm{v}|^2}{2k_\mathrm{B}T} \right] du dv dw \notag \\
&= \int_{0}^{\infty} |\bm{v}|^3 \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \exp \left[ -\frac{m|\bm{v}|^2}{2k_\mathrm{B}T} \right] d|\bm{v}| \int_0^{2\pi} d\varphi \int_0^{\pi} \sin\theta d\theta \notag \\
&= 4\pi \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \int_{0}^{\infty} |\bm{v}|^3 \exp \left[ -\frac{m|\bm{v}|^2}{2k_\mathrm{B}T} \right] d|\bm{v}|
\end{align}
$$

Using Eq. (22), the mean velocity of molecules is described by

$$
\begin{equation}
\overline{v'} = 4\pi \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}} \frac{2k_\mathrm{B}^2 T^2}{m^2} = \sqrt{\frac{8 k_\mathrm{B} T}{\pi m}}
\end{equation}
$$

## Conclusion

Through the discussions in the previous sections, mean relative velocity $\overline{v_\mathrm{r}}$ and mean velocity $\overline{v'}$ are characterized by Eq. (25) and Eq. (27), respectively.
Substituting these results to Eq. (7), the mean free path is described by

$$
\begin{equation}
% \label{eq:Bird1994_4.55}
\lambda = \frac{1}{\sqrt{2} \sigma_\mathrm{T} N} = \frac{1}{\sqrt{2} \pi d^2 N}
\end{equation}
$$

Applying this result to Eq. (1) and additionally using the ideal gas law, $PV = Nk_\mathrm{B}T$ ($N$ is the number of molecules), the following Knudsen number description is obtained.

$$
\begin{equation}
\mathrm{Kn} = \frac{\lambda}{L} = \frac{1}{\sqrt{2} \pi d^2 NL} = \frac{k_\mathrm{B}T}{\sqrt{2} \pi d^2 PL}
\end{equation}
$$

## Reference

1. G.A. Bird “Molecular Gas Dynamics and the Direct Simulation of Gas Flows”, Oxford University Press, 1994.
