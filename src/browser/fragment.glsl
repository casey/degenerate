#version 300 es

precision highp float;

in vec2 uv;
out vec4 color;

uniform sampler2D source;

#define I texture(source, uv)

uniform uint operation;

vec4 apply_operation() {
  switch (operation) {
    // Identity
    case 0u:
      return I;
    // Invert
    case 1u:
      return vec4(1.0 - I.rgb, 1.0);
    // Error
    default:
      return vec4(0.0, 1.0, 0.0, 1.0);
  }
}

uniform uint mask;

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
  color = is_masked() ? apply_operation() : vec4(I.xyz, 1.0);
}
