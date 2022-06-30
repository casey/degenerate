// Reset state and clear the canvas
reboot();

// Set the x mask
x();

// Set the scale factor to 0.5
scale(0.5);

// Render to the canvas 8 times, each time
// toggling the `wrap` option.
//
// Calls to `render` apply a transformation matrix
// to the sampling coordinates, if those coordinates fall out of
// bounds and `wrap` is toggled on, we wrap the transformed coordinates
// between [-1, 1], else we use the default color.
for (_ of range(8)) {
  render();
  wrap();
}
