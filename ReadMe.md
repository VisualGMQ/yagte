Yet Another Geometric Tool

---

[David Eberly的Geomentric Tool](https://www.geometrictools.com/index.html)库融合了计算几何，计算机图形学和图形学API等众多算法和知识。本工程旨在使用纯Rust仿写Geomentric Tool，用于学习这些知识点并创造一些便于之后使用的代码。

## 目前的进度

* :no_entry_sign: : 编写完成，未测试
* :heavy_check_mark: : 编写完成，测试通过
* :o: : 无需完成

2D求最短距离：

||Point|Line|Segment|Ray|Triangle|Polyline|Polygon|
|:-|:--:|:--:|:-----:|:--:|:------:|:------:|:-----:|
|Point|:o:|:o:|:o:|:o:|:o:|:o:|
|Line||:o:||:o:|||
|Segment|||||||
|Ray|||||||
|Triangle|||||||
|Polyline|||||||
|Polygon|||||||


2D几何体之间的最近点：

||Point|Line|Segment|Ray|Triangle|Rect|Polyline|Polygon|
|:-|:--:|:--:|:-----:|:--:|:------:|:--:|:------:|:-----:|
|Point|:no_entry_sign:|:o:|:o:|:o:||:o:|
|Line|||||||
|Segment|||||||
|Ray|||||||
|Triangle|||||||
|Rect|||||||
|Polyline|||||||
|Polygon|||||||

2D几何体之间的相交判断：

||Point|Line|Segment|Ray|Triangle|Rect|Polyline|Polygon|
|:-|:--:|:--:|:-----:|:--:|:------:|:--:|:------:|:-----:|
|Point|:no_entry_sign:||||||
|Line|||||||
|Segment|||||||
|Ray|||||||
|Triangle|||||||
|Rect|||||||
|Polyline|||||||
|Polygon|||||||

点在几何体内/上的判断：

|Point|Line|Segment|Ray|Triangle|Rect|Polyline|Polygon|
|:--:|:--:|:-----:|:--:|:------:|:--:|:------:|:-----:|
|:no_entry_sign:|||||:o:|