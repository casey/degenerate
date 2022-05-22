#version 300 es

// Generate a single screen-filling triangle.
// See this post for details:
// https://stackoverflow.com/a/59739538/66450

out vec2 uv;

const vec2 vertices[3]= vec2[3](vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));

void main() {
  gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
  uv = (gl_Position.xy + 1.0) / 2.0;
}
