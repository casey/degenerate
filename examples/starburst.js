// Reset state and clear the canvas
reboot();

// Store all available masks
const masks = [all, circle, cross, square, top, x];

// Seed the deterministic random number generator
rng.seed(3);

rotateColor('green', 0.1 * TAU);

rotate(0.1 * TAU);

// Render to the canvas 10 times setting a
// random mask each time
for (let i = 0; i < 20; i++) {
  rng.choose(masks)();
  render();
}

rotateColor('blue', 0.1 * TAU);

rotate(0.1 * TAU);

// Render to the canvas 10 times setting a
// random mask each time
for (let i = 0; i < 10; i++) {
  rng.choose(masks)();
  render();
}

// Press `Shift + Enter` to execute
