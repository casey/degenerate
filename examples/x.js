// Reset state and clear the canvas
reboot();

// Set the x mask
x();

// Set the scale factor to 0.5
scale(0.5);

// Render to the canvas 8 times, each time
// toggling the `wrap` option.
//
// This will wrap sampled pixels in between [-1, 1]
// instead of using the default color.
for (_ of range(8)) {
  render();
  wrap();
}
