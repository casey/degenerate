reboot();
sample();
while (true) {
  if (checkbox('record')) {
    record(true);
    oscillatorFrequency(0);
  } else {
    record(false);
    oscillatorFrequency(60);
  }

  await render();
}
