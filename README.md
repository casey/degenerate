# degenerate

Degenerate is a generative image language.

Below is a sample image made with degenerate:

![](./images/44.png)

with the corresponding program:

```
$ degenerate rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save
```

## Compiling

`degenerate` can render to a terminal or to a window. To render to a window,
`degenerate` must be built with the optional `window` feature.

## Usage

```bash
$ degenerate [COMMAND]...
```

`COMMAND`s may take zero or more `:`-separated arguments, and are currently
undocumented. The best way to learn what they do is to peruse the [image
tests](images). The name of each image is the `degenerate` program that
produced it. The image tests are reproduced below, with each preceded by its
`degenerate` invocation.

## Prior Art

Degenerate builds on the techniques used in
[casey/blaster](https://github.com/casey/blaster), an audio-reactive visuals
engine.
