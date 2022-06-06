<h1 align="center"><code>degenerate</code></h1>
<div align="center">A generative image programming language</div>
<br>
<div align="center">
  <a href="https://github.com/casey/degenerate/actions">
    <img src="https://github.com/casey/degenerate/workflows/CI/badge.svg" alt="build status">
  </a>
</div>
<br>

## Quick Start

- Go to https://degenerate.computer

- Paste this code into the page:
  ```
computer.rotateColor('green', 0.05);
computer.circle();
computer.scale(0.75);
computer.wrap();
for (let i = 0; i < 8; i++) {
  computer.render();
}
computer.rotate(0.8333);
computer.rotateColor('blue', 0.05);
for (let i = 0; i < 8; i++) {
  computer.render();
}
```

- It should look like this:

![gorgeous example image](example.jpg)

Please consult the
[Degenerate Programmer's Manual](https://degenerate.computer/man) for more
information.

## Credits

`degenerate` is written by [Casey Rodarmor](https://rodarmor.com) and
[Liam Scalzulli](https://liam.rs).

## Prior Art

Degenerate builds on the techniques used in
[casey/blaster](https://github.com/casey/blaster), an audio-reactive visuals
engine.
