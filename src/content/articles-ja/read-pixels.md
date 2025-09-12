---
title: 'フラグメントシェーダからCPUへのデータ受け渡し'
description: 'ウェブブラウザ上でGPGPUを行うために昔使われていた、WebGLの小技を紹介します。フラグメントシェーダで計算した結果を色として表示し、その色をreadPixelsという関数で読みだすことで、GPUからCPUへデータを受け渡すことが出来ます。'
pubDate: 2025-09-12
updatedDate: 2025-09-12
heroImage: ''
tags: ['programming']
---

今回は、私が2010年代後半に使っていたWebGLに関する小技を紹介してみたい。

WebGLとOpenGL ES Shading Language(GLSL ES)を用いると、ウェブブラウザ上で比較的簡単にグラフィック処理を行うことが出来る。
せっかくウェブブラウザ上でGPUリソースを利用するインターフェースがあるので、これを用いて数値計算をやってみたくなるのだが、
WebGLはGPGPU(General-purpose computing on graphics processing units)を積極的にサポートしているわけではないので、いくつか工夫が必要になる。
今となっては、より便利なインターフェースが存在するので、実用的な技術ではないが、データ処理の手法として面白いので紹介してみたい。

## フラグメントシェーダのデータを色に変換する

もともとフラグメントシェーダの役割は、各ピクセルで表示するべき色を決めることである。
この処理をプログラミングしてやることで、表示されるポリゴンに好きな色を指定したり、グラデーションをつけたりすることもできる。
そこで色を指定するための計算を、何かしらの数値計算アルゴリズムに置き換えてやれば、ピクセルの数だけ並列に数値計算を行うことが出来る。

フラグメントシェーダでの計算結果は、基本的には画面上に色として表示される。
一方、表示された色はWebGLのreadPixelsという関数を使うとJavaScript側で読みだすことも出来る。
なので、渡したいデータを色として表示して、それを読みだしてやれば、フラグメントシェーダからCPUへデータを受け渡すことが出来る。

色はRGBAの値で指定されるが、フラグメントシェーダではRGBAの値を0&ndash;1のfloatで指定するのに対して、JavaScript側では、RGBAの値を8ビットのunsigned intとして受け取るので注意しよう。

32ビットのintを色に変換するには、次のように8ビットごとにバイナリを取り出して、0&ndash;1のfloatに変換してやればよい。

```glsl
vec4 intToVec4(int num) {
    int rIntValue = num & 0x000000FF;
    int gIntValue = (num & 0x0000FF00) >> 8;
    int bIntValue = (num & 0x00FF0000) >> 16;
    int aIntValue = (num & 0xFF000000) >> 24;
    vec4 numColor = vec4(float(rIntValue)/255.0, float(gIntValue)/255.0, float(bIntValue)/255.0, float(aIntValue)/255.0);
    return numColor;
}
```

同様に32ビットのuintも次のように色に変換できる。

```glsl
vec4 uintToVec4(uint num) {
    uint rIntValue = num & 0x000000FFu;
    uint gIntValue = (num & 0x0000FF00u) >> 8;
    uint bIntValue = (num & 0x00FF0000u) >> 16;
    uint aIntValue = (num & 0xFF000000u) >> 24;
    vec4 numColor = vec4(float(rIntValue)/255.0, float(gIntValue)/255.0, float(bIntValue)/255.0, float(aIntValue)/255.0);
    return numColor;
}
```

32ビットfloatの場合は、ビット列を変えずにfloatをuintに変換した後、uintを色に変換してやればよい。
ただし、ここで使うfloatBitsToUintはWebGL2 (GLSL ES 3.00)以降でしか使えないので、WebGL1 (GLSL ES 1.00)の場合はこれをFloatの計算として実装する必要があり非常に難しい。

```glsl
vec4 floatToVec4(float val) {
    uint conv = floatBitsToUint(val);
    return uintToVec4(conv);
}
```

## 色をデータとして読みだす

画面全体の色を、int、unsigned int、floatの配列としてJavaScript側で読み出す関数は次のような感じになる。

まず、読み出しに必要な分だけバイナリ配列の領域を用意して、そこに色データを読み出す。
必要なサイズは、ピクセル数×8ビット×RGBAである。
これを32ビットごとのまとまりとして認識し直せば、もともとのデータが得られる。

```javascript
function readInt32Array() {
  let pixels = new Uint8Array(gl.drawingBufferWidth * gl.drawingBufferHeight * 4);
  gl.readPixels(
    0,
    0,
    gl.drawingBufferWidth,
    gl.drawingBufferHeight,
    gl.RGBA,
    gl.UNSIGNED_BYTE,
    pixels
  );
  return new Int32Array(pixels.buffer);
}

function readUint32Array() {
  let pixels = new Uint8Array(gl.drawingBufferWidth * gl.drawingBufferHeight * 4);
  gl.readPixels(
    0,
    0,
    gl.drawingBufferWidth,
    gl.drawingBufferHeight,
    gl.RGBA,
    gl.UNSIGNED_BYTE,
    pixels
  );
  return new Uint32Array(pixels.buffer);
}

function readFloat32Array() {
  let pixels = new Uint8Array(gl.drawingBufferWidth * gl.drawingBufferHeight * 4);
  gl.readPixels(
    0,
    0,
    gl.drawingBufferWidth,
    gl.drawingBufferHeight,
    gl.RGBA,
    gl.UNSIGNED_BYTE,
    pixels
  );
  return new Float32Array(pixels.buffer);
}
```

## あとがき

この方法は、とくにWebGL2が登場する前のWebGL1の時代に、GPGPUをやるためのトリックとしてよく知られていた手法だ。
WebGL2が登場してからは、Transform Feedbackがサポートされるようになり、色読み出しを行わなくてもGPU-CPU間のデータの受け渡しができるようになっていたのだが、
Transform FeedbackはVertexシェーダからのデータの受け渡しにしか対応しておらず、私自身はあまり使いこなすことができなかった。
GPUはそもそもピクセル単位での計算を得意としていて、そこで並列計算した結果をガサッと読みだすことが出来たら便利、という感覚があり、大量に頂点を生成してVertexシェーダで計算する、というのがどうもしっくり来なかったのである。
今となっては、WebGPUのcompute shaderが使えるようになり、より抽象化されたインターフェースでウェブブラウザ上でもGPGPUができるようになって来ている。
そのため、実用上はほぼ将来性のない手法なのだが、GPUの特性とグラフィックスAPIの制約を考慮した、面白いハックとして今でも参考になると思う。
