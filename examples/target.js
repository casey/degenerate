// Reset state and clear the canvas
reboot()

// Set the circle mask
circle();

// Set the scale factor to 0.5
scale(0.5);

// Render to the canvas 8 times
//
// The initial render draws a white circle
// scaled by 0.5
//
// Each subsequent render samples the output
// of the previous render, inverts a white circle,
// and scales it down by 0.5
//
// This iterated render process produces a target
// pattern
for (_ of range(8)) {
  render();
}

// Press `Shift + Enter` to execute
