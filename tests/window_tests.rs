use super::*;

#[test]
fn simple() -> Result {
  Test::new()?
    .program("window")
    .run_with_timeout(Duration::from_millis(250))
}
