alpha(0.75);
circle();
transform(0, [2, 2], [0, 0]);
for (let i = 0; i < 8; i++) {
  render();
  wrap(i % 2 == 0);
}
