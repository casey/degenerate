import init, {test} from './degenerate.js';

await init('degenerate_bg.wasm');

window.test = test;
