if (frame == 0) {
  this.r = 5 / 6 * TAU;
}

let s = 1 / 0.75;

rotateColor('green', 0.05 * TAU);

circle();

scale(s);

wrap(true);

for (let i = 0; i < 8; i++) {
  render();
}

if (checkbox('rotate')) {
  this.r += delta() / 30000 * TAU;
}

transform(this.r, [s, s], [0, 0]);

rotateColor('blue', 0.05 * TAU);

for (let i = 0; i < 8; i++) {
  render();
}
