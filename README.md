<h1 align="center"><code>degenerate</code></h1>
<div align="center">A generative image programming language</div>
<br>
<div align="center">
  <a href="https://crates.io/crates/degenerate">
    <img src="https://img.shields.io/crates/v/degenerate.svg" alt="crates.io version">
  </a>
  <a href="https://github.com/casey/degenerate/actions">
    <img src="https://github.com/casey/degenerate/workflows/CI/badge.svg" alt="build status">
  </a>
  <a href="https://github.com/casey/degenerate/releases">
    <img src="https://img.shields.io/github/downloads/casey/degenerate/total.svg" alt="downloads">
  </a>
</div>
<br>

## Quick Start

- Install `rustup` by following the instructions [here](https://rustup.rs/).
- Run `cargo install degenerate`
- Run a degenerate program:
  ```
    degenerate resize:1024 rotate-color:g:0.05 circle scale:0.5 wrap \
    for:8 apply loop \
    rotate-color:b:0.05 \
    for:8 apply loop \
    save
  ```
- Open `memory.png`, which should look like this:

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
