reboot();

rotateColor('green', 0.05 * TAU);

circle();

scale(0.75);

wrap();

for (_ of range(8)) {
  render();
}

rotate((5 / 6) * TAU);

rotateColor('blue', 0.05 * TAU);

for (_ of range(8)) {
  render();
}