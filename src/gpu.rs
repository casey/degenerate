use super::*;

// TODO:
// - should frequency texture be a-weighted?
// - make all masks audio reactive
// - make alpha audio reactive
// - make color transform and coordinate transform audio reactive
// - make default color audio reactive
// - add equalizer mask - bar up or down (from bottom or middle?) depending on frequency bucket intensity
// - add frequency mask - bars on or off depending on frequency bucket intensity
// - add slider for overriding audio level

// let db = self
//   .audio_frequency_data
//   .iter()
//   .enumerate()
//   .map(|(i, decibels)| {
//     let f = (i as f32 / self.audio_frequency_data.len() as f32)
//       * self.audio_context.sample_rate()
//       / 2.0;
//     let f2 = f * f;
//     let weight = 1.2588966 * 148840000.0 * f2 * f2
//       / ((f2 + 424.36) * (f2 + 11599.29) * (f2 + 544496.41)).sqrt()
//       * (f2 + 148840000.0);
//     weight * decibels
//   })
//   .sum::<f32>();

// self.gl.uniform1f(Some(self.uniform("db")), db);

pub(crate) struct Gpu {
  analyser_node: AnalyserNode,
  audio_time_domain_array: Float32Array,
  audio_time_domain_data: Vec<f32>,
  audio_time_domain_texture: WebGlTexture,
  canvas: HtmlCanvasElement,
  destination: WebGlTexture,
  frame_buffer: WebGlFramebuffer,
  gl: WebGl2RenderingContext,
  height: u32,
  lock_resolution: bool,
  resolution: u32,
  source: WebGlTexture,
  uniforms: BTreeMap<String, WebGlUniformLocation>,
  width: u32,
  window: Window,
}

impl Gpu {
  pub(super) fn new(
    window: &Window,
    canvas: &HtmlCanvasElement,
    analyser_node: &AnalyserNode,
  ) -> Result<Self> {
    let mut context_options = WebGlContextAttributes::new();

    context_options
      .alpha(false)
      .antialias(false)
      .depth(false)
      .stencil(false);

    if js_sys::eval("window.preserveDrawingBuffer")
      .map_err(JsValueError)?
      .as_bool()
      == Some(true)
    {
      context_options.preserve_drawing_buffer(true);
    }

    let gl = canvas
      .get_context_with_context_options("webgl2", &context_options.into())
      .map_err(JsValueError)?
      .ok_or("Failed to retrieve webgl2 context")?
      .cast::<WebGl2RenderingContext>()?;

    gl.enable(WebGl2RenderingContext::CULL_FACE);

    let program = {
      let program = gl.create_program().ok_or("Failed to create program")?;

      let vertex = gl
        .create_shader(WebGl2RenderingContext::VERTEX_SHADER)
        .ok_or("Failed to create shader")?;

      gl.shader_source(&vertex, include_str!("vertex.glsl"));
      gl.compile_shader(&vertex);

      if !gl.get_shader_parameter(&vertex, WebGl2RenderingContext::COMPILE_STATUS) {
        return Err(
          gl.get_shader_info_log(&vertex)
            .ok_or("Failed to get shader info log")?
            .into(),
        );
      }

      let fragment = gl
        .create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)
        .ok_or("Failed to create shader")?;

      gl.shader_source(&fragment, include_str!("fragment.glsl"));
      gl.compile_shader(&fragment);

      if !gl.get_shader_parameter(&fragment, WebGl2RenderingContext::COMPILE_STATUS) {
        return Err(
          gl.get_shader_info_log(&fragment)
            .ok_or("Failed to get shader info log")?
            .into(),
        );
      }

      gl.attach_shader(&program, &vertex);
      gl.attach_shader(&program, &fragment);

      gl.link_program(&program);

      if !gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS) {
        return Err(
          gl.get_program_info_log(&program)
            .ok_or("Failed to get program log info")?
            .into(),
        );
      }

      gl.use_program(Some(&program));

      program
    };

    let width = canvas.width();
    let height = canvas.height();
    let resolution = width.max(height);

    let frame_buffer = gl
      .create_framebuffer()
      .ok_or("Failed to create framebuffer")?;

    let uniform_count = gl
      .get_program_parameter(&program, WebGl2RenderingContext::ACTIVE_UNIFORMS)
      .cast::<js_sys::Number>()?
      .value_of() as u32;

    let uniforms = (0..uniform_count)
      .map(|i| {
        let info = gl.get_active_uniform(&program, i).unwrap();
        let name = info.name();
        let location = gl.get_uniform_location(&program, &name).unwrap();
        (name, location)
      })
      .collect::<BTreeMap<String, WebGlUniformLocation>>();

    gl.uniform1i(Some(uniforms.get("source").unwrap()), 0);
    gl.uniform1i(Some(uniforms.get("audio_time_domain").unwrap()), 1);

    let audio_time_domain_texture = gl
      .create_texture()
      .ok_or("Failed to create audio_time_domain texture")?;

    gl.active_texture(WebGl2RenderingContext::TEXTURE1);
    gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&audio_time_domain_texture),
    );

    gl.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MIN_FILTER,
      WebGl2RenderingContext::NEAREST.try_into()?,
    );

    gl.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MAG_FILTER,
      WebGl2RenderingContext::NEAREST.try_into()?,
    );

    let fft_size = analyser_node.fft_size();

    gl.tex_storage_2d(
      WebGl2RenderingContext::TEXTURE_2D,
      1,
      WebGl2RenderingContext::R32F,
      fft_size as i32,
      1,
    );

    Ok(Self {
      source: Self::create_texture(&gl, resolution)?,
      destination: Self::create_texture(&gl, resolution)?,
      analyser_node: analyser_node.clone(),
      audio_time_domain_array: Float32Array::new_with_length(fft_size),
      audio_time_domain_data: vec![0.0; fft_size as usize],
      audio_time_domain_texture,
      canvas: canvas.clone(),
      frame_buffer,
      gl,
      height,
      lock_resolution: false,
      resolution,
      uniforms,
      width,
      window: window.clone(),
    })
  }

  pub(crate) fn present(&self) -> Result {
    self.gl.bind_framebuffer(
      WebGl2RenderingContext::READ_FRAMEBUFFER,
      Some(&self.frame_buffer),
    );

    self
      .gl
      .bind_framebuffer(WebGl2RenderingContext::DRAW_FRAMEBUFFER, None);

    self.gl.framebuffer_texture_2d(
      WebGl2RenderingContext::READ_FRAMEBUFFER,
      WebGl2RenderingContext::COLOR_ATTACHMENT0,
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.source),
      0,
    );

    let width = self.width as i32;
    let height = self.height as i32;
    let resolution = self.resolution as i32;

    let dx = (resolution - width) / 2;
    let dy = (resolution - height) / 2;

    self.gl.blit_framebuffer(
      dx,
      dy,
      width + dx,
      height + dy,
      0,
      0,
      width,
      height,
      WebGl2RenderingContext::COLOR_BUFFER_BIT,
      WebGl2RenderingContext::NEAREST,
    );

    Ok(())
  }

  pub(crate) fn render(&mut self, filter: &Filter) -> Result {
    self.resize()?;

    self.gl.bind_framebuffer(
      WebGl2RenderingContext::FRAMEBUFFER,
      Some(&self.frame_buffer),
    );

    self.gl.framebuffer_texture_2d(
      WebGl2RenderingContext::FRAMEBUFFER,
      WebGl2RenderingContext::COLOR_ATTACHMENT0,
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.destination),
      0,
    );

    self.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
    self
      .gl
      .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.source));

    self.gl.active_texture(WebGl2RenderingContext::TEXTURE1);
    self.gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.audio_time_domain_texture),
    );
    self
      .analyser_node
      .get_float_time_domain_data(&mut self.audio_time_domain_data);
    self
      .audio_time_domain_array
      .copy_from(&self.audio_time_domain_data);
    self
      .gl
      .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        0,
        0,
        self.audio_time_domain_data.len() as i32,
        1,
        WebGl2RenderingContext::RED,
        WebGl2RenderingContext::FLOAT,
        Some(&self.audio_time_domain_array),
      )
      .map_err(JsValueError)?;

    self.gl.uniform1f(Some(self.uniform("alpha")), filter.alpha);

    self.gl.uniform3f(
      Some(self.uniform("default_color")),
      filter.default_color[0],
      filter.default_color[1],
      filter.default_color[2],
    );

    self
      .gl
      .uniform1ui(Some(self.uniform("divisor")), filter.mask_mod_divisor);

    self
      .gl
      .uniform1ui(Some(self.uniform("remainder")), filter.mask_mod_remainder);

    self
      .gl
      .uniform1ui(Some(self.uniform("nrows")), filter.mask_rows_rows);

    self
      .gl
      .uniform1ui(Some(self.uniform("step")), filter.mask_rows_step);

    let mut similarity = Similarity2::<f32>::identity();
    similarity.append_rotation_mut(&UnitComplex::from_angle(-filter.rotation));
    if filter.scale != 0.0 {
      similarity.append_scaling_mut(filter.scale);
    }

    self.gl.uniform_matrix4fv_with_f32_array(
      Some(self.uniform("color_transform")),
      false,
      &filter.color_transform,
    );

    self.gl.uniform_matrix3fv_with_f32_array(
      Some(self.uniform("coordinate_transform")),
      false,
      similarity.inverse().to_homogeneous().as_slice(),
    );

    self
      .gl
      .uniform1ui(Some(self.uniform("wrap")), filter.wrap as u32);

    self.gl.uniform1ui(Some(self.uniform("mask")), filter.mask);

    self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

    mem::swap(&mut self.source, &mut self.destination);

    Ok(())
  }

  fn create_texture(gl: &WebGl2RenderingContext, resolution: u32) -> Result<WebGlTexture> {
    let texture = gl.create_texture().ok_or("Failed to create texture")?;

    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

    gl.tex_storage_2d(
      WebGl2RenderingContext::TEXTURE_2D,
      1,
      WebGl2RenderingContext::RGBA8,
      resolution as i32,
      resolution as i32,
    );

    gl.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MIN_FILTER,
      WebGl2RenderingContext::NEAREST.try_into()?,
    );

    gl.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MAG_FILTER,
      WebGl2RenderingContext::NEAREST.try_into()?,
    );

    Ok(texture)
  }

  pub(crate) fn lock_resolution(&mut self, resolution: u32) {
    self.width = resolution;
    self.height = resolution;
    self.resolution = resolution;
    self.lock_resolution = true;
  }

  pub(crate) fn resize(&mut self) -> Result {
    if self.lock_resolution {
      if self.canvas.height() == self.height && self.canvas.width() == self.width {
        return Ok(());
      }
    } else {
      let css_pixel_height: f64 = self.canvas.client_height().try_into()?;
      let css_pixel_width: f64 = self.canvas.client_width().try_into()?;

      let device_pixel_ratio = self.window.device_pixel_ratio();
      let device_pixel_height = (css_pixel_height * device_pixel_ratio).ceil() as u32;
      let device_pixel_width = (css_pixel_width * device_pixel_ratio).ceil() as u32;

      if self.canvas.height() == device_pixel_height && self.canvas.width() == device_pixel_width {
        return Ok(());
      }

      self.width = device_pixel_width;
      self.height = device_pixel_height;
      self.resolution = self.width.max(self.height);
    }

    self.canvas.set_height(self.height);
    self.canvas.set_width(self.width);

    self
      .gl
      .uniform1f(Some(self.uniform("resolution")), self.resolution as f32);

    self
      .gl
      .viewport(0, 0, self.resolution as i32, self.resolution as i32);

    self.clear()?;

    self.present()?;

    Ok(())
  }

  pub(crate) fn save_image(&self) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    self.gl.bind_framebuffer(
      WebGl2RenderingContext::FRAMEBUFFER,
      Some(&self.frame_buffer),
    );

    let mut array = vec![0; (self.resolution * self.resolution * 4) as usize];
    self
      .gl
      .read_pixels_with_opt_u8_array(
        0,
        0,
        self.resolution as i32,
        self.resolution as i32,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        Some(&mut array),
      )
      .map_err(JsValueError)?;

    let image = ImageBuffer::from_raw(self.resolution, self.resolution, array)
      .ok_or("Failed to create ImageBuffer")?;

    Ok(image)
  }

  fn uniform(&self, name: &str) -> &WebGlUniformLocation {
    self
      .uniforms
      .get(name)
      .ok_or_else(|| format!("Uniform `{name}` is missing.",))
      .unwrap()
  }

  pub(crate) fn clear(&mut self) -> Result {
    self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    self.gl.delete_texture(Some(&self.source));
    self.gl.delete_texture(Some(&self.destination));

    self.source = Self::create_texture(&self.gl, self.resolution)?;
    self.destination = Self::create_texture(&self.gl, self.resolution)?;

    Ok(())
  }
}
