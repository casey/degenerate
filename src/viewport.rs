use super::*;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Viewport {
  Fill,
  Fit,
  Stretch,
}

impl Viewport {
  pub(crate) fn transform(self, dimensions: Vector2<usize>) -> Matrix3<f64> {
    let d = dimensions.map(|element| element as f64);
    let aspect = d.x / d.y;
    let landscape = d.x > d.y;

    let m = Matrix3::identity()
      .append_translation(&Vector2::from_element(0.5))
      .append_nonuniform_scaling(&Vector2::new(1.0 / d.x, 1.0 / d.y))
      .append_scaling(2.0)
      .append_translation(&Vector2::from_element(-1.0));

    let scale = match self {
      Self::Fill => {
        if landscape {
          Vector2::new(1.0, 1.0 / aspect)
        } else {
          Vector2::new(aspect, 1.0)
        }
      }
      Self::Fit => {
        if landscape {
          Vector2::new(aspect, 1.0)
        } else {
          Vector2::new(1.0, 1.0 / aspect)
        }
      }
      Self::Stretch => Vector2::new(1.0, 1.0),
    };

    m.append_nonuniform_scaling(&scale)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub(crate) fn coordinates(
    viewport: Viewport,
    dimensions: Vector2<usize>,
    pixel: Vector2<usize>,
  ) -> Vector2<f64> {
    viewport
      .transform(dimensions)
      .transform_point(&pixel.map(|element| element as f64).into())
      .coords
  }

  pub(crate) fn pixel(
    viewport: Viewport,
    dimensions: Vector2<usize>,
    coordinates: Vector2<f64>,
  ) -> Vector2<isize> {
    viewport
      .transform(dimensions)
      .try_inverse()
      .unwrap()
      .transform_point(&coordinates.into())
      .map(|element| element.round() as isize)
      .coords
  }

  fn case(
    d: (usize, usize),
    c: (usize, usize),
    fill: (f64, f64),
    fit: (f64, f64),
    stretch: (f64, f64),
  ) {
    let d = Vector2::new(d.0, d.1);
    let c = Vector2::new(c.0, c.1);
    assert_eq!(
      coordinates(Viewport::Fill, d, c),
      Vector2::new(fill.0, fill.1)
    );
    assert_eq!(coordinates(Viewport::Fit, d, c), Vector2::new(fit.0, fit.1));
    assert_eq!(
      coordinates(Viewport::Stretch, d, c),
      Vector2::new(stretch.0, stretch.1)
    );
  }

  #[test]
  fn center_pixel_is_at_origin() {
    case((1, 1), (0, 0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0));
  }

  #[test]
  fn coordinates_are_in_center_of_pixel() {
    case((2, 2), (0, 0), (-0.5, -0.5), (-0.5, -0.5), (-0.5, -0.5));
  }

  #[test]
  fn pixel_origin() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(0.0, 0.0)
      ),
      Vector2::new(1, 1)
    );
  }

  #[test]
  fn pixel_upper_left() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(-1.0, -1.0)
      ),
      Vector2::new(-1, -1)
    );
  }

  #[test]
  fn pixel_upper_right() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(1.0, -1.0)
      ),
      Vector2::new(2, -1)
    );
  }

  #[test]
  fn pixel_lower_left() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(-1.0, 1.0)
      ),
      Vector2::new(-1, 2)
    );
  }

  #[test]
  fn pixel_lower_right() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(1.0, 1.0)
      ),
      Vector2::new(2, 2)
    );
  }

  #[test]
  fn pixel_upper_left_oob() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(-2.0, -2.0)
      ),
      Vector2::new(-2, -2)
    );
  }

  #[test]
  fn pixel_upper_right_oob() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(2.0, -2.0)
      ),
      Vector2::new(3, -2)
    );
  }

  #[test]
  fn pixel_lower_left_oob() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(-2.0, 2.0)
      ),
      Vector2::new(-2, 3)
    );
  }

  #[test]
  fn pixel_lower_right_oob() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(2.0, 2.0)
      ),
      Vector2::new(3, 3)
    );
  }

  #[test]
  fn pixel_upper_left_mid() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(-0.5, -0.5)
      ),
      Vector2::new(0, 0)
    );
  }

  #[test]
  fn pixel_upper_right_mid() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(0.5, -0.5)
      ),
      Vector2::new(1, 0)
    );
  }

  #[test]
  fn pixel_lower_left_mid() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(-0.5, 0.5)
      ),
      Vector2::new(0, 1)
    );
  }

  #[test]
  fn pixel_lower_right_mid() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(2, 2),
        Vector2::new(0.5, 0.5)
      ),
      Vector2::new(1, 1)
    );
  }

  #[test]
  fn pixel_origin_large() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(5, 5),
        Vector2::new(0.0, 0.0)
      ),
      Vector2::new(2, 2)
    );
  }

  #[test]
  fn pixel_lower_right_mid_large() {
    assert_eq!(
      pixel(
        Viewport::Stretch,
        Vector2::new(5, 5),
        Vector2::new(0.5, 0.5)
      ),
      Vector2::new(3, 3)
    );
  }
}
