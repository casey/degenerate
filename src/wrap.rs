use super::*;

pub(crate) trait Wrap {
  fn wrap(self) -> Vector2<f64>;
}

impl Wrap for Vector2<f64> {
  fn wrap(self) -> Vector2<f64> {
    Vector2::new(
      (self.x + 1.0).rem_euclid(2.0) - 1.0,
      (self.y + 1.0).rem_euclid(2.0) - 1.0,
    )
  }
}

#[cfg(test)]
mod tests {
  use {super::*, approx::assert_ulps_eq};

  #[test]
  fn wrap_lower_right() {
    assert_ulps_eq!(Vector2::new(1.1, 1.1).wrap(), Vector2::new(-0.9, -0.9));
  }

  #[test]
  fn wrap_upper_left() {
    assert_ulps_eq!(Vector2::new(-1.1, -1.1).wrap(), Vector2::new(0.9, 0.9));
  }

  #[test]
  fn wrap_lower_right_large() {
    assert_ulps_eq!(Vector2::new(3.1, 3.1).wrap(), Vector2::new(-0.9, -0.9));
  }

  #[test]
  fn wrap_upper_left_large() {
    assert_ulps_eq!(Vector2::new(-3.1, -3.1).wrap(), Vector2::new(0.9, 0.9));
  }

  #[test]
  fn wrap_lower_right_larger() {
    assert_ulps_eq!(Vector2::new(5.1, 5.1).wrap(), Vector2::new(-0.9, -0.9));
  }

  #[test]
  fn wrap_upper_left_larger() {
    assert_ulps_eq!(Vector2::new(-5.1, -5.1).wrap(), Vector2::new(0.9, 0.9));
  }
}
