'use strict';

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
    this.state = {
      alpha: 1.0,
      defaultColor: [0.0, 0.0, 0.0],
      mask: Computer.MASK_ALL,
      maskModDivisor: 0,
      maskModRemainder: 0,
      maskRowsRows: 0,
      maskRowsStep: 0,
      operation: Computer.OPERATION_INVERT,
      rotation: 0.0,
      scale: 1.0,
      wrap: false,
    }
  }

  apply() {
    self.postMessage(JSON.stringify(this.state));
  }

  alpha(alpha) {
    this.state.alpha = alpha;
  }

  all() {
    this.state.mask = Computer.MASK_ALL;
  }

  circle() {
    this.state.mask = Computer.MASK_CIRCLE;
  }

  cross() {
    this.state.mask = Computer.MASK_CROSS;
  }

  mod(divisor, remainder) {
    this.state.maskModDivisor = divisor;
    this.state.maskModRemainder = remainder;
    this.state.mask = Computer.MASK_MOD;
  }

  rows(nrows, step) {
    this.state.maskRowsRows = nrows;
    this.state.maskRowsStep = step;
    this.state.mask = Computer.MASK_ROWS;
  }

  square() {
    this.state.mask = Computer.MASK_SQUARE;
  }

  top() {
    this.state.mask = Computer.MASK_TOP;
  }

  x() {
    this.state.mask = Computer.MASK_X;
  }

  debug() {
    this.state.operation = OPERATION_DEBUG;
  }

  identity() {
    this.state.operation = Computer.OPERATION_IDENTITY;
  }

  invert() {
    this.state.operation = Computer.OPERATION_INVERT;
  }

  rotateColor(matrix) {
    this.state.operationRotateColorMatrix = matrix;
    this.state.operation = OPERATION_ROTATE_COLOR;
  }

  scale(scale) {
    this.state.scale *= scale;
  }

  rotate(rotation) {
    this.state.rotation += rotation;
  }

  defaultColor(defaultColor) {
    this.state.defaultColor = defaultColor;
  }

  wrap(wrap) {
    this.state.wrap = wrap;
  }
}

const computer = new Computer();

let f = new Function();

self.addEventListener("message", function(event) {
  const data = JSON.parse(event.data);
  switch (data.messageType) {
    case "script":
      let script = data.payload;
      f = new Function(script);
      break;
    case "run":
      f();
      break;
  }
});
