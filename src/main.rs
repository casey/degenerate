use {
  crate::{
    color_axis::ColorAxis, command::Command, computer::Computer, mask::Mask, operation::Operation,
    viewport::Viewport, wrap::Wrap,
  },
  image::{ImageBuffer, RgbaImage},
  nalgebra::{
    Affine2, DMatrix, Matrix3, Point2, Rotation3, Similarity2, UnitComplex, Vector2, Vector3,
    Vector4,
  },
  rand::{rngs::StdRng, seq::SliceRandom, SeedableRng},
  std::{f64, path::PathBuf, str::FromStr},
  strum::EnumString,
};

#[cfg(target_arch = "wasm32")]
mod browser;
mod color_axis;
mod command;
mod computer;
mod mask;
#[cfg(not(target_arch = "wasm32"))]
mod native;
mod operation;
mod viewport;
mod wrap;

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
  #[cfg(not(target_arch = "wasm32"))]
  crate::native::run();
}
