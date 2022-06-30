// Store all available masks
const masks = [all, circle, cross, square, top, x];

// Seed the deterministic random number generator
rng.seed(3);

// Rotate color about the green color axis
rotateColor('green', 0.1 * TAU);

// Rotate the canvas
rotate(0.1 * TAU);

// Render to the canvas 20 times,
// each time setting a random mask
for (let i = 0; i < 20; i++) {
  rng.choose(masks)();
  render();
}

// Rotate color about the blue color axis
rotateColor('blue', 0.1 * TAU);

// Rotate the canvas
rotate(0.1 * TAU);

// Render to the canvas 10 times,
// each time setting a random mask
for (let i = 0; i < 10; i++) {
  rng.choose(masks)();
  render();
}

// Press `Shift + Enter` to execute
