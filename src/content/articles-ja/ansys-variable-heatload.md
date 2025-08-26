---
title: 'Ansys熱解析における時間および温度依存の熱入力の設定'
description: 'Ansys Workbench上で熱過渡解析をする際に、APDLスクリプトを用いて時間および温度に依存した熱入力（表面への熱流束または物体内の発熱）を設定する方法について解説します。'
pubDate: 2025-08-25
updatedDate: 2025-08-26
heroImage: ''
tags: ['thermal', 'numerical analysis', 'programming']
---

Ansys Mechanicalは様々な分野に対応した、有限要素解析機能を提供しており、熱解析の分野においても、定常解析および過渡解析の両方をサポートしている。
特にAnsys Workbench環境では、モデル構築、解析、結果の可視化までをGUIベースで操作できる便利なユーザーインターフェースを備えている。

一方で、GUIベースの操作では対応しきれない機能も存在し、そのような場合にはAPDLやPythonスクリプトを利用して操作を補完する必要がある。
今回は、そのような場合の一例として、熱過渡解析における時間および温度に依存した熱入力の設定方法について紹介する。
時間に依存した熱流束や発熱の設定はWorkbenchからの設定が可能で、さらに時間依存の対流や輻射境界条件を設定することも可能である。
しかし、より柔軟に時間および温度に依存した熱入力を設定する場合には、スクリプト操作が必要となるので、物体表面への熱流束と物体そのものの発熱の双方について、APDLスクリプトを用いた設定方法を紹介する。

## 時間および温度に依存した熱入力の設定

### 物体表面への熱流束の設定

以下のAPDLスクリプトは、時間および温度に依存した表面熱流束を物体表面に設定する方法を示している。
このスクリプトを使用する際に、いくつか注意すべき点を挙げておく。

- スクリプトを実行する前に、Workbench環境で熱流束を設定したい表面に対して、Named Selectionを"EXTERNAL"として作成しておく必要がある（プロジェクトツリーのModelを右クリック -> Insert -> Named Selection）。
- ここで、Named Selection "EXTERNAL"はWorkbench環境で定義されたオブジェクトである。APDLソルバーから"EXTERNAL"にアクセスする際には、このオブジェクトはSurfaceではなく、ノードの集合として転送される。
- 同様に、APDLソルバーからNamed Selectionのオブジェクトにアクセスする場合、VertexおよびEdgeのNamed Selectionはノードの集合として転送され、BodyのNamed Selectionは要素の集合として転送される。
- Named SelectionがAPDL側でどのようなデータになっているかを確認するには、`ds.dat`ファイルを見るとよい。このファイルは通常、解析実行後に`your_project\dp0\SYS\MECH`の下に生成される。
- このスクリプトはAnsys Workbench環境で使用することを想定しているため、`/PREP7`は含めない。`/PREP7`コマンドを含めると、GUIベースで設定と競合してエラーを発生させる可能性がある。

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

### 物体内の発熱の設定

時間および温度に依存した発熱を与えたい場合には、以下のAPDLスクリプトを用いると設定できる。
物体表面に熱流束を与える場合との違いは対象の選択部分で、今回はWorkbench上でNamed Selectionを物体の体積として指定する必要がある。
このオブジェクトをAPDL側から参照すると、今回はノードではなくElementの集合として認識されるので、このElementの集合に熱入力をを与えればよい。

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

一般に、熱過渡解析において時間依存の熱入力を使用することは問題ないが、ノード温度依存性を用いる場合には注意が必要である。
ノード温度依存性が線形である場合（e.g. 対流境界）、モデル全体も線形で、効率的に解くことができる。
しかし、ノード温度依存性が非線形である場合、計算時間が大幅に長くなったり、収束性に問題が発生することがある。
今回提示したAPDLスクリプトは、ノード温度に関して区分的に線形の熱負荷を導入しており、このような問題は、ソルバー内で非線形問題として扱われる。
そのため、解析の収束性に問題が発生する可能性があることに注意する必要がある。
