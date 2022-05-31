// clear the environment

const ALL = 0;

let mask = ALL;
let alpha = 1.0;

function apply() {
  self.postMessage(JSON.stringify({
    'alpha': alpha,
  }));
}

let f = new Function();

self.addEventListener("message", function(event) {
  const data = JSON.parse(event.data);
  switch (data.messageType) {
    case "script":
      console.log('Setting program...');
      let script = data.payload;
      f = new Function(script);
      break;
    case "run":
      console.log('Running program...');
      f();
      break;
  }
});
