use super::*;

pub(crate) struct Gpu {
  analyser_node: AnalyserNode,
  audio_frequency_array: Float32Array,
  audio_frequency_data: Vec<f32>,
  audio_frequency_texture: WebGlTexture,
  audio_time_domain_array: Float32Array,
  audio_time_domain_data: Vec<f32>,
  audio_time_domain_texture: WebGlTexture,
  canvas: HtmlCanvasElement,
  decibels_max: f32,
  decibels_min: f32,
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
      .alpha(true)
      .antialias(false)
      .depth(false)
      .stencil(false);

    if js_sys::eval("window.preserveDrawingBuffer")?.as_bool() == Some(true) {
      context_options.preserve_drawing_buffer(true);
    }

    let gl = canvas
      .get_context_with_context_options("webgl2", &context_options.into())?
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
    gl.uniform1i(Some(uniforms.get("audio_frequency").unwrap()), 2);

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

    let audio_frequency_texture = gl
      .create_texture()
      .ok_or("Failed to create audio_frequency texture")?;

    gl.active_texture(WebGl2RenderingContext::TEXTURE2);
    gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&audio_frequency_texture),
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

    let frequency_bin_count = analyser_node.frequency_bin_count();

    gl.tex_storage_2d(
      WebGl2RenderingContext::TEXTURE_2D,
      1,
      WebGl2RenderingContext::R32F,
      frequency_bin_count as i32,
      1,
    );

    Ok(Self {
      source: Self::create_texture(&gl, resolution)?,
      destination: Self::create_texture(&gl, resolution)?,
      analyser_node: analyser_node.clone(),
      audio_time_domain_array: Float32Array::new_with_length(fft_size),
      audio_time_domain_data: vec![0.0; fft_size as usize],
      audio_time_domain_texture,
      audio_frequency_array: Float32Array::new_with_length(frequency_bin_count),
      audio_frequency_data: vec![0.0; frequency_bin_count as usize],
      audio_frequency_texture,
      canvas: canvas.clone(),
      decibels_min: -100.0,
      decibels_max: -30.0,
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

    self
      .analyser_node
      .get_float_time_domain_data(&mut self.audio_time_domain_data);
    self
      .audio_time_domain_array
      .copy_from(&self.audio_time_domain_data);
    self.gl.active_texture(WebGl2RenderingContext::TEXTURE1);
    self.gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.audio_time_domain_texture),
    );
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
      )?;

    self
      .analyser_node
      .get_float_frequency_data(&mut self.audio_frequency_data);

    let scale_factor = 1.0 / (self.decibels_max - self.decibels_min);

    for bucket in &mut self.audio_frequency_data {
      *bucket = ((*bucket - self.decibels_min) * scale_factor).clamp(0.0, 1.0);
    }

    self
      .audio_frequency_array
      .copy_from(&self.audio_frequency_data);
    self.gl.active_texture(WebGl2RenderingContext::TEXTURE2);
    self.gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.audio_frequency_texture),
    );
    self
      .gl
      .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        0,
        0,
        self.audio_frequency_data.len() as i32,
        1,
        WebGl2RenderingContext::RED,
        WebGl2RenderingContext::FLOAT,
        Some(&self.audio_frequency_array),
      )?;

    self.gl.uniform1f(Some(self.uniform("alpha")), filter.alpha);

    self.gl.uniform3f(
      Some(self.uniform("default_color")),
      filter.default_color[0],
      filter.default_color[1],
      filter.default_color[2],
    );

    self.gl.uniform_matrix4fv_with_f32_array(
      Some(self.uniform("color_transform")),
      false,
      &filter.color_transform.as_slice(),
    );

    self.gl.uniform_matrix3fv_with_f32_array(
      Some(self.uniform("coordinate_transform")),
      false,
      &filter.coordinate_transform.as_slice(),
    );

    self
      .gl
      .uniform1ui(Some(self.uniform("wrap")), filter.wrap as u32);

    self.gl.uniform1i(
      Some(self.uniform("field")),
      match filter.field {
        Field::All => 0,
        Field::Check => 1,
        Field::Circle => 2,
        Field::Cross => 3,
        Field::Equalizer => 4,
        Field::Frequency => 5,
        Field::Mod { divisor, remainder } => {
          self
            .gl
            .uniform1ui(Some(self.uniform("mod_divisor")), divisor);
          self
            .gl
            .uniform1ui(Some(self.uniform("mod_remainder")), remainder);
          6
        }
        Field::Rows { on, off } => {
          self.gl.uniform1ui(Some(self.uniform("rows_on")), on);
          self.gl.uniform1ui(Some(self.uniform("rows_off")), off);
          7
        }
        Field::Square => 8,
        Field::TimeDomain => 9,
        Field::Top => 10,
        Field::Wave => 11,
        Field::X => 12,
      },
    );

    self
      .gl
      .uniform1ui(Some(self.uniform("coordinates")), filter.coordinates as u32);

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
    self.gl.read_pixels_with_opt_u8_array(
      0,
      0,
      self.resolution as i32,
      self.resolution as i32,
      WebGl2RenderingContext::RGBA,
      WebGl2RenderingContext::UNSIGNED_BYTE,
      Some(&mut array),
    )?;

    let image = ImageBuffer::from_raw(self.resolution, self.resolution, array)
      .ok_or("Failed to create ImageBuffer")?;

    Ok(image)
  }

  pub(crate) fn set_decibel_range(&mut self, min: f32, max: f32) {
    self.decibels_min = min;
    self.decibels_max = max;
  }

  fn uniform(&self, name: &str) -> &WebGlUniformLocation {
    self
      .uniforms
      .get(name)
      .ok_or_else(|| format!("Uniform `{name}` is missing.",))
      .unwrap()
  }

  pub(crate) fn clear(&mut self) -> Result {
    self
      .gl
      .clear_bufferfv_with_f32_array(WebGl2RenderingContext::COLOR, 0, &[0.0, 0.0, 0.0, 1.0]);

    self.gl.delete_texture(Some(&self.source));
    self.gl.delete_texture(Some(&self.destination));

    self.source = Self::create_texture(&self.gl, self.resolution)?;
    self.destination = Self::create_texture(&self.gl, self.resolution)?;

    Ok(())
  }
}
