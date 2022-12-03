'use strict';

importScripts('gl-matrix-min.js', 'randchacha_browser.min.js');

glMatrix.glMatrix.setMatrixArrayType(Array);

let mat2 = glMatrix.mat2;
let mat2d = glMatrix.mat2d;
let mat3 = glMatrix.mat3;
let mat4 = glMatrix.mat4;
let quat = glMatrix.quat;
let quat2 = glMatrix.quat2;
let vec2 = glMatrix.vec2;
let vec3 = glMatrix.vec3;
let vec4 = glMatrix.vec4;

// Field that covers all pixels.
//
// ```
// all();
// render();
// ```
//
// All is the default field, so the above example could have been written
// as:
//
// ```
// render();
// ```
function all() {
  filter.field = 'All';
}

// Set the alpha blending factor. `alpha` will be used to blend the
// transformed color with the original color. See `www/fragment.glsl` for
// the blend equation.
//
// ```
// alpha(0.5);
// render();
// ```
function alpha(alpha) {
  filter.alpha = alpha;
}

// Assert that `condition` is true, otherwise throw `message`.
function assert(condition, message) {
  if (!condition) {
    throw message ?? 'assertion failed';
  }
}

// A checkerboard pattern.
//
// ```
// check;
// render();
// ```
function check() {
  filter.field = 'Check';
}

// Choose a random element from `array`.
//
// ```
// scale(choose([1,2,3]));
// ```
function choose(array) {
  return state.rng.choose(array);
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
  self.postMessage(
    JSON.stringify({
      widget: {
        name,
        widget: 'checkbox',
      },
    })
  );
  return !!widgets['checkbox-' + name];
}

// A circle.
//
// ```
// circle();
// render();
// ```
function circle() {
  filter.field = 'Circle';
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

// A cross field.
//
// ```
// cross();
// render();
// ```
function cross() {
  filter.field = 'Cross';
}

// Set the decibel range for normalization of raw frequency data into values
// usable in the fragment shader. Frequency data, by default, is expressed in
// decibels. Decibels are logarithmic, with 0 representing the loudest possible
// sound, and -∞ representing the quietest possible sound. This is inconvenient
// and unintuitive to work with, so frequency data decibel values are
// normalized to values between 0 and 1, where 0 is the silence and 1 is loud.
// This normalization requires selecting cut-off min and max decibel values.
// Values below `min` are clamped to 0, and values above `max` are clamped to
// 1. Setting the min value too low will cause noise to appear in the
// normalized frequency data. Setting the min value too high will remove quiet
// sounds from the frequency data. Setting the max value too high will reduce
// the dynamic range of the normalized values, and setting the max value too
// low will clip loud sounds, causing them to all map to 1. The default range
// is [-100, -30], which is reasonable for most applications.
//
// ```
// equalizer()
// record();
// wrap(true);
// while(true) {
//   let min = slider('min', -300, 0, 1, -100);
//   let max = slider('max', -300, 0, 1, -30);
//   decibelRange(min, max);
//   clear();
//   await render();
// }
// ```
function decibelRange(min, max) {
  self.postMessage(JSON.stringify({ decibelRange: { min, max } }));
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
  filter.defaultColor = defaultColor;
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
  return state.delta;
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
  return Date.now() - state.start;
}

// An equalizer pattern.
//
// ```
// record();
// equalizer();
// while(true) {
//   clear();
//   await render();
// }
// ```
function equalizer() {
  filter.field = 'Equalizer';
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
    state.frameCallbacks.push(resolve);
  });
}

// A frequency field.
function frequency() {
  filter.field = 'Frequency';
}

// Set the color transformation to the identity transformation. The identity
// transformation returns the sampled pixel unchanged. Useful for applying
// transformations, such as scales or rotation, without changing the sampled
// pixels.
//
// ```
// identity();
// render();
// ```
function identity() {
  mat4.identity(filter.colorTransform);
}

// If `coordinates` is true, use the coordinate of the sample as the input color,
// instead of the color of the pixel in the source image. Defaults to false.
// Useful for creating gradients or debugging coordinate transforms.
//
// When true, RGB will be set to (x, y, 0)
//
// ```
// coordinates(true);
// render();
// ```
function coordinates(coordinates) {
  filter.coordinates = coordinates;
}

// Set the color transformation to inversion.
//
// ```
// x();
// invert();
// render();
// ```
//
// Inversion is the default color transformation, so the above example
// could have been written as:
//
// ```
// x();
// render();
// ```
function invert() {
  mat4.fromScaling(filter.colorTransform, vec3.fromValues(-1, -1, -1));
}

// Field that covers pixels where the pixel's index mod `divisor` is equal to `remainder`.
//
// ```
// mod(7,0);
// render();
// ```
function mod(divisor, remainder) {
  filter.field = { Mod: { divisor, remainder } };
}

// Set the oscillator gain. The oscillator produces a sine wave tone, useful
// for debugging audio-reactive scripts.
//
// ```
// equalizer()
// while(true) {
//   oscillatorFrequency(slider('frequency', 0, 20000, 1, 0));
//   oscillatorGain(slider('gain', 0, 1, 0.01, 0.25));
//   clear();
//   await render();
// }
// ```
function oscillatorGain(oscillatorGain) {
  self.postMessage(JSON.stringify({ oscillatorGain }));
}

// Set the oscillator frequency to `hz` hertz. The oscillator produces a sine wave tone,
// useful for debugging audio-reactive scripts.
//
// ```
// equalizer()
// while(true) {
//   oscillatorFrequency(slider('frequency', 0, 20000, 1, 0));
//   oscillatorGain(slider('gain', 0, 1, 0.01, 0.25));
//   clear();
//   await render();
// }
// ```
function oscillatorFrequency(oscillatorFrequency) {
  self.postMessage(JSON.stringify({ oscillatorFrequency }));
}

// Create a new radio button widget with the label `name` and options `options`,
// and return the selected option. `options` must be a list of strings. Calls with
// same `name` will all refer to the same radio button widget, making it safe to
// call repeatedly.
//
// ```
// while(true) {
//   reboot();
//   switch (radio('field', ['x', 'circle', 'cross'])) {
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
  self.postMessage(
    JSON.stringify({
      widget: {
        name,
        widget: {
          radio: { options },
        },
      },
    })
  );
  return widgets['radio-' + name] ?? options[0];
}

// Reset the image filter and clear the canvas.
// ```
// x();
// render();
// reboot();
// ```
function reboot() {
  reset();
  clear();
}

// Enable audio recording.
function record() {
  self.postMessage(JSON.stringify('record'));
}

// Send the current filter to the main thread to be rendered. Like `frame()`,
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
  self.postMessage(JSON.stringify({ render: filter }));
  await frame();
}

// Reset the image filter.
//
// ```
// x();
// render();
// reset();
// ```
function reset() {
  filter = new Filter();
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

// Use rotation as the color transformation.
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
  switch (axis) {
    case 'red':
      mat4.fromXRotation(filter.colorTransform, radians);
      break;
    case 'green':
      mat4.fromYRotation(filter.colorTransform, radians);
      break;
    case 'blue':
      mat4.fromZRotation(filter.colorTransform, radians);
      break;
  }
}

// Set coordinate transform to a rotation.
//
// ```
// rotate(0.1);
// x();
// render();
// ```
function rotate(rotation) {
  transform(rotation, [1.0, 1.0], [0.0, 0.0]);
}

// Field that covers pixels where `pixel.y % (nrows + step) < nrows`. Will cover `nrows` pixels and then
// skip `step` pixels.
//
// ```
// rows(1, 9);
// render();
// ```
function rows(on, off) {
  filter.field = { Rows: { on, off } };
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

// Seed RNG with `n`.
//
// ```
// seed(1);
// let a = choose([1,2,3]);
// seed(1);
// let b = choose([1,2,3]);
// console.assert(a == b);
// ```
function seed(n) {
  state.rng.seed(n);
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
  transform(0, [scale, scale], [0.0, 0.0]);
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
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Create a new slider widget with the given `name`, `min`, `max`, `step`, and
// `initial` values. Calls with same `name` will all refer to the same slider,
// making it safe to call repeatedly.
//
// ```
// x()
// while(true) {
//   rotateColor('green', slider('color rotation', 0, TAU, 0.001, 0));
//   clear();
//   await render();
// }
// ```
function slider(name, min, max, step, initial) {
  self.postMessage(
    JSON.stringify({
      widget: {
        name,
        widget: {
          slider: {
            min: min ?? 0,
            max: max ?? 1,
            step: step ?? 0.001,
            initial: initial ?? min ?? 0,
          },
        },
      },
    })
  );
  return widgets['slider-' + name] ?? initial;
}

// A square field.
//
// ```
// square();
// render();
// ```
function square() {
  filter.field = 'Square';
}

// A field that covers pixels where the audio time domain data is large.
function timeDomain() {
  filter.field = 'TimeDomain';
}

// Execute the filter `times` times.
function times(times) {
  filter.times = times;
}

// A field covering the top half of the canvas.
//
// ```
// top();
// render();
// ```
function top() {
  filter.field = 'Top';
}

// Set the coordinate transform using `rotation`, `scale`, and `translation`.
// Arguments that are omitted or undefined are skipped.
//
// ```
// circle();
// render();
// transform(TAU / 3, [2.0, 0.5], [0.1, 0.5]);
// render();
// ```
function transform(rotation, scale, translation) {
  mat3.identity(filter.positionTransform);
  mat3.rotate(filter.positionTransform, filter.positionTransform, rotation);
  mat3.scale(filter.positionTransform, filter.positionTransform, scale);
  mat3.translate(
    filter.positionTransform,
    filter.positionTransform,
    translation
  );
}

// A Waveform field.
//
// ```
// record();
// wave();
// while(true) {
//   clear();
//   await render();
// }
// ```
function wave() {
  filter.field = 'Wave';
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
  filter.wrap = warp;
}

// An X field.
//
// ```
// x();
// render();
// ```
function x() {
  filter.field = 'X';
}

// The ratio of a circle's circumference to its diameter. Useful for expressing
// rotations in radians, where a full 360° turn is equal to `2 * PI`. For
// example, to rotate 1/4 turn, use `rotate(1/4 * 2 * PI)`.
const PI = Math.PI;

// The ratio of a circle's circumference to its radius. Useful for expressing
// rotations in radians, where a full 360° turn is equal to `TAU` For example,
// to rotate 1/4 turn, use `rotate(1/4 * TAU)`.
const TAU = Math.PI * 2;

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

class Filter {
  constructor() {
    this.alpha = 1.0;
    this.colorTransform = mat4.fromScaling(
      mat4.create(),
      vec3.fromValues(-1, -1, -1)
    );
    this.positionTransform = mat3.create();
    this.coordinates = false;
    this.defaultColor = [0.0, 0.0, 0.0];
    this.field = 'All';
    this.times = 1;
    this.wrap = false;
  }
}

class State {
  constructor() {
    this.delta = 0;
    this.frameCallbacks = [];
    this.rng = new Rng();
    this.start = Date.now();
  }
}

let filter = new Filter();
let lastFrame = 0;
let state = null;
let widgets = {};

self.addEventListener('message', async function (event) {
  const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
  const message = JSON.parse(event.data);
  switch (message.tag) {
    case 'frame':
      if (state) {
        for (let callback of state.frameCallbacks) {
          callback();
        }
        state.frameCallbacks.length = 0;
      }
      let now = Date.now();
      if (state && lastFrame > 0) {
        state.delta = now - lastFrame;
      }
      lastFrame = now;
      break;
    case 'script':
      state = new State();
      try {
        await new AsyncFunction(message.content)();
      } catch (error) {
        self.postMessage(JSON.stringify({ error: error.toString() }));
      }
      self.postMessage(JSON.stringify('done'));
      break;
    case 'widget':
      widgets[message.content.key] = message.content.value;
      break;
  }
});
