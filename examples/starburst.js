reboot();

const fields = [all, circle, cross, square, top, x];

seed(3);

rotateColor('green', 0.1 * TAU);

rotate(0.1 * TAU);

for (let i = 0; i < 20; i++) {
  choose(fields)();
  render();
}

rotateColor('blue', 0.1 * TAU);

rotate(0.2 * TAU);

for (let i = 0; i < 10; i++) {
  choose(fields)();
  render();
}
