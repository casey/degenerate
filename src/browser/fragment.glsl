#version 300 es

precision highp float;

in vec2 uv;
out vec4 color;

uniform sampler2D source;

#define I texture(source, uv)

uniform bool invert;

vec4 operation() {
  if (invert)
    return vec4(1.0 - I.rgb, 1.0);
  return I;
}

uniform bool circle;
uniform bool x;

bool is_masked() {
  if (circle)
    return length((uv - 0.5) * 2.0) < 1.0;
  if (x)
    return min(abs((1.0 - uv.x) - uv.y), abs(uv.x - uv.y)) < 0.125;
  return true;
}

void main() {
  color = is_masked() ? operation() : vec4(I.xyz, 1.0);
}
