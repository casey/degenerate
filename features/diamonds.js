rotateColor('green', 0.05 * TAU);
circle();
scale(2.0);
transform(0.3333 * TAU, [2.0, 2.0], [0.0, 0.0]);
wrap(true);
for (let i = 0; i < 8; i++) {
  render();
}
transform(0.3333 * TAU + 0.8333 * TAU, [2.0, 2.0], [0.0, 0.0]);
rotateColor('blue', 0.05 * TAU);
for (let i = 0; i < 8; i++) {
  render();
}
