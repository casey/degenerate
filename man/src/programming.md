# Programming

Degenerate programs are written in JavaScript and sent to a Web Worker for
executation.


The JavaScript API available to degenerate programs is reproduced
below.



Image filters have a number of properties that determine what they do. The
chapter on [rendering](rendering.md) has more .

pixels will be sampled

The primary components of a filter are a mask, which determines which pixels
the filter will operate on, a transformation, which determines which

and an "operation", which determines what will
happen to those pixels.
TODO explain:
transformations
wrapping
default color

## Writing Degenerate Programs

Degenerate Programs are written in JavaScript and executed in a WebWorker.
Programs send messages back to the main browser thread, which renders the output
in a WebGL context.

TODO link:
1. Examples
2. worker.js
3. fragment.glsl
4. app.rs
5. gpu.rs

[mdBook](mdBook.md)

Given the paucity of this manual, the current primary reference to degenerate
is the code itself.



#### Mask

#### Operation

#### Transformation

#### Alpha

#### Wrap

# JavaScript API

```javascript
{{#include ../../www/worker.js}}
```
