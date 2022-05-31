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

let alpha = 1.0;
let mask = ALL;
let operation = INVERT;

function apply() {
  self.postMessage(JSON.stringify({
    'alpha': alpha,
    'mask': mask,
    'operation': operation
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
