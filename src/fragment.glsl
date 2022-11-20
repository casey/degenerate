#version 300 es

precision highp float;

const int FIELD_ALL = 0;
const int FIELD_CHECK = 1;
const int FIELD_CIRCLE = 2;
const int FIELD_CROSS = 3;
const int FIELD_EQUALIZER = 4;
const int FIELD_FREQUENCY = 5;
const int FIELD_MOD = 6;
const int FIELD_ROWS = 7;
const int FIELD_SQUARE = 8;
const int FIELD_TIME_DOMAIN = 9;
const int FIELD_TOP = 10;
const int FIELD_WAVE = 11;
const int FIELD_X = 12;

uniform bool coordinates;
uniform bool wrap;
uniform float alpha;
uniform float resolution;
uniform int field;
uniform mat3 coordinate_transform;
uniform mat4 color_transform;
uniform sampler2D audio_frequency;
uniform sampler2D audio_time_domain;
uniform sampler2D source;
uniform uint divisor;
uniform uint nrows;
uniform uint remainder;
uniform uint step;
uniform vec3 default_color;

out vec4 output_color;

vec2 quadrant(vec2 position) {
  return (position + 1.0) / 2.0;
}

vec3 octant(vec3 position) {
  return (position + 1.0) / 2.0;
}

float audio_frequency_sample(vec2 position) {
  return texture(audio_frequency, quadrant(position)).r;
}

float audio_time_domain_sample(vec2 position) {
  return texture(audio_time_domain, quadrant(position)).r;
}

float field_all() {
  return -1.0;
}

float field_check(vec2 p) {
  ivec2 i = ivec2((p + 1.0) * 4.0);
  if (i.x % 2 != i.y % 2) {
    return -1.0;
  } else {
    return 1.0;
  }
}

float field_circle(vec2 p, float radius) {
  return length(p) - radius;
}

float field_cross(vec2 p, float size, float thickness, float radius) {
  vec2 b = vec2(size, thickness);
  p = abs(p);
  p = (p.y > p.x) ? p.yx : p.xy;
  vec2  q = p - b;
  float k = max(q.y, q.x);
  vec2  w = (k > 0.0) ? q : vec2(thickness - p.x, -k);
  return sign(k) * length(max(w, 0.0)) + radius;
}

float field_equalizer(vec2 p) {
  return quadrant(p).y - audio_frequency_sample(p);
}

float field_frequency(vec2 p, float threshold) {
  return threshold - audio_frequency_sample(p);
}

float field_mod(uvec2 px, uint divisor, uint remainder) {
  if (divisor == 0u) {
    return 1.0;
  } else if ((px.y * uint(resolution) + px.x) % divisor == remainder) {
    return -1.0;
  } else {
    return 1.0;
  }
}

float field_none() {
  return 1.0;
}

float field_rows(uvec2 p, uint nrows, uint step) {
  if (p.y % (nrows + step) < nrows) {
    return -1.0;
  } else {
    return 1.0;
  }
}

float field_box(vec2 p, float width, float height) {
  vec2 d = abs(p) - vec2(width, height);
  return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float field_time_domain(vec2 p) {
  return -abs(audio_time_domain_sample(p));
}

float field_top(vec2 p) {
  return -p.y;
}

float field_wave(vec2 p, float thickness) {
  return abs(p.y - audio_time_domain_sample(p)) - thickness;
}

float field_x(vec2 p, float size, float radius) {
  p = abs(p);
  return length(p - min(p.x + p.y, size) * 0.5) - radius;
}

float distance_field(vec2 p, uvec2 px) {
  switch (field) {
    case FIELD_ALL:
      return field_all();
    case FIELD_CHECK:
      return field_check(p);
    case FIELD_CIRCLE:
      return field_circle(p, 1.0);
    case FIELD_CROSS:
      return field_cross(p, 1.0, 0.25, 0.0);
    case FIELD_EQUALIZER:
      return field_equalizer(p);
    case FIELD_FREQUENCY:
      return field_frequency(p, 0.125);
    case FIELD_MOD:
      return field_mod(px, divisor, remainder);
    case FIELD_ROWS:
      return field_rows(px, nrows, step);
    case FIELD_SQUARE:
      return field_box(p, 0.5, 0.5);
    case FIELD_TIME_DOMAIN:
      return field_time_domain(p);
    case FIELD_TOP:
      return field_top(p);
    case FIELD_WAVE:
      return field_wave(p, 0.1);
    case FIELD_X:
      return field_x(p, 2.0, 0.25);
    default:
      return field_none();
  }
}

void main() {
  // Get fragment coordinates and transform to [-1, 1]
  vec2 position = gl_FragCoord.xy / resolution * 2.0 - 1.0;

  // Transform position by coordinate transform matrix
  vec2 transformed = (coordinate_transform * vec3(position, 1.0)).xy;

  // Wrap transformed position to be within [-1, 1] if enabled
  vec2 wrapped = wrap
    ? mod(transformed + 1.0, 2.0) - 1.0
    : transformed;

  // Sample color if in-bounds, otherwise use default color
  vec3 input_color = coordinates ? vec3(quadrant(wrapped), 0.0)
    : abs(wrapped.x) <= 1.0 && abs(wrapped.y) <= 1.0 ? texture(source, quadrant(wrapped)).rgb
    : default_color;

  // Sample original color
  vec3 original_color = texture(source, gl_FragCoord.xy / resolution).rgb;

  // Calculate position in pixel coordinates, [0, resolution)
  uvec2 pixel_position = uvec2(quadrant(wrapped) * resolution);

  // Convert color from [0,1] to [-1,-1]
  vec3 color_vector = input_color * 2.0 - 1.0;

  // Transform color vector using color transform
  vec4 transformed_color_vector = color_transform * vec4(color_vector, 1.0);

  // Convert color back from [-1,-1] to [0,1]
  vec3 transformed_color = octant(transformed_color_vector.xyz);

  // Get the signed distance from the field
  float distance = distance_field(wrapped, pixel_position);

  // Set alpha to zero if distance is negative
  float alpha = distance <= 0.0 ? alpha : 0.0;

  // Perform alpha blending
  vec3 output_color_rgb = transformed_color * alpha + original_color * (1.0 - alpha);

  // Extend output color with opaque alpha channel
  output_color = vec4(output_color_rgb, 1.0);
}
