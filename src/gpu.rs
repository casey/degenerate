use super::*;

pub(crate) struct Gpu {
  canvas: HtmlCanvasElement,
  frame_buffer: WebGlFramebuffer,
  gl: WebGl2RenderingContext,
  height: u32,
  resolution: u32,
  source_texture: Cell<usize>,
  textures: [WebGlTexture; 2],
  uniforms: BTreeMap<String, WebGlUniformLocation>,
  width: u32,
  window: Window,
}

impl Gpu {
  pub(super) fn new(canvas: &HtmlCanvasElement, window: &Window) -> Result<Self> {
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

    let textures = [
      Self::create_texture(&gl, resolution)?,
      Self::create_texture(&gl, resolution)?,
    ];

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
      .collect();

    Ok(Self {
      canvas: canvas.clone(),
      frame_buffer,
      gl,
      height,
      resolution,
      source_texture: Cell::new(0),
      textures,
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
      Some(&self.textures[self.source_texture.get()]),
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

  pub(crate) fn render(&mut self, state: &State) -> Result {
    log::trace!("Applying state {:?}", state);

    self.gl.bind_framebuffer(
      WebGl2RenderingContext::FRAMEBUFFER,
      Some(&self.frame_buffer),
    );

    self.gl.framebuffer_texture_2d(
      WebGl2RenderingContext::FRAMEBUFFER,
      WebGl2RenderingContext::COLOR_ATTACHMENT0,
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source_texture.get() ^ 1]),
      0,
    );

    self.gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source_texture.get()]),
    );

    self.gl.uniform1f(Some(self.uniform("alpha")), state.alpha);

    self.gl.uniform3f(
      Some(self.uniform("default_color")),
      state.default_color[0],
      state.default_color[1],
      state.default_color[2],
    );

    self
      .gl
      .uniform1ui(Some(self.uniform("divisor")), state.mask_mod_divisor);

    self
      .gl
      .uniform1ui(Some(self.uniform("remainder")), state.mask_mod_remainder);

    self
      .gl
      .uniform1ui(Some(self.uniform("nrows")), state.mask_rows_rows);

    self
      .gl
      .uniform1ui(Some(self.uniform("step")), state.mask_rows_step);

    let axis_vector = match state.operation_rotate_color_axis.as_ref() {
      "red" => Ok(Vector3::x()),
      "green" => Ok(Vector3::y()),
      "blue" => Ok(Vector3::z()),
      _ => Err("Invalid color rotation axis"),
    }?;

    self.gl.uniform_matrix3fv_with_f32_array(
      Some(self.uniform("color_rotation")),
      false,
      Rotation3::new(axis_vector * state.operation_rotate_color_turns * f32::consts::TAU)
        .matrix()
        .as_slice(),
    );

    let mut similarity = Similarity2::<f32>::identity();
    similarity.append_rotation_mut(&UnitComplex::from_angle(-state.rotation * f32::consts::TAU));
    similarity.append_scaling_mut(state.scale);

    self.gl.uniform_matrix3fv_with_f32_array(
      Some(self.uniform("transform")),
      false,
      similarity.inverse().to_homogeneous().as_slice(),
    );

    self
      .gl
      .uniform1ui(Some(self.uniform("wrap")), state.wrap as u32);

    self.gl.uniform1ui(Some(self.uniform("mask")), state.mask);

    self
      .gl
      .uniform1ui(Some(self.uniform("operation")), state.operation);

    self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

    self.source_texture.set(self.source_texture.get() ^ 1);

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

  pub(crate) fn resize(&mut self) -> Result {
    let css_pixel_height: f64 = self.canvas.client_height().try_into()?;
    let css_pixel_width: f64 = self.canvas.client_width().try_into()?;

    let device_pixel_ratio = self.window.device_pixel_ratio();
    let device_pixel_height = (css_pixel_height * device_pixel_ratio).ceil() as u32;
    let device_pixel_width = (css_pixel_width * device_pixel_ratio).ceil() as u32;

    if self.canvas.height() == device_pixel_height && self.canvas.width() == device_pixel_width {
      return Ok(());
    }

    self.canvas.set_height(device_pixel_height);
    self.canvas.set_width(device_pixel_width);

    self.width = self.canvas.width();
    self.height = self.canvas.height();
    self.resolution = self.width.max(self.height);

    self
      .gl
      .uniform1f(Some(self.uniform("resolution")), self.resolution as f32);

    self
      .gl
      .viewport(0, 0, self.resolution as i32, self.resolution as i32);

    self.gl.delete_texture(Some(&self.textures[0]));
    self.gl.delete_texture(Some(&self.textures[1]));

    self.textures = [
      Self::create_texture(&self.gl, self.resolution)?,
      Self::create_texture(&self.gl, self.resolution)?,
    ];

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
}
