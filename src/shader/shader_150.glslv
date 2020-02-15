#version 150 core

in vec4 vertex_position;
in vec3 vertex_color;
in vec3 vertex_normal;

out vec4 color;
out vec3 position;
out vec3 normal;

void main() {
    color = vec4(vertex_color, 1.0);
    normal = vertex_normal;
    position = vec3(vertex_position);
    gl_Position = vertex_position;
}