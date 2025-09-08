---
title: 'Standard Atmosphere (U.S. Standard Atmosphere 1976, ISA, ICAO)'
description: 'U.S. Standard Atmosphere 1976, ISA (International Standard Atmosphere), ICAO Standard Atmosphere are well-known models that describe the variation of atmospheric properties such as temperature and pressure with altitude. This article explains the calculation methods for various atmospheric parameters based on these models.'
pubDate: 2025-09-08
updatedDate: 2025-09-08
heroImage: ''
tags: ['thermal']
---

The following tool allows you to calculate the temperature, pressure, and other atmospheric properties at a given altitude based on the U.S. Standard Atmosphere 1976 model.

<!-- markdownlint-disable MD033 MD045 -->
<div align="center">
  <iframe
    src="/widgets/atmosphere-calculator.html"
    title="U.S. Standard Atmosphere 1976 Calculator"
    width="100%"
    height="470"
    loading="lazy"
    style="max-width: 720px; width: 100%; border:1px solid #ddd; border-radius:8px; background:#fff;">
  </iframe>
</div>
<!-- markdownlint-enable MD033 MD045 -->

## Overview of the Atmosphere Standards

The Standard Atmosphere is a model that describes how atmospheric properties such as temperature, pressure, and density varywith altitude.
The following standards are commonly used in various fields:

- **International Standard Atmosphere (ISA)** [[1]](#references)
- **U.S. Standard Atmosphere 1976** [[2]](#references)
- **ICAO STANDARD ATMOSPHERE** [[3]](#references)

ISA and ICAO Standard Atmosphere are defined by ISO (International Organization for Standardization) and ICAO (International Civil Aviation Organization) respectively.
These models cover the properties of the atmosphere approximately from sea level to an altitude of 80 km.
U.S. Standard Atmosphere 1976 is defined by NOAA (National Oceanic and Atmospheric Administration), NASA (National Aeronautics and Space Administration), and USAF (United States Air Force).
This model covers the properties of the atmosphere from sea level to an altitude of 1000 km.
From sea level to an altitude of 86 km, the definitions in U.S. Standard Atmosphere 1976 are nearly identical to those in the ISA and ICAO Standard Atmosphere.

## Geopotential Height

In each of these atmospheric models, it is common to express altitude using geopotential height.
We consider the gravitational potential $\Phi(Z)$, which satisfies the following relationship.

$$
\begin{equation}
\Phi(Z) = \int_{0}^{Z} g(Z) ~dZ
\end{equation}
$$

In principle, the gravitational acceleration $g(Z)$ varies with altitude, and thus the increase in gravitational potential is not constant with respect to geometric height $Z$.
The geopotential height $H$ is a transformed height, defined so that the gravitational potential changes proportionally to it with a constant gravitational acceleration $g_0$.
Using the geopotential height, the evaluation of atmospheric properties becomes simpler because the gravitational acceleration can be treated as a constant.

$$
\begin{equation}
H = \frac{\Phi(Z)}{g_0}
\end{equation}
$$

The variation of gravitational acceleration with altitude is expressed as follows.

$$
\begin{equation}
g(Z) = g_0 \left( \frac{r_0}{r_0 + Z} \right)^2
\end{equation}
$$

Thus, the relationship between geopotential height $H$ and geometric height $Z$ is given by:

$$
\begin{equation}
H = \int_{0}^{Z} \left( \frac{r_0}{r_0 + Z} \right)^2 ~dZ = \frac{r_0 Z}{r_0 + Z}
\end{equation}
$$

$$
\begin{equation}
Z = \frac{r_0 H}{r_0 - H}
\end{equation}
$$

## Molecular Scale Temperature

Molecular scale temperature is a modified temperature, to account for changes in the mean molecular weight with altitude.
By applying molecular scale temperature in the ideal gas law, mean molecular weight can be treated as a constant value $M_0$.

$$
\begin{equation}
T_M = T \frac{M_0}{M}
\end{equation}
$$

In U.S. Standard Atmosphere 1976, the mean molecular weight is assumed to be constant from sea level to an altitude of 80 km, and varies with altitude above 80 km.
Therefore, from sea level to an altitude of 80 km, molecular scale temperature $T_M$ can be considered the same as the kinematic temperature $T$.
ISA and ICAO Standard Atmosphere define the model up to an altitude of 80 km, and do not introduce the concept of molecular scale temperature.

## Temperature Variation with Altitude

For all of these models, the atmosphere is divided into several layers in altitude, and the temperature variation in each layer is represented by a linear function or a constant value.
Table 1 shows the temperature variation in each layer according to U.S. Standard Atmosphere 1976.
In this table, altitude is expressed as geopotential height. The maximum altitude of 84.852 km corresponds to a geometric height of 86 km.
We do not discussed in this article, but U.S. Standard Atmosphere 1976 also defines atmospheric properties above 86 km up to 1000 km.

_Table 1: Temperature Variation in U.S. Standard Atmosphere 1976_

| Layer | Geopotential height $H$ [km] | Temperature gradient $\frac{dT_M}{dH}$ [K/km] |
| ----- | ---------------------------- | --------------------------------------------- |
| 0     | 0                            | -6.5                                          |
| 1     | 11                           | 0                                             |
| 2     | 20                           | 1.0                                           |
| 3     | 32                           | 2.8                                           |
| 4     | 47                           | 0                                             |
| 5     | 51                           | -2.8                                          |
| 6     | 71                           | -2.0                                          |
| 7     | 84.852                       | 0                                             |

The temperature variation defined in ISA and ICAO Standard Atmosphere is shown in Table 2.
Although there are minor differences in the applicable range near sea level and at around 80 km altitude, the basic temperature variation is the same as that of U.S. Standard Atmosphere 1976.

_Table 2: Temperature Variation in ISA and ICAO Standard Atmosphere_

| Geopotential height (km) | Temperature (K)          | Temperature gradient (K/km) |
| ------------------------ | ------------------------ | --------------------------- |
| -2.00 (-5.00 for ICAO)   | 301.15 (320.65 for ICAO) | -6.5                        |
| 0.00                     | 288.15                   | -6.5                        |
| 11.00                    | 216.65                   | 0                           |
| 20.00                    | 216.65                   | 1.0                         |
| 32.00                    | 228.65                   | 2.8                         |
| 47.00                    | 270.65                   | 0                           |
| 51.00                    | 270.65                   | -2.8                        |
| 71.00                    | 214.65                   | -2.0                        |
| 80.00                    | 196.65                   |                             |

## Pressure Variation with Altitude

The pressure variation with altitude can be calculated using the previously defined temperature variation model and the assumption of hydrostatic equilibrium.
Hydrostatic equilibrium refers to the state where the gravitation force and the pressure gradient force are balanced, as shown in Figure 1.
This relationship can be expressed as follows.

$$
\begin{equation}
dP = -\rho g(Z) ~dZ
\end{equation}
$$

Using geopotential height, it can be rewritten as:

$$
\begin{equation}
dP = -\rho g_0 ~dH
\end{equation}
$$

![standard-atmosphere-1](../figures/standard-atmosphere-1.svg)
_Figure 1: Hydrostatic Equilibrium._

Based on the ideal gas law, the following relationship holds:

$$
\begin{equation}
P = \rho \frac{R^*}{M} T
\end{equation}
$$

This relationship can be rewritten in the following form using molecular scale temperature.

$$
\begin{equation}
P = \rho \frac{R^*}{M_0} T_M
\end{equation}
$$

Based on the equation of hydrostatic equilibrium and the ideal gas law, the following equation can be derived:

$$
\begin{equation}
\frac{dP}{P} = -\frac{M_0g_0}{R^*T_M} ~dH
\end{equation}
$$

Integrating this equation from sea level to altitude $H$, we obtain the pressure as a function of temperature.

$$
\begin{equation}
\ln P - \ln P_0 = -\frac{M_0g_0}{R^*} \int_{0}^{H} \frac{1}{T_M} ~dH
\end{equation}
$$

In the temperature variation model, the temperature change with altitude is linear or constant in each layer.
Thus, the integration can be performed for each layer and summed sequentially to obtain the pressure at arbitrary altitude.

The integration result for the linear temperature variation is expressed as follows.

$$
\begin{align}
\ln P - \ln P_i &= -\frac{M_0g_0}{R^*} \left[ \frac{1}{L_i} \ln (L_i(H- H_i) + T_i) \right]_{H_i}^{H} \notag \\
&= -\frac{M_0g_0}{R^*L_i} \Big\{ \ln(L_i(H- H_i) + T_i) - \ln T_i \Big\} \notag \\
&= -\frac{M_0g_0}{R^*L_i} \ln \left( \frac{L_i(H- H_i) + T_i}{T_i} \right)
\end{align}
$$

$$
\begin{align}
P &= P_i \left( \frac{L_i(H- H_i) + T_i}{T_i} \right)^{-\frac{M_0 g_0}{R^* L_i}}
\end{align}
$$

For the case where the temperature is constant, the integration result is expressed as shown below.

$$
\begin{align}
\ln P - \ln P_i &= -\frac{M_0g_0}{R^*} \left[ \frac{H}{T_i} \right]_{H_i}^{H} = -\frac{M_0g_0}{R^*T_i} (H - H_i)
\end{align}
$$

$$
\begin{align}
P &= P_i \exp\left\{ -\frac{M_0 g_0}{R^* T_i} (H - H_i) \right\}
\end{align}
$$

## Other Atmospheric Properties

Other properties such as density and dynamic viscosity can also be calculated from temperature and pressure.
Density can be obtained from the ideal gas law as follows.

$$
\begin{equation}
\rho = \frac{P M_0}{R^* T_M}
\end{equation}
$$

Dynamic viscosity can be calculated as follows.
The constants $\beta$ and $S$ are empirical constants based on experiments, and this formula is not applicable above 86 km.

$$
\begin{equation}
\mu = \frac{\beta T^{\frac{3}{2}}}{T + S}
\end{equation}
$$

Kinematic viscosity can be calculated from dynamic viscosity and density.

$$
\begin{equation}
\nu = \frac{\mu}{\rho}
\end{equation}
$$

Thermal conductivity of atmosphere $k$ can be calculated as follows.

$$
\begin{equation}
k = \frac{2.64638 \times 10^{-3} T^\frac{3}{2}}{T + 245.4 \times 10^{-\frac{12}{T}}}
\end{equation}
$$

<!-- ## High-Altitude Extension of the U.S. Standard Atmosphere 1976 -->

## Nomenclature

| Symbol  | Description              | Value                   |
| ------- | ------------------------ | ----------------------- |
| $R^*$   | Gas constant             | 8.31432 J/(mol·K)       |
| $M_0$   | Molar mass of air        | 0.0289644 kg/mol        |
| $g_0$   | Gravity at sea level     | 9.80665 m/s²            |
| $P_0$   | Pressure at sea level    | 101325 Pa               |
| $T_0$   | Temperature at sea level | 288.15 K                |
| $\beta$ |                          | 1.458e-6 kg/(m·s·K^0.5) |
| $S$     | Sutherland constant      | 110.4 K                 |
| $Z$     | Geometric height         | m                       |
| $H$     | Geopotential height      | m                       |
| $r_0$   | Radius of the Earth      | 6356766 m               |

## References

1. Standard Atmosphere (identical with the ICAO and WMO Standard Atmospheres from 2 to 32 km), First Edition 1975-05-15, ISO 2533-1975
2. U.S. Standard Atmosphere, 1976, NOAA-S/T 76-1562
3. Manual of the ICAO Standard Atmosphere, extended to 80 kilometers (262 500 feet), 1993, Third Edition, Doc 7488/3
