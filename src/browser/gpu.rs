use super::*;

pub(crate) struct Gpu {
  canvas: HtmlCanvasElement,
  frame_buffer: WebGlFramebuffer,
  gl: WebGl2RenderingContext,
  mask: WebGlUniformLocation,
  operation: WebGlUniformLocation,
  source: Cell<usize>,
  textures: [WebGlTexture; 2],
}

impl Gpu {
  const VERTICES: [f32; 12] = [
    -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
  ];

  pub(super) fn new(canvas: &HtmlCanvasElement) -> Result<Self> {
    let gl = canvas
      .get_context("webgl2")
      .map_err(JsValueError)?
      .ok_or("Failed to retrieve gl")?
      .cast::<WebGl2RenderingContext>()?;

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

    let vertices = Float32Array::new_with_length(Self::VERTICES.len().try_into()?);
    vertices.copy_from(&Self::VERTICES);

    let position = gl.get_attrib_location(&program, "position");

    gl.bind_buffer(
      WebGl2RenderingContext::ARRAY_BUFFER,
      gl.create_buffer().as_ref(),
    );

    gl.buffer_data_with_opt_array_buffer(
      WebGl2RenderingContext::ARRAY_BUFFER,
      Some(&vertices.buffer()),
      WebGl2RenderingContext::STATIC_DRAW,
    );

    gl.enable_vertex_attrib_array(position.try_into()?);

    gl.vertex_attrib_pointer_with_i32(
      position.try_into()?,
      2,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      0,
    );

    let textures = [
      Self::create_texture(&gl, canvas.width().try_into()?)?,
      Self::create_texture(&gl, canvas.width().try_into()?)?,
    ];

    let frame_buffer = gl
      .create_framebuffer()
      .ok_or("Failed to create frame buffer")?;

    Ok(Self {
      canvas: canvas.clone(),
      frame_buffer,
      gl,
      mask,
      operation,
      source: Cell::new(0),
      textures,
    })
  }

  pub(crate) fn render_to_canvas(&self) -> Result {
    self
      .gl
      .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    self.gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source.get()]),
    );

    self
      .gl
      .uniform1ui(Some(&self.mask), Self::mask_uniform(&Mask::All));

    self.gl.uniform1ui(
      Some(&self.operation),
      Self::operation_uniform(&Operation::Identity),
    );

    self.gl.draw_arrays(
      WebGl2RenderingContext::TRIANGLES,
      0,
      (Self::VERTICES.len() / 2).try_into()?,
    );

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
      Some(&self.textures[self.source.get() ^ 1]),
      0,
    );

    self.gl.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source.get()]),
    );

    self
      .gl
      .uniform1ui(Some(&self.mask), Self::mask_uniform(computer.mask()));

    self.gl.uniform1ui(
      Some(&self.operation),
      Self::operation_uniform(computer.operation()),
    );

    self.gl.draw_arrays(
      WebGl2RenderingContext::TRIANGLES,
      0,
      (Self::VERTICES.len() / 2).try_into()?,
    );

    self.source.set(self.source.get() ^ 1);

    Ok(())
  }

  fn create_texture(gl: &WebGl2RenderingContext, size: usize) -> Result<WebGlTexture> {
    let texture = gl.create_texture().ok_or("Failed to create texture")?;

    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

    gl.tex_storage_2d(
      WebGl2RenderingContext::TEXTURE_2D,
      1,
      WebGl2RenderingContext::RGBA8,
      size.try_into()?,
      size.try_into()?,
    );

    gl.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MIN_FILTER,
      WebGl2RenderingContext::LINEAR.try_into()?,
    );

    gl.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_WRAP_S,
      WebGl2RenderingContext::CLAMP_TO_EDGE.try_into()?,
    );

    gl.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_WRAP_T,
      WebGl2RenderingContext::CLAMP_TO_EDGE.try_into()?,
    );

    Ok(texture)
  }

  pub(crate) fn resize(&mut self, size: usize) -> Result {
    self.gl.viewport(
      (self.canvas.width() as i32 - size as i32) / 2,
      (self.canvas.height() as i32 - size as i32) / 2,
      self.canvas.width().try_into()?,
      self.canvas.height().try_into()?,
    );

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
