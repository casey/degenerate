let rotation = 5 / 6 * TAU;
let scale = 1.0;

while(true) {
  reboot();

  rotateColor('green', 0.05 * TAU);

  circle();

  scale *= 0.75;
  scale(scale);

  wrap(!filter.wrap);

  for (let i = 0; i < 8; i++) {
    render();
  }

  if (checkbox('rotate')) {
    rotation += delta() / 30000 * TAU;
  }

  rotate(rotation);

  rotateColor('blue', 0.05 * TAU);

  for (let i = 0; i < 8; i++) {
    render();
  }

  await frame();
}
