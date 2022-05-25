#version 300 es

// INSERT_GENERATED_CODE_HERE

precision highp float;

uniform float alpha;
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

// let i = Point2::new(col as f64, row as f64);
// let v = transform.transform_point(&i);
// let v = similarity.transform_point(&v);
// let v = if self.wrap { v.wrap() } else { v };
// let i = inverse
//   .transform_point(&v)
//   .map(|element| element.round() as isize);
//
// i = integer pixel coordinates
// v = floating point vector with ori
//
// all coordinates are between 0 and 1 with the same origin
// all values are floats

void main() {
  ivec2 coordinates = ivec2(gl_FragCoord.xy - 0.5);
  vec2 position = gl_FragCoord.xy / float(resolution) * 2.0 - 1.0;
  vec2 transformed = (similarity * vec3(position, 1.0)).xy;
  ivec2 coordinates_transformed = ivec2(((transformed + 1.0) / 2.0) * float(resolution));
  vec3 transformed_pixel = texelFetch(source, coordinates_transformed, 0).rgb;
  vec3 untransformed_pixel = texelFetch(source, coordinates, 0).rgb;
  if (is_masked(coordinates_transformed, transformed)) {
    vec3 over = apply_operation(transformed, transformed_pixel);
    color = vec4(over * alpha + untransformed_pixel * (1.0 - alpha), 1.0);
  } else {
    color = vec4(untransformed_pixel, 1.0);
  }
}
