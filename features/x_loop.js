x();
scale(2.0);
for (let i = 0; i < 8; i++) {
  render();
  wrap(i % 2 === 0);
}
