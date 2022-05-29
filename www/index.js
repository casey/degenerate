import init, {run} from '/degenerate.js';

let audio = document.getElementsByTagName('audio')[0];
let context = new AudioContext();
let analyser = context.createAnalyser();

let source = context.createMediaElementSource(audio);
source.connect(analyser);

var data = new Float32Array(analyser.fftSize);

analyser.getFloatTimeDomainData(data);

await init('/degenerate_bg.wasm');

run();
