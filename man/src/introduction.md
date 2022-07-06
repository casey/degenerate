# Introduction

Degenerate is a programmable generative art engine. It is developed on
[GitHub](https://github.com/casey/degenerate/) and deployed at
[degenerate.computer](https://degenerate.computer).

## Image Filters

Degenerate operates as a chain of image filters. The output of each filter is
used as the input of the next filter. Degenerate programs change the state of
the the current image filter, and call `render()` to apply it.

Image filters have a number of properties that determine what they do. The
chapter on [rendering](rendering.md) has more .

#### Mask

#### Operation

#### Transformation

#### Alpha

#### Wrap

The primary components of a filter are a mask, which determines which pixels
the filter will operate on, a transformation, which determines which

and an "operation", which determines what will
happen to those pixels.

For example, to set the mask to an X, you can do `x()`, and to set the
operation to color rotation by one half rotation about the green axis, do
`rotateColor('green', 0.5 * TAU)`. An finally, to see the results, do
`render()`. The complete program looks like this:

```javascript
x();
rotateColor('green', 0.5 * TAU);
render();
```

TODO explain:
transformations
wrapping
default color

Go to [degenerate.computer](https://degenerate.computer) and copy and paste the
program into the text area.

Nothing happened, because you have to hit `Shift + Enter` for the program to
run. Try it!

## Learning JavaScript

Degenerate is programmed with JavaScript. An introduction to JavaScript is
outside of the scope of this document. The reader is referred the excellent
[Mozilla Developer Network](https://developer.mozilla.org/en-US/)
[introduction to JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript),
[JavaScript tutorial](https://developer.mozilla.org/en-US/docs/Learn/JavaScript).
and
[JavaScript reference](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference),

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

First off, check out
[worker.js](https://github.com/casey/degenerate/blob/master/www/worker.js),
which provides the environment in which degenerate programs run. A variety of
functions are available for manipulating the current state.

Secondly, check out
[fragment.glsl](https://github.com/casey/degenerate/blob/master/src/fragment.glsl),
the fragment shader that runs on the GPU and does the hard work of rendering
filters.

And finally, check out
[image.spec.ts](https://github.com/casey/degenerate/blob/master/tests/images.spec.ts),
which contains test programs, and
[the images directory](https://github.com/casey/degenerate/tree/master/images),
which contains the corresponding images. Most test programs are simple, and
exercise individual features, but some are more complex, like `smear`,
`kaleidoscope`, `grain`, and `pattern`.

## Suggestions

Experiment, experiment, experiment! Clever combinations of commands give
surprising results. Try changing the filter state and rendering in a loop, or
just mash `Shift + Enter` over and over again to see what happens.

## Saving your creations

1. Make the resolution nice and large with `resolution(4096)`
2. Render something cool
3. Save it with `save()`
