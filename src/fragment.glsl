#version 300 es

precision highp float;

uniform sampler2D source;
uniform uint mask;
uniform uint operation;
uniform uint resolution;

out vec4 color;

vec4 apply_operation(vec4 pixel) {
  switch (operation) {
    // Identity
    case 0u:
      return pixel;
    // Invert
    case 1u:
      return vec4(1.0 - pixel.rgb, 1.0);
    // Error
    default:
      return vec4(0.0, 1.0, 0.0, 1.0);
  }
}

bool is_masked(vec2 position) {
  switch (mask) {
    // X
    case 0u:
      return abs(abs(position.x) - abs(position.y)) < 0.25;
    // Circle
    case 1u:
      return length(position) < 1.0;
    // All
    case 2u:
       return true;
    // Error
    default:
      return false;
  }
}

void main() {
  vec2 position = gl_FragCoord.xy / float(resolution) * 2.0 - 1.0;
  vec4 pixel = texelFetch(source, ivec2(gl_FragCoord.xy), 0);
  color = is_masked(position) ? apply_operation(pixel) : vec4(pixel.xyz, 1.0);
}
