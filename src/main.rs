use {
  crate::{
    color_axis::ColorAxis, command::Command, computer::Computer, gpu::Gpu, mask::Mask,
    operation::Operation, software::Software, wrap::Wrap,
  },
  nalgebra::{
    Affine2, DMatrix, Matrix3, Point2, Rotation3, Similarity2, UnitComplex, Vector2, Vector3,
    Vector4,
  },
  rand::{rngs::StdRng, seq::SliceRandom, SeedableRng},
  std::{f64, str::FromStr, sync::Arc},
  strum::EnumString,
};

mod gpu;

#[cfg(target_arch = "wasm32")]
mod browser;
mod color_axis;
mod command;
mod computer;
mod mask;

#[cfg(not(target_arch = "wasm32"))]
mod native;
mod operation;
mod software;
mod wrap;

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
  #[cfg(not(target_arch = "wasm32"))]
  crate::native::run();
}
