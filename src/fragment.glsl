#version 300 es

// INSERT_GENERATED_CODE_HERE

precision highp float;

uniform mat3 color_rotation;
uniform mat3 similarity;
uniform sampler2D source;
uniform uint divisor;
uniform uint mask;
uniform uint nrows;
uniform uint operation;
uniform uint remainder;
uniform uint resolution;
uniform uint step;

out vec4 color;

vec3 apply_operation(vec3 pixel) {
  switch (operation) {
    case IDENTITY:
      return pixel;
    case INVERT:
      return 1.0 - pixel;
    case ROTATE_COLOR:
      vec3 position = (pixel * 2.0 - 1.0);
      vec3 rotated = color_rotation * position;
      vec3 color = (rotated + 1.0) / 2.0;
      return color;
    default:
      return vec3(0.0, 1.0, 0.0);
  }
}

bool is_masked(ivec2 pixel, vec2 position) {
  switch (mask) {
    case ALL:
      return true;
    case CIRCLE:
      return length(position) < 1.0;
    case CROSS:
      return abs(position.x) < 0.25 || abs(position.y) < 0.25;
    case MOD:
      return divisor == 0u ? false : ((resolution - 1u - uint(pixel.y)) * resolution + uint(pixel.x)) % divisor == remainder;
    case ROWS:
      return (resolution - 1u - uint(pixel.y)) % (nrows + step) < nrows;
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

// transform w/similarity
// apply wrap
// transform back to integer coordinates
// get pixel or default

void main() {
  ivec2 coordinates = ivec2(gl_FragCoord.xy - 0.5);
  vec2 position = gl_FragCoord.xy / float(resolution) * 2.0 - 1.0;
  vec2 transformed = (similarity * vec3(position, 1.0)).xy;
  ivec2 coordinates_transformed = ivec2(((transformed + 1.0) / 2.0) * float(resolution));
  vec3 pixel = texelFetch(source, coordinates_transformed, 0).rgb;
  vec3 result = is_masked(coordinates, transformed) ? apply_operation(pixel) : pixel;
  color = vec4(result, 1.0);
}
