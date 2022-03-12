use super::*;

pub(crate) trait Coordinates {
  fn coordinates(self, dimensions: Vector2<usize>, fit: bool) -> Vector2<f64>;
}

impl Coordinates for Vector2<usize> {
  fn coordinates(self, dimensions: Vector2<usize>, fit: bool) -> Vector2<f64> {
    let d = dimensions.map(|element| element as f64);
    let c = self.map(|element| element as f64);

    let stretch =
      (c + Vector2::from_element(0.5)).component_div(&d) * 2.0 - Vector2::from_element(1.0);

    if fit {
      stretch
        .component_div(&d.yx())
        .component_mul(&Vector2::from_element(d.x.min(d.y)))
    } else {
      stretch
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn center_pixel_is_at_origin() {
    assert_eq!(
      Vector2::new(0, 0).coordinates(Vector2::new(1, 1), false),
      Vector2::new(0.0, 0.0)
    );
  }

  #[test]
  fn coordinates_are_in_center_of_pixel() {
    assert_eq!(
      Vector2::new(0, 0).coordinates(Vector2::new(2, 2), false),
      Vector2::new(-0.5, -0.5)
    )
  }
}
