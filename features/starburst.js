const fields = [all, circle, cross, square, top, x];
rng.seed(3);
rotateColor('green', 0.1 * TAU);
rotate(0.1 * TAU);
for (let i = 0; i < 20; i++) {
  rng.choose(fields)();
  render();
}
rotateColor('blue', 0.1 * TAU);
rotate(0.2 * TAU);
for (let i = 0; i < 10; i++) {
  rng.choose(fields)();
  render();
}
