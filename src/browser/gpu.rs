use super::*;

use indoc::indoc;

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

  uniform bool aberrate;
  uniform bool invert;
  uniform bool invert_b;
  uniform bool invert_g;
  uniform bool invert_r;
  uniform bool mirror_h;
  uniform bool mirror_v;
  uniform bool pixelate;
  uniform bool rotate;
  uniform bool spin;

  const mat3 rgb_to_yiq = mat3(
    0.299,     0.587,     0.114,
    0.595716, -0.274453, -0.321263,
    0.211456, -0.522591,  0.311135
  );

  const mat3 yiq_to_rgb = mat3(
    1.0,  0.9563,  0.6210,
    1.0, -0.2721, -0.6474,
    1.0, -1.1070,  1.7046
  );

  vec4 _rotate() {
    vec3 yiq = rgb_to_yiq * texture(source, uv).rgb;
    float hue = atan(yiq.b, yiq.g);
    float hue_ = hue + 15.0;
    float chroma = sqrt(yiq.b * yiq.b + yiq.g * yiq.g);
    vec3 yiq_ = vec3(yiq.r, chroma * cos(hue_), chroma * sin(hue_));
    return vec4(yiq_to_rgb * yiq_, 1.0);
  }

  vec2 barrel(vec2 coord, float amt) {
    vec2 cc = coord - 0.5;
    float dist = dot(cc, cc);
    return coord + cc * dist * amt;
  }

  float sat(float t) {
    return clamp(t, 0.0, 1.0);
  }

  float linterp(float t) {
    return sat(1.0 - abs(2.0 * t - 1.0));
  }

  float remap(float t, float a, float b) {
    return sat((t - a) / (b - a));
  }

  const float max_distort = 2.2;
  const int num_iter = 12;
  const float reci_num_iter_f = 1.0 / float(num_iter);

  vec3 spectrum_offset(float t) {
    vec3 ret;
    float lo = step(t, 0.5);
    float hi = 1.0 - lo;
    float w = linterp(remap(t, 1.0 / 6.0, 5.0 / 6.0));
    ret = vec3(lo, 1.0, hi) * vec3(1.0 - w, w, 1.0 - w);
    return pow(ret, vec3(1.0 / 2.2));
  }

  vec4 _aberrate() {
    vec3 sumcol = vec3(0.0);
    vec3 sumw = vec3(0.0);

    for (int i = 0; i < num_iter; ++i) {
      float t = float(i) * reci_num_iter_f;
      vec3 w = spectrum_offset(t);
      sumw += w;
      sumcol += w * texture(source, barrel(uv, max_distort * t)).rgb;
    }

    return vec4(sumcol.rgb / sumw, 1.0);
  }

  vec4 _spin() {
    mat2 m = mat2(cos(0.5), -sin(0.5), sin(0.5), cos(0.5));
    return texture(source, (uv - 0.5) * m + 0.5);
  }

  vec4 operation() {
    if (aberrate) return _aberrate();
    if (invert) return vec4(1.0 - I.rgb, 1.0);
    if (invert_b) return vec4(I.rg, 1.0 - I.b, 1.0);
    if (invert_g) return vec4(I.r, 1.0 - I.g, I.b, 1.0);
    if (invert_r) return vec4(1.0 - I.r, I.gb, 1.0);
    if (mirror_h) return texture(source, vec2(uv.x < 0.5 ? uv.x * 2.0 : 1.0 - (uv.x - 0.5) * 2.0, uv.y));
    if (mirror_v) return texture(source, vec2(uv.x, uv.y < 0.5 ? uv.y * 2.0 : 1.0 - (uv.y - 0.5) * 2.0));
    if (pixelate) return texture(source, round(uv * 10.0) * 1.0 / (10.0));
    if (rotate) return _rotate();
    if (spin) return _spin();
    return I;
  }

  uniform bool all;
  uniform bool bottom;
  uniform bool circle;
  uniform bool cross;
  uniform bool left;
  uniform bool right;
  uniform bool square;
  uniform bool top;
  uniform bool x;

  bool is_masked() {
    if (bottom) return uv.y > 0.5;
    if (circle) length((uv - 0.5) * 2.0) < 0.5;
    if (cross) return abs(uv.x - 0.5) < 0.1 || abs(uv.y - 0.5) < 0.1;
    if (left) return uv.x < 0.5;
    if (right) return uv.x > 0.5;
    if (square) return abs(uv.x - 0.5) < 0.25 && abs(uv.y - 0.5) < 0.25;
    if (top) return uv.y < 0.5;
    if (x) return min(abs((1.0 - uv.x) - uv.y), abs(uv.x - uv.y)) < 0.1;
    return true;
  }

  void main() {
    color = is_masked() ? operation() : vec4(I.xyz, 1.0);
  }
"};

const DEFAULT_MASK: &str = "all";
const DEFAULT_OPERATION: &str = "invert";

#[derive(Clone)]
struct ShaderDescription {
  code: String,
  shader_type: u32,
}

pub(crate) struct Gpu {
  animation_frame_callback: Option<Closure<dyn FnMut(f64)>>,
  animation_frame_pending: bool,
  canvas: HtmlCanvasElement,
  context: WebGl2RenderingContext,
  frame_buffer: WebGlFramebuffer,
  input: bool,
  length: usize,
  mask: WebGlUniformLocation,
  nav: HtmlElement,
  operation: WebGlUniformLocation,
  program: WebGlProgram,
  resize: bool,
  source: usize,
  stderr: Stderr,
  textarea: HtmlTextAreaElement,
  textures: Vec<WebGlTexture>,
  window: Window,
}

impl Gpu {
  pub(super) fn init() -> Result {
    let window = window();

    let document = window.get_document();

    let textarea = document.select("textarea")?.cast::<HtmlTextAreaElement>()?;

    let canvas = document.select("canvas")?.cast::<HtmlCanvasElement>()?;

    let nav = document.select("nav")?.cast::<HtmlElement>()?;

    let stderr = Stderr::get();

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

    let source = 0;

    let mut textures = Vec::new();
    for _ in 0..2 {
      textures.push(Self::create_texture(&context)?);
    }

    let mask = context
      .get_uniform_location(&program, "circle")
      .ok_or("Failed to get `mask` uniform location")?;

    let operation = context
      .get_uniform_location(&program, DEFAULT_OPERATION)
      .ok_or("Failed to get `operation` uniform location")?;

    let frame_buffer = context
      .create_framebuffer()
      .ok_or("Failed to create frame buffer")?;

    let app = Arc::new(Mutex::new(Self {
      animation_frame_callback: None,
      animation_frame_pending: false,
      canvas,
      context,
      frame_buffer,
      input: false,
      length,
      mask,
      nav,
      operation,
      program,
      resize: true,
      source,
      stderr: stderr.clone(),
      textarea: textarea.clone(),
      textures,
      window: window.clone(),
    }));

    let local = app.clone();
    app.lock().unwrap().animation_frame_callback = Some(Closure::wrap(Box::new(move |timestamp| {
      let mut app = local.lock().unwrap();
      let result = app.on_animation_frame(timestamp);
      app.stderr.update(result);
    })
      as Box<dyn FnMut(f64)>));

    let local = app.clone();
    window.add_event_listener("resize", move || {
      let mut app = local.lock().unwrap();
      let result = app.on_resize();
      app.stderr.update(result);
    })?;

    textarea.add_event_listener("input", move || {
      let mut app = app.lock().unwrap();
      let result = app.on_input();
      stderr.update(result);
    })?;

    Ok(())
  }

  fn render_to_canvas(&mut self) -> Result {
    self
      .context
      .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    self.context.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source]),
    );

    self.context.uniform1i(Some(&self.mask), 0);
    self.context.uniform1i(Some(&self.operation), 0);

    self.context.draw_arrays(
      WebGl2RenderingContext::TRIANGLES,
      0,
      self.length.try_into()?,
    );

    Ok(())
  }

  fn render_to_texture(&mut self) -> Result {
    self.context.bind_framebuffer(
      WebGl2RenderingContext::FRAMEBUFFER,
      Some(&self.frame_buffer),
    );

    self.context.framebuffer_texture_2d(
      WebGl2RenderingContext::FRAMEBUFFER,
      WebGl2RenderingContext::COLOR_ATTACHMENT0,
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source ^ 1]),
      0,
    );

    self.context.bind_texture(
      WebGl2RenderingContext::TEXTURE_2D,
      Some(&self.textures[self.source]),
    );

    self.context.uniform1i(Some(&self.mask), 1);
    self.context.uniform1i(Some(&self.operation), 1);

    self.context.draw_arrays(
      WebGl2RenderingContext::TRIANGLES,
      0,
      self.length.try_into()?,
    );

    self.source ^= 1;

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
      WebGl2RenderingContext::RGBA.try_into()?,
      WebGl2RenderingContext::UNSIGNED_BYTE,
      &canvas,
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

    Ok((vertex_data.len() / 3).try_into()?)
  }

  pub(super) fn on_resize(&mut self) -> Result {
    self.resize = true;
    self.request_animation_frame()?;
    Ok(())
  }

  pub(super) fn on_input(&mut self) -> Result {
    self.input = true;
    self.request_animation_frame()?;
    Ok(())
  }

  fn request_animation_frame(&mut self) -> Result {
    if self.animation_frame_pending {
      return Ok(());
    }

    self
      .window
      .request_animation_frame(
        self
          .animation_frame_callback
          .as_ref()
          .unwrap()
          .as_ref()
          .dyn_ref()
          .unwrap(),
      )
      .map_err(JsValueError)?;

    self.animation_frame_pending = true;

    Ok(())
  }

  fn on_animation_frame(&mut self, timestamp: f64) -> Result {
    self.animation_frame_pending = false;

    log::trace!("Animation frame timestamp {}s", timestamp);

    let resize = self.resize;

    if self.resize {
      let css_pixel_height: f64 = self.canvas.client_height().try_into()?;
      let css_pixel_width: f64 = self.canvas.client_width().try_into()?;
      let device_pixel_ratio = self.window.device_pixel_ratio();
      let device_pixel_height = css_pixel_height * device_pixel_ratio;
      let device_pixel_width = css_pixel_width * device_pixel_ratio;

      let height = if cfg!(debug_assertions) {
        device_pixel_height / 32.0
      } else {
        device_pixel_height
      };

      let width = if cfg!(debug_assertions) {
        device_pixel_width / 32.0
      } else {
        device_pixel_width
      };

      self.canvas.set_height(height.ceil() as u32);
      self.canvas.set_width(width.ceil() as u32);
      self.resize = false;
    }

    if self.input {
      self.nav.set_class_name("fade-out");

      let program = self
        .textarea
        .value()
        .split_whitespace()
        .into_iter()
        .map(Command::from_str)
        .collect::<Result<Vec<Command>>>()?;

      for command in program {
        match command {
          Command::Mask(mask) => {
            self.context.uniform1i(Some(&self.mask), 0);
            self.mask = self
              .context
              .get_uniform_location(&self.program, &mask.to_string())
              .ok_or("Failed to get mask location")?;
          }
          Command::Apply => self.render_to_texture()?,
          _ => {}
        }
      }

      self.render_to_canvas()?;
    }

    Ok(())
  }
}
