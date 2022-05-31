// clear the environment

const ALL = 'All';
const CIRCLE = 'Circle';
const SQUARE = 'Square';
const X = 'X';

function MOD(divisor, remainder) {
  return {Mod: {divisor: divisor, remainder: remainder}};
}

function ROWS(nrows, step) {
  return {Rows: {nrows: nrows}};
}

const DEBUG = 'Debug';
const IDENTITY = 'Identity'
const INVERT = 'Invert';

function ROTATE_COLOR(axis, turns) {
  return {RotateColor: [axis, turns]};
}

function ROTATE(value) {
  rotation += value;
}

function SCALE(value) {
  scale *= value;
}

function DEFAULT_COLOR(x, y, z) {
  default_color = [x, y, z];
}

let alpha = 1.0;
let default_color = [0.0, 0.0, 0.0];
let mask = ALL;
let operation = INVERT;
let rotation = 0.0;
let scale = 1.0;
let wrap = false;

function apply() {
  self.postMessage(JSON.stringify({
    alpha,
    default_color,
    mask,
    operation,
    rotation,
    scale,
    wrap
  }));
}

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
