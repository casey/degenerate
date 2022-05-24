#version 300 es

// INSERT_GENERATED_CODE_HERE

precision highp float;

uniform int divisor;
uniform int mask;
uniform int operation;
uniform int remainder;
uniform sampler2D source;
uniform uint resolution;

out vec4 color;

vec4 apply_operation(vec4 pixel) {
  switch (operation) {
    case IDENTITY:
      return pixel;
    case INVERT:
      return vec4(1.0 - pixel.rgb, 1.0);
    default:
      return vec4(0.0, 1.0, 0.0, 1.0);
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
      return divisor == 0 ? false : ((int(resolution) - 1 - pixel.y) * int(resolution) + pixel.x) % divisor == remainder;
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
  ivec2 coordinates = ivec2(gl_FragCoord.xy - 0.5);
  vec4 pixel = texelFetch(source, coordinates, 0);
  vec2 position = gl_FragCoord.xy / float(resolution) * 2.0 - 1.0;
  color = is_masked(coordinates, position) ? apply_operation(pixel) : vec4(pixel.xyz, 1.0);
}
