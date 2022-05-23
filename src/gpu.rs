use super::*;

pub(crate) struct Gpu {
  canvas: HtmlCanvasElement,
  frame_buffer: WebGlFramebuffer,
  gl: WebGl2RenderingContext,
  mask_uniform: WebGlUniformLocation,
  operation_uniform: WebGlUniformLocation,
  resolution_uniform: WebGlUniformLocation,
  source_texture: Cell<usize>,
  textures: [WebGlTexture; 2],
  width: u32,
  height: u32,
  resolution: u32,
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

      gl.shader_source(&vertex, include_str!("vertex.glsl").trim());
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

      gl.shader_source(&fragment, include_str!("fragment.glsl").trim());
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

    let mask_uniform = gl
      .get_uniform_location(&program, "mask")
      .ok_or("Could not find uniform `mask`")?;

    let operation_uniform = gl
      .get_uniform_location(&program, "operation")
      .ok_or("Could not find uniform `operation`")?;

    let resolution_uniform = gl
      .get_uniform_location(&program, "resolution")
      .ok_or("Could not find uniform `resolution`")?;

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

    Ok(Self {
      canvas: canvas.clone(),
      frame_buffer,
      gl,
      mask_uniform,
      operation_uniform,
      resolution_uniform,
      source_texture: Cell::new(0),
      textures,
      width,
      height,
      resolution,
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

    self.gl.uniform1ui(
      Some(&self.mask_uniform),
      Self::mask_uniform(computer.mask()),
    );

    self.gl.uniform1ui(
      Some(&self.operation_uniform),
      Self::operation_uniform(computer.operation()),
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
      .uniform1ui(Some(&self.resolution_uniform), self.resolution);

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

  fn operation_uniform(operation: &Operation) -> u32 {
    match operation {
      Operation::Identity => 0,
      Operation::Invert => 1,
      _ => panic!("Invalid operation"),
    }
  }

  fn mask_uniform(mask: &Mask) -> u32 {
    match mask {
      Mask::X => 0,
      Mask::Circle => 1,
      Mask::All => 2,
      _ => panic!("Invalid mask"),
    }
  }
}