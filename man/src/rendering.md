# Rendering

Degenerate programs are written in JavaScript and sent to a Web Worker for
executation. The program then sends back a series of `State` objects, which are
used to configure the renderer. The renderer renders to a full-page `<canvas>`
element using WebGL.

Rendering is performed by applying a series of image filters, with the output
of each filter being fed as input to the next.

The various parts of the renderer are described below. You may want to skip
ahead to the description of the fragment shader, which neatly

## `App`

`App`, in [app.rs](https://github.com/casey/degenerate/blob/master/src/app.rs),
is the top-level struct which contains logic for configuring the application
responding to user-input.

## `Gpu`

`Gpu`, in [gpu.rs](https://github.com/casey/degenerate/blob/master/src/gpu.rs),
is responsible for setting up the WebGL context, updating the fragment shader
with values from `State` objects, and executing the rendering pipeline.

## Vertex Shader

The vertex shader, in
[vertex.glsl](https://github.com/casey/degenerate/blob/master/src/vertex.glsl),
emits a full-screen triangle.

## Fragment Shader

The fragment shader, in
[fragment.glsl](https://github.com/casey/degenerate/blob/master/src/fragment.glsl),
determines the color of each pixel of the triangle produced by the vertex shader,
thus performing the bulk of the rendering.

See the
[source file](https://github.com/casey/degenerate/blob/master/src/fragment.glsl),
which is well commented, for details.
