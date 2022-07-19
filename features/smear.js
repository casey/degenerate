const fields = [all, circle, cross, square, top, x];
rng.seed(9);
rotateColor('green', 0.01 * TAU);
rotate(0.01 * TAU);
for (let i = 0; i < 100; i++) {
  rng.choose(fields)();
  render();
}
rotateColor('blue', 0.01 * TAU);
rotate(0.02 * TAU);
for (let i = 0; i < 100; i++) {
  rng.choose(fields)();
  render();
}
