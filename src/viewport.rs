use super::*;

// todo:
// - make fit or fill default
// - iterate over programs when building tests
// - if no memory is available, still generate actual memory
// - inline degenerate programs in tests and stop auto-generating
// - test fit fill and stretch in square aspect ratio
// - fix watch issue

#[derive(Copy, Clone, Debug)]
pub(crate) enum Viewport {
  Fill,
  Fit,
  Stretch,
}

impl Viewport {
  pub(crate) fn coordinates(self, dimensions: Vector2<usize>, i: Vector2<usize>) -> Vector2<f64> {
    let d = dimensions.map(|element| element as f64);
    let c = i.map(|element| element as f64);

    let stretch =
      (c + Vector2::from_element(0.5)).component_div(&d) * 2.0 - Vector2::from_element(1.0);

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
}
