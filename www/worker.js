"use strict";

class Rng {
  static MIN = -2147483648;
  static MAX = 2147483647;

  constructor(seed) {
    this._seed = seed;
    this._value = seed;
  }

  map(val, minFrom, maxFrom, minTo, maxTo) {
    return ((val - minFrom) / (maxFrom - minFrom)) * (maxTo - minTo) + minTo;
  }

  next(min, max) {
    this._value = this.shift(this._value);
    return Math.floor(this.map(this._value, Rng.MIN, Rng.MAX, min, max + 1));
  }

  shift(value) {
    value ^= value << 13;
    value ^= value >> 17;
    value ^= value << 5;
    return value;
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

  static OPERATION_ROTATE_COLOR_AXIS_RED = 0;
  static OPERATION_ROTATE_COLOR_AXIS_GREEN = 1;
  static OPERATION_ROTATE_COLOR_AXIS_BLUE = 2;

  constructor() {
    this.rng = new Rng(0);
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

  choose(masks) {
    switch (masks[this.rng.next(0, masks.length - 1)]) {
      case "all":
        this.all();
        break;
      case "circle":
        this.circle();
        break;
      case "cross":
        this.cross();
        break;
      case "square":
        this.square();
        break;
      case "top":
        this.top();
        break;
      case "x":
        this.x();
        break;
      default:
        throw 'Invalid mask';
    }
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

  seed(seed) {
    this.rng = new Rng(seed);
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
