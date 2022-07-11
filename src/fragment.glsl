#version 300 es

precision highp float;

const int MASK_ALL = 0;
const int MASK_CHECK = 1;
const int MASK_CIRCLE = 2;
const int MASK_CROSS = 3;
const int MASK_EQUALIZER = 4;
const int MASK_FREQUENCY = 5;
const int MASK_MOD = 6;
const int MASK_ROWS = 7;
const int MASK_SAMPLE = 8;
const int MASK_SQUARE = 9;
const int MASK_TOP = 10;
const int MASK_WAVE = 11;
const int MASK_X = 12;

uniform bool coordinates;
uniform bool wrap;
uniform float alpha;
uniform float resolution;
uniform int mask;
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

float audio_time_domain_sample(vec2 position) {
  return texture(audio_time_domain, vec2((position.x + 1.0) / 2.0, 0.5)).r;
}

float audio_frequency_sample(vec2 position) {
  return texture(audio_frequency, vec2((position.x + 1.0) / 2.0, 0.5)).r;
}

bool masked(vec2 position, uvec2 pixel_position) {
  switch (mask) {
    case MASK_ALL:
      return true;
    case MASK_CHECK:
      ivec2 i = ivec2((position + 1.0) * 4.0);
      return i.x % 2 != i.y % 2;
    case MASK_CIRCLE:
      return length(position) < 1.0;
    case MASK_CROSS:
      return abs(position.x) < 0.25 || abs(position.y) < 0.25;
    case MASK_EQUALIZER:
      return position.y / 2.0 + 0.5 < audio_frequency_sample(position);
    case MASK_FREQUENCY:
      return abs(audio_frequency_sample(position)) > 0.125;
    case MASK_MOD:
      if (divisor == 0u) {
        return false;
      } else {
        return (pixel_position.y * uint(resolution) + pixel_position.x) % divisor == remainder;
      }
    case MASK_ROWS:
      return pixel_position.y % (nrows + step) < nrows;
    case MASK_SAMPLE:
      return abs(audio_time_domain_sample(position)) > 0.01;
    case MASK_SQUARE:
      return abs(position.x) < 0.5 && abs(position.y) < 0.5;
    case MASK_TOP:
      return position.y > 0.0;
    case MASK_X:
      return abs(abs(position.x) - abs(position.y)) < 0.25;
    case MASK_WAVE:
      return abs(position.y - audio_time_domain_sample(position)) < 0.1;
    default:
      return false;
  }
}

vec2 uv(vec2 position) {
  return (position + 1.0) / 2.0;
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
  vec3 input_color = coordinates ? vec3(uv(wrapped), 0.0)
    : abs(wrapped.x) <= 1.0 && abs(wrapped.y) <= 1.0 ? texture(source, uv(wrapped)).rgb
    : default_color;

  // Sample original color
  vec3 original_color = texture(source, gl_FragCoord.xy / resolution).rgb;

  // Calculate position in pixel coordinates, [0, resolution)
  uvec2 pixel_position = uvec2(uv(wrapped) * resolution);

  // Convert color from [0,1] to [-1,-1]
  vec3 color_vector = input_color * 2.0 - 1.0;

  // Transform color vector using color transform
  vec4 transformed_color_vector = color_transform * vec4(color_vector, 1.0);

  // Convert color back from [-1,-1] to [0,1]
  vec3 transformed_color = (transformed_color_vector.xyz + 1.0) / 2.0;

  // Set alpha to 0.0 if outside the mask
  float alpha = masked(wrapped, pixel_position) ? alpha : 0.0;

  // Perform alpha blending
  vec3 output_color_rgb = transformed_color * alpha + original_color * (1.0 - alpha);

  // Extend output color with opaque alpha channel
  output_color = vec4(output_color_rgb, 1.0);
}
