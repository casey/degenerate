"use strict";

importScripts('randchacha_browser.min.js');

class Rng {
  constructor(seed) {
    const _seed = new Uint8Array(32);
    _seed[0] = seed;
    this._rng = new randchacha.ChaChaRng(_seed);
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

class Computer {
  static MASK_ALL = 0;
  static MASK_CIRCLE = 1;
  static MASK_CROSS = 2;
  static MASK_MOD = 3;
  static MASK_ROWS = 4;
  static MASK_SQUARE = 5;
  static MASK_TOP = 6;
  static MASK_X = 7;

  static OPERATION_DEBUG = 0;
  static OPERATION_IDENTITY = 1;
  static OPERATION_INVERT = 2;
  static OPERATION_ROTATE_COLOR = 3;

  constructor() {
    this.state = {
      alpha: 1.0,
      defaultColor: [0.0, 0.0, 0.0],
      mask: Computer.MASK_ALL,
      maskModDivisor: 0,
      maskModRemainder: 0,
      maskRowsRows: 0,
      maskRowsStep: 0,
      operation: Computer.OPERATION_INVERT,
      operationRotateColorAxis: "red",
      operationRotateColorTurns: 0.0,
      rotation: 0.0,
      scale: 1.0,
      wrap: false,
    };
  }

  all() {
    this.state.mask = Computer.MASK_ALL;
  }

  alpha(alpha) {
    this.state.alpha = alpha;
  }

  render() {
    self.postMessage(JSON.stringify({ render: this.state }));
  }

  circle() {
    this.state.mask = Computer.MASK_CIRCLE;
  }

  cross() {
    this.state.mask = Computer.MASK_CROSS;
  }

  debug() {
    this.state.operation = Computer.OPERATION_DEBUG;
  }

  defaultColor(defaultColor) {
    this.state.defaultColor = defaultColor;
  }

  identity() {
    this.state.operation = Computer.OPERATION_IDENTITY;
  }

  invert() {
    this.state.operation = Computer.OPERATION_INVERT;
  }

  mod(divisor, remainder) {
    this.state.maskModDivisor = divisor;
    this.state.maskModRemainder = remainder;
    this.state.mask = Computer.MASK_MOD;
  }

  rotate(rotation) {
    this.state.rotation += rotation;
  }

  rotateColor(axis, turns) {
    this.state.operationRotateColorAxis = axis;
    this.state.operationRotateColorTurns = turns;
    this.state.operation = Computer.OPERATION_ROTATE_COLOR;
  }

  rows(nrows, step) {
    this.state.maskRowsRows = nrows;
    this.state.maskRowsStep = step;
    this.state.mask = Computer.MASK_ROWS;
  }

  scale(scale) {
    this.state.scale *= scale;
  }

  square() {
    this.state.mask = Computer.MASK_SQUARE;
  }

  top() {
    this.state.mask = Computer.MASK_TOP;
  }

  wrap() {
    this.state.wrap = !this.state.wrap;
  }

  x() {
    this.state.mask = Computer.MASK_X;
  }
}

const rng = new Rng(0);
const computer = new Computer();

let g;

self.addEventListener("message", function (event) {
  const data = JSON.parse(event.data);
  if (data.script) {
    g = Object.getPrototypeOf(function* () {}).constructor(data.script)();
  } else {
    let result = g.next();
    if (result.done) self.postMessage(JSON.stringify("done"));
  }
});
