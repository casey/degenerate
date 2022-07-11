#version 300 es

// Generate a single screen-filling triangle.
// See this post for details:
// https://stackoverflow.com/a/59739538/66450

const vec4 VERTICES[3] = vec4[3](
  vec4(-1.0, -1.0, 0.0, 1.0),
  vec4(3.0, -1.0, 0.0, 1.0),
  vec4(-1.0, 3.0, 0.0, 1.0)
);

void main() {
  gl_Position = VERTICES[gl_VertexID];
}
