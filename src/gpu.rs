use {super::*, Mask::*};

pub(crate) struct Gpu {
  canvas: HtmlCanvasElement,
  frame_buffer: WebGlFramebuffer,
  gl: WebGl2RenderingContext,
  height: u32,
  resolution: u32,
  source_texture: Cell<usize>,
  textures: [WebGlTexture; 2],
  uniforms: Uniforms,
  width: u32,
}

struct Uniforms {
  divisor: WebGlUniformLocation,
  mask: WebGlUniformLocation,
  operation: WebGlUniformLocation,
  remainder: WebGlUniformLocation,
  resolution: WebGlUniformLocation,
  nrows: WebGlUniformLocation,
  step: WebGlUniformLocation,
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
        constants.push_str(&format!("const int {} = {};\n", mask, index));
      }

      for (index, operation) in Operation::VARIANTS.iter().enumerate() {
        constants.push_str(&format!("const int {} = {};\n", operation, index));
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

    let uniforms = Uniforms {
      mask: Self::get_uniform_location(&gl, &program, "mask"),
      operation: Self::get_uniform_location(&gl, &program, "operation"),
      resolution: Self::get_uniform_location(&gl, &program, "resolution"),
      divisor: Self::get_uniform_location(&gl, &program, "divisor"),
      remainder: Self::get_uniform_location(&gl, &program, "remainder"),
      nrows: Self::get_uniform_location(&gl, &program, "nrows"),
      step: Self::get_uniform_location(&gl, &program, "step"),
    };

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

    match computer.mask() {
      Mod { divisor, remainder } => {
        self
          .gl
          .uniform1i(Some(&self.uniforms.divisor), *divisor as i32);
        self
          .gl
          .uniform1i(Some(&self.uniforms.remainder), *remainder as i32);
      }
      Rows { nrows, step } => {
        self.gl.uniform1i(Some(&self.uniforms.nrows), *nrows as i32);
        self.gl.uniform1i(Some(&self.uniforms.step), *step as i32);
      }
      _ => {}
    }

    self.gl.uniform1i(
      Some(&self.uniforms.mask),
      Mask::VARIANTS
        .iter()
        .position(|mask| *mask == computer.mask().as_ref())
        .expect("Mask should always be present")
        .try_into()?,
    );

    self.gl.uniform1i(
      Some(&self.uniforms.operation),
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

    Ok(texture)
  }

  pub(crate) fn resize(&mut self) -> Result {
    self.width = self.canvas.width();
    self.height = self.canvas.height();
    self.resolution = self.width.max(self.height);

    self
      .gl
      .uniform1ui(Some(&self.uniforms.resolution), self.resolution);

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

  fn get_uniform_location(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    name: &str,
  ) -> WebGlUniformLocation {
    gl.get_uniform_location(program, name)
      .ok_or_else(|| format!("Could not find uniform `{}`", name))
      .unwrap()
  }
}