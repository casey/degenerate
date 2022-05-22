use super::*;

pub(crate) struct Gpu {
  canvas: HtmlCanvasElement,
  frame_buffer: WebGlFramebuffer,
  gl: WebGl2RenderingContext,
  mask: WebGlUniformLocation,
  operation: WebGlUniformLocation,
  resolution: WebGlUniformLocation,
  source_texture: Cell<usize>,
  textures: [WebGlTexture; 2],
}

impl Gpu {
  pub(super) fn new(canvas: &HtmlCanvasElement) -> Result<Self> {
    let gl = canvas
      .get_context("webgl2")
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

    let mask = gl
      .get_uniform_location(&program, "mask")
      .ok_or("Could not find uniform `mask`")?;

    let operation = gl
      .get_uniform_location(&program, "operation")
      .ok_or("Could not find uniform `operation`")?;

    let resolution = gl
      .get_uniform_location(&program, "resolution")
      .ok_or("Could not find uniform `resolution`")?;

    let textures = [
      Self::create_texture(&gl, canvas.width().try_into()?)?,
      Self::create_texture(&gl, canvas.width().try_into()?)?,
    ];

    let frame_buffer = gl
      .create_framebuffer()
      .ok_or("Failed to create framebuffer")?;

    Ok(Self {
      canvas: canvas.clone(),
      frame_buffer,
      gl,
      mask,
      operation,
      resolution,
      source_texture: Cell::new(0),
      textures,
    })
  }

  pub(crate) fn render_to_canvas(&self) -> Result {
    self
      .gl
      .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    self.gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source_texture.get()]),
    );

    self
      .gl
      .uniform1ui(Some(&self.mask), Self::mask_uniform(&Mask::All));

    self.gl.uniform1ui(
      Some(&self.operation),
      Self::operation_uniform(&Operation::Identity),
    );

    self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

    Ok(())
  }

  pub(crate) fn render_to_texture(&self, computer: &Computer) -> Result {
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
      .uniform1ui(Some(&self.mask), Self::mask_uniform(computer.mask()));

    self.gl.uniform1ui(
      Some(&self.operation),
      Self::operation_uniform(computer.operation()),
    );

    self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

    self.source_texture.set(self.source_texture.get() ^ 1);

    Ok(())
  }

  fn create_texture(gl: &WebGl2RenderingContext, size: u32) -> Result<WebGlTexture> {
    let texture = gl.create_texture().ok_or("Failed to create texture")?;

    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

    gl.tex_storage_2d(
      WebGl2RenderingContext::TEXTURE_2D,
      1,
      WebGl2RenderingContext::RGBA8,
      size as i32,
      size as i32,
    );

    Ok(texture)
  }

  pub(crate) fn resize(&mut self) -> Result {
    let width = self.canvas.width();
    let height = self.canvas.height();
    let size = width.max(height);

    self.gl.uniform1ui(Some(&self.resolution), size);

    self.gl.viewport(0, 0, size as i32, size as i32);

    self.gl.delete_texture(Some(&self.textures[0]));
    self.gl.delete_texture(Some(&self.textures[1]));

    self.textures = [
      Self::create_texture(&self.gl, size)?,
      Self::create_texture(&self.gl, size)?,
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
