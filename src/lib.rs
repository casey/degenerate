use {
  crate::{
    color_axis::ColorAxis, command::Command, computer::Computer, gpu::Gpu, mask::Mask,
    operation::Operation, wrap::Wrap,
  },
  nalgebra::{
    Affine2, DMatrix, Matrix3, Point2, Rotation3, Similarity2, UnitComplex, Vector2, Vector3,
    Vector4,
  },
  rand::{rngs::StdRng, seq::SliceRandom, SeedableRng},
  std::{f64, str::FromStr, sync::Arc},
  strum::EnumString,
};

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

mod browser;
mod color_axis;
mod command;
mod computer;
mod mask;
mod operation;
mod wrap;
