---
title: 'Tracking the Floating Point Error'
description: 'If rounding errors for floating point numbers are critical for your numerical calculation. What is the correct way to estimate and track the error? Looking into the definition in IEEE 754-2019 and basic methodology for the error estimation.'
pubDate: 2025-05-18
updatedDate: 2025-05-19
heroImage: ''
tags: ['programming', 'numerical analysis']
---

## Floating Point Format

Before starting error discussion on floating point numbers, let us first review the format of floating point numbers.
There are two types of floating point numbers: binary floating point and decimal floating point.
In this article, the discussion focuses on the binary floating point, which is commonly used for describing real numbers in computers.

The basic concept of the floating point number can be described by the following:

- Sign bit, which is a 1-bit data, represents a positive or negative sign of the value
- Exponent bits describe the power of 2 as $2^e$
- Significand bits divide the range between $2^e$ and $2^{e+1}$ into equal intervals to represent real numbers inbetween

This concept can be described by Eq. (1) and a 32-bit floating point number example for showing PI is described in Figure 1.

$$
\begin{equation}
v = (-1)^S \times 2^{E-bias} \times (1 + 2^{1-p} \times T)
\end{equation}
$$

![floating-point-error-1](./floating-point-error-1.svg)
_Figure 1: Structure of IEEE 754 floating-point format, PI in 32-bit floating point number as an example._

For typical floating point formats in 16, 32, 64, or 128 bits, the corresponding parameters are specified by the following table.

| Parameter              | binary16  | binary32  | binary64  | binary128  |
| ---------------------- | --------- | --------- | --------- | ---------- |
| bias                   | 15        | 127       | 1023      | 16383      |
| sign bit, $S$          | 1         | 1         | 1         | 1          |
| exponent bits, $w$     | 5         | 8         | 11        | 15         |
| significand bits, $t$  | 10        | 23        | 52        | 112        |
| precision in bits, $p$ | 11        | 24        | 53        | 113        |
| unit roundoff, $u$     | $2^{-11}$ | $2^{-24}$ | $2^{-53}$ | $2^{-113}$ |

## Rounding Error

To handle real numbers in a computer, the given number has to be converted to a floating point number.
In IEEE 754-2019 [[1]](#reference), the following five types of rounding attributes are defined for the conversion.
For the binary floating point numbers, the default rounding method is roundTiesToEven.

| Rounding Attribute  | Rounding to ...                                                                                                                                |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| roundTiesToEven     | the nearest floating point number. If two floating point numbers are equally near, one with an even least significant digit shall be returned. |
| roundTiesToAway     | the nearest floating point number. If two floating point numbers are equaly near, one with greater magnitude shall be returned.                |
| roundTowardPositive | the floating point number closest to and no less than the given number.                                                                        |
| roundTowardNegative | the floating point number closest to and no greater then the given number.                                                                     |
| roundTowardZero     | the floating point number closest to and no greater in magnitude than the given number.                                                        |

## Error Estimation

When describing a real number $x \in \mathbb{R}$ in a floating point number, the amount of error varies depending on $|x|$.
However, the error ratio $\delta$ can be generally described in the following form.

$$
\begin{equation}
fl(x) = x(1 + \delta), \quad |\delta| < u
\end{equation}
$$

As the first step, the absolute amount of error is smaller than the amount of 1 significant bit.

$$
\begin{equation}
|fl(x) - x| < 2^{E-bias} \times \frac{1}{2^t}
\end{equation}
$$

Furthermore, assuming that $x$ is rounded to the nearest floating point number, the amount of error can be reduced to half.

$$
\begin{equation}
|fl(x) - x| \le 2^{E-bias} \times u
\end{equation}
$$

Based on this fact, the error ratio can be evaluated as shown below.

$$
\begin{equation}
\left| \frac{fl(x) - x}{x} \right| < \frac{2^{E-bias} \times u}{2^{E-bias}} = u
\end{equation}
$$

If the exponent part of the floating point number is $2^{E-bias}$, the range of original real number should be $[2^{E-bias}- \frac{2^{E-bias}}{2^{p+1}},~2^{E-bias+1}-\frac{2^{E-bias}}{2^{p}})$. So, it looks insufficient to represent $x$ by $2^{E-bias}$ when evaluating the error ratio.
However, if $x < 2^{E-bias}$, the maximum error is less than half of $2^{E-bias} \times \frac{1}{2^{p}}$.
Thus, such case does not correspond to the maximum error ratio, and $2^{E-bias}$ can be used to represent $x$ for error ratio evaluation.

## Error Amount for Inner Product

When $x$ and $y$ are error-free floating point numbers, it is common to assume that the error ratio caused by a single arithmetic operation is described by

$$
\begin{equation}
fl(x~\mathrm{op}~y) = (x~\mathrm{op}~y)(1 + \delta),\quad |\delta| \le u,
\quad \mathrm{op} = +,~-,~*,~/
\end{equation}
$$

For now, we won't go into the dtails of the internal calculation steps of each operation, but accept this assumption for further error evaluation.
The main focus of the following discusion is the error amount of the inner product as shown in Eq. (7).
Since the amount of error depends on the order of operations, we assess the error amount when executing the operations from the left side to the right side.

$$
\begin{equation}
s_i = x_1 y_1 + \cdots + x_i y_i
\end{equation}
$$

Looking into the inner product operation from $i = 1, 2, \cdots ,$ the concrete descriptions are the following.

$$
\begin{equation}
\hat{s}_1 = fl(x_1 y_1) = x_1 y_1 (1 + \delta_1)
\end{equation}
$$

$$
\begin{align}
\hat{s}_2 &= fl(\hat{s}_1 + x_2 y_2) = (\hat{s}_1 + x_2 y_2 (1 + \delta_2))(1 + \delta_3) \notag \\
&= (x_1 y_1 (1 + \delta_1) + x_2 y_2 (1 + \delta_2))(1 + \delta_3) \notag \\
&= x_1 y_1 (1 + \delta_1)(1 + \delta_3) + x_2 y_2 (1 + \delta_2)(1 + \delta_3)
\end{align}
$$

$$
\begin{align}
\hat{s}_3 &= fl(\hat{s}_2 + x_3 y_3) = (\hat{s}_2 + x_3 y_3 (1 + \delta_4))(1 + \delta_5) \notag \\
&= (x_1 y_1 (1 + \delta_1)(1 + \delta_3) + x_2 y_2 (1 + \delta_2)(1 + \delta_3) + x_3 y_3 (1 + \delta_4))(1 + \delta_5) \notag \\
&= x_1 y_1 (1 + \delta_1)(1 + \delta_3)(1 + \delta_5) + x_2 y_2 (1 + \delta_2)(1 + \delta_3)(1 + \delta_5) \notag \\
&\hspace{12pt}+ x_3 y_3 (1 + \delta_4)(1 + \delta_5)
\end{align}
$$

n-th order inner product can be described by

$$
\begin{align}
\hat{s}_n &= fl(\hat{s}_{n-1} + x_n y_n) = (\hat{s}_{n-1} + x_n y_n(1 + \delta_{2n-2}))(1 + \delta_{2n-1}) \notag \\
&= x_1y_1 (1 + \delta_1)(1 + \delta_3) \cdots (1 + \delta_{2n-1}) \notag \\
&\hspace{11pt}+ x_2y_2 (1 + \delta_2)(1 + \delta_3) \cdots (1 + \delta_{2n-1}) \notag \\
&\hspace{11pt}+ x_3y_3 (1 + \delta_4)(1 + \delta_5) \cdots (1 + \delta_{2n-1}) \notag \\
&\hspace{51pt}\vdots \notag \\
&\hspace{11pt}+ x_ny_n (1 + \delta_{2n-2})(1 + \delta_{2n-1}),
\end{align}
$$

Since $|\delta_i| < u$ for any $i = 1, 2, \cdots$, it is possible to bound the error range by simpler form.
In the discussion of [[2]](#reference), the range of the multiple $(1 + \delta_i)$ product is described by the following relation.

$$
\begin{align}
\prod_{i=1}^n (1 + \delta_i) = 1 + \theta_n, \quad \mathrm{where} \quad |\theta_n| \le \frac{nu}{1 - nu}
\end{align}
$$

Using this relation, the amount of error can be bounded as shown below.

$$
\begin{align}
|\bm{x}^T \bm{y} - fl(\bm{x}^T \bm{y})| \le \frac{nu}{1 - nu} \sum_{i=1}^n |x_i y_i|
\end{align}
$$

## Bernoulli's Inequality

Actually, the relation of Eq. (12) can be written in slightly tighter form as shown in the equations below [[3]](#reference).
The discussion assumes the parameter range of $n = 2, 3, \cdots$, and $0 < u \ll 1$.

$$
\begin{align}
\prod_{i=1}^n (1 + \delta_i) \ge (1 - u)^n > 1 - nu
\end{align}
$$

$$
\begin{align}
\prod_{i=1}^n (1 + \delta_i) \le (1 + u)^n \le 1 + \frac{nu}{1 + (1-n)u}
\end{align}
$$

The lower bound Eq. (14) corresponds to Bernoulli's inequality, and it can be confirmed easily by the mathematical induction.
When $k=2$, the following inequality is valid.

$$
\begin{align}
(1-u)^2 = 1 - 2u + u^2 > 1-2u
\end{align}
$$

Assuming that $(1 - u)^k \ge 1 - ku$ is valid, the following relation can be obtained by multiplying $(1 - u)$.

$$
\begin{align}
(1 - u)^{k+1} \ge (1 - ku)(1 - u) = 1 - (k+1)u + ku^2 > 1 - (k+1)u
\end{align}
$$

This completes the induction proof for Eq. (14).

Applying $(1 - \frac{u}{1+u})^n$ to Eq. (14), we obtain the following relation.

$$
\begin{align}
\left( 1 - \frac{u}{1+u} \right)^n > 1 - n \frac{u}{1+u}
\end{align}
$$

$$
\begin{align}
\frac{1}{(1+u)^n} > \frac{1+u-nu}{1+u}
\end{align}
$$

If $1 + u -nu > 0$, the following relation is obtained.

$$
\begin{align}
(1 + u)^n < \frac{1+u}{1+u-nu} = 1 + \frac{nu}{1 + (1-n)u}
\end{align}
$$

## Reference

1. "IEEE Standard for Floating-Point Arithmetic" in IEEE Std 754-2019 (Revision of IEEE 754-2008), pp.1-84, 22 July 2019, doi: [10.1109/IEEESTD.2019.8766229](https://doi.org/10.1109/IEEESTD.2019.8766229).
2. Nicholas J. Higham, "Accuracy and Stability of Numerical Algorithms, Second Edition", Society for industrial and applied mathematics, 2002.
3. Dragoslav S. Mitrinović, "Analytic Inequalities", Springer Berlin, Heidelberg, 1970.
