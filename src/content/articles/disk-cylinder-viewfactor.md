---
title: 'View Factors for Disks and Cylinder Walls'
description: 'We will derive various view factors for disks and cylinder walls with finite area, based on the analytical view factor from a plate element to a disk.'
pubDate: 2025-08-11
updatedDate: 2025-08-11
heroImage: ''
tags: ['thermal']
---

In a previous article, we derived the analytical solution for the [disk view factor from a plate element](https://thermocraft.space/articles/disk-viewfactor/).
Using this result, we can derive various view factors related to disks and cylinders with finite areas.

## View Factor between Two Disks

As shown in Figure 1, a disk with radius $R_1$ (disk1) and a disk with radius $R_2$ (disk2) are placed parallel to each other, separated by a distance $h$.
The view factor from disk1 to disk2 can be evaluated by integrating the disk view factor from a plate element on disk1 and dividing by the area of disk1.

![disk-viewfactor-1](../figures/disk-cylinder-viewfactor-1.svg)
_Figure 1: Geometrical Configuration of two Parallel Disks for View Factor Evaluation._

We derived the disk view factor from a plate element in the [previous article](https://thermocraft.space/articles/disk-viewfactor/).
The corresponding expression (disk view factor from a plate element with offset $a$ from the disk center axis) is given by

$$
\begin{align}
F_\mathrm{offset} &= \frac{1}{2} - \frac{1}{2}\frac{a^2 + h^2 - R_2^2}{\sqrt{(R_2^2 + a^2 + h^2)^2 - 4a^2R_2^2}}
\end{align}
$$

Integrating Eq. (1) over disk1, the result is given by

$$
\begin{align}
&\int_0^{2\pi} \int_0^{R_1} \left(\frac{1}{2} - \frac{1}{2}\frac{a^2 + h^2 - R_2^2}{\sqrt{(R_2^2 + a^2 + h^2)^2 - 4a^2R_2^2}}\right) a~ da d\theta \notag \\
&=  \pi \int_0^{R_1} a~ da - \pi \int_0^{R_1} \frac{(a^2 + h^2 - R_2^2)}{\sqrt{(R_2^2 + a^2 + h^2)^2 - 4a^2R_2^2}} a~ da \notag \\
&= \frac{\pi}{2} R_1^2 - \pi \int_0^{R_1} \frac{(a^2 + h^2 - R_2^2)}{\sqrt{(a^2 + h^2 - R_2^2)^2 + 4h^2R_2^2}} a~ da \notag \\
&= \frac{\pi}{2} R_1^2 - \frac{\pi}{2} \left[ \sqrt{(a^2 + h^2 -R_2^2)^2 + 4h^2R_2^2} \right]_0^{R_1} \notag \\
&= \frac{\pi}{2} R_1^2 - \frac{\pi}{2} \left\{  \sqrt{(R_1^2 + h^2 -R_2^2)^2 + 4h^2R_2^2} - (h^2 +R_2^2) \right\}
\end{align}
$$

Dividing the integration result by the area of disk1, we obtain the view factor from disk1 to disk2.

$$
\begin{align}
F_{12} &= \frac{1}{2} - \frac{1}{2} \left\{  \sqrt{\left(1 + \frac{h^2}{R_1^2} -\frac{R_2^2}{R_1^2}\right)^2 + \frac{4h^2R_2^2}{R_1^4}} - \left(\frac{h^2}{R_1^2} +\frac{R_2^2}{R_1^2}\right) \right\} \notag \\
&=\frac{1}{2}\left(1 + \frac{h^2}{R_1^2} +\frac{R_2^2}{R_1^2}\right) - \frac{1}{2}  \sqrt{\left(1 + \frac{h^2}{R_1^2} -\frac{R_2^2}{R_1^2}\right)^2 + \frac{4h^2R_2^2}{R_1^4}}
\end{align}
$$

If $R_1 = R_2 = R$, the view factor can be simplified as follows:

$$
\begin{equation}
F_{12} = \frac{1}{2} \left\{ 2 + \frac{h^2}{R^2} - \sqrt{\frac{h^4}{R^4} + 4\frac{h^2}{R^2}} \right\}
\end{equation}
$$

## View Factors for Cylindrical Container Inner Walls

For the next step, we consider the view factors between the inner walls of a cylindrical container, as shown in Figure 2.

![disk-viewfactor-2](../figures/disk-cylinder-viewfactor-2.svg)
_Figure 2: View Factor Evaluation for Cylinder Inner Walls._

The view factor from the bottom surface to the top surface can be expressed by Eq. (4).
From the bottom surface, either the top surface or the side wall can be seen, so the view factor from the bottom surface to the side wall is expressed as follows.

$$
\begin{align}
F_{13} &= 1 - F_{12}
= \frac{1}{2} \left\{  - \frac{h^2}{R^2} + \sqrt{\frac{h^4}{R^4} + 4\frac{h^2}{R^2}} \right\}
\end{align}
$$

Conversely, the view factor from the side wall to the bottom surface can be determined by the reciprocal relation.
The view factor from the side wall to the top surface is identical to that from the side wall to the bottom surface.

$$
\begin{align}
F_{31} &= \frac{A_1}{A_3} F_{13}
= \frac{R}{4h} \left\{  - \frac{h^2}{R^2} + \sqrt{\frac{h^4}{R^4} + 4\frac{h^2}{R^2}} \right\} \notag \\
&= - \frac{h}{4R} + \frac{1}{4} \sqrt{\frac{h^2}{R^2} + 4}
\end{align}
$$

From the cylinder side wall, the top, bottom, and side wall can be seen.
Thus, the view factor from the side wall to itself is expressed as follows.

$$
\begin{align}
F_{33} = 1 - 2 F_{31}
= 1 + \frac{h}{2R} - \sqrt{\frac{h^2}{4R^2} + 1}
\end{align}
$$

## View Factors for Cylindrical Inner Wall Segments

Now, let us think about the view factor for a part of the side wall.
As shown in Figure 3, we divide the cylinder wall into two parts: the upper part (3A) and the lower part (3B).

![disk-viewfactor-3](../figures/disk-cylinder-viewfactor-3.svg)
_Figure 3: View Factor Evaluation for a Part of Cylinder Inner Wall._

Using the result of Eq. (5), we can calculate the view factor from the bottom surface to a side wall with some offset.

$$
\begin{align}
&F_{1-3A} = F_{1-3A3B} - F_{1-3B} \notag \\
&= \frac{1}{2} \left\{  - \frac{h_A^2 + 2h_A h_B}{R^2} + \sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\}
\end{align}
$$

For this result, it is possible to use the reciprocal relation to find the view factor from the side wall to the bottom surface.

$$
\begin{align}
&F_{3A-1} = \frac{A_1}{A_{3A}} F_{1-3A} \notag \\
&= \frac{R}{4h_A} \left\{  - \frac{h_A^2 + 2h_A h_B}{R^2} + \sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\}
\end{align}
$$

Using the result of Eq. (9), we can find the view factor from one side wall to the next side wall.

$$
\begin{align}
&F_{3A-3B} = F_{3A-2} - F_{3A-1} \notag \\
&=-\frac{h_A}{4R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} - \frac{R}{4h_A} \notag \\
&\quad \times \left\{  - \frac{h_A^2 + 2h_A h_B}{R^2}+ \sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\} \notag \\
&=\frac{h_B}{2R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} - \frac{R}{4h_A} \left\{\sqrt{\frac{(h_A + h_B)^4}{R^4} + 4\frac{(h_A + h_B)^2}{R^2}} - \sqrt{\frac{h_B^4}{R^4} + 4\frac{h_B^2}{R^2}} \right\} \notag \\
&=\frac{h_B}{2R} + \frac{1}{4} \sqrt{\frac{h_A^2}{R^2} + 4} + \frac{h_B}{4h_A} \sqrt{\frac{h_B^2}{R^2} + 4} - \frac{h_A + h_B}{4h_A} \sqrt{\frac{(h_A + h_B)^2}{R^2} + 4}
\end{align}
$$

Finally, we can find the view factor between two side wall sections with some distance.
As shown in Figure 4, the cylindrical side wall is divided into three parts: the upper part (3A), the middle part (3B), and the lower part (3C).

![disk-viewfactor-4](../figures/disk-cylinder-viewfactor-4.svg)
_Figure 4: View Factor between Cylinder Inner Walls._

As we already know the view factor between two side walls next to each other, we can use this result to find the view factor from 3A to 3C.

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

Since we are free to set the values of $h_A, h_B, h_C$, we can find the view factor between side walls with arbitrary spacing and width.

## Reference

1. John R. Howell, M. Pinar Mengüç, "Radiative transfer configuration factor catalog: A listing of relations for common geometries", Journal of Quantitative Spectroscopy and Radiative Transfer, Volume 112, Issue 5, 2011, Pages 910-912, [https://doi.org/10.1016/j.jqsrt.2010.10.002](https://doi.org/10.1016/j.jqsrt.2010.10.002)
2. A Catalog of Configuration Factors, 3rd Edition, [https://www.thermalradiation.net/indexCat.html](https://www.thermalradiation.net/indexCat.html)
3. View Factor Calculator, [https://sterad.net](https://sterad.net)
