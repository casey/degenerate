reboot();
sample();
oscillatorGain(0.75);
oscillatorFrequency(60);
while (true) {
  await render();
}
