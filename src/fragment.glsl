#version 300 es

// INSERT_GENERATED_CODE_HERE

precision highp float;

uniform bool wrap;
uniform float alpha;
uniform float resolution;
uniform mat3 color_rotation;
uniform mat3 transform;
uniform sampler2D audio_time_domain;
uniform sampler2D source;
uniform uint divisor;
uniform uint mask;
uniform uint nrows;
uniform uint operation;
uniform uint remainder;
uniform uint step;
uniform vec3 default_color;

out vec4 output_color;

vec3 apply(vec2 position, vec3 color) {
  switch (operation) {
    case DEBUG:
      return floor(vec3((position.x + 1.0) / 2.0, 0.0, 1.0 - (position.y + 1.0) / 2.0) * 16.0) / 16.0;
    case IDENTITY:
      return color;
    case INVERT:
      return 1.0 - color;
    case ROTATE_COLOR:
      return (color_rotation * (color * 2.0 - 1.0) + 1.0) / 2.0;
    case WAVEFORM:
      // TODO:
      // - how can this be invertable? Would xor if it was binary data.
      // - could use audio intenisity as alpha value
      return texture(audio_time_domain, vec2((position.x + 1.0) / 2.0, 0.5)).rrr;
    default:
      return vec3(0.0, 1.0, 0.0);
  }
}

bool masked(vec2 position, uvec2 pixel_position) {
  switch (mask) {
    case ALL:
      return true;
    case CIRCLE:
      return length(position) < 1.0;
    case CROSS:
      return abs(position.x) < 0.25 || abs(position.y) < 0.25;
    case MOD:
      if (divisor == 0u) {
        return false;
      } else {
        return (pixel_position.y * uint(resolution) + pixel_position.x) % divisor == remainder;
      }
    case ROWS:
      return pixel_position.y % (nrows + step) < nrows;
    case SQUARE:
      return abs(position.x) < 0.5 && abs(position.y) < 0.5;
    case TOP:
      return position.y > 0.0;
    case X:
      return abs(abs(position.x) - abs(position.y)) < 0.25;
    default:
      return false;
  }
}

void main() {
  // Get fragment coordinates and transform to [-1, 1]
  vec2 position = gl_FragCoord.xy / resolution * 2.0 - 1.0;

  // Transform position by transform matrix
  vec2 transformed = (transform * vec3(position, 1.0)).xy;

  // Wrap transformed position to be within [-1, 1] if enabled
  vec2 wrapped = wrap
    ? mod(transformed + 1.0, 2.0) - 1.0
    : transformed;

  // Sample color if in-bounds, otherwise use default color
  vec3 input_color = wrapped.x >= -1.0 && wrapped.x <= 1.0 && wrapped.y >= -1.0 && wrapped.y <= 1.0
    ? texture(source, (wrapped + 1.0) / 2.0).rgb
    : default_color;

  // Sample original color
  vec3 original_color = texture(source, gl_FragCoord.xy / resolution).rgb;

  // Calculate position in pixel coordinates, [0, resolution)
  uvec2 pixel_position = uvec2((wrapped + 1.0) / 2.0 * resolution);

  // If within mask…
  vec3 output_color_rgb = masked(wrapped, pixel_position)
    // …set output color to alpha blended output of operation…
    ? apply(wrapped, input_color) * alpha + original_color * (1.0 - alpha)
    // …otherwise use original color
    : original_color;

  // Extend output color with opaque alpha channel
  output_color = vec4(output_color_rgb, 1.0);
}
