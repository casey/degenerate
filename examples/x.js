reboot();

x();

scale(0.5);

for (_ of range(8)) {
  render();
  wrap(!state.wrap);
}
