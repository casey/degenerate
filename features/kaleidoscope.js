rotateColor('green', 0.05 * TAU);
circle();
scale(0.75);
wrap(true);
for (let i = 0; i < 8; i++) {
  render();
}
rotate(state.rotation + 0.8333 * TAU);
rotateColor('blue', 0.05 * TAU);
for (let i = 0; i < 8; i++) {
  render();
}