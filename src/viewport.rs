use super::*;

// todo: make fit or fill default

#[derive(Copy, Clone, Debug)]
pub(crate) enum Viewport {
  Fill,
  Fit,
  Stretch,
}

impl Viewport {
  pub(crate) fn coordinatess(self, dimensions: Vector2<usize>, i: Vector2<usize>) -> Vector2<f64> {
    let d = dimensions.map(|element| element as f64);
    let c = i.map(|element| element as f64);

    let mut stretch =
      (c + Vector2::from_element(0.5)).component_div(&d) * 2.0 - Vector2::from_element(1.0);

    match self {
      Self::Fill => {
        if d.x > d.y {
          stretch.x *= d.x / d.y;
        } else {
          stretch.y *= d.y / d.x;
        }
        stretch
      }
      Self::Fit => {
        stretch.component_mul(&Vector2::from_element(d.x.min(d.y)).component_div(&d.yx()))
      }
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

  #[test]
  fn stretch() {
    assert_eq!(
      Viewport::Stretch.coordinates(Vector2::new(2, 1), Vector2::new(0, 0)),
      Vector2::new(-0.5, 0.0)
    );
  }

  #[test]
  fn fit() {
    assert_eq!(
      Viewport::Fit.coordinates(Vector2::new(4, 2), Vector2::new(0, 0)),
      Vector2::new(-0.75, -0.25)
    );
    assert_eq!(
      Viewport::Fit.coordinates(Vector2::new(2, 4), Vector2::new(0, 0)),
      Vector2::new(-0.25, -0.75)
    );
  }

  //         ....
  // xxxx    xxxx
  // xxxx -> xxxx
  //         ....

  #[test]
  fn fill() {
    assert_eq!(
      Viewport::Fill.coordinates(Vector2::new(4, 2), Vector2::new(0, 0)),
      Vector2::new(-1.5, -0.5)
    );
    // assert_eq!(
    //   Viewport::Fill.coordinates(Vector2::new(2, 4), Vector2::new(0, 0)),
    //   Vector2::new(-0.5, -1.5)
    // );
    // assert_eq!(
    //   Viewport::Fit.coordinates(Vector2::new(2, 4), Vector2::new(0, 0)),
    //   Vector2::new(-0.25, -0.75)
    // );
  }
}
