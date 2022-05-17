import init, {run} from './degenerate.js';

window.errors = [];
window.done = false;

await init('degenerate_bg.wasm');

run();
