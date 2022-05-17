use {super::*, std::sync::Once};

mod browser;
mod native;

macro_rules! image_test {
  (
    name: $name:ident,
    program: $program:literal $(,)?
  ) => {
    mod $name {
      use super::*;

      #[test]
      fn native() -> Result {
        native::test(stringify!($name), $program)
      }

      #[tokio::test]
      #[ignore]
      async fn browser() -> Result {
        browser::test(stringify!($name), $program).await
      }
    }
  };
  (
    name: $name:ident,
    program: $program:literal,
    browser: true $(,)?
  ) => {
    mod $name {
      use super::*;

      #[test]
      fn native() -> Result {
        native::test(stringify!($name), $program)
      }

      #[tokio::test]
      async fn browser() -> Result {
        browser::test(stringify!($name), $program).await
      }
    }
  };
}

fn clean() {
  static CLEAN: Once = Once::new();

  CLEAN.call_once(|| {
    for result in fs::read_dir("images").unwrap() {
      let entry = result.unwrap();
      let path = entry.path();
      let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

      if file_name.ends_with(".native-actual-memory.png")
        || file_name.ends_with(".browser-actual-memory.png")
      {
        fs::remove_file(path).unwrap();
      }
    }
  });
}

fn set_label_red(_path: &str) -> Result {
  #[cfg(target_os = "macos")]
  {
    let status = Command::new("xattr")
      .args(["-wx", "com.apple.FinderInfo"])
      .arg("0000000000000000000C00000000000000000000000000000000000000000000")
      .arg(_path)
      .status()?;

    if !status.success() {
      panic!("xattr failed: {}", status);
    }
  }

  Ok(())
}

image_test! {
  name: all,
  program: "all apply save",
  browser: true,
}

image_test! {
  name: alpha,
  program: "alpha:0.5 x apply save",
  browser: true,
}

image_test! {
  name: apply,
  program: "apply save",
  browser: true,
}

image_test! {
  name: autosave,
  program: "autosave square apply load:0.png x apply load:1.png save",
}

image_test! {
  name: brilliance,
  program: "x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save",
  browser: true,
}

image_test! {
  name: carpet,
  program: "circle scale:0.5 for:8 apply wrap loop save",
  browser: true,
}

image_test! {
  name: circle,
  program: "circle apply save",
  browser: true,
}

image_test! {
  name: circle_scale,
  program: "scale:0.5 circle apply all scale:0.9 wrap apply save",
  browser: true,
}

image_test! {
  name: concentric_circles,
  program: "scale:0.99 circle for:100 apply loop save",
  browser: true,
}

image_test! {
  name: cross,
  program: "cross apply save",
  browser: true,
}

image_test! {
  name: default,
  program: "comment:foo save",
  browser: true,
}

image_test! {
  name: diamonds,
  program: "rotate:0.3333 rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save",
  browser: true,
}

image_test! {
  name: grain,
  program: "rotate:0.111 for:16 square apply circle apply loop save",
  browser: true,
}

image_test! {
  name: kaleidoscope,
  program: "rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save",
  browser: true,
}

image_test! {
  name: mod_3,
  program: "mod:3:0 apply save",
  browser: true,
}

image_test! {
  name: orbs,
  program: "rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save",
  browser: true,
}

image_test! {
  name: pattern,
  program: "alpha:0.75 circle scale:0.5 for:8 apply wrap loop save",
  browser: true,
}

image_test! {
  name: random_default_seed,
  program: "random:all:circle:cross:square:top:x apply save",
  browser: true,
}

image_test! {
  name: random_zero_seed,
  program: "random:all:circle:cross:square:top:x  apply save",
  browser: true,
}

image_test! {
  name: random_nonzero_seed,
  program: "seed:2 random:all:circle:cross:square:top:x apply save",
  browser: true,
}

image_test! {
  name: read,
  program: "read save",
}

image_test! {
  name: resize_256_x_load,
  program: "autosave resize:256 x apply load:0.png save",
}

image_test! {
  name: resize_512,
  program: "resize:512 save",
}

image_test! {
  name: resize_default_pixel,
  program: "resize:3 default:0:255:0 scale:0.5 apply save",
}

image_test! {
  name: resize_rectangular,
  program: "resize:512:256 save",
}

image_test! {
  name: resize_starts_from_corner,
  program: "default:0:255:0 resize:512 save",
}

image_test! {
  name: rotate,
  program: "rotate:0.05 x apply save",
  browser: true,
}

image_test! {
  name: rotate_0125_square,
  program: "rotate:0.125 square apply save",
  browser: true,
}

image_test! {
  name: rotate_1_square,
  program: "rotate:1.0 square apply save",
  browser: true,
}

image_test! {
  name: rotate_color_05_red,
  program: "rotate-color:red:0.5 all apply save",
  browser: true,
}

image_test! {
  name: rotate_color_blue_05_all,
  program: "rotate-color:blue:0.5 all apply save",
  browser: true,
}

image_test! {
  name: rotate_color_blue_1_all,
  program: "rotate-color:blue:1.0 all apply save",
  browser: true,
}

image_test! {
  name: rotate_color_blue_all,
  program: "rotate-color:b:0.5 all apply save",
  browser: true,
}

image_test! {
  name: rotate_color_g,
  program: "rotate-color:g:0.5 all apply save",
  browser: true,
}

image_test! {
  name: rotate_color_green,
  program: "rotate-color:green:0.5 all apply save",
  browser: true,
}

image_test! {
  name: rotate_color_green_all,
  program: "rotate-color:green:1.0 all save",
  browser: true,
}

image_test! {
  name: rotate_color_r,
  program: "rotate-color:r:0.5 all apply save",
  browser: true,
}

image_test! {
  name: rotate_color_red_all,
  program: "rotate-color:red:1.0 all save",
  browser: true,
}

image_test! {
  name: rotate_nonsquare_aspect_ratio,
  program: "resize:512:256 rotate:0.05 x apply save load save",
}

image_test! {
  name: rotate_scale_x,
  program: "rotate:0.05 scale:2 x apply save",
  browser: true,
}

image_test! {
  name: rotate_square,
  program: "rotate:0.05 square for:2 apply loop save",
  browser: true,
}

image_test! {
  name: rotate_square_for_x,
  program: "rotate:0.05 square for:2 apply loop x for:1 apply loop save",
  browser: true,
}

image_test! {
  name: rows,
  program: "rows:1:1 apply save",
  browser: true,
}

image_test! {
  name: rows_overflow,
  program: "rows:18446744073709551615:18446744073709551615 apply save",
}

image_test! {
  name: rug,
  program: "rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save",
  browser: true,
}

image_test! {
  name: save,
  program: "save",
  browser: true,
}

image_test! {
  name: save_and_load,
  program: "resize:512:256 all apply save all apply load",
}

image_test! {
  name: scale,
  program: "scale:0.5 circle apply save",
  browser: true,
}

image_test! {
  name: scale_circle_for,
  program: "circle scale:0.5 for:8 apply loop save",
  browser: true,
}

image_test! {
  name: scale_circle_wrap,
  program: "scale:0.5 circle wrap apply save",
  browser: true,
}

image_test! {
  name: scale_rotate,
  program: "scale:2 rotate:0.05 x apply save",
  browser: true,
}

image_test! {
  name: scale_x,
  program: "scale:2 x apply save",
  browser: true,
}

image_test! {
  name: smear,
  program: "seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random:all:circle:cross:square:top:x apply loop rotate-color:b:0.01 rotate:0.01 for:100 random:all:circle:cross:square:top:x apply loop save",
}

image_test! {
  name: square,
  program: "square apply save",
  browser: true,
}

image_test! {
  name: square_top,
  program: "square apply top apply save",
  browser: true,
}

image_test! {
  name: starburst,
  program: "seed:8 rotate-color:g:0.1 rotate:0.1 for:10 random:all:circle:cross:square:top:x apply loop rotate-color:b:0.1 rotate:0.1 for:10 random:all:circle:cross:square:top:x apply loop save",
  browser: true,
}

image_test! {
  name: top,
  program: "top apply save",
  browser: true,
}

image_test! {
  name: viewport_fill_square,
  program: "fill x apply save",
  browser: true,
}

image_test! {
  name: viewport_fill_landscape,
  program: "resize:512:256 fill x apply save",
}

image_test! {
  name: viewport_fill_portrait,
  program: "resize:256:512 fill x apply save",
}

image_test! {
  name: viewport_fit_square,
  program: "fit x apply save",
  browser: true,
}

image_test! {
  name: viewport_fit_landscape,
  program: "resize:512:256 fit x apply save",
}

image_test! {
  name: viewport_fit_portrait,
  program: "resize:256:512 fit x apply save",
}

image_test! {
  name: viewport_override,
  program: "resize:512:256 fit stretch x apply save",
}

image_test! {
  name: viewport_stretch_square,
  program: "stretch x apply save",
  browser: true,
}

image_test! {
  name: viewport_stretch_landscape,
  program: "resize:512:256 stretch x apply save",
}

image_test! {
  name: viewport_stretch_portrait,
  program: "resize:256:512 stretch x apply save",
}

image_test! {
  name: x,
  program: "x apply save",
  browser: true,
}

image_test! {
  name: x_cross_load,
  program: "autosave x apply cross apply load:0.png save",
}

image_test! {
  name: x_loop,
  program: "x scale:0.5 for:8 apply wrap loop save",
  browser: true,
}

image_test! {
  name: x_scale,
  program: "x scale:0.5 for:8 apply loop save",
  browser: true,
}

image_test! {
  name: x_wrap,
  program: "x apply scale:0.5 wrap identity all apply save",
  browser: true,
}

image_test! {
  name: debug_operation,
  program: "debug apply save",
  browser: true,
}

image_test! {
  name: debug_operation_landscape,
  program: "resize:512:256 debug apply save",
}

image_test! {
  name: double_apply_fill_landscape,
  program: "resize:512:256 fill x apply apply save",
}

image_test! {
  name: double_apply_fit_landscape,
  program: "resize:512:256 fit x apply apply save",
}

image_test! {
  name: double_apply_stretch_landscape,
  program: "resize:512:256 stretch x apply apply save",
}

image_test! {
  name: double_apply_fill_portrait,
  program: "resize:256:512 fill x apply apply save",
}

image_test! {
  name: double_apply_fit_portrait,
  program: "resize:256:512 fit x apply apply save",
}

image_test! {
  name: double_apply_stretch_portrait,
  program: "resize:256:512 stretch x apply apply save",
}

image_test! {
  name: double_apply_fill_square,
  program: "fill x apply apply save",
  browser: true,
}

image_test! {
  name: double_apply_fit_square,
  program: "fit x apply apply save",
  browser: true,
}

image_test! {
  name: double_apply_stretch_square,
  program: "stretch x apply apply save",
  browser: true,
}

image_test! {
  name: double_apply_with_scale,
  program: "resize:512:256 scale:0.5 x apply apply save",
}

image_test! {
  name: mod_zero_is_always_false,
  program: "mod:0:1 apply save",
  browser: true,
}

image_test! {
  name: square_colors,
  program: "rotate:0.01 rotate-color:g:0.1 square for:10 apply loop save",
  browser: true,
}
