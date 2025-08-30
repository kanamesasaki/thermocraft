---
title: 'Derivation of the Hertz-Knudsen Equation'
description: 'Hertz-Knudsen Equation (also known as Hertz-Knudsen-Langmuir Equation) is a classical model for representing mass flux due to condensation and evaporation. In this article, we will derive the Hertz-Knudsen Equation starting from the Maxwell-Boltzmann distribution.'
pubDate: 2025-08-31
updatedDate: 2025-08-31
heroImage: ''
tags: ['thermal', 'statistical mechanics']
---

Hertz-Knudsen Equation (also known as Hertz-Knudsen-Langmuir Equation) is a classical model for representing mass flux due to condensation and evaporation.
Assuming the Maxwell-Boltzmann distribution for the molecule velocity distribution near the liquid-vapor interface, we can estimate the mass flux for both condensation and evaporation processes.
By summing these contributions, we obtain the overall mass flux, which is called the Hertz-Knudsen Equation.

## Maxwell-Boltzmann Distribution

First, we review the equation for the Maxwell-Boltzmann distribution.

$$
\begin{equation}
% \label{eq:M-B}
dw_v = \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}}
\exp \left[ -\frac{m(v_x^2+v_y^2+v_z^2)}{2k_\mathrm{B}T} \right] dv_x dv_y dv_z
\end{equation}
$$

Multiplying Eq. (1) by the number of molecules in unit volume, we obtain Eq. (2).
Eq. (2) represents the distribution for the number of particles in velocity space.

$$
\begin{equation}
dN_v = \frac{N}{V} \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}}
\exp \left[ -\frac{m(v_x^2+v_y^2+v_z^2)}{2k_\mathrm{B}T} \right] dv_x dv_y dv_z
\end{equation}
$$

Based on the particle number distribution given by Eq. (2), we can estimate the number of molecules that collide with the liquid-vapor interface.

## Estimation of Mass Flux Impinging on the Liquid-Vapor Interface

Particles with a negative velocity $v_z$ and located at $z \le |v_z| \Delta t$ will collide with the interface within the time interval of $\Delta t$.
Thus, the number of molecules colliding with the unit area interface for unit time can be expressed as follows.

$$
\begin{align}
\nu_z &= \int_{-\infty}^{\infty} \int_{-\infty}^{\infty} \int_{-\infty}^{0}
\frac{N}{V} \left( \frac{m}{2\pi k_\mathrm{B} T} \right)^{\frac{3}{2}}
\exp \left[ -\frac{m(v_x^2 + v_y^2 + v_z^2)}{2k_\mathrm{B}T} \right]
|v_z|~dv_x dv_y dv_z \notag \\
&= \frac{N}{V} \sqrt{\frac{m}{2\pi k_\mathrm{B} T}} \int_{-\infty}^{0}
\exp \left[ -\frac{m v_z^2}{2k_\mathrm{B}T} \right] |v_z|~dv_z \notag \\
&= \frac{N}{V} \sqrt{\frac{m}{2\pi k_\mathrm{B} T}}
\left[ \frac{k_\mathrm{B}T}{m} \exp \left[ - \frac{m v_z^2}{2k_\mathrm{B}T} \right] \right]_{-\infty}^{0} \notag \\
&= \frac{Nk_\mathrm{B}T}{mV} \sqrt{\frac{m}{2\pi k_\mathrm{B} T}}
= \frac{P}{\sqrt{2\pi m k_\mathrm{B} T}}
\end{align}
$$

To perform the integration over $v_x$ and $v_y$, we used the result of the Gaussian integral given in Eq. (4).

$$
\begin{equation}
% \label{eq:gauss}
\int_{-\infty}^{\infty} \exp \left[ -\alpha x^2 \right] dx = \sqrt{\frac{\pi}{\alpha}}
\end{equation}
$$

As shown in Eq. (3), we obtained the number of molecules impinging on the liquid-vapor interface.
Multiplying $\nu_z$ by the mass of a molecule $m$ gives the mass flux impinging on the interface.

$$
\begin{equation}
j = m\nu_z = P \sqrt{\frac{m}{2\pi k_\mathrm{B} T}}
\end{equation}
$$

The Maxwell-Boltzmann distribution is applicable for the gas molecules in the equilibrium state.
The region of interest here is where the gas molecules interact with the liquid-vapor interface.
This is not a state where the gas molecules are freely moving and interacting with each other.
Also, if there is overall condensation or evaporation occurring, the system is in a non-equilibrium state.
Therefore, the use of the Maxwell-Boltzmann distribution is an assumption to simply describe the velocity distribution of the molecules.

## Hertz-Knudsen Equation

Here, we introduce the condensation coefficient $\sigma_c$ and the evaporation coefficient $\sigma_e$, which represent the fraction of molecules that actually condense or evaporate upon colliding with the liquid-vapor interface.

$$
\begin{equation}
% \label{eq:Marek_Straub_2001_Eq3}
\sigma_e = \frac{\text{number of molecules transferred to the vapor phase}}{\text{number of molecules emitted from the interface}}
\end{equation}
$$

$$
\begin{equation}
% \label{eq:Marek_Straub_2001_Eq4}
\sigma_c = \frac{\text{number of molecules absorbed by the liquid phase}}{\text{number of molecules impinging on the interface}}
\end{equation}
$$

Using these coefficients, the mass fluxes associated with condensation and evaporation can be expressed as follows.
In Eqs. (8) and (9), the evaporation direction is taken as positive.

$$
\begin{equation}
j_e = \sigma_e \sqrt{\frac{m}{2\pi k_\mathrm{B}}} \frac{P_\mathrm{sat}(T_l)}{\sqrt{T_l}}
\end{equation}
$$

$$
\begin{equation}
j_c = - \sigma_c \sqrt{\frac{m}{2\pi k_\mathrm{B}}} \frac{P_v}{\sqrt{T_v}}
\end{equation}
$$

By summing the mass fluxes associated with condensation and evaporation, we obtain the Hertz-Knudsen Equation that represents the overall mass flux.

$$
\begin{equation}
% \label{eq:hertz-knudsen}
j^{LV} = \sqrt{\frac{m}{2\pi k_\mathrm{B}}} \left( \sigma_e \frac{P_\mathrm{sat}(T_l)}{\sqrt{T_l}} - \sigma_c \frac{P_v}{\sqrt{T_v}} \right)
\end{equation}
$$

The remaining question is how to determine the values of the condensation coefficient $\sigma_c$ and the evaporation coefficient $\sigma_e$.
For the case where the liquid-vapor interface is in equilibrium, the overall mass flux $j^{LV}$ becomes zero.
Also, $T_l = T_v$ and $P_v = P_{sat}(T_l)$ hold for equilibrium. Thus, we obtain $\sigma_e = \sigma_c$.
On the other hand, in a non-equilibrium state where evaporation or condensation is occurring, $\sigma_e$ and $\sigma_c$ are generally not equal, and their values need to be determined through experiments or simulation.
For water evaporation and condensation, numerous studies have been conducted to measure these coefficients.
However, the results can vary depending on the evaluation methodologies, making it challenging to uniquely determine the values of $\sigma_e$ and $\sigma_c$ for given temperature and pressure conditions [[1]](#reference).

## Vapor-Liquid Equilibrium

In this section, we will discuss what is the equilibrium state at the liquid-vapor interface (i.e. Vapor-Liquid Equilibrium) [[2]](#reference).
For simplicity, we focus on the case of a single component system consisting of only vapor phase and liquid phase.
To begin with, we assume that the entire system is isolated and in equilibrium.
These assumptions can be expressed as follows:

$$
\begin{align}
dU &= 0 \\
dS &= dS_l + dS_v = 0 \\
dV &= dV_l + dV_v = 0 \\
dN &= dN_l + dN_v = 0
\end{align}
$$

The differential change of the system's energy $dU$ can be expressed as shown in Eq. (15).
In this equation, chemical potential $\mu$ corresponds to the Gibbs free energy per mole.

$$
\begin{equation}
dU = T_l dS_l + T_v dS_v - P_l dV_l - P_v dV_v + \mu_l dN_l + \mu_v dN_v
\end{equation}
$$

This equation can be re-written by using Eqs. (11)--(14).

$$
\begin{equation}
dU = (T_v - T_l)dS_v - (P_v - P_l)dV_v + (\mu_v - \mu_l)dN_v = 0
\end{equation}
$$

Since $dS_v, dV_v, dN_v$ are independent variables, each coefficient must be zero, to satisfy Eq. (16).
Thus, the following relationships hold at vapor-liquid equilibrium.

$$
\begin{align}
P_l &= P_v \\
T_l &= T_v \\
\mu_l &= \mu_v
\end{align}
$$

These relationships have the following physical meanings.

1. The pressures of the liquid and vapor phases are equal -> The forces at the interface are balanced, and the interface does not move.
2. The temperatures of the liquid and vapor phases are equal -> There is no heat transfer.
3. The chemical potentials of the liquid and vapor phases are equal -> There is no mass transfer.

The symbols used in this section are summarized in the following table.

| Symbol                | Description                          | Unit (SI)              |
| --------------------- | ------------------------------------ | ---------------------- |
| $T$                   | Temperature                          | $\mathrm{K}$           |
| $N$                   | Amount of substance                  | $\mathrm{mol}$         |
| $V$                   | Volume                               | $\mathrm{m^3}$         |
| $P_\mathrm{sat}$      | Saturation pressure                  | $\mathrm{Pa}$          |
| $P$                   | Pressure                             | $\mathrm{Pa}$          |
| $\sigma_e,\ \sigma_c$ | Evaporation/Condensation coefficient |                        |
| $U$                   | Internal energy                      | $\mathrm{J}$           |
| $S$                   | Entropy                              | $\mathrm{J\,K^{-1}}$   |
| $\mu$                 | Chemical potential                   | $\mathrm{J\,mol^{-1}}$ |

## Reference

1. R. Marek, J. Straub, "Analysis of the evaporation coefficient and the condensation coefficient of water", International Journal of Heat and Mass Transfer, Volume 44, Issue 1, 2001, Pages 39-53, doi: [10.1016/S0017-9310(00)00086-7](<https://doi.org/10.1016/S0017-9310(00)00086-7>).
2. John M. Prausnitz, Rüdiger N. Lichtenthaler, Edmundo Gomes de Azevedo, "Molecular Thermodynamics of Fluid-Phase Equilibria", 3rd Edition, Prentice Hall, 1999.
