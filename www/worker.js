"use strict";

class Rng {
  constructor(seed) {
    this._seed = seed;
    this.a = 1103515245;
    this.c = 12345;
    this.m = 0x80000000;
  }

  choose(masks) {
    masks[this.#nextRange(0, masks.length - 1)]();
  }

  seed(seed) {
    this._seed = seed;
  }

  #nextInt() {
    this._seed = (this.a * this._seed + this.c) % this.m;
    return this._seed;
  }

  #nextRange(min, max) {
    return min + Math.floor((this.#nextInt() / this.m) * (max - min))
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
      operationRotateColorAxis: "r",
      operationRotateColorTurns: 1.0,
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

  apply() {
    self.postMessage(JSON.stringify({ apply: this.state }));
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
  switch (data.messageType) {
    case "script":
      g = Object.getPrototypeOf(function* () {}).constructor(data.payload)();
      break;
    case "run":
      let result = g.next();
      if (result.done) self.postMessage(JSON.stringify("done"));
      break;
  }
});
