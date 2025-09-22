---
title: 'Analysis of Bimetal Deformation'
description: 'If two plates with different materials are bonded together, it is called bimetall. Due to the different thermal expansion coefficient (CTE), a bimetallic strip causes deformation by temperature change.'
pubDate: 2025-08-02
updatedDate: 2025-09-22
heroImage: ''
tags: ['structural mechanics', 'thermal']
---

If two plates with different materials are bonded together, it may cause deformation by the temperature change.
This deformation is caused by the different thermal expansion coefficients (CTE) of each material, and such materials are called bimetals.

In some devices, this property is actively utilized, such as bimetallic thermometers and thermostats.
In the field of space applications, bimetals are used in thermal louvers for spacecraft.
This device is desined to control the amount of radiative heat rejection, by rotating the louver blades with bimetal actuators.
The advantages of this design are that the actuator can be activated automatically by the temperature of the actuator itself, and it does not require moving parts such as motors or bearings for rotation.
On the other hand, the thermal louvers consist of the blades, actuators, and frames to hold them, which results in a certain amount of weight.

Thermal louvers are not commonly used in the recent space missions, but some applications can be found in exploration missions, where the spacecraft experiences significant thermal environment changes throughout the mission phases.

_Table 1: Application Examples of Spacecraft Thermal Louver_

| Spacecraft Name      | COSPAR ID            | Reference           |
| -------------------- | -------------------- | ------------------- |
| Mariner 4            | 1964-077A            | [[3]](#references)  |
| Pioneer 6            | 1965-105A            | [[4]](#references)  |
| Voyager 1, Voyager 2 | 1977-084A, 1977-076A | [[5]](#references)  |
| Ohzora               | 1984-015A            | [[6]](#references)  |
| Sakigake             | 1985-001A            | [[6]](#references)  |
| Suisei               | 1985-073A            | [[6]](#references)  |
| Akebono              | 1989-016A            | [[6]](#references)  |
| Magellan             | 1989-033B            | [[7]](#references)  |
| HALCA                | 1997-005A            | [[6]](#references)  |
| Landsat 7            | 1999-020A            | [[8]](#references)  |
| Rosetta              | 2004-006A            | [[9]](#references)  |
| Kaguya               | 2007-001A            | [[10]](#references) |
| Dawn                 | 2007-043A            | [[11]](#references) |
| Parker Solar Probe   | 2018-065A            | [[12]](#references) |
| Psyche               | 2023-157A            | [[13]](#references) |
| Europa Clipper       | 2024-182A            | [[14]](#references) |

## Analytical Expression of Bending Radius

Under uniform temperature changes without external forces or moments, the deformation (bending radius $\rho$) of a bimetallic strip can be expressed analytically.
As shown in Figure 1, the position of the neutral line where no strain occurs is set as the origin in the Z direction, and the position of the bottom surface of the object is set as $R$.

![bimetal-1](../figures/bimetal-1.svg)
_Figure 1: Deformation of Bi-metal Beam._

In this configuration, the axial force and moment can be expressed as follows:

$$
\begin{align*}
N_x &= \int^{R+h_1}_{R} \sigma_1 b\ dz + \int^{R+h_1+h_2}_{R+h_1} \sigma_2 b\ dz \\
&= \int^{R+h_1}_{R} E_1 \left( \frac{z}{\rho} - \alpha_1 \Delta T \right) b\ dz + \int^{R+h_1+h_2}_{R+h_1} E_2 \left( \frac{z}{\rho} - \alpha_2 \Delta T \right) b\ dz \\
&= E_1 b \left[ \frac{z^2}{2\rho} - \alpha_1 \Delta T z \right]^{R+h_1}_{R} + E_2 b \left[ \frac{z^2}{2\rho} - \alpha_2 \Delta T z \right]^{R+h_1+h_2}_{R+h_1} \\
&= E_1 b \left( \frac{2Rh_1 + h_1^2}{2\rho} - \alpha_1 \Delta T h_1 \right) + E_2 b \left( \frac{2(R+h_1)h_2 + h_2^2}{2\rho} - \alpha_2 \Delta T h_2 \right)
\end{align*}
$$

$$
\begin{align*}
M_y &= \int^{R+h_1}_{R} \sigma_1 zb\ dz + \int^{R+h_1+h_2}_{R+h_1} \sigma_2 zb\ dz \\
&= \int^{R+h_1}_{R} E_1 \left( \frac{z}{\rho} - \alpha_1 \Delta T \right) zb\ dz + \int^{R+h_1+h_2}_{R+h_1} E_2 \left( \frac{z}{\rho} - \alpha_2 \Delta T \right) zb\ dz \\
&= E_1 b \left[ \frac{z^3}{3\rho} - \frac{\alpha_1 \Delta T z^2}{2} \right]^{R+h_1}_{R} + E_2 b \left[ \frac{z^3}{3\rho} - \frac{\alpha_2 \Delta T z^2}{2} \right]^{R+h_1+h_2}_{R+h_1} \\
&= E_1 b \left( \frac{3R^2h_1 + 3Rh_1^2 + h_1^3}{3\rho} - \frac{\alpha_1 \Delta T (2Rh_1 + h_1^2)}{2} \right) \\
&+ E_2 b \left( \frac{3(R+h_1)^2h_2 + 3(R+h_1)h_2^2 + h_2^3}{3\rho} - \frac{\alpha_2 \Delta T (2(R+h_1)h_2 + h_2^2)}{2} \right)
\end{align*}
$$

The relationship between strain and bending radius is described by Eq. (1).

$$
\begin{equation}
% \label{eq:curvature}
\epsilon(z) = \frac{(\rho - z)d\theta - \rho d\theta}{\rho d\theta} = \frac{-z}{\rho}
\end{equation}
$$

Since we assume that there are no external forces, the total stress in the cross-section of the bimetallic strip (axial force) is zero. This leads to the relationship expressed in Eq. (2).

$$
\begin{equation}
% \label{eq:force}
R (E_1 h_1 + E_2 h_2) + E_2 h_1 h_2 + \frac{1}{2} (E_1 h_1^2 + E_2 h_2^2)= \rho \Delta T (E_1 \alpha_1 h_1 + E_2 \alpha_2 h_2)
\end{equation}
$$

Similarly, as the moment is also zero over the cross-section, the following relationship can be obtained.

$$
\begin{align}
% \label{eq:moment}
&R^2 (E_1 h_1 + E_2 h_2) + R (E_1 h_1^2 + 2 E_2 h_1 h_2 + E_2 h_2^2) + \frac{1}{3} E_1 h_1^3 + E_2 h_1^2 h_2 + E_2 h_1 h_2^2 + \frac{1}{3} E_2 h_2^3 \notag \\
&= E_1 \alpha_1 \Delta T \rho R h_1 + E_2 \alpha_2 \Delta T \rho (R + h_1)h_2 + \frac{1}{2} E_1 \alpha_1 \Delta T \rho h_1^2 + \frac{1}{2} E_2 \alpha_2 \Delta T \rho h_2^2
\end{align}
$$

Using the axial force relation Eq. (2), we can reduce the order of $R$ in the moment relation Eq. (3).

$$
\begin{align}
& \frac{1}{2} R (E_1 h_1^2 + 2 E_2 h_1 h_2 + E_2 h_2^2) + \frac{1}{3} E_1 h_1^3 + E_2 h_1^2 h_2 + E_2 h_1 h_2^2 + \frac{1}{3} E_2 h_2^3 \notag \\
&= \rho E_2 \alpha_2 \Delta T h_1 h_2 + \frac{1}{2} \rho E_1 \alpha_1 \Delta T h_1^2 + \frac{1}{2} \rho E_2 \alpha_2 \Delta T h_2^2
\end{align}
$$

Deleting $R$ from the following two equations, we can obtain the bending radius $\rho$.

$$
\begin{align*}
&R (E_1 h_1 + E_2 h_2) (E_1 {h_1}^2 + 2 E_2 h_1 h_2 + E_2 {h_2}^2) \notag \\
&+ \frac{2}{3} (E_1 h_1 + E_2 h_2) ( E_1 {h_1}^3 + 3 E_2 {h_1}^2 h_2 + 3 E_2 h_1 {h_2}^2 + E_2 {h_2}^3 ) \\
&~~= \rho \Delta T (E_1 h_1 + E_2 h_2) (2 E_2 \alpha_2 h_1 h_2 + E_1 \alpha_1 {h_1}^2 + E_2 \alpha_2 {h_2}^2)
\end{align*}
$$

$$
\begin{align*}
&R (E_1 h_1 + E_2 h_2) (E_1 {h_1}^2 + 2 E_2 h_1 h_2 + E_2 {h_2}^2) + \frac{1}{2} (E_1 {h_1}^2 + 2 E_2 h_1 h_2 +E_2 {h_2}^2)^2 \\
&~~= \rho \Delta T (E_1 \alpha_1 h_1 + E_2 \alpha_2 h_2)(E_1 {h_1}^2 + 2 E_2 h_1 h_2 + {E_2}^2)
\end{align*}
$$

Some intermediate steps are shown below.

$$
\begin{align*}
&\rho \Delta T (E_1 h_1 + E_2 h_2) (2 E_2 \alpha_2 h_1 h_2 + E_1 \alpha_1 {h_1}^2 + E_2 \alpha_2 {h_2}^2) \notag \\
&- \frac{2}{3} (E_1 h_1 + E_2 h_2) ( E_1 {h_1}^3 + 3 E_2 {h_1}^2 h_2 + 3 E_2 h_1 {h_2}^2 + E_2 {h_2}^3 ) \\
&= \rho \Delta T (E_1 \alpha_1 h_1 + E_2 \alpha_2 h_2)(E_1 {h_1}^2 + 2 E_2 h_1 h_2 + {E_2}^2) -
\frac{1}{2} (E_1 {h_1}^2 + 2 E_2 h_1 h_2 +E_2 {h_2}^2)^2 \\
\end{align*}
$$

$$
\begin{align*}
&\rho \Delta T E_1 E_2 (\alpha_2 {h_1}^2 h_2 - \alpha_1 {h_1}^2 h_2 + \alpha_2 h_1 {h_2}^2 - \alpha_1 h_1 {h_2}^2) \notag \\
&= \frac{1}{6} ({E_1}^2 {h_1}^4 + 4 E_1 E_2 {h_1}^3 h_2 + 6 E_1 E_2 {h_1}^2 {h_2}^2 + 4 E_1 E_2 h_1 {h_2}^3 + {E_2}^2 {h_2}^4 )
\end{align*}
$$

Finally, the bending radius $\rho$ can be expressed as follows.

$$
\begin{equation}
\frac{1}{\rho} = \frac{6(\alpha_2 - \alpha_1) \Delta T E_1 E_2 h_1 h_2 (h_1 + h_2)}{{E_1}^2 {h_2}^4 + 2 E_1 E_2 h_1 h_2 (2 {h_1}^2 + 3 h_1 h_2 + 2{h_2}^2) + {E_2}^2 {h_2}^4}
\end{equation}
$$

## References

1. 小松 敬治, "機械構造弾性力学-弾性力学の基礎とMATLABによる有限要素解析-", 森北出版, 2013, ISBN: [978-4-627-66981-9](https://www.morikita.co.jp/books/mid/066981)
2. S. Timoshenko, "Analysis of Bi-Metal Thermostats," J. Opt. Soc. Am. 11, 233-255 (1925), doi: [10.1364/JOSA.11.000233](https://doi.org/10.1364/JOSA.11.000233)
3. F. GABRON, R. W. JOHNSON, J. M. F. VICKERS, J. W. LUCAS, "Thermal scale modeling of the Mariner IV SPACECRAFT", AIAA 3rd Aerospace Sciences Meeting, 1966, doi: [10.2514/6.1966-23](https://doi.org/10.2514/6.1966-23)
4. O. W. Clausen, J, P. Kirkpatrick, "Thermal tests of an improved louver system for spacecraft thermal control", AIAA 4th Aerospace Sciences Meeting, 1969, doi: [10.2514/6.1969-627](https://doi.org/10.2514/6.1969-627)
5. Heacock RL. The Voyager Spacecraft. Proceedings of the Institution of Mechanical Engineers. 1980;194(1):211-224. doi: [10.1243/PIME_PROC_1980_194_026_02](https://doi.org/10.1243/PIME_PROC_1980_194_026_02)
6. 大西 晃, 科学衛星の熱設計の歩みと熱物性研究について, 2012, [http://www.netsubussei.jp/group/onishi.pdf](http://www.netsubussei.jp/group/onishi.pdf)
7. James C. Neuman, Joseph A. Buescher, Gregory J. Esterl, "Magellan Spacecraft Thermal Control System Design and Performance," AIAA 28th Thermophysics Conference, 1993, doi: [10.2514/6.1993-2844](https://doi.org/10.2514/6.1993-2844)
8. Choi, M., "Validation of Landsat-7 ETM+ MEM Thermal Improvement in Thermal Vacuum Tests and in Flight Due to Lower Louver Set Points," SAE Technical Paper 1999-01-2629, 1999, doi: [10.4271/1999-01-2629](https://doi.org/10.4271/1999-01-2629).
9. Härtel, K., Morgenroth, L., Reichenberger, K., Domingo, M. et al., "Thermal Design and Test of ROSETTA Platform Louvres," SAE Technical Paper 2000-01-2276, 2000, doi: [10.4271/2000-01-2276](https://doi.org/10.4271/2000-01-2276)
10. Hiroyuki MINAMINO, Michio TAKAHASHI, Satoshi TAYAMA, Yutaka TAKANO, Takeshi SASAKI, Shuichi MATSUMOTO, Shingo IKEGAMI, 月周回衛星「かぐや」衛星システムの開発, Aeronautical and Space Sciences Japan, 2008, Volume 56, Issue 656, Pages 229-235, doi: [10.14822/kjsass.56.656_229](https://doi.org/10.14822/kjsass.56.656_229)
11. Thomas, V.C., Makowski, J.M., Brown, G.M. et al. The Dawn Spacecraft. Space Sci Rev 163, 175–249 (2011). doi: [10.1007/s11214-011-9852-2](https://doi.org/10.1007/s11214-011-9852-2)
12. Carl J. Ercol, G. Allan Holtzman, "Post-Launch and Early Mission Thermal Performance of Parker Solar Probe", 49th International Conference on Environmental Systems, 2019, [https://hdl.handle.net/2346/84498](https://hdl.handle.net/2346/84498)
13. Isabel SOTO ARMAÑANZAS, Jose Javier VIÑALS ABELAN, Ben KWONG, Paul LINGGI, "Passive Thermal Control Louvers Mechanical Reliability", 50th International Conference on Environmental Systems, 2020, [https://hdl.handle.net/2346/86250](https://hdl.handle.net/2346/86250)
14. Pradeep Bhandari, A. J. Mastropietro, Razmig Kandilian, Jenny Hua, Sean Reilly, Paul Woodmansee, Tyler Schmidt, Mark Duran, "Thermal Control Technologies for Europa Clipper Mission", 49th International Conference on Environmental Systems, 2019, [https://hdl.handle.net/2346/84421](https://hdl.handle.net/2346/84421)
