use super::*;

pub(crate) trait Pixel {
  fn pixel(self, dimensions: Vector2<usize>) -> Vector2<usize>;
}

impl Pixel for Vector2<f32> {
  fn pixel(self, dimensions: Vector2<usize>) -> Vector2<usize> {
    Vector2::new(
      ((self.x + 1.0) / 2.0 * dimensions.x as f32 - 0.5).round() as usize,
      ((self.y + 1.0) / 2.0 * dimensions.y as f32 - 0.5).round() as usize,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn origin() {
    assert_eq!(
      Vector2::new(0.0, 0.0).pixel(Vector2::new(2, 2)),
      Vector2::new(1, 1)
    );
  }

  #[test]
  fn upper_left() {
    assert_eq!(
      Vector2::new(-1.0, -1.0).pixel(Vector2::new(2, 2)),
      Vector2::new(0, 0)
    );
  }

  #[test]
  fn upper_right() {
    assert_eq!(
      Vector2::new(1.0, -1.0).pixel(Vector2::new(2, 2)),
      Vector2::new(2, 0)
    );
  }

  #[test]
  fn lower_left() {
    assert_eq!(
      Vector2::new(-1.0, 1.0).pixel(Vector2::new(2, 2)),
      Vector2::new(0, 2)
    );
  }

  #[test]
  fn lower_right() {
    assert_eq!(
      Vector2::new(1.0, 1.0).pixel(Vector2::new(2, 2)),
      Vector2::new(2, 2)
    );
  }

  #[test]
  fn upper_left_mid() {
    assert_eq!(
      Vector2::new(-0.5, -0.5).pixel(Vector2::new(2, 2)),
      Vector2::new(0, 0)
    );
  }

  #[test]
  fn upper_right_mid() {
    assert_eq!(
      Vector2::new(0.5, -0.5).pixel(Vector2::new(2, 2)),
      Vector2::new(1, 0)
    );
  }

  #[test]
  fn lower_left_mid() {
    assert_eq!(
      Vector2::new(-0.5, 0.5).pixel(Vector2::new(2, 2)),
      Vector2::new(0, 1)
    );
  }

  #[test]
  fn lower_right_mid() {
    assert_eq!(
      Vector2::new(0.5, 0.5).pixel(Vector2::new(2, 2)),
      Vector2::new(1, 1)
    );
  }
}
