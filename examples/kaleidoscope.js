if (this.r == undefined) {
  this.r = 5 / 6 * TAU;
}

let s = 1 / 0.75;

reboot();

rotateColor('green', 0.05 * TAU);

circle();

scale(s);

wrap(true);

times(8);

render();

if (checkbox('rotate')) {
  this.r += delta() / 30000 * TAU;
}

transform(this.r, [s, s], [0, 0]);

rotateColor('blue', 0.05 * TAU);

render();
