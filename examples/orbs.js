reboot();
rotateColor('green', 0.05 * TAU);
circle();
scale(0.75);
wrap();
for (let i = 0; i < 8; i++) {
  render();
}
rotateColor('blue', 0.05 * TAU);
for (let i = 0; i < 8; i++) {
  render();
}
