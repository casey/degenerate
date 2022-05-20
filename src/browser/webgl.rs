use super::*;

static VERTEX: &str = indoc! {"
  #version 300 es

  in vec4 position;
  out vec2 uv;

  void main() {
    uv = position.xy * 0.5 + 0.5;
    gl_Position = position;
  }
"};

static FRAGMENT: &str = indoc! {"
  #version 300 es

  precision highp float;

  in vec2 uv;
  out vec4 color;

  uniform sampler2D source;

  #define I texture(source, uv)

  uniform bool invert;

  vec4 operation() {
    if (invert)
      return vec4(1.0 - I.rgb, 1.0);
    return I;
  }

  uniform bool circle;
  uniform bool x;

  bool is_masked() {
    if (circle)
      return length((uv - 0.5) * 2.0) < 1.0;
    if (x)
      return min(abs((1.0 - uv.x) - uv.y), abs(uv.x - uv.y)) < 0.125;
    return true;
  }

  void main() {
    color = is_masked() ? operation() : vec4(I.xyz, 1.0);
  }
"};

#[derive(Clone)]
struct ShaderDescription {
  code: String,
  shader_type: u32,
}

pub(crate) struct WebGl {
  context: WebGl2RenderingContext,
  frame_buffer: WebGlFramebuffer,
  length: usize,
  program: WebGlProgram,
  source: AtomicUsize,
  textures: Vec<WebGlTexture>,
}

impl WebGl {
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

    let program = Self::create_program(
      &context,
      vec![
        ShaderDescription {
          code: VERTEX.into(),
          shader_type: WebGl2RenderingContext::VERTEX_SHADER,
        },
        ShaderDescription {
          code: FRAGMENT.into(),
          shader_type: WebGl2RenderingContext::FRAGMENT_SHADER,
        },
      ],
    )?;

    let length = Self::setup_triangles(&context, &program)?;

    let textures = (0..2)
      .map(|_| Self::create_texture(&context))
      .collect::<Result<Vec<WebGlTexture>, _>>()?;

    let frame_buffer = context
      .create_framebuffer()
      .ok_or("Failed to create frame buffer")?;

    Ok(Self {
      context,
      frame_buffer,
      length,
      program,
      source: AtomicUsize::new(0),
      textures,
    })
  }

  pub(crate) fn render_to_canvas(&self, state: &Computer) -> Result {
    self
      .context
      .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    self.context.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source.load(Ordering::Relaxed)]),
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
      self.length.try_into()?,
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
      Some(&self.textures[self.source.load(Ordering::Relaxed) ^ 1]),
      0,
    );

    self.context.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source.load(Ordering::Relaxed)]),
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
      self.length.try_into()?,
    );

    self
      .source
      .store(self.source.load(Ordering::Relaxed) ^ 1, Ordering::Relaxed);

    Ok(())
  }

  fn create_program(
    gl: &WebGl2RenderingContext,
    descriptions: Vec<ShaderDescription>,
  ) -> Result<WebGlProgram> {
    let program = gl.create_program().ok_or("Failed to create program")?;

    descriptions
      .iter()
      .map(|desc| Self::create_shader(gl, desc.clone()).unwrap())
      .for_each(|shader| gl.attach_shader(&program, &shader));

    gl.link_program(&program);

    if !gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS) {
      return Err(
        gl.get_program_info_log(&program)
          .ok_or("Failed to get program log info")?
          .into(),
      );
    }

    gl.use_program(Some(&program));

    Ok(program)
  }

  fn create_shader(
    gl: &WebGl2RenderingContext,
    description: ShaderDescription,
  ) -> Result<WebGlShader> {
    let shader = gl
      .create_shader(description.shader_type)
      .ok_or("Failed to create shader")?;

    gl.shader_source(&shader, description.code.trim());
    gl.compile_shader(&shader);

    if !gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS) {
      return Err(
        gl.get_shader_info_log(&shader)
          .ok_or("Failed to get shader info log")?
          .into(),
      );
    }

    Ok(shader)
  }

  fn create_texture(gl: &WebGl2RenderingContext) -> Result<WebGlTexture> {
    let canvas = gl
      .canvas()
      .ok_or("Failed to get canvas off of WebGL context")?
      .cast::<HtmlCanvasElement>()?;

    let texture = gl.create_texture().ok_or("Failed to create texture")?;

    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_canvas_element(
      WebGl2RenderingContext::TEXTURE_2D,
      0,
      WebGl2RenderingContext::RGBA.try_into()?,
      canvas.width().try_into()?,
      canvas.height().try_into()?,
      0,
      WebGl2RenderingContext::RGBA,
      WebGl2RenderingContext::UNSIGNED_BYTE,
      &canvas,
    )
    .map_err(JsValueError)?;

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

  fn setup_triangles(gl: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<usize> {
    let vertex_data = vec![
      vec![-1.0, -1.0, 0.0],
      vec![1.0, -1.0, 0.0],
      vec![1.0, 1.0, 0.0],
      vec![1.0, 1.0, 0.0],
      vec![-1.0, 1.0, 0.0],
      vec![-1.0, -1.0, 0.0],
    ]
    .iter()
    .flatten()
    .cloned()
    .collect::<Vec<f32>>();

    let vertices = js_sys::Float32Array::new_with_length(vertex_data.len().try_into()?);
    vertices.copy_from(&vertex_data);

    let position = gl.get_attrib_location(program, "position");

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
      3,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      0,
    );

    Ok(vertex_data.len() / 3)
  }
}
