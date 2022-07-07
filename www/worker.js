'use strict';

importScripts('randchacha_browser.min.js');

// Mask all pixels.
//
// ```
// all();
// render();
// ```
//
// `MASK_ALL` is the default mask, so the above example could have been written
// as:
//
// ```
// render();
// ```
function all() {
  state.mask = MASK_ALL;
}

// Set the alpha blending factor. `alpha` will be used to blend the output of
// the current operation with the original color. See `www/fragment.glsl` for
// the blend equation.
//
// ```
// alpha(0.5);
// render();
// ```
function alpha(alpha) {
  state.alpha = alpha;
}

// Mask pixels in a checkerboard pattern.
//
// ```
// check;
// render();
// ```
function check() {
  state.mask = MASK_CHECK;
}

// Create a new checkbox widget with the label `name`, and return true if it is
// checked. Calls with same `name` will all refer to the same checkbox, making
// it safe to call repeatedly.
//
// ```
// while(true) {
//  reboot();
//  if (checkbox('x')) {
//    x();
//  } else {
//    circle();
//  }
//  await render();
// }
// ```
function checkbox(name) {
  self.postMessage(JSON.stringify({ checkbox: name }));
  return !!widgets['widget-checkbox-' + name];
}

// Mask pixels within a circle.
//
// ```
// circle();
// render();
// ```
function circle() {
  state.mask = MASK_CIRCLE;
}

// Clear the canvas.
//
// ```
// check();
// render();
// clear();
// ```
function clear() {
  self.postMessage(JSON.stringify('clear'));
}

// Mask pixels within a cross shape.
//
// ```
// cross();
// render();
// ```
function cross() {
  state.mask = MASK_CROSS;
}

// Set the operation to the debug operation.The debug operation is permanently
// unstable, and may change at any time.
//
// ```
// debug();
// render();
// ```
function debug() {
  state.operation = OPERATION_DEBUG;
}

// Set the default color. The default color is returned whenever a pixel is sampled
// out of bounds due to a rotation, scale, or other sample coordinate transformation.
//
// ```
// defaultColor([255, 0, 255]);
// rotate(0.01 * TAU);
// render();
// ```
function defaultColor(defaultColor) {
  state.defaultColor = defaultColor;
}

// Return the number of milliseconds that have elapsed between this frame and the last.
// Returns 0 for the first frame.
//
// ```
// let rotation = 0;
//
// while(true) {
//   reboot();
//   x();
//   rotation += delta() / 30000 * TAU;
//   rotate(rotation);
//   await render();
// }
// ```
function delta() {
  return lastDelta;
}

// Return the number of milliseconds that have elapsed since the page was loaded.
//
// ```
// while(true) {
//   reboot()
//   circle();
//   scale(0.75 * elapsed() / 20000);
//   await render();
// }
// ```
function elapsed() {
  return Date.now() - start;
}

// Display an error message in the console at the bottom of the page.
//
// ```
// error('foo');
// ```
function error(error) {
  self.postMessage(JSON.stringify({error}));
}

// Returns a promise that resolves when the browser is ready to display a new
// frame. Call `await frame()` in your rendering loop to only render when
// necessary and make sure each frame is displayed after rendering.
//
// ```
// scale(0.99);
// while (true) {
//   circle();
//   render();
//   x()
//   render();
//   await frame();
// }
// ```
async function frame() {
  await new Promise((resolve, reject) => {
    frameCallbacks.push(resolve);
  });
}

// Set the operation to the identity operation. The identity operation returns the
// sampled pixel unchanged. Useful for applying transformations, such as scales or
// rotation, without changing the sampled pixels.
//
// ```
// identity();
// render();
// ```
function identity() {
  state.operation = OPERATION_IDENTITY;
}

// Set the operation to the invert operation. The invert operation inverts the sample
// pixels RGB components.
//
// ```
// x();
// invert();
// render();
// ```
//
// The invert operation is the default operation, so the above example could have been
// written as:
//
// ```
// x();
// render();
// ```
function invert() {
  state.operation = OPERATION_INVERT;
}

// Mask pixels where the pixel's index mod `divisor` is equal to `remainder`.
//
// ```
// mod(7,0);
// render();
// ```
function mod(divisor, remainder) {
  state.maskModDivisor = divisor;
  state.maskModRemainder = remainder;
  state.mask = MASK_MOD;
}

// Set the oscillator frequency to `hz` hertz. The oscillator produces a sine wave tone,
// useful for debugging audio-reactive programs.
function oscillatorFrequency(hz) {
  self.postMessage(JSON.stringify({'oscillatorFrequency': hz}));
}

// Create a new radio button widget with the label `name` and options `options`,
// and return the selected option. `options` must be a list of strings. Calls with
// same `name` will all refer to the same radio button widget, making it safe to
// call repeatedly.
//
// ```
// while(true) {
//   reboot();
//   switch (radio('shape', ['x', 'circle', 'cross'])) {
//     case 'x':
//       x();
//       break;
//     case 'circle':
//       circle();
//       break;
//     case 'cross':
//       cross();
//       break;
//   }
//   await render();
// }
// ```
function radio(name, options) {
  self.postMessage(JSON.stringify({ radio: [name, options] }))
  return widgets['widget-radio-' + name] ?? options[0];
}


// Yield values in the half-open range `[0,iterations)`.
//
// ```
// x();
// scale(0.9);
// for(_ of range(10)) {
//   render();
//   await sleep(1000);
// }
// ```
function* range(iterations) {
  for (let i = 0; i < iterations; i++) {
    yield i;
  }
}

// Reset the image filter state and clear the canvas.
// ```
// x();
// render();
// reboot();
// ```
function reboot() {
  reset();
  clear();
}

// Send the current state to the main thread to be rendered. Like `frame()`,
// returns a promise that will resolve when the browser is ready to display a
// new frame. Use `await frame();` when you want to render multiple times before
// presenting a new frame, and `await render();` when you want to render once
// per frame.
//
// ```
// scale(0.99);
// circle();
// while (true) {
//   await render();
// }
// ```
async function render() {
  self.postMessage(JSON.stringify({ render: state }));
  await frame();
}

// Reset the image filter state.
//
// ```
// x();
// render();
// reset();
// ```
function reset() {
  state = new State();
}

// Set resolution to a fixed value. Normally, the resolution increases and
// decreases automatically as the window is resized. This is usually what you
// want, but it is convenient to override it if you want to render at a fixed
// size, for example for saving high-resolution images:
//
// ```
// resolution(4096);
// x();
// render();
// save();
// ```
function resolution(resolution) {
  if (Number.isInteger(resolution)) {
    self.postMessage(JSON.stringify({ resolution }));
  }
}

// Set `rotation` to the current rotation.
//
// ```
// rotate(0.1);
// x();
// render();
// ```
function rotate(rotation) {
  state.rotation = rotation;
}

// Set the roate color operation. The rotate color operation interpets the sample pixel
// as a vector in three dimensional space and rotates it about the `axis` by `radians`.
//
// Valid values for `axis` are `red`, `green`, and `blue`. Applying `rotateColor` multiple
// times around different axes is a good way to get a variety of colors. Since `rotateColor`
// rotates the vector around the provided color axis, it will not change the amount of the
// axis's color. So if you want red, e.g., rotate about the `green` or `blue` axes.
//
// ```
// rotateColor('red', 0.5 * TAU);
// all();
// render();
// ```
function rotateColor(axis, radians) {
  state.operationRotateColorAxis = axis;
  state.operationRotateColorRadians = radians;
  state.operation = OPERATION_ROTATE_COLOR;
}

// Mask pixels where `pixel.y % (nrows + step) < nrows`. Will mask `nrows` pixels and then
// skip `step` pixels.
//
// ```
// rows(1, 9);
// render();
// ```
function rows(nrows, step) {
  state.maskRowsRows = nrows;
  state.maskRowsStep = step;
  state.mask = MASK_ROWS;
}

// Set the sample operation. The sample operation samples the currently playing
// audio's time domain data using the x coordinate of the current pixel
// position, and rotates the current pixel in HSL color space by the intensity
// of the audio at that position.
function sample() {
  state.operation = OPERATION_SAMPLE;
}

// Save the current canvas as a PNG.
//
// ```
// resolution(4096);
// x();
// render();
// save();
// ```
function save() {
  self.postMessage(JSON.stringify('save'));
}

// Set the current scale to `scale`. The scale factor is applied to sample coordinates before
// looking up the pixel under those coordinates.
//
// ```
// circle();
// render();
// scale(0.5);
// render();
// ```
function scale(scale) {
  state.scale = scale;
}

// Return a promise that resolves after `ms` milliseconds.
//
// ```
// circle();
// render();
// await sleep(1000);
// scale(0.5);
// render();
// ```
function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// Mask pixels within a square.
//
// ```
// square();
// render();
// ```
function square() {
  state.mask = MASK_SQUARE;
}

// Mask the pixels in the upper half of the canvas.
//
// ```
// top();
// render();
// ```
function top() {
  state.mask = MASK_TOP;
}

// Set wrap. When `wrap` is `true`, out of bounds samples will be wrapped back within bounds.
//
// ```
// x();
// wrap();
// scale(0.1);
// render();
// ```
function wrap(warp) {
  state.wrap = warp;
}

// Mask pixels in an X shape.
//
// ```
// x();
// render();
// ```
function x() {
  state.mask = MASK_X;
}

// The ratio of a circle's circumference to its diameter. Useful for expressing
// rotations in radians, where a full 360° turn is equal to `2 * PI`. For
// example, to rotate 1/4 turn, use `rotate(1/4 * 2 * PI)`.
const PI = Math.PI;

// The ratio of a circle's circumference to its radius. Useful for expressing
// rotations in radians, where a full 360° turn is equal to `TAU` For example,
// to rotate 1/4 turn, use `rotate(1/4 * TAU)`.
const TAU = Math.PI * 2;

// Mask constants. The mask determines which pixels the current operation will
// be applied to. These values should be kept in sync with those in
// `www/fragment.glsl`. See the corresponding functions and case statements,
// e.g., `all()` in this file and `case MASK_ALL:` in `www/fragment.glsl`, for
// more details and the mask definition, respectively.
const MASK_ALL = 0;
const MASK_CHECK = 1;
const MASK_CIRCLE = 2;
const MASK_CROSS = 3;
const MASK_MOD = 4;
const MASK_ROWS = 5;
const MASK_SQUARE = 6;
const MASK_TOP = 7;
const MASK_X = 8;

// Operation constants. The operation determines how pixels selected by the mask
// will be modified. These values should be kept in sync with those in
// `www/fragment.glsl`. See the corresponding functions and case statements,
// e.g., `invert()` in this file and `case OPERATION_INVERT:` in
// `www/fragment.glsl`, for more details and the operation definitions,
// respectively.
const OPERATION_DEBUG = 0;
const OPERATION_IDENTITY = 1;
const OPERATION_INVERT = 2;
const OPERATION_ROTATE_COLOR = 3;
const OPERATION_SAMPLE = 4;

class Rng {
  constructor(seed) {
    this.seed(0);
  }

  choose(array) {
    return array[this._rng.nextU32() % array.length];
  }

  seed(seed) {
    const _seed = new Uint8Array(32);
    _seed[0] = seed;
    this._rng = new randchacha.ChaChaRng(_seed);
  }
}

// State objects encapsulate the current image filter state.
class State {
  constructor() {
    this.alpha = 1.0;
    this.defaultColor = [0.0, 0.0, 0.0];
    this.mask = MASK_ALL;
    this.maskModDivisor = 0;
    this.maskModRemainder = 0;
    this.maskRowsRows = 0;
    this.maskRowsStep = 0;
    this.operation = OPERATION_INVERT;
    this.operationRotateColorAxis = 'red';
    this.operationRotateColorRadians = 0.0;
    this.rotation = 0.0;
    this.scale = 1.0;
    this.wrap = false;
  }
}

let frameCallbacks = [];
let lastDelta = 0;
let lastFrame = 0;
let rng = new Rng();
let start = Date.now();
let state = new State();
let widgets = {};

self.addEventListener('message', async function (event) {
  const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
  const message = JSON.parse(event.data);
  switch (message.tag) {
    case 'checkbox':
      widgets['widget-checkbox-' + message.content.name] = message.content.value;
      break;
    case 'radio':
      widgets['widget-radio-' + message.content.name] = message.content.value;
      break;
    case 'script':
      frameCallbacks = [];
      try {
        await new AsyncFunction(message.content)();
      } catch (error) {
        self.postMessage(JSON.stringify({'error': error.toString()}));
      }
      self.postMessage(JSON.stringify('done'));
      break;
    case 'frame':
      for (let callback of frameCallbacks) {
        callback();
      }
      frameCallbacks = [];
      let now = Date.now();
      if (lastFrame > 0) {
        lastDelta = now - lastFrame;
      }
      lastFrame = now;
      break;
  }
});
