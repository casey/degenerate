use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
  #[structopt(long)]
  width: usize,
  #[structopt(long)]
  height: usize,
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn filter(arguments: &Arguments, a: &[u8], b: &mut [u8]) {
  for row in 0..arguments.height {
    for col in 0..arguments.width {
      let i = (row * arguments.width + col) * 3;

      if row < arguments.height / 2 {
        b[i + 0] = a[i + 0];
        b[i + 1] = a[i + 1];
        b[i + 2] = a[i + 2];
      } else {
        b[i + 0] = !a[i + 0];
        b[i + 1] = !a[i + 1];
        b[i + 2] = !a[i + 2];
      }
    }
  }
}

fn main() -> Result<()> {
  let arguments = Arguments::from_args();

  let a = vec![0; arguments.width * arguments.height * 3];
  let mut b = vec![0; arguments.width * arguments.height * 3];

  filter(&arguments, &a, &mut b);

  let img: image::RgbImage =
    image::ImageBuffer::from_raw(arguments.width as u32, arguments.height as u32, b).unwrap();

  img.save("output.png").unwrap();

  Ok(())
}
