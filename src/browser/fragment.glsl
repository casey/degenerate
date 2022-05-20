#version 300 es

precision highp float;

uniform sampler2D source;
uniform uint mask;
uniform uint operation;

in vec2 uv;

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

bool is_masked() {
  switch (mask) {
    // X
    case 0u:
      return min(abs((1.0 - uv.x) - uv.y), abs(uv.x - uv.y)) < 0.125;
    // Circle
    case 1u:
      return length((uv - 0.5) * 2.0) < 1.0;
    // All
    case 2u:
       return true;
    // Error
    default:
      return false;
  }
}

void main() {
  vec4 pixel = texture(source, uv);
  color = is_masked() ? apply_operation(pixel) : vec4(pixel.xyz, 1.0);
}
