use {super::*, Mask::*};

pub(crate) struct Gpu {
  canvas: HtmlCanvasElement,
  frame_buffer: WebGlFramebuffer,
  gl: WebGl2RenderingContext,
  height: u32,
  resolution: u32,
  source_texture: Cell<usize>,
  textures: [WebGlTexture; 2],
  width: u32,
  uniforms: BTreeMap<String, WebGlUniformLocation>,
}

impl Gpu {
  pub(super) fn new(canvas: &HtmlCanvasElement) -> Result<Self> {
    let mut context_options = WebGlContextAttributes::new();

    context_options
      .alpha(false)
      .antialias(false)
      .depth(false)
      .stencil(false);

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

      let mut constants = String::new();

      for (index, mask) in Mask::VARIANTS.iter().enumerate() {
        constants.push_str(&format!("const uint {} = {}u;\n", mask, index));
      }

      for (index, operation) in Operation::VARIANTS.iter().enumerate() {
        constants.push_str(&format!("const uint {} = {}u;\n", operation, index));
      }

      gl.shader_source(
        &fragment,
        &include_str!("fragment.glsl").replace("// INSERT_GENERATED_CODE_HERE", &constants),
      );
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

    let uniforms = (0..)
      .map_while(|i| gl.get_active_uniform(&program, i))
      .map(|info| {
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
    })
  }

  pub(crate) fn render_to_canvas(&self) -> Result {
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

  pub(crate) fn apply(&self, computer: &Computer) -> Result {
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

    self
      .gl
      .uniform1f(Some(self.uniform("alpha")), computer.alpha() as f32);

    self.gl.uniform3f(
      Some(self.uniform("default_color")),
      computer.default().x as f32 / 255.0,
      computer.default().y as f32 / 255.0,
      computer.default().z as f32 / 255.0,
    );

    match computer.mask() {
      Mod { divisor, remainder } => {
        self.gl.uniform1ui(Some(self.uniform("divisor")), *divisor);
        self
          .gl
          .uniform1ui(Some(self.uniform("remainder")), *remainder);
      }
      Rows { nrows, step } => {
        self.gl.uniform1ui(Some(self.uniform("nrows")), *nrows);
        self.gl.uniform1ui(Some(self.uniform("step")), *step);
      }
      _ => {}
    }

    if let Operation::RotateColor(axis, turns) = computer.operation() {
      self.gl.uniform_matrix3fv_with_f32_array(
        Some(self.uniform("color_rotation")),
        false,
        Rotation3::new(axis.vector() * *turns * f64::consts::TAU)
          .matrix()
          .map(|element| element as f32)
          .as_slice(),
      );
    }

    self.gl.uniform_matrix3fv_with_f32_array(
      Some(self.uniform("transform")),
      false,
      computer
        .transform()
        .inverse()
        .to_homogeneous()
        .map(|element| element as f32)
        .as_slice(),
    );

    self
      .gl
      .uniform1ui(Some(self.uniform("wrap")), computer.wrap() as u32);

    self.gl.uniform1ui(
      Some(self.uniform("mask")),
      Mask::VARIANTS
        .iter()
        .position(|mask| *mask == computer.mask().as_ref())
        .expect("Mask should always be present")
        .try_into()?,
    );

    self.gl.uniform1ui(
      Some(self.uniform("operation")),
      Operation::VARIANTS
        .iter()
        .position(|operation| *operation == computer.operation().as_ref())
        .expect("Operation should always be present")
        .try_into()?,
    );

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

    Ok(())
  }

  fn uniform(&self, name: &str) -> &WebGlUniformLocation {
    self.uniforms.get(name).unwrap()
  }
}
