#version 300 es

// INSERT_GENERATED_CODE_HERE

precision highp float;

uniform bool wrap;
uniform float alpha;
uniform float resolution;
uniform mat3 color_rotation;
uniform mat3 similarity;
uniform sampler2D source;
uniform uint divisor;
uniform uint mask;
uniform uint nrows;
uniform uint operation;
uniform uint remainder;
uniform uint step;

out vec4 color;

vec3 apply_operation(vec2 position, vec3 pixel) {
  switch (operation) {
    case DEBUG:
      return floor(vec3((position.x + 1.0) / 2.0, 0.0, 1.0 - (position.y + 1.0) / 2.0) * 16.0) / 16.0;
    case IDENTITY:
      return pixel;
    case INVERT:
      return 1.0 - pixel;
    case ROTATE_COLOR:
      return (color_rotation * (pixel * 2.0 - 1.0) + 1.0) / 2.0;
    default:
      return vec3(0.0, 1.0, 0.0);
  }
}

bool is_masked(vec2 pixel, vec2 position) {
  switch (mask) {
    case ALL:
      return true;
    case CIRCLE:
      return length(position) < 1.0;
    case CROSS:
      return abs(position.x) < 0.25 || abs(position.y) < 0.25;
    case MOD:
      return divisor == 0u ? false : ((uint(resolution) - 1u - uint(pixel.y)) * uint(resolution) + uint(pixel.x)) % divisor == remainder;
    case ROWS:
      return (uint(resolution) - 1u - uint(pixel.y)) % (nrows + step) < nrows;
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
  vec2 i = gl_FragCoord.xy - 0.5;
  vec2 v = gl_FragCoord.xy / resolution * 2.0 - 1.0;
  vec2 vt = (similarity * vec3(v, 1.0)).xy;
  vec2 vtw = wrap ? mod(vt + 1.0, 2.0) - 1.0 : vt;
  vec2 it = floor(((vtw + 1.0) / 2.0) * resolution);
  vec3 pt = texelFetch(source, ivec2(it), 0).rgb;
  vec3 p = texture(source, gl_FragCoord.xy / resolution).rgb;
  if (is_masked(it, vtw)) {
    vec3 over = apply_operation(vtw, pt);
    color = vec4(over * alpha + p * (1.0 - alpha), 1.0);
  } else {
    color = vec4(p, 1.0);
  }
}
