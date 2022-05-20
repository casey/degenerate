use super::*;

pub(crate) struct Gpu {
  canvas: HtmlCanvasElement,
  context: WebGl2RenderingContext,
  frame_buffer: WebGlFramebuffer,
  program: WebGlProgram,
  source: Cell<usize>,
  textures: [WebGlTexture; 2],
}

impl Gpu {
  const VERTICES: [f32; 12] = [
    -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
  ];

  pub(super) fn new(canvas: &HtmlCanvasElement) -> Result<Self> {
    let context = canvas
      .get_context("webgl2")
      .map_err(JsValueError)?
      .ok_or("Failed to retrieve context")?
      .cast::<WebGl2RenderingContext>()?;

    context.viewport(
      0,
      0,
      canvas.width().try_into()?,
      canvas.height().try_into()?,
    );

    let program = {
      let program = context.create_program().ok_or("Failed to create program")?;

      let vertex = context
        .create_shader(WebGl2RenderingContext::VERTEX_SHADER)
        .ok_or("Failed to create shader")?;

      context.shader_source(&vertex, include_str!("vertex.glsl").trim());
      context.compile_shader(&vertex);

      if !context.get_shader_parameter(&vertex, WebGl2RenderingContext::COMPILE_STATUS) {
        return Err(
          context
            .get_shader_info_log(&vertex)
            .ok_or("Failed to get shader info log")?
            .into(),
        );
      }

      let fragment = context
        .create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)
        .ok_or("Failed to create shader")?;

      context.shader_source(&fragment, include_str!("fragment.glsl").trim());
      context.compile_shader(&fragment);

      if !context.get_shader_parameter(&fragment, WebGl2RenderingContext::COMPILE_STATUS) {
        return Err(
          context
            .get_shader_info_log(&fragment)
            .ok_or("Failed to get shader info log")?
            .into(),
        );
      }

      context.attach_shader(&program, &vertex);
      context.attach_shader(&program, &fragment);

      context.link_program(&program);

      if !context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS) {
        return Err(
          context
            .get_program_info_log(&program)
            .ok_or("Failed to get program log info")?
            .into(),
        );
      }

      context.use_program(Some(&program));

      program
    };

    let vertices = Float32Array::new_with_length(Self::VERTICES.len().try_into()?);
    vertices.copy_from(&Self::VERTICES);

    let position = context.get_attrib_location(&program, "position");

    context.bind_buffer(
      WebGl2RenderingContext::ARRAY_BUFFER,
      context.create_buffer().as_ref(),
    );

    context.buffer_data_with_opt_array_buffer(
      WebGl2RenderingContext::ARRAY_BUFFER,
      Some(&vertices.buffer()),
      WebGl2RenderingContext::STATIC_DRAW,
    );

    context.enable_vertex_attrib_array(position.try_into()?);

    context.vertex_attrib_pointer_with_i32(
      position.try_into()?,
      2,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      0,
    );

    let textures = [
      Self::create_texture(&context, canvas.width().try_into()?)?,
      Self::create_texture(&context, canvas.width().try_into()?)?,
    ];

    let frame_buffer = context
      .create_framebuffer()
      .ok_or("Failed to create frame buffer")?;

    Ok(Self {
      canvas: canvas.clone(),
      context,
      frame_buffer,
      program,
      source: Cell::new(0),
      textures,
    })
  }

  pub(crate) fn render_to_canvas(&self, state: &Computer) -> Result {
    self
      .context
      .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    self.context.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source.get()]),
    );

    self.context.uniform1i(
      self
        .context
        .get_uniform_location(&self.program, &state.mask().to_string())
        .as_ref(),
      0,
    );

    self.context.uniform1i(
      self
        .context
        .get_uniform_location(&self.program, &state.operation().to_string())
        .as_ref(),
      0,
    );

    self.context.draw_arrays(
      WebGl2RenderingContext::TRIANGLES,
      0,
      (Self::VERTICES.len() / 2).try_into()?,
    );

    Ok(())
  }

  pub(crate) fn render_to_texture(&self, state: &Computer) -> Result {
    self.context.bind_framebuffer(
      WebGl2RenderingContext::FRAMEBUFFER,
      Some(&self.frame_buffer),
    );

    self.context.framebuffer_texture_2d(
      WebGl2RenderingContext::FRAMEBUFFER,
      WebGl2RenderingContext::COLOR_ATTACHMENT0,
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source.get() ^ 1]),
      0,
    );

    self.context.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source.get()]),
    );

    self.context.uniform1i(
      self
        .context
        .get_uniform_location(&self.program, &state.mask().to_string())
        .as_ref(),
      1,
    );

    self.context.uniform1i(
      self
        .context
        .get_uniform_location(&self.program, &state.operation().to_string())
        .as_ref(),
      1,
    );

    self.context.draw_arrays(
      WebGl2RenderingContext::TRIANGLES,
      0,
      (Self::VERTICES.len() / 2).try_into()?,
    );

    self.source.set(self.source.get() ^ 1);

    Ok(())
  }

  fn create_texture(context: &WebGl2RenderingContext, size: usize) -> Result<WebGlTexture> {
    let texture = context.create_texture().ok_or("Failed to create texture")?;

    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

    context.tex_storage_2d(
      WebGl2RenderingContext::TEXTURE_2D,
      1,
      WebGl2RenderingContext::RGBA8,
      size.try_into()?,
      size.try_into()?,
    );

    context.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MIN_FILTER,
      WebGl2RenderingContext::LINEAR.try_into()?,
    );

    context.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_WRAP_S,
      WebGl2RenderingContext::CLAMP_TO_EDGE.try_into()?,
    );

    context.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_WRAP_T,
      WebGl2RenderingContext::CLAMP_TO_EDGE.try_into()?,
    );

    Ok(texture)
  }

  pub(crate) fn resize(&mut self, size: usize) -> Result {
    self.context.viewport(
      (self.canvas.width() as i32 - size as i32) / 2,
      (self.canvas.height() as i32 - size as i32) / 2,
      self.canvas.width().try_into()?,
      self.canvas.height().try_into()?,
    );

    self.context.delete_texture(Some(&self.textures[0]));
    self.context.delete_texture(Some(&self.textures[1]));

    self.textures = [
      Self::create_texture(&self.context, size)?,
      Self::create_texture(&self.context, size)?,
    ];

    Ok(())
  }
}
