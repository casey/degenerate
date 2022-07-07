reboot();
sample();
while (true) {
  if (checkbox('record')) {
    record(true);
    oscillatorGain(0.0);
  } else {
    record(false);
    oscillatorGain(0.25);
  }

  await render();
}
