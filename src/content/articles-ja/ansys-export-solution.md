---
title: 'APDLコマンドを用いたAnsys Workbench解析結果のエクスポート'
description: 'When using Ansys Workbench, users may need to export their solutions in a specific format for post-processing or further analysis. This article provides some examples of how to generate customized output for transient thermal analysis using APDL commands.'
pubDate: 2025-08-24
updatedDate: 2025-08-24
heroImage: ''
tags: ['thermal', 'numerical analysis', 'programming']
---

Ansys Mechanicalは様々な分野に対応した、有限要素解析機能を提供しており、熱解析の分野においても、定常解析および過渡解析の両方をサポートしている。
特にAnsys Workbench環境では、モデル構築、解析、結果の可視化までをGUIベースで操作できる便利なユーザーインターフェースを備えている。

一方で、GUIベースの操作では対応しきれない機能も存在し、解析結果の操作においても、
特定のフォーマットで解析結果やモデルデータをエクスポートすることには対応していない場合がある。
このような場合、APDL（ANSYS Parametric Design Language）コマンドを使用することで、必要な出力を生成することができる。

今回紹介するスクリプトは、Ansys Workbench 2023 R1のTransient Thermalで動作することは確認しているが、他のバージョンや解析用途では動作しない可能性もあるため、注意してもらいたい。

## 全ノード温度の全時刻ステップでのエクスポート

基本的な例として、Ansys Workbenchにおける過渡熱解析を考えてみよう。
得られた温度および熱流束の結果は、Workbench環境内で簡単に可視化できる。
しかし、これらの結果をテキスト形式でエクスポートしたい場合、サポートされているGUIオプションはそれほど多くなく、期待した形式での出力が得られないことがある。
そのような場合、APDLコマンドを使用して、より柔軟に解をエクスポートすることができる。

以下に示すAPDLスクリプトは、全ノードの温度を全時刻ステップでCSVファイルにエクスポートする方法を示している。
スクリプトの内容はそれほど複雑ではないが、以下に注意すべきポイントをいくつか挙げておく。

- 解析結果のデータファイル"file.rth"はたいてい次のようなディレクトリに置かれている："project_directory\dp0\SYS\MECH".
- NodeIDを書き出す際にはFloatとして扱う必要がある。Integerとして書き出そうとすると、NodeIDのバイナリデータが適切にキャストされず、そのままIntegerとして扱われて、本来のNodeIDではない巨大な値が出力されることがある。
- NodeIDが不連続である可能性も考慮して、ノードについてイテレートする際にはNDNEXTを使用して次のNodeIDを取得する。
- APDLでは、データの書き出し形式はFortranベースの記述によって指定される。
  - A8: 8文字のテキストデータ
  - F8.2: 8文字、小数点以下2桁の浮動小数点数
  - F10.0: 10文字、小数点以下0桁の浮動小数点数

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

## ノード座標のエクスポート

熱解析の結果を、後処理や他のシミュレーションへの入力として利用する場合、ノードの座標情報も必要になることがある。
ノード温度のエクスポートと同様に、APDLスクリプトを使用してノード座標をエクスポートすることができる。

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
