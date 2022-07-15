rotateColor('green', 0.05 * TAU);
circle();
scale(1 / 0.75);
wrap(true);
for (let i = 0; i < 8; i++) {
  render();
}
transform(0.8333 * TAU, [1 / 0.75, 1 / 0.75], [0, 0]);
rotateColor('blue', 0.05 * TAU);
for (let i = 0; i < 8; i++) {
  render();
}
