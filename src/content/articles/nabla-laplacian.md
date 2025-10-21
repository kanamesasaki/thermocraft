---
title: 'Nabla and Laplacian in Cylindrical and Spherical Coordinate Systems'
description: 'This article derives the expressions for Nabla and Laplacian in cylindrical and spherical coordinate systems using the general representation of Nabla in curvilinear coordinate systems.'
pubDate: 2025-10-21
updatedDate: 2025-10-21
heroImage: ''
tags: ['mathematics', 'fluid dynamics']
---

In the context of fluid dynamics, it is often useful to employ cylindrical and spherical coordinate systems.
Thus, let's derive the expressions for Nabla and Laplacian in these coordinate systems.
There are several approaches to derive Nabla and Laplacian in different coordinate systems, but to avoid tedious calculations using the chain rule, we will use the general representation of Nabla in curvilinear coordinate systems.

## Nabla in Curvilinear Coordinate Systems

First of all, the Nabla operator in a Cartesian Coordinate system is expressed as shown in Eq. (1).
For clarity in notation, we denote the parameters of the Cartesian Coordinate system as $\tilde{x}_i$.

$$
\begin{equation}
\nabla = \boldsymbol{e}_i \frac{\partial}{\partial \tilde{x}_i}
\end{equation}
$$

This expression can be generalized as shown in Eq. (2) to be applicable in curvilinear coordinate systems.

$$
\begin{equation}
\nabla = \boldsymbol{g}^i \frac{\partial}{\partial x^i}
\end{equation}
$$

In Eq. (2), $\boldsymbol{g}^i$ is a contravariant basis vector, which is related to a covariant basis vector $\boldsymbol{g}_i$ as defined in Eq. (3).
A covariant basis vector is a partial derivative of the position vector $\boldsymbol{r}$ with respect to a parameter of a curvilinear coordinate system.
This vector indicates the direction of the position vector's change when the parameter slightly changes.

$$
\begin{equation}
\boldsymbol{g}_i = \frac{\partial \boldsymbol{r}}{\partial x^i}
\end{equation}
$$

A contravariant basis vector is defined to satisfy the following relationship.

$$
\begin{equation}
\boldsymbol{g}_i \cdot \boldsymbol{g}^j = \delta_i^j
\end{equation}
$$

We can confirm that the expression in Eq. (2) is a generalization of Eq. (1) by transforming the basis vectors.
When transforming from one set of covariant basis vectors to another, the following relationship holds.

$$
\begin{equation}
\boldsymbol{g}_i = \frac{\partial \boldsymbol{r}}{\partial x^i} = \frac{\partial \bar{x}^j}{\partial x^i} \frac{\partial \boldsymbol{r}}{\partial \bar{x}^j} = \frac{\partial \bar{x}^j}{\partial x^i} \bar{\boldsymbol{g}}_j
\end{equation}
$$

Based on this relationship, we can also derive the transformation formula for contravariant basis vectors.

$$
\begin{equation}
\boldsymbol{g}^i = \frac{\partial x^i}{\partial \bar{x}^j} \bar{\boldsymbol{g}}^j
\end{equation}
$$

Using this relationship, we can transform the Nabla operator in curvilinear coordinate systems to the basis of the Cartesian Coordinate system.
The resulting expression matches the well-known representation in Eq. (1).

$$
\begin{equation}
\nabla = \boldsymbol{g}^i \frac{\partial}{\partial x^i} = \boldsymbol{e}_j \frac{\partial x^i}{\partial \tilde{x}_j} \frac{\partial}{\partial x^i} = \boldsymbol{e}_j \frac{\partial}{\partial \tilde{x}_j}
\end{equation}
$$

## Cylindrical Coordinate System

The position vector in Cartesian coordinates can be described using the parameters of cylindrical coordinates as follows.

$$
\begin{equation}
\boldsymbol{r} = x \boldsymbol{e}_x + y \boldsymbol{e}_y + z \boldsymbol{e}_z
= r \cos \theta\ \boldsymbol{e}_x + r \sin \theta\ \boldsymbol{e}_y + z \boldsymbol{e}_z
\end{equation}
$$

From this expression, we can derive the covariant basis vectors as shown in Eqs. (9)--(11).
Covariant basis vectors are generally not orthogonal, but in the cylindrical coordinate system, the obtained three vectors are orthogonal to each other.

$$
\begin{align}
&\boldsymbol{g}_r = \frac{\partial \boldsymbol{r}}{\partial r} = \cos \theta \boldsymbol{e}_x + \sin \theta \boldsymbol{e}_y \\
&\boldsymbol{g}_\theta = \frac{\partial \boldsymbol{r}}{\partial \theta} = - r \sin \theta \boldsymbol{e}_x + r \cos \theta \boldsymbol{e}_y \\
&\boldsymbol{g}_z = \frac{\partial \boldsymbol{r}}{\partial z} = \boldsymbol{e}_z
\end{align}
$$

Since the obtained covariant basis vectors are orthogonal to each other, the contravariant basis vectors $(\boldsymbol{g}^r, \boldsymbol{g}^\theta, \boldsymbol{g}^z)$ are in the same directions as their respective covariant basis vectors, but with lengths that are the reciprocals.
We obtain the Nabla operator in the cylindrical coordinate system as shown in Eq. (12), where we denote the normalized covariant basis vectors as $(\boldsymbol{e}_r, \boldsymbol{e}_\theta, \boldsymbol{e}_z)$.

$$
\begin{equation}
\nabla = \boldsymbol{g}^r \frac{\partial}{\partial r} + \boldsymbol{g}^\theta \frac{\partial}{\partial \theta} + \boldsymbol{g}^z \frac{\partial}{\partial z}
= \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_z \frac{\partial}{\partial z}
\end{equation}
$$

Laplacian in the cylindrical coordinate system can be derived by calculating the inner product of Nabla.

$$
\begin{equation}
\frac{\partial}{\partial \theta} \left[ \begin{array}{c}
\boldsymbol{e}_r \\ \boldsymbol{e}_\theta
\end{array} \right] =
\left[ \begin{array}{cc}
-\sin \theta & \cos \theta \\
- \cos \theta & - \sin \theta
\end{array} \right]
\left[ \begin{array}{c}
\boldsymbol{e}_x \\ \boldsymbol{e}_y
\end{array} \right] =
\left[ \begin{array}{c}
\boldsymbol{e}_\theta \\ - \boldsymbol{e}_r
\end{array} \right]
\end{equation}
$$

$$
\begin{align}
\Delta &= \nabla \cdot \nabla \notag \\
&= \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_z \frac{\partial}{\partial z} \right) \cdot \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_z \frac{\partial}{\partial z} \right) \notag \\
&= \frac{\partial^2}{\partial r^2} + \boldsymbol{e}_\theta \cdot \frac{1}{r} \left( \frac{\partial}{\partial \theta} \boldsymbol{e}_r \right) \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{\partial^2}{\partial z^2} \notag \\
&= \frac{\partial^2}{\partial r^2} + \frac{1}{r} \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{\partial^2}{\partial z^2} \\
\end{align}
$$

## Spherical Coordinate System

The position vector in Cartesian coordinates can be described using the parameters of spherical coordinates as follows.

$$
\begin{equation}
\boldsymbol{r} = x \boldsymbol{e}_x + y \boldsymbol{e}_y + z \boldsymbol{e}_z
= r \sin \theta \cos \phi\ \boldsymbol{e}_x + r \sin \theta \sin \phi\ \boldsymbol{e}_y + r \cos \theta \boldsymbol{e}_z
\end{equation}
$$

Based on this expression, we can derive the covariant basis vectors as shown below.

$$
\begin{align}
&\boldsymbol{g}_r = \frac{\partial \boldsymbol{r}}{\partial r} = \sin \theta \cos \phi\ \boldsymbol{e}_x + \sin \theta \sin \phi\ \boldsymbol{e}_y + \cos \theta\ \boldsymbol{e}_z \\
&\boldsymbol{g}_\theta = \frac{\partial \boldsymbol{r}}{\partial \theta} = r \cos \theta \cos \phi\ \boldsymbol{e}_x + r \cos \theta \sin \phi\ \boldsymbol{e}_y - r \sin \theta\ \boldsymbol{e}_z \\
&\boldsymbol{g}_\phi = \frac{\partial \boldsymbol{r}}{\partial \phi} = - r \sin \theta \sin \phi\ \boldsymbol{e}_x + r \sin \theta \cos \phi\ \boldsymbol{e}_y
\end{align}
$$

Same as the cylindrical coordinate system, the obtained covariant basis vectors are orthogonal to each other.
Thus, the corresponding contravariant basis vectors $(\boldsymbol{g}^r, \boldsymbol{g}^\theta, \boldsymbol{g}^\phi)$ are in the same directions as the covariant basis vectors, but with lengths that are the reciprocals.
Denoting the normalized covariant basis vectors as $(\boldsymbol{e}_r, \boldsymbol{e}_{\theta}, \boldsymbol{e}_{\phi})$, we obtain the Nabla operator in the spherical coordinate system as shown in Eq. (19).

$$
\begin{equation}
\nabla = \boldsymbol{g}^r \frac{\partial}{\partial r} + \boldsymbol{g}^\theta \frac{\partial}{\partial \theta} + \boldsymbol{g}^\phi \frac{\partial}{\partial \phi}
= \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_\phi \frac{1}{r \sin \theta} \frac{\partial}{\partial \phi}
\end{equation}
$$

Laplacian in the spherical coordinate system can be derived by calculating the inner product of Nabla.

$$
\begin{align}
\frac{\partial}{\partial \theta} \left[ \begin{array}{c}
\boldsymbol{e}_r \\ \boldsymbol{e}_\theta \\ \boldsymbol{e}_\phi
\end{array} \right] =
\left[ \begin{array}{ccc}
\cos \theta \cos \phi & \cos \theta \sin \phi & - \sin \theta \\
- \sin \theta \cos \phi & - \sin \theta \sin \phi & - \cos \theta \\
0 & 0 & 0
\end{array} \right]
\left[ \begin{array}{c}
\boldsymbol{e}_x \\ \boldsymbol{e}_y \\ \boldsymbol{e}_z
\end{array} \right] =
\left[ \begin{array}{c}
\boldsymbol{e}_\theta \\ - \boldsymbol{e}_r \\ 0
\end{array} \right]
\end{align}
$$

$$
\begin{align}
\frac{\partial}{\partial \phi} \left[ \begin{array}{c}
\boldsymbol{e}_r \\ \boldsymbol{e}_\theta \\ \boldsymbol{e}_\phi
\end{array} \right] &=
\left[ \begin{array}{ccc}
- \sin \theta \sin \phi & \sin \theta \cos \phi & 0 \\
- \cos \theta \sin \phi & \cos \theta \cos \phi & 0 \\
- \cos \phi & - \sin \phi & 0
\end{array} \right]
\left[ \begin{array}{c}
\boldsymbol{e}_x \\ \boldsymbol{e}_y \\ \boldsymbol{e}_z
\end{array} \right] \notag \\
&=
\left[ \begin{array}{c}
\sin \theta\ \boldsymbol{e}_\phi \\ \cos \theta\ \boldsymbol{e}_\phi \\ - \sin \theta\ \boldsymbol{e}_r - \cos \theta\ \boldsymbol{e}_\theta
\end{array} \right]
\end{align}
$$

$$
\begin{align}
\Delta &= \nabla \cdot \nabla \notag \\
&= \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_\phi \frac{1}{r \sin \theta}\frac{\partial}{\partial \phi} \right) \cdot \left( \boldsymbol{e}_r \frac{\partial}{\partial r} + \boldsymbol{e}_\theta \frac{1}{r} \frac{\partial}{\partial \theta} + \boldsymbol{e}_\phi \frac{1}{r \sin \theta}\frac{\partial}{\partial \phi} \right) \notag \\
&= \frac{\partial^2}{\partial r^2} + \frac{2}{r} \frac{\partial}{\partial r} + \frac{1}{r^2} \frac{\partial^2}{\partial \theta^2} + \frac{1}{r^2 \tan \theta} \frac{\partial}{\partial \theta} + \frac{1}{r^2 \sin^2 \theta} \frac{\partial^2}{\partial \phi^2}
\end{align}
$$
