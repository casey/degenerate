// Reset state and clear the canvas
reboot();

// Rotate color about the green color axis
rotateColor('green', 0.05 * TAU);

// Set the circle mask
circle();

// Scale the canvas
scale(3 / 4);

// Wrap pixels
wrap();

// Render to the canvas 8 times
for (_ of range(8))
  render();

// Rotate the canvas
rotate((5 / 6) * TAU);

// Rotate color about the blue color axis
rotateColor('blue', 0.05 * TAU);

// Render to the canvas 8 times
for (_ of range(8))
  render();

// Press `Shift + Enter` to execute
