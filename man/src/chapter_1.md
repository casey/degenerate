# Chapter 1

The Degenerate Programmers Manual is bit spare at the moment. Apologies!

## Execution Model

Degenerate operates as a chain of filters, with the output of each filter
feeding into the next. Commands are issued to change the state of the current
filter, which is then applied with `computer.render()`.

The primary components of a filter are a "mask", which determines which pixels
the filter will operate on, and an "operation", which determines what will
happen to those pixels.

For example, to set the mask to an X, you can do `computer.x()`, and to set the
operation to color rotation by one half rotation about the green axis, do
`computer.rotateColor('green', 0.5)`. An finally, to see the results, do
`computer.render()`. The complete program looks like this:

```javascript
computer.x();
computer.rotateColor('green', 0.5)`;
computer.render();
```

Go to [degenerate.computer](https://degenerate.computer) and copy and paste the
program into the text area.

Nothing happened, because you have to hit `Shift + Enter` for the program to
run. Try it!

## Language

Degenerate is programmed with JavaScript. An introduction to JavaScript is
outside of the scope of this document. The reader is referred the excellent
[Mozilla Developer Network](https://developer.mozilla.org/en-US/)
[introduction to JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript),
[JavaScript tutorial](https://developer.mozilla.org/en-US/docs/Learn/JavaScript).
and
[JavaScript reference](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference),

## Reference

Given the paucity of this manual, the current primary reference to degenerate
is the code itself.

First off, check out
[worker.js](https://github.com/casey/degenerate/blob/master/www/worker.js),
which provides the environment in which degenerate programs run. The primary
interface is the `Computer` class. One is ready in the `computer` variable for
you to use.

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

1. Make the resolution nice and large with `computer.resolution(4096)`
2. Render something cool
3. Save it with `computer.save()`
