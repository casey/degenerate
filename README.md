# degenerate

Degenerate is an algorithmic image generator base on image filter chains. It is inspired by [blaster](https://github.com/casey/blaster).

## Usage

Filters are parsed from `:`-separated arguments, for instance:

```bash
$ cargo run -- --output output.png resize:512:512 top
```

will resize a matrix of pixels to be `512` columns and `512` rows and apply the
`top` filter, inverting the pixels in the top half of the image, ultimately
producing the following output:

<!-- Insert output here -->

The single best source for learning how each filter works is
[integration.rs](https://github.com/casey/degenerate/blob/master/tests/integration.rs),
where integration tests are written for each `degenerate` filter.
