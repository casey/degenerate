rotate(0.3333 * TAU);
rotateColor('green', 0.05 * TAU);
circle();
scale(0.5);
wrap();
for (let i = 0; i < 8; i++) {
  render();
}
rotate(0.8333 * TAU);
rotateColor('blue', 0.05 * TAU);
for (let i = 0; i < 8; i++) {
  render();
}