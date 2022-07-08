alpha(0.75);
circle();
scale(0.5);
for (let i = 0; i < 8; i++) {
  render();
  wrap(!filter.wrap);
}
