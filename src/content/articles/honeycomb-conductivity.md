---
title: 'Honeycomb Core Density and Thermal Conductivity'
description: 'To include a honeycomb panel in a spacecraft system thermal model, the effective density and thermal conductivity as a panel must be estimated. We will discuss the evaluation methodology for honeycomb core effective properties, and introduce a calculation tool.'
pubDate: 2025-09-01
updatedDate: 2025-09-01
heroImage: ''
tags: ['thermal']
---

Using the following tool, you can estimate the effective density and thermal conductivity of the honeycomb core.
Please enter the dimensions of the honeycomb core and set the density and thermal conductivity of the core material.

<!-- markdownlint-disable MD033 MD045 -->
<div align="center">
  <iframe
    src="/widgets/honeycomb-calculator.html"
    title="Honeycomb Core Thermal Conductivity Calculator"
    width="100%"
    height="570"
    loading="lazy"
    style="max-width: 720px; width: 100%; border:1px solid #ddd; border-radius:8px; background:#fff;">
  </iframe>
</div>
<!-- markdownlint-enable MD033 MD045 -->

![honeycomb-conductivity-1](../figures/honeycomb-conductivity-1.svg)
_Figure 1: Honeycomb Core Dimensions._

## Evaluation Method for Honeycomb Core Effective Properties

In aerospace applications, honeycomb panels are often used because of their lightweight and high-stiffness characteristics.
Regarding the thermal characteristics, honeycomb panels typically have low thermal conductivity and exhibit anisotropic behavior.
When performing system thermal analysis of a spacecraft, details of the honeycomb core geometry cannot be included in the model.
Thus, effective properties such as density and thermal conductivity must be estimated and incorporated into the model as material parameters.

Using the core dimensions and material properties, the effective density and thermal conductivity of the honeycomb core can be estimated [[1]](#references).
We assume that the core shape is a regular hexagon as shown in Figure 1, but the basic idea remains the same for other core geometries.
The honeycomb core is made by shaping and bonding sheet materials, and so the wall thickness varies with direction.

Regarding the effective density $\rho_\mathrm{eff}$, we evaluate the area ratio of the core material in the horizontal cross-section.

$$
\begin{equation}
\mathrm{Reference~Area:}\quad \frac{S}{2} \times \left( \frac{S}{2\sqrt{3}} + \frac{S}{\sqrt{3}} \right) = \frac{\sqrt{3}}{4}S^2
\end{equation}
$$

$$
\begin{equation}
\mathrm{Core~Material~Area:}\quad \delta \times \left( \frac{S}{\sqrt{3}} + \frac{S}{\sqrt{3}} \right) = \frac{2}{\sqrt{3}}S\delta
\end{equation}
$$

$$
\begin{equation}
\rho_\mathrm{eff} = \frac{\frac{2}{\sqrt{3}}S\delta \times \rho}{\frac{\sqrt{3}}{4}S^2} = \frac{8\delta \rho}{3S}
\end{equation}
$$

The effective thermal conductivity must be evaluated for each direction.
For the thickness direction, the thermal conductivity is expressed in two ways: using the core material's thermal conductivity $k$ and the effective thermal conductivity in the thickness direction $k_H$.

$$
\begin{equation}
\frac{k\times \frac{2}{\sqrt{3}}S\delta}{h} = \frac{2kS\delta}{\sqrt{3}h}\quad[\mathrm{W/K}]
\end{equation}
$$

$$
\begin{equation}
\frac{k_H\times \frac{\sqrt{3}}{4}S^2}{h} = \frac{\sqrt{3}k_H S^2}{4h}\quad[\mathrm{W/K}]
\end{equation}
$$

Since these two expressions are equal, the effective thermal conductivity in the thickness direction $k_H$ can be expressed as follows.

$$
\begin{equation}
k_H = \frac{8}{3} \frac{k\delta}{S}
\end{equation}
$$

Effective thermal conductivities in the in-plane directions can be evaluated in the same way.
The effective thermal conductivity in the L-direction can be expressed in the following two ways.

$$
\begin{equation}
\frac{k\times \delta h}{\frac{2}{\sqrt{3}}S} = \frac{\sqrt{3}k\delta h}{2S}\quad[\mathrm{W/K}]
\end{equation}
$$

$$
\begin{equation}
\frac{k_L\times \frac{S}{2}h}{\frac{\sqrt{3}}{2}S} = \frac{k_L h}{\sqrt{3}}\quad[\mathrm{W/K}]
\end{equation}
$$

Since these two expressions are equal, the effective thermal conductivity in the L-direction $k_L$ can be expressed as follows.

$$
\begin{equation}
k_L = \frac{3}{2} \frac{k\delta}{S}
\end{equation}
$$

Also, the effective thermal conductivity in the W-direction $k_W$ can be expressed in the following two ways.

$$
\begin{equation}
\frac{k\times \delta h}{\frac{2}{\sqrt{3}}S} = \frac{\sqrt{3}k\delta h}{2S}\quad[\mathrm{W/K}]
\end{equation}
$$

$$
\begin{equation}
\frac{k_W\times h\left(\frac{S}{2\sqrt{3}} + \frac{S}{\sqrt{3}} \right)}{S} = \frac{\sqrt{3}k_W h}{2}\quad[\mathrm{W/K}]
\end{equation}
$$

Equating these two expressions gives the effective thermal conductivity in the W-direction $k_W$ as follows.

$$
\begin{equation}
k_W = \frac{k\delta}{S}
\end{equation}
$$

## References

1. David G. Gilmore, "Spacecraft Thermal Control Handbook. Vol. 1: Fundamental Technologies", The Aerospace Corporation Press, California, 2002
