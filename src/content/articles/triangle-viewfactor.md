---
title: 'Triangle View Factor from a Plate Element'
description: 'Analytical derivation of a triangle view factor from a plate element, using line integration.'
pubDate: 2025-07-15
updatedDate: 2025-07-15
heroImage: ''
tags: ['thermal']
---

## Introduction

In this article, we will derive analytical view factor expressions for an arbitrary triangle from a plate element.
We describe the disk geometry and relative position with respect to the plate element by parameters $(R, h, \omega)$ as shown in Figure 1.

![disk-viewfactor-1](./disk-viewfactor-1.svg)
_Figure 1: Geometrical Configuration of a Disk and a Plate Element for View Factor Evaluation._

## View Factor Evaluation based on the Area Integration

A disk view factor from a plate element can be derived, by executing the area integration based on the view factor definition.

$$
\begin{align}
\left[ \begin{array}{c} x_i \\ y_i \\ z_i \end{array} \right] = \left[ \begin{array}{c} r_i \sin\theta_i \cos\varphi_i \\ r_i \sin\theta_i \sin\varphi_i \\ r_i \cos\theta_i \end{array} \right],
\quad \mathrm{where} \quad r_i = \sqrt{x_i^2 + y_i^2 + z_i^2}.
\end{align}
$$

$$
\begin{align}
\theta_i &= \arccos \left( \frac{z_i}{r_i} \right) = \arccos \left( \frac{z_i}{\sqrt{x_i^2 + y_i^2 + z_i^2}} \right), \\
\varphi_i &= \arctan \left( \frac{y_i}{x_i} \right).
\end{align}
$$

$$
\begin{align}
\cos\varphi_i &= \frac{x_i}{\sqrt{x_i^2 + y_i^2}}, \quad
\sin\varphi_i = \frac{y_i}{\sqrt{x_i^2 + y_i^2}}.
\end{align}
$$

$$
\begin{align}
\cos\theta_i &= \frac{z_i}{\sqrt{x_i^2 + y_i^2 + z_i^2}}, \quad
\sin\theta_i = \sqrt{\frac{x_i^2 + y_i^2}{x_i^2 + y_i^2 + z_i^2}}
\end{align}
$$

A vector which is perpendicular to $\overrightarrow{O U_1}$ and $\overrightarrow{O U_2}$ is given by the cross product of these two vectors.

$$
\begin{align}
\boldsymbol{n}_{12} &= \overrightarrow{O U_1} \times \overrightarrow{O U_2}
= \left[ \begin{array}{c} \sin\theta_1 \cos\varphi_1 \\ \sin\theta_1 \sin\varphi_1 \\ \cos\theta_1 \end{array} \right] \times \left[ \begin{array}{c} \sin\theta_2 \cos\varphi_2 \\ \sin\theta_2 \sin\varphi_2 \\ \cos\theta_2 \end{array} \right] \notag \\
&= \left[ \begin{array}{c}
\sin\theta_1\sin\varphi_1\cos\theta_1 - \cos\theta_1\sin\theta_2\sin\varphi_2 \\
\cos\theta_1\sin\theta_2\cos\varphi_2 - \sin\theta_1\cos\varphi_1\cos\theta_2 \\
\sin\theta_1\sin\theta_2(\cos\varphi_1\sin\varphi_2 - \sin\varphi_1\cos\varphi_2)
\end{array} \right].
\end{align}
$$

$$
\begin{align}
&\overrightarrow{O U_2'} = \frac{\overrightarrow{O U_2} - \overrightarrow{O U_1}(\overrightarrow{O U_1} \cdot \overrightarrow{O U_2})}{\sqrt{1 - (\overrightarrow{O U_1} \cdot \overrightarrow{O U_2})^2}} \notag \\
&= \frac{1}{\sqrt{1 - (\overrightarrow{O U_1} \cdot \overrightarrow{O U_2})^2}} \left( \left[ \begin{array}{c} \sin\theta_2 \cos\varphi_2 \\ \sin\theta_2 \sin\varphi_2 \\ \cos\theta_2 \end{array} \right] - (\overrightarrow{O U_1} \cdot \overrightarrow{O U_2}) \left[ \begin{array}{c} \sin\theta_1 \cos\varphi_1 \\ \sin\theta_1 \sin\varphi_1 \\ \cos\theta_1 \end{array} \right]\right).
\end{align}
$$

$$
\begin{align}
\overrightarrow{O U_1} \cdot \overrightarrow{O U_2} &= \sin\theta_1 \sin\theta_2 (\cos\varphi_1 \cos\varphi_2 + \sin\varphi_1 \sin\varphi_2) + \cos\theta_1 \cos\theta_2 \notag \\
&= \sin\theta_1 \sin\theta_2 \cos(\varphi_2 - \varphi_1) + \cos\theta_1 \cos\theta_2.
\end{align}
$$

$$
\begin{align}
1 - (\overrightarrow{O U_1} \cdot \overrightarrow{O U_2})^2 &= 1 - \sin^2\theta_1 \sin^2\theta_2 \cos^2(\varphi_2 - \varphi_1) - \cos^2\theta_1 \cos^2\theta_2 \notag \\
&\hspace{11pt}-2 \sin\theta_1 \sin\theta_2 \cos\theta_1 \cos\theta_2 \cos(\varphi_2 - \varphi_1)
\end{align}
$$

$$
\begin{align}
\cos\psi_{12}
&= \sqrt{\frac{x_1^2 + y_1^2}{x_1^2 + y_1^2 + z_1^2}}\sqrt{\frac{x_2^2 + y_2^2}{x_2^2 + y_2^2 + z_2^2}} \left(\frac{x_1}{\sqrt{x_1^2 + y_1^2}} \frac{x_2}{\sqrt{x_2^2 + y_2^2}} + \frac{y_1}{\sqrt{x_1^2 + y_1^2}} \frac{y_2}{\sqrt{x_2^2 + y_2^2}} \right) \notag \\
&+ \frac{z_1}{\sqrt{x_1^2 + y_1^2 + z_1^2}} \frac{z_2}{\sqrt{x_2^2 + y_2^2 + z_2^2}} \notag \\
&= \frac{x_1x_2 + y_1y_2 + z_1z_2}{\sqrt{(x_1^2 + y_1^2 + z_1^2)(x_2^2 + y_2^2 + z_2^2)}}
\end{align}
$$

$$
\begin{align}
\sin\psi_{12} &= \sqrt{1 - \frac{(x_1x_2 + y_1y_2 + z_1z_2)^2}{(x_1^2 + y_1^2 + z_1^2)(x_2^2 + y_2^2 + z_2^2)}} \notag \\
&= \frac{\sqrt{(x_1^2 + y_1^2 + z_1^2)(x_2^2 + y_2^2 + z_2^2) - (x_1x_2 + y_1y_2 + z_1z_2)^2}}{\sqrt{(x_1^2 + y_1^2 + z_1^2)(x_2^2 + y_2^2 + z_2^2)}}
\end{align}
$$

The line from $U_1$ to $U_2$ on the spherical surface is expressed as follows.

$$
\begin{align}
&\left[ \begin{array}{c} x \\ y \\ z \end{array}\right] =
\cos\psi~ \overrightarrow{O U_1} + \sin\psi~ \overrightarrow{O U_2'},
\quad \mathrm{where} \quad \psi \in \left[ 0, ~\arccos(\overrightarrow{O U_1} \cdot \overrightarrow{O U_2}) \right]. \\
&\frac{d}{d\psi} \left[ \begin{array}{c} x \\ y \\ z \end{array}\right] =
-\sin\psi~ \overrightarrow{O U_1} + \cos\psi~ \overrightarrow{O U_2'}.
\end{align}
$$

The orientation of the plate element is given by $(0, 0, 1)$.
Thus, the line integral from $U_1$ to $U_2$ on the spherical surface is expressed as follows.

$$
\begin{align}
x &= \cos\psi~ \sin\theta_1 \cos\varphi_1 + \sin\psi~ \frac{\sin\theta_2 \cos\varphi_2 - \cos\psi_{12} \sin\theta_1 \cos\varphi_1}{\sin\psi_{12}} \\
y &= \cos\psi~ \sin\theta_1 \sin\varphi_1 + \sin\psi~ \frac{\sin\theta_2 \sin\varphi_2 - \cos\psi_{12} \sin\theta_1 \sin\varphi_1}{\sin\psi_{12}}
\end{align}
$$

$$
\begin{align}
L_{12} &= \int \frac{ydx - xdy}{2\pi} \notag \\
&= \frac{1}{2\pi} \int_0^{\psi_{12}} \left( \cos\psi~ \sin\theta_1 \sin\varphi_1 + \sin\psi~ \frac{\sin\theta_2 \sin\varphi_2 - \cos\psi_{12} \sin\theta_1 \sin\varphi_1}{\sin\psi_{12}} \right) \notag \\
&\quad\times \left( -\sin\psi~ \sin\theta_1 \cos\varphi_1 + \cos\psi~ \frac{\sin\theta_2 \cos\varphi_2 - \cos\psi_{12} \sin\theta_1 \cos\varphi_1}{\sin\psi_{12}} \right) \notag \\
&\quad - \left( \cos\psi~ \sin\theta_1 \cos\varphi_1 + \sin\psi~ \frac{\sin\theta_2 \cos\varphi_2 - \cos\psi_{12} \sin\theta_1 \cos\varphi_1}{\sin\psi_{12}} \right) \notag \\
&\quad\times \left( -\sin\psi~ \sin\theta_1 \sin\varphi_1 + \cos\psi~ \frac{\sin\theta_2 \sin\varphi_2 - \cos\psi_{12} \sin\theta_1 \sin\varphi_1}{\sin\psi_{12}} \right) d\psi \notag \\
&= \frac{1}{2\pi \sin\psi_{12}} \int_0^{\psi_{12}} \sin\theta_1 \sin\varphi_1 (\sin\theta_2 \cos\varphi_2 - \cos\psi_{12} \sin\theta_1 \cos\varphi_1) \notag \\
&\hspace{70pt} - \sin\theta_1 \cos\varphi_1 (\sin\theta_2 \sin\varphi_2 - \cos\psi_{12} \sin\theta_1 \sin\varphi_1) d\psi \notag \\
&= \frac{\psi_{12}}{2\pi \sin\psi_{12}} \big(\sin\theta_1 \sin\varphi_1 (\sin\theta_2 \cos\varphi_2 - \cos\psi_{12} \sin\theta_1 \cos\varphi_1) \notag \\
&\hspace{70pt}- \sin\theta_1 \cos\varphi_1 (\sin\theta_2 \sin\varphi_2 - \cos\psi_{12} \sin\theta_1 \sin\varphi_1)\big) \notag \\
&= \frac{\psi_{12}}{2\pi \sin\psi_{12}} \sin\theta_1 \sin\theta_2 (\sin\varphi_1 \cos\varphi_2 - \cos\varphi_1 \sin\varphi_2)
\end{align}
$$

This expression can be re-written using the cartesian coordinates.

$$
\begin{align}
L_{12} &= \frac{1}{2\pi} \frac{y_1 x_2 - x_1 y_2}{\sqrt{(x_1^2 + y_1^2 + z_1^2)(x_2^2 + y_2^2 + z_2^2) - (x_1x_2 + y_1y_2 + z_1z_2)^2}} \arccos \left( \frac{x_1x_2 + y_1y_2 + z_1z_2}{\sqrt{(x_1^2 + y_1^2 + z_1^2)(x_2^2 + y_2^2 + z_2^2)}} \right) \notag \\
\end{align}
$$

Integrating this formula in X and Y directions, we can obtain the view factor from the finite area to the triangle.

$$
\scriptsize
\begin{align}
\iint &\frac{(y_1 - Y)(x_2 - X) - (x_1 - X) (y_2 - Y)}{\sqrt{((x_1 - X)^2 + (y_1 - Y)^2 + z_1^2)((x_2 - X)^2 + (y_2 - Y)^2 + z_2^2) - ((x_1 - X)(x_2 - X) + (y_1 - Y)(y_2 - Y) + z_1z_2)^2}} \notag \\
&\times \arccos \left( \frac{(x_1 - X)(x_2 - X) + (y_1 - Y)(y_2 - Y) + z_1z_2}{\sqrt{((x_1 - X)^2 + (y_1 - Y)^2 + z_1^2)((x_2 - X)^2 + (y_2 - Y)^2 + z_2^2)}} \right) dX dY
\end{align}
$$

To avoid double integration, let us apply the Green's theorem to convert the area integral into a line integral.

## Reference

1. A Catalog of Configuration Factors, 3rd Edition, [https://www.thermalradiation.net/indexCat.html](https://www.thermalradiation.net/indexCat.html)
2.
3. View Factor Calculator, [https://sterad.net](https://sterad.net)
