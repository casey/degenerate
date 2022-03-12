use super::*;

// todo: make fit or fill default

#[derive(Copy, Clone, Debug)]
pub(crate) enum Viewport {
  Fit,
  Stretch,
}

impl Viewport {
  pub(crate) fn coordinates(self, dimensions: Vector2<usize>, i: Vector2<usize>) -> Vector2<f64> {
    let d = dimensions.map(|element| element as f64);
    let c = i.map(|element| element as f64);

    let stretch =
      (c + Vector2::from_element(0.5)).component_div(&d) * 2.0 - Vector2::from_element(1.0);

    match self {
      Self::Fit => stretch
        .component_div(&d.yx())
        .component_mul(&Vector2::from_element(d.x.min(d.y))),
      Self::Stretch => stretch,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn center_pixel_is_at_origin() {
    assert_eq!(
      Viewport::Stretch.coordinates(Vector2::new(1, 1), Vector2::new(0, 0)),
      Vector2::new(0.0, 0.0)
    );
  }

  #[test]
  fn coordinates_are_in_center_of_pixel() {
    assert_eq!(
      Viewport::Stretch.coordinates(Vector2::new(2, 2), Vector2::new(0, 0)),
      Vector2::new(-0.5, -0.5)
    )
  }
}
