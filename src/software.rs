use super::*;

pub(crate) struct Software;

impl Gpu for Software {
  fn apply(&self, _state: &Computer) -> Result {
    Ok(())
    // let similarity = self.similarity.inverse();
    // let dimensions = self.dimensions();
    // let transform = self.viewport.transform(dimensions);
    // let inverse = transform.inverse();
    // let mut output = self.memory.clone();
    // for col in 0..self.memory.ncols() {
    //   for row in 0..self.memory.nrows() {
    //     let i = Point2::new(col as f64, row as f64);
    //     let v = transform.transform_point(&i);
    //     let v = similarity.transform_point(&v);
    //     let v = if self.wrap { v.wrap() } else { v };
    //     let i = inverse
    //       .transform_point(&v)
    //       .map(|element| element.round() as isize);
    //     if self.mask.is_masked(dimensions, i, v) {
    //       let input = if i.x >= 0
    //         && i.y >= 0
    //         && i.x < self.memory.ncols() as isize
    //         && i.y < self.memory.nrows() as isize
    //       {
    //         self.memory[(i.y as usize, i.x as usize)]
    //       } else {
    //         self.default
    //       };
    //       let over = self.operation.apply(v, input.xyz()).map(|c| c as f64);
    //       let under = self.memory[(row, col)].xyz().map(|c| c as f64);
    //       let combined =
    //         (over * self.alpha + under * (1.0 - self.alpha)) / (self.alpha + (1.0 - self.alpha));
    //       output[(row, col)] = Vector4::new(
    //         combined.x as u8,
    //         combined.y as u8,
    //         combined.z as u8,
    //         ALPHA_OPAQUE,
    //       );
    //     }
    //   }
    // }
    // self.memory = output;
    // self.autosave()
  }
}

impl Software {
  pub(crate) fn new() -> Self {
    Self
  }
}
