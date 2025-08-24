---
title: 'Exporting Ansys Workbench Solutions using APDL Commands'
description: 'When using Ansys Workbench, users may need to export their solutions in a specific format for post-processing or further analysis. This article provides some examples of how to generate customized output for transient thermal analysis using APDL commands.'
pubDate: 2025-08-24
updatedDate: 2025-08-24
heroImage: ''
tags: ['thermal', 'numerical analysis', 'programming']
---

Ansys Mechanical provides a powerful environment for finite element analysis. For thermal applications, both steady-state and transient thermal analyses are supported in the Workbench environment.
The Workbench environment provides a convenient user interface to set up and solve the thermal model and visualize the analysis results.
However, sometimes users need to export their analysis results to a specific format for post-processing or input into other software tools.
As the Workbench may not directly support the desired output format, users can use APDL (Ansys Parametric Design Language) commands to generate the required output.

The functionality of the presented scripts is checked on Ansys Workbench 2023 R1, Transient Thermal.
Depending on the software version and the used analysis system, the commands may need to be adjusted.

## Exporting Node Temperatures for Every Time Step

As a basic example, consider a transient thermal analysis in Ansys Workbench.
The obtained temperature and heat flux results are easily visualized in the Workbench environment.
However, if we want to export these results in a text format, the supported GUI options may be limited.
In such cases, we can use APDL commands to export the solutions in a more flexible manner.

The APDL script shown below demonstrates how to export all node temperatures for every time step into a CSV file.
The script and included comments provide a rough description of the process, and the points below are additional notes that may be helpful for understanding the script:

- The results file "file.rth" is typically under "project_directory\dp0\SYS\MECH".
- When writing NodeID, this data should be handled as Float. If you try to write it as an Integer, the binary data of the NodeID may be read as Integer without proper casting, resulting in a wrong (typically very large) value.
- In the APDL, the data writing format is specified by the Fortran based description.
  - A8: Text data for 8 characters
  - F8.2: Floating-point number with 8 characters, 2 decimal places
  - F10.0: Floating-point number with 10 characters, 0 decimal places

```apdl
! Enter Post-Processing
/POST1
! Specify result file as file.rth
FILE,'file','rth'
! Load the first result set to get the model geometry
SET,1,1

*GET,NUM_SETS,ACTIVE,0,SET,NSET

! Select and count all nodes
NSEL,S,ALL
*GET,NumSelectedNodes,NODE,0,COUNT
*GET,MinNodeID,NODE,0,NUM,MIN

! Open the export file
*CFOPEN,NodeTemperature,csv

! Each header text is 8 characters
*VWRITE,' Time[s]','  NodeID','Temp[°C]'
(A8,',',A10,',',A10)

! --- Write Data Rows ---
*DO,I,1,NUM_SETS
    SET,,,,,,,I
    *GET,TIMEVAL,ACTIVE,0,SET,TIME

    CurrentNodeID = MinNodeID
    *DO,N,1,NumSelectedNodes
        *GET,TEMPVAL,NODE,CurrentNodeID,TEMP

        ! Write Time, NodeID, and Temperature
        *VWRITE,TIMEVAL,CurrentNodeID,TEMPVAL
        (F8.2,',',F10.0,',',F10.2)

        CurrentNodeID = NDNEXT(CurrentNodeID)
    *ENDDO
*ENDDO

*CFCLOSE
```

## Exporting Node Coordinates

To use the thermal analysis results as input for other simulations or post-processing tools, you may also need the node coordinates.
Similar to exporting node temperatures, you can export the node coordinates using an APDL script.

```apdl
! Enter Post-Processing
/POST1
! Specify result file as file.rth
FILE,'file','rth'

! Load the first result set to get the model geometry
SET,1,1

! Open the export file
*CFOPEN, NodeCoordinates, csv

! Each header text is 8 characters
*VWRITE, '  NodeID', '    X[m]', '    Y[m]', '    Z[m]'
(A8, ',', A16, ',', A16, ',', A16)


! Select all nodes in the model.
NSEL,S,ALL

! Get the total count of currently selected nodes.
*GET, NumSelectedNodes, NODE, 0, COUNT

! Get the lowest node number among the selected set to start the loop.
*GET, CurrentNodeID, NODE, 0, NUM, MIN

! Loop through all selected nodes.
*DO, i, 1, NumSelectedNodes

  ! Check if the CurrentNodeID is valid (not zero).
  *IF, CurrentNodeID, NE, 0, THEN

    ! Get the X, Y, and Z coordinates for the current node ID.
    nx = NX(CurrentNodeID)
    ny = NY(CurrentNodeID)
    nz = NZ(CurrentNodeID)

    ! Write the node ID and its coordinates to the file.
    ! The F8.0 format is used for the ID to handle it as a floating-point number
    ! without decimals, preventing data type mismatch errors.
    *VWRITE, CurrentNodeID, nx, ny, nz
    (F8.0, ',', E16.8, ',', E16.8, ',', E16.8)

    ! Get the next node ID from the list of selected nodes.
    CurrentNodeID = NDNEXT(CurrentNodeID)

  *ENDIF
*ENDDO

*CFCLOSE
```
