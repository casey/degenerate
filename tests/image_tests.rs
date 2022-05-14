use {super::*, std::sync::Once};

macro_rules! image_test {
  (name: $name:ident, program: $program:literal $(,)?) => {
    #[test]
    fn $name() -> Result {
      image_test(stringify!($name), $program)
    }
  };

  (name: $name:ident, program: $program:literal, ignore: true $(,)?) => {
    #[test]
    #[ignore]
    fn $name() -> Result {
      image_test(stringify!($name), $program)
    }
  };
}

fn image_test(name: &str, program: &str) -> Result {
  static CLEAN: Once = Once::new();

  CLEAN.call_once(|| {
    for result in fs::read_dir("images").unwrap() {
      let entry = result.unwrap();
      let path = entry.path();
      if path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .ends_with(".actual-memory.png")
      {
        fs::remove_file(path).unwrap();
      }
    }
  });

  let destination = format!("images/{}.actual-memory.png", name);

  fs::remove_file(&destination).ok();

  let tempdir = Test::new()?.program(program).run_and_return_tempdir()?;

  let actual_path = tempdir.path().join("memory.png");

  let actual_image = image::open(&actual_path)?;

  let expected_path = format!("images/{}.png", name);

  if !Path::new(&expected_path).is_file() || actual_image != image::open(&expected_path)? {
    fs::rename(&actual_path, &destination)?;

    #[cfg(target_os = "macos")]
    {
      let status = Command::new("xattr")
        .args(["-wx", "com.apple.FinderInfo"])
        .arg("0000000000000000000C00000000000000000000000000000000000000000000")
        .arg(&destination)
        .status()?;

      if !status.success() {
        panic!("xattr failed: {}", status);
      }
    }

    panic!(
      "Image test failed:\nExpected: {}\nActual:   {}",
      expected_path, destination,
    );
  }

  Ok(())
}

image_test! {
  name: all,
  program: "all apply save",
}

image_test! {
  name: alpha,
  program: "alpha:0.5 x apply save",
}

image_test! {
  name: apply,
  program: "apply save",
}

image_test! {
  name: autosave,
  program: "autosave square apply load:0.png x apply load:1.png save",
}

image_test! {
  name: brilliance,
  program: "comment:slow x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save",
  ignore: true,
}

image_test! {
  name: carpet,
  program: "comment:slow circle scale:0.5 for:8 apply wrap loop save",
  ignore: true,
}

image_test! {
  name: circle,
  program: "circle apply save",
}

image_test! {
  name: circle_scale,
  program: "scale:0.5 circle apply all scale:0.9 wrap apply save",
}

image_test! {
  name: concentric_circles,
  program: "comment:slow scale:0.99 circle for:100 apply loop save",
  ignore: true,
}

image_test! {
  name: cross,
  program: "cross apply save",
}

image_test! {
  name: default,
  program: "comment:foo save",
}

image_test! {
  name: diamonds,
  program: "comment:slow rotate:0.3333 rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save",
  ignore: true,
}

image_test! {
  name: grain,
  program: "comment:slow rotate:0.111 for:16 square apply circle apply loop save",
  ignore: true,
}

image_test! {
  name: kaleidoscope,
  program: "comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save",
  ignore: true,
}

image_test! {
  name: mod_3,
  program: "mod:3:0 apply save",
}

image_test! {
  name: orbs,
  program: "comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save",
  ignore: true,
}

image_test! {
  name: pattern,
  program: "comment:slow alpha:0.75 circle scale:0.5 for:8 apply wrap loop save",
  ignore: true,
}

image_test! {
  name: random_mask,
  program: "random-mask apply save",
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
}

image_test! {
  name: rotate_0125_square,
  program: "rotate:0.125 square apply save",
}

image_test! {
  name: rotate_1_square,
  program: "rotate:1.0 square apply save",
}

image_test! {
  name: rotate_color_05_red,
  program: "rotate-color:red:0.5 all apply save",
}

image_test! {
  name: rotate_color_blue_05_all,
  program: "rotate-color:blue:0.5 all apply save",
}

image_test! {
  name: rotate_color_blue_1_all,
  program: "rotate-color:blue:1.0 all apply save",
}

image_test! {
  name: rotate_color_blue_all,
  program: "rotate-color:b:0.5 all apply save",
}

image_test! {
  name: rotate_color_g,
  program: "rotate-color:g:0.5 all apply save",
}

image_test! {
  name: rotate_color_green,
  program: "rotate-color:green:0.5 all apply save",
}

image_test! {
  name: rotate_color_green_all,
  program: "rotate-color:green:1.0 all save",
}

image_test! {
  name: rotate_color_r,
  program: "rotate-color:r:0.5 all apply save",
}

image_test! {
  name: rotate_color_red_all,
  program: "rotate-color:red:1.0 all save",
}

image_test! {
  name: rotate_nonsquare_aspect_ratio,
  program: "resize:512:256 rotate:0.05 x apply save load save",
}

image_test! {
  name: rotate_scale_x,
  program: "rotate:0.05 scale:2 x apply save",
}

image_test! {
  name: rotate_square,
  program: "rotate:0.05 square for:2 apply loop save",
}

image_test! {
  name: rotate_square_for_x,
  program: "rotate:0.05 square for:2 apply loop x for:1 apply loop save",
}

image_test! {
  name: rows,
  program: "rows:1:1 apply save",
}

image_test! {
  name: rows_overflow,
  program: "rows:18446744073709551615:18446744073709551615 apply save",
}

image_test! {
  name: rug,
  program: "comment:slow rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save",
  ignore: true,
}

image_test! {
  name: save,
  program: "save",
}

image_test! {
  name: save_and_load,
  program: "resize:512:256 all apply save all apply load",
}

image_test! {
  name: scale,
  program: "scale:0.5 circle apply save",
}

image_test! {
  name: scale_circle_for,
  program: "comment:slow circle scale:0.5 for:8 apply loop save",
  ignore: true,
}

image_test! {
  name: scale_circle_wrap,
  program: "scale:0.5 circle wrap apply save",
}

image_test! {
  name: scale_rotate,
  program: "scale:2 rotate:0.05 x apply save",
}

image_test! {
  name: scale_x,
  program: "scale:2 x apply save",
}

image_test! {
  name: seed_random_mask,
  program: "seed:2 random-mask apply save",
}

image_test! {
  name: smear,
  program: "comment:slow seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-mask apply loop rotate-color:b:0.01 rotate:0.01 for:100 random-mask apply loop save",
  ignore: true,
}

image_test! {
  name: square,
  program: "square apply save",
}

image_test! {
  name: square_top,
  program: "square apply top apply save",
}

image_test! {
  name: starburst,
  program: "comment:slow seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-mask apply loop rotate-color:b:0.1 rotate:0.1 for:10 random-mask apply loop save",
  ignore: true,
}

image_test! {
  name: top,
  program: "top apply save",
}

image_test! {
  name: viewport_fill_square,
  program: "fill x apply save",
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
}

image_test! {
  name: x_cross_load,
  program: "autosave x apply cross apply load:0.png save",
}

image_test! {
  name: x_loop,
  program: "comment:slow x scale:0.5 for:8 apply wrap loop save",
  ignore: true,
}

image_test! {
  name: x_scale,
  program: "comment:slow x scale:0.5 for:8 apply loop save",
  ignore: true,
}

image_test! {
  name: x_wrap,
  program: "x apply scale:0.5 wrap identity all apply save",
}

image_test! {
  name: debug_operation,
  program: "debug apply save",
}

image_test! {
  name: debug_operation_landscape,
  program: "resize:512:256 debug apply save",
}

image_test! {
  name: double_x_apply_bug_fill_landscape,
  program: "resize:512:256 fill x apply apply save",
}

image_test! {
  name: double_x_apply_bug_fit_landscape,
  program: "resize:512:256 fit x apply apply save",
}

image_test! {
  name: double_x_apply_bug_stretch_landscape,
  program: "resize:512:256 stretch x apply apply save",
}

image_test! {
  name: double_x_apply_bug_fill_portrait,
  program: "resize:256:512 fill x apply apply save",
}

image_test! {
  name: double_x_apply_bug_fit_portrait,
  program: "resize:256:512 fit x apply apply save",
}

image_test! {
  name: double_x_apply_bug_stretch_portrait,
  program: "resize:256:512 stretch x apply apply save",
}

image_test! {
  name: double_x_apply_bug_fill_square,
  program: "fill x apply apply save",
}

image_test! {
  name: double_x_apply_bug_fit_square,
  program: "fit x apply apply save",
}

image_test! {
  name: double_x_apply_bug_stretch_square,
  program: "stretch x apply apply save",
}
