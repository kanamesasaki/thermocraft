---
title: 'Time- and Temperature-Dependent Heat Load for Ansys Transient Thermal'
description: 'In this article, we will explore the implementation of time- and temperature-dependent heat loads (surface heat flux and volumetric heat generation) in Ansys Transient Thermal simulations using APDL scripts.'
pubDate: 2025-08-25
updatedDate: 2025-08-26
heroImage: ''
tags: ['thermal', 'numerical analysis', 'programming']
---

Ansys Mechanical provides a powerful environment of finite element analysis for various applications, including thermal analysis.
The Workbench environment provides a convenient user interface to set up, solve the thermal model and visualize the analysis results.
However, some workflows are not directly supported by the GUI, and users may need to rely on scripting to achieve their goals.

One example is the application of time- and temperature-dependent heat loads in a transient thermal analysis.
The time-dependent heat flux (on surfaces) and heat generation (volumetric) are supported by GUI options in the Workbench environment.
Also, the time-varying convection and radiation boundaries can be defined using the Workbench GUI.
However, more general time- and temperature-dependent heat loads are not supported, and users can utilize APDL scripts to define such loads.

## Time- and Temperature-Dependent Heat Load

### Surface Heat Flux

The script shown below demonstrates how to apply a time- and temperature-dependent surface heat flux using APDL.
To use this script, the following points should be noted:

- Users should define a Named Selection "EXTERNAL" for the target surface (Right-click at Model -> Insert -> Named Selection) in the Workbench environment.
- Named Selection "EXTERNAL" is defined in the Workbench environment. To access "EXTERNAL" from the APDL solver environment, this data is transferred as a set of nodes, instead of surface/area objects.
- Similarly, when accessing Named Selection from APDL solver environment, Named Selection for Vertex and Edge are transferred as a set of nodes. Named selection for Body is transferred as a set of elements.
- To check the Named Selection data on the APDL side, users can check `ds.dat` file typically under `your_project\dp0\SYS\MECH` after running the simulation.
- Since this script is intended for use in Ansys Workbench, `/PREP7` command should not be included, which may cause conflicts with the GUI-based model settings.

```apdl
! ---
! Apply a Time- and Temperature-Dependent Surface Heat Flux
! ---
!
! PREREQUISITE:
! This script is designed to be used in Ansys Workbench as "Commands (APDL)".
! You must have a Named Selection of NODES "EXTERNAL" created in Workbench.
! ---


! Selects the component named "EXTERNAL"
! 'cm' = component, 'sel' = select, 's' = new selection
cmsel,s,EXTERNAL

! Selects Elements from the previously selected Nodes
! 'e'=elements, 'sl'=select, 'n'=from nodes
esln,s,0

! Define HFLUX_TABLE as a TABLE with 4 x 3 dimensions
! TEMP 4 rows, TIME 3 columns
*DIM,HFLUX_TABLE,TABLE,4,3,1,TEMP,TIME

! Define the table row index, TEMP [C]
HFLUX_TABLE(1,0,1) = 0
HFLUX_TABLE(2,0,1) = 22
HFLUX_TABLE(3,0,1) = 24
HFLUX_TABLE(4,0,1) = 99

! Define the table column index, TIME [s]
HFLUX_TABLE(0,1,1) = 0
HFLUX_TABLE(0,2,1) = 10
HFLUX_TABLE(0,3,1) = 50

! Fill in the Heat Flux values [W/m2]
! Column 1: Time = 0
HFLUX_TABLE(1,1,1) = 0
HFLUX_TABLE(2,1,1) = 0
HFLUX_TABLE(3,1,1) = 0
HFLUX_TABLE(4,1,1) = 0

! Column 2: Time = 10
HFLUX_TABLE(1,2,1) = 1000
HFLUX_TABLE(2,2,1) = 1000
HFLUX_TABLE(3,2,1) = 5000
HFLUX_TABLE(4,2,1) = 5000

! Column 3: Time = 50
HFLUX_TABLE(1,3,1) = 4000
HFLUX_TABLE(2,3,1) = 4000
HFLUX_TABLE(3,3,1) = 9000
HFLUX_TABLE(4,3,1) = 9000


! SFE = Apply a Surface load on a Face of an Element
sfe,all,,HFLUX,,%HFLUX_TABLE%


! Reselects all entities in the model
allsel,all
```

### Volumetric Heat Generation

When you want to apply a time- and temperature-dependent volumetric heat generation, you can use the following APDL script.
The key changes from the surface heat flux case is the selection of volume.
In this case, the Named Selection should specify a body in the Workbench environment, and this data is transferred as a set of elements in the APDL solver environment.
Finally, the `bfe` command with the `HGEN` option can generate a body force (heat) on the selected elements.

```apdl
! ---
! Apply a Time- and Temperature-Dependent Volumetric Heat Generation
! ---
!
! PREREQUISITE:
! This script is designed to be used in Ansys Workbench as "Commands (APDL)".
! You must have a Named Selection of NODES "BODY" created in Workbench.
! ---


! Selects the component named "BODY"
! 'cm' = component, 'sel' = select, 's' = new selection
cmsel,s,BODY

! Define HEAT_GEN_TABLE as a TABLE with 4 x 3 dimensions
! TEMP 4 rows, TIME 3 columns
*DIM,HEAT_GEN_TABLE,TABLE,4,3,1,TEMP,TIME

! Define the table row index, TEMP [C]
HEAT_GEN_TABLE(1,0,1) = 0
HEAT_GEN_TABLE(2,0,1) = 22
HEAT_GEN_TABLE(3,0,1) = 24
HEAT_GEN_TABLE(4,0,1) = 99

! Define the table column index, TIME [s]
HEAT_GEN_TABLE(0,1,1) = 0
HEAT_GEN_TABLE(0,2,1) = 10
HEAT_GEN_TABLE(0,3,1) = 50

! Fill in the Heat Generation values [W/m3]
! Column 1: Time = 0
HEAT_GEN_TABLE(1,1,1) = 0
HEAT_GEN_TABLE(2,1,1) = 0
HEAT_GEN_TABLE(3,1,1) = 0
HEAT_GEN_TABLE(4,1,1) = 0

! Column 2: Time = 10
HEAT_GEN_TABLE(1,2,1) = 1000
HEAT_GEN_TABLE(2,2,1) = 1000
HEAT_GEN_TABLE(3,2,1) = 5000
HEAT_GEN_TABLE(4,2,1) = 5000

! Column 3: Time = 50
HEAT_GEN_TABLE(1,3,1) = 4000
HEAT_GEN_TABLE(2,3,1) = 4000
HEAT_GEN_TABLE(3,3,1) = 9000
HEAT_GEN_TABLE(4,3,1) = 9000


! BFE = Apply a Body Force load to an Element
bfe,all,hgen,,%HEAT_GEN_TABLE%

! Reselects all entities in the model
allsel,all
```

## Remarks

In general, it is not problematic to use time-dependent heat loads in transient thermal analysis, but care must be taken to use node-temperature dependency.
If the node-temperature dependency is linear (e.g. convection boundary), the entire model is also linear which can be solved efficiently.
However, if the node-temperature dependency is nonlinear, it may lead to significantly longer computation time and convergence issues.
The presented APDL scripts introduce piecewise linear heat loads, with respect to the node temperature.
Such problems are handled as nonlinear problems in the solver. Thus, users should observe the solution behavior closely and be prepared to adjust their approach as needed.
