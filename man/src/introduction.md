# Introduction

Degenerate is a programmable generative art engine. It is developed on
[GitHub](https://github.com/casey/degenerate/) and deployed on the web at
[degenerate.computer](https://degenerate.computer).

Degenerate operates as a chain of image filters. The output of each filter is
used as the input of the next filter. Degenerate programs change the state of
the the current image filter and call `render()` to apply it.

Image filters have a number of properties, including a transformation, which
determines where in the input image pixels will be sampled from; a mask, which
determines which of those pixels will be modified; and an operation, which
determines how those pixels will be modified.

Try copying this example program into the text input field at
[degenerate.computer](https://degenerate.computer):

```javascript
// Set the scale component of the current transformation
scale(0.75);
// Use an x-shaped mask
x();
// Use a tenth-turn about the green color axis as the operation
rotateColor('green', 0.1 * TAU);
// Apply the current image filter
render();
```

Press the `Run` button or `Shift + Enter` to run it. Try running it repeatedly
to see the effects of iterated rendering.

For more information about the JavaScript programming API, see
[Programming](programming.md). For more details about how the rendering engine
works, see [Rendering](rendering.md).

## Development

Degenerate is developed on [GitHub](https://github.com/casey/degenerate/). If
you're interested in contributing, take a look at the codebase, and pop into
[the Discord](https://discord.gg/87cjuz4FYg) for some suggestions for a good
first issue.

## Suggestions

Experiment, experiment, experiment! Clever combinations of commands craft
charming consequences. Check out and modify some examples by using the
drop-down menu in the upper right corner of the page. Try changing the filter
state and rendering in a loop, or just mash `Shift + Enter` over and over again
to see what happens.

## Saving your creations

```javascript
// Make the resolution nice and big
resolution(4096);
// Render something cool
x();
render();
// Save it
save();
```

## Learning JavaScript

Degenerate is programmed with JavaScript. An introduction to JavaScript is
outside of the scope of this document. The reader is referred the excellent
[Mozilla Developer Network](https://developer.mozilla.org/en-US/)
[introduction to JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript),
[JavaScript tutorial](https://developer.mozilla.org/en-US/docs/Learn/JavaScript).
and
[JavaScript reference](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference),
