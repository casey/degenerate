'use strict';

importScripts('randchacha_browser.min.js');

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

const PI = Math.PI;
const TAU = Math.PI * 2;

const MASK_ALL = 0;
const MASK_CHECK = 1;
const MASK_CIRCLE = 2;
const MASK_CROSS = 3;
const MASK_MOD = 4;
const MASK_ROWS = 5;
const MASK_SQUARE = 6;
const MASK_TOP = 7;
const MASK_X = 8;

const OPERATION_DEBUG = 0;
const OPERATION_IDENTITY = 1;
const OPERATION_INVERT = 2;
const OPERATION_ROTATE_COLOR = 3;
const OPERATION_SAMPLE = 4;

let state;
reset();

function all() {
  state.mask = MASK_ALL;
}

function alpha(alpha) {
  state.alpha = alpha;
}

function elapsed() {
  return (Date.now() - start) / 1000;
}

async function frame() {
  await new Promise((resolve, reject) => {
    frameCallbacks.push({resolve, reject});
  });
}

async function render() {
  self.postMessage(JSON.stringify({ render: state }));
  await frame();
}

function resolution(resolution) {
  if (Number.isInteger(resolution)) {
    self.postMessage(JSON.stringify({ resolution }));
  }
}

function check() {
  state.mask = MASK_CHECK;
}

function checkbox(name) {
  self.postMessage(JSON.stringify({ checkbox: name }));
  return !!widgets[name];
}

function circle() {
  state.mask = MASK_CIRCLE;
}

function clear() {
  self.postMessage(JSON.stringify('clear'));
}

function cross() {
  state.mask = MASK_CROSS;
}

function debug() {
  state.operation = OPERATION_DEBUG;
}

function defaultColor(defaultColor) {
  state.defaultColor = defaultColor;
}

function delta() {
  return lastDelta;
}

function elapsed() {
  return Date.now() - start;
}

function identity() {
  state.operation = OPERATION_IDENTITY;
}

function invert() {
  state.operation = OPERATION_INVERT;
}

function mod(divisor, remainder) {
  state.maskModDivisor = divisor;
  state.maskModRemainder = remainder;
  state.mask = MASK_MOD;
}

function record(record) {
  self.postMessage(JSON.stringify({record}));
}

function reset() {
  state = {
    alpha: 1.0,
    defaultColor: [0.0, 0.0, 0.0],
    mask: MASK_ALL,
    maskModDivisor: 0,
    maskModRemainder: 0,
    maskRowsRows: 0,
    maskRowsStep: 0,
    operation: OPERATION_INVERT,
    operationRotateColorAxis: 'red',
    operationRotateColorRadians: 0.0,
    rotation: 0.0,
    scale: 1.0,
    wrap: false,
  };
}

function reboot() {
  reset();
  clear();
}

function rotate(rotation) {
  state.rotation += rotation;
}

function rotateColor(axis, radians) {
  state.operationRotateColorAxis = axis;
  state.operationRotateColorRadians = radians;
  state.operation = OPERATION_ROTATE_COLOR;
}

function rows(nrows, step) {
  state.maskRowsRows = nrows;
  state.maskRowsStep = step;
  state.mask = MASK_ROWS;
}

function save() {
  self.postMessage(JSON.stringify('save'));
}

function scale(scale) {
  state.scale *= scale;
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

function oscillatorFrequency(hz) {
  self.postMessage(JSON.stringify({'oscillatorFrequency': hz}));
}

function square() {
  state.mask = MASK_SQUARE;
}

function top() {
  state.mask = MASK_TOP;
}

function sample() {
  state.operation = OPERATION_SAMPLE;
}

function wrap() {
  state.wrap = !state.wrap;
}

function x() {
  state.mask = MASK_X;
}

function* range(iterations) {
  for (let i = 0; i < iterations; i++) {
    yield i;
  }
}

const rng = new Rng();
const start = Date.now();

let frameCallbacks = [];
let lastDelta = 0;
let lastFrame = 0;
let widgets = {};

self.addEventListener('message', async function (event) {
  const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
  const message = JSON.parse(event.data);
  switch (message.tag) {
    case 'checkbox':
      widgets[message.content.name] = message.content.value;
      break;
    case 'script':
      for (var callbacks of frameCallbacks) {
        callbacks.reject();
      }
      frameCallbacks = [];
      await new AsyncFunction(message.content)();
      self.postMessage(JSON.stringify('done'));
      break;
    case 'frame':
      for (var callbacks of frameCallbacks) {
        callbacks.resolve();
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
