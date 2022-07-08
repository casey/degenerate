#version 300 es

precision highp float;

#include <hsl.glsl>

const uint MASK_ALL = 0u;
const uint MASK_CHECK = 1u;
const uint MASK_CIRCLE = 2u;
const uint MASK_CROSS = 3u;
const uint MASK_MOD = 4u;
const uint MASK_ROWS = 5u;
const uint MASK_SQUARE = 6u;
const uint MASK_TOP = 7u;
const uint MASK_X = 8u;

const uint OPERATION_DEBUG = 0u;
const uint OPERATION_IDENTITY = 1u;
const uint OPERATION_INVERT = 2u;
const uint OPERATION_ROTATE_COLOR = 3u;
const uint OPERATION_SAMPLE = 4u;

uniform bool wrap;
uniform float alpha;
uniform float resolution;
uniform mat3 transform;
uniform mat4 color_transform;
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

// all operations are
//
// transform * color
//
// mask returns sample alpha instead of bool using sdf
//
// rotate: rotation
// sample: rotation axis
// invert: reflection
// debug: rotate by amount and axis that differs depending on square
//
// loudness, time domain sample, frequency domain sample, x position, y position:
// size, sharpness, operation rotation amount, operation rotation axis:
//
// function that generates widgets for everything
// - x * amount of rotation
//
// position transform
// color transform
// lots of values between 0 and 1
// flip single axis
//
// rename mask: shape
//
// render to temporary buffer and use as mask to combine shapes

vec3 apply(vec2 position, vec3 color) {
  switch (operation) {
    case OPERATION_DEBUG:
      return floor(vec3((position.x + 1.0) / 2.0, 0.0, 1.0 - (position.y + 1.0) / 2.0) * 16.0) / 16.0;
    case OPERATION_IDENTITY:
      return color;
    case OPERATION_INVERT:
    case OPERATION_ROTATE_COLOR:
      vec3 v = color * 2.0 - 1.0;
      vec4 t = color_transform * vec4(v, 1.0);
      return (t.xyz + 1.0) / 2.0;
    case OPERATION_SAMPLE:
      float lightness = abs(texture(audio_time_domain, vec2((position.x + 1.0) / 2.0, 0.5)).r);
      vec3 hsl = rgb2hsl(color);
      hsl.z = lightness;
      vec3 rgb = hsl2rgb(hsl);
      return rgb;
    default:
      return vec3(0.0, 1.0, 0.0);
  }
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
    case MASK_MOD:
      if (divisor == 0u) {
        return false;
      } else {
        return (pixel_position.y * uint(resolution) + pixel_position.x) % divisor == remainder;
      }
    case MASK_ROWS:
      return pixel_position.y % (nrows + step) < nrows;
    case MASK_SQUARE:
      return abs(position.x) < 0.5 && abs(position.y) < 0.5;
    case MASK_TOP:
      return position.y > 0.0;
    case MASK_X:
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
