use super::*;

pub(crate) trait Coordinates {
  fn coordinates(self, dimensions: Vector2<usize>) -> Vector2<f64>;
}

impl Coordinates for Vector2<usize> {
  fn coordinates(self, dimensions: Vector2<usize>) -> Vector2<f64> {
    // todo: rewrite using map
    Vector2::new(
      ((self.x as f64 + 0.5) / dimensions.x as f64) * 2.0 - 1.0,
      ((self.y as f64 + 0.5) / dimensions.y as f64) * 2.0 - 1.0,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn center_pixel_is_at_origin() {
    assert_eq!(
      Vector2::new(0, 0).coordinates(Vector2::new(1, 1)),
      Vector2::new(0.0, 0.0)
    );
  }

  #[test]
  fn coordinates_are_in_center_of_pixel() {
    assert_eq!(
      Vector2::new(0, 0).coordinates(Vector2::new(2, 2)),
      Vector2::new(-0.5, -0.5)
    )
  }
}
