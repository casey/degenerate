"use strict";

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

class Computer {
  static MASK_ALL = 0;
  static MASK_CHECK = 1;
  static MASK_CIRCLE = 2;
  static MASK_CROSS = 3;
  static MASK_MOD = 4;
  static MASK_ROWS = 5;
  static MASK_SQUARE = 6;
  static MASK_TOP = 7;
  static MASK_X = 8;

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
    return this;
  }

  alpha(alpha) {
    this.state.alpha = alpha;
    return this;
  }

  async frame() {
    await new Promise((resolve, reject) => {
      frameResolvers.push(resolve);
    });
  }

  render() {
    self.postMessage(JSON.stringify({ render: this.state }));
    return this;
  }

  resolution(resolution) {
    if (Number.isInteger(resolution)) {
      self.postMessage(JSON.stringify({ resolution }));
    }
  }

  check() {
    this.state.mask = Computer.MASK_CHECK;
    return this;
  }

  circle() {
    this.state.mask = Computer.MASK_CIRCLE;
    return this;
  }

  cross() {
    this.state.mask = Computer.MASK_CROSS;
    return this;
  }

  debug() {
    this.state.operation = Computer.OPERATION_DEBUG;
    return this;
  }

  defaultColor(defaultColor) {
    this.state.defaultColor = defaultColor;
    return this;
  }

  identity() {
    this.state.operation = Computer.OPERATION_IDENTITY;
    return this;
  }

  invert() {
    this.state.operation = Computer.OPERATION_INVERT;
    return this;
  }

  mod(divisor, remainder) {
    this.state.maskModDivisor = divisor;
    this.state.maskModRemainder = remainder;
    this.state.mask = Computer.MASK_MOD;
    return this;
  }

  rotate(rotation) {
    this.state.rotation += rotation;
    return this;
  }

  rotateColor(axis, turns) {
    this.state.operationRotateColorAxis = axis;
    this.state.operationRotateColorTurns = turns;
    this.state.operation = Computer.OPERATION_ROTATE_COLOR;
    return this;
  }

  rows(nrows, step) {
    this.state.maskRowsRows = nrows;
    this.state.maskRowsStep = step;
    this.state.mask = Computer.MASK_ROWS;
    return this;
  }

  save() {
    self.postMessage(JSON.stringify("save"));
    return this;
  }

  scale(scale) {
    this.state.scale *= scale;
    return this;
  }

  square() {
    this.state.mask = Computer.MASK_SQUARE;
    return this;
  }

  top() {
    this.state.mask = Computer.MASK_TOP;
    return this;
  }

  wrap() {
    this.state.wrap = !this.state.wrap;
    return this;
  }

  x() {
    this.state.mask = Computer.MASK_X;
    return this;
  }
}

const AsyncFunction = Object.getPrototypeOf(async function(){}).constructor;

const rng = new Rng();

const computer = new Computer();

function* range(iterations) {
  for (let i = 0; i < iterations; i++) {
    yield i;
  }
}

let frameResolvers= [];

self.addEventListener("message", async function (event) {
  const message = JSON.parse(event.data);
  switch (message.tag) {
    case "script":
      await (new AsyncFunction(message.content))();
      self.postMessage(JSON.stringify("done"));
      break;
    case "frame":
      for (var resolve of frameResolvers) {
        resolve();
      }
      frameResolvers = [];
      break;
  }
});
