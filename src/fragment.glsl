#version 300 es

precision highp float;

uniform sampler2D source;
uniform uint mask;
uniform uint operation;
uniform uint resolution;

out vec4 color;

vec4 apply_operation(vec4 pixel) {
  switch (operation) {
    case Identity:
      return pixel;
    case Invert:
      return vec4(1.0 - pixel.rgb, 1.0);
    default:
      return vec4(0.0, 1.0, 0.0, 1.0);
  }
}

bool is_masked(vec2 position) {
  switch (mask) {
    case All:
      return true;
    case Circle:
      return length(position) < 1.0;
    case Cross:
      return abs(position.x) < 0.25 || abs(position.y) < 0.25;
    case X:
      return abs(abs(position.x) - abs(position.y)) < 0.25;
    default:
      return false;
  }
}

void main() {
  vec4 pixel = texelFetch(source, ivec2(gl_FragCoord.xy - 0.5), 0);
  vec2 position = gl_FragCoord.xy / float(resolution) * 2.0 - 1.0;
  color = is_masked(position) ? apply_operation(pixel) : vec4(pixel.xyz, 1.0);
}
