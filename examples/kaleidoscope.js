let rotation = 5 / 6 * TAU;

while(true) {
  reboot();

  rotateColor('green', 0.05 * TAU);

  circle();

  scale(filter.scale * 0.75);

  wrap(!filter.wrap);

  for (_ of range(8)) {
    render();
  }

  if (checkbox('rotate')) {
    rotation += delta() / 30000 * TAU;
  }

  rotate(filter.rotation + rotation);

  rotateColor('blue', 0.05 * TAU);

  for (_ of range(8)) {
    render();
  }

  await frame();
}
