while (true) {
  reboot();
  if (checkbox('alpha')) {
    alpha(0.75);
  } else {
    alpha(1);
  }
  circle();
  scale(0.5);
  for (_ of range(8)) {
    render();
    wrap();
  }
  await frame();
}
