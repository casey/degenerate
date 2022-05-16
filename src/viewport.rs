use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Viewport {
  Fill,
  Fit,
  Stretch,
}

impl Viewport {
  pub(crate) fn coordinates(
    self,
    dimensions: Vector2<usize>,
    pixel: Vector2<usize>,
  ) -> Vector2<f64> {
    let d = dimensions.map(|element| element as f64);
    let f = pixel.map(|element| element as f64);

    let stretch =
      (f + Vector2::from_element(0.5)).component_div(&d) * 2.0 - Vector2::from_element(1.0);

    let aspect = d.x / d.y;

    let landscape = d.x > d.y;

    match self {
      Self::Fill => {
        if landscape {
          Vector2::new(stretch.x, stretch.y / aspect)
        } else {
          Vector2::new(stretch.x * aspect, stretch.y)
        }
      }
      Self::Fit => {
        if landscape {
          Vector2::new(stretch.x * aspect, stretch.y)
        } else {
          Vector2::new(stretch.x, stretch.y / aspect)
        }
      }
      Self::Stretch => stretch,
    }
  }

  pub(crate) fn pixel(
    self,
    dimensions: Vector2<usize>,
    coordinates: Vector2<f64>,
  ) -> Vector2<isize> {
    let d = dimensions.map(|element| element as f64);

    let aspect = d.x / d.y;

    let landscape = d.x > d.y;

    let stretch = match self {
      Self::Fill => {
        if landscape {
          Vector2::new(coordinates.x, coordinates.y * aspect)
        } else {
          Vector2::new(coordinates.x / aspect, coordinates.y)
        }
      }
      Self::Fit => {
        if landscape {
          Vector2::new(coordinates.x / aspect, coordinates.y)
        } else {
          Vector2::new(coordinates.x, coordinates.y * aspect)
        }
      }
      Self::Stretch => coordinates,
    };

    (((stretch + Vector2::from_element(1.0)) / 2.0).component_mul(&d) - Vector2::from_element(0.5))
      .map(|element| element.round() as isize)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn case(
    dimensions: (usize, usize),
    coordinates: (usize, usize),
    fill: (f64, f64),
    fit: (f64, f64),
    stretch: (f64, f64),
  ) {
    let dimensions = Vector2::new(dimensions.0, dimensions.1);
    let coordinates = Vector2::new(coordinates.0, coordinates.1);
    assert_eq!(
      Viewport::Fill.coordinates(dimensions, coordinates),
      Vector2::new(fill.0, fill.1)
    );
    assert_eq!(
      Viewport::Fit.coordinates(dimensions, coordinates),
      Vector2::new(fit.0, fit.1)
    );
    assert_eq!(
      Viewport::Stretch.coordinates(dimensions, coordinates),
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
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(0.0, 0.0)),
      Vector2::new(1, 1)
    );
  }

  #[test]
  fn pixel_upper_left() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(-1.0, -1.0)),
      Vector2::new(-1, -1)
    );
  }

  #[test]
  fn pixel_upper_right() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(1.0, -1.0)),
      Vector2::new(2, -1)
    );
  }

  #[test]
  fn pixel_lower_left() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(-1.0, 1.0)),
      Vector2::new(-1, 2)
    );
  }

  #[test]
  fn pixel_lower_right() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(1.0, 1.0)),
      Vector2::new(2, 2)
    );
  }

  #[test]
  fn pixel_upper_left_oob() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(-2.0, -2.0)),
      Vector2::new(-2, -2)
    );
  }

  #[test]
  fn pixel_upper_right_oob() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(2.0, -2.0)),
      Vector2::new(3, -2)
    );
  }

  #[test]
  fn pixel_lower_left_oob() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(-2.0, 2.0)),
      Vector2::new(-2, 3)
    );
  }

  #[test]
  fn pixel_lower_right_oob() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(2.0, 2.0)),
      Vector2::new(3, 3)
    );
  }

  #[test]
  fn pixel_upper_left_mid() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(-0.5, -0.5)),
      Vector2::new(0, 0)
    );
  }

  #[test]
  fn pixel_upper_right_mid() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(0.5, -0.5)),
      Vector2::new(1, 0)
    );
  }

  #[test]
  fn pixel_lower_left_mid() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(-0.5, 0.5)),
      Vector2::new(0, 1)
    );
  }

  #[test]
  fn pixel_lower_right_mid() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(2, 2), Vector2::new(0.5, 0.5)),
      Vector2::new(1, 1)
    );
  }

  #[test]
  fn pixel_origin_large() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(5, 5), Vector2::new(0.0, 0.0)),
      Vector2::new(2, 2)
    );
  }

  #[test]
  fn pixel_lower_right_mid_large() {
    assert_eq!(
      Viewport::Stretch.pixel(Vector2::new(5, 5), Vector2::new(0.5, 0.5)),
      Vector2::new(3, 3)
    );
  }
}
