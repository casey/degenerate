<h1 align="center"><code>degenerate</code></h1>
<div align="center">An prorammable generative art engine</div>
<br>
<div align="center">
  <a href="https://discord.gg/87cjuz4FYg">
    <img src="https://img.shields.io/discord/695580069837406228?logo=discord" alt="chat on discord">
  </a>
  <a href="https://github.com/casey/degenerate/actions">
    <img src="https://github.com/casey/degenerate/workflows/CI/badge.svg" alt="build status">
  </a>
</div>
<br>

Degenerate is an programmable generative art engine that runs in the browser
that can be programmed using Rust or JavaScript. It is deployed at
[degenerate.computer](https://degenerate.computer).

Quick Start
-----------

- Go to https://degenerate.computer

- Paste this code into the text area:
  ```
  rotateColor('green', 0.05 * TAU);
  circle();
  scale(0.75);
  wrap();
  for (let i = 0; i < 8; i++) {
    render();
  }
  rotate(0.8333 * TAU);
  rotateColor('blue', 0.05 * TAU);
  for (let i = 0; i < 8; i++) {
    render();
  }
  ```

- Press `Shift + Enter`

- It should look like this:

![gorgeous example image](example.jpg)

Please consult the
[Degenerate Programmer's Manual](https://degenerate.computer/man) for more
information.

Credits
-------

`degenerate` is written by [Casey Rodarmor](https://rodarmor.com) and
[Liam Scalzulli](https://liam.rs).

Prior Art
---------

Degenerate builds on the techniques used in
[casey/blaster](https://github.com/casey/blaster), an audio-reactive visuals
engine.
