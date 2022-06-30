// Set the alpha value to 0.75
alpha(0.75);

// Set the circle mask
circle();

// Set the scale factor to 0.5
scale(0.5);

// Render to the canvas 8 times
for (let i = 0; i < 8; i++) {
  render();
  wrap();
}
