#version 150 core

in vec4 vertex_position;
in vec3 vertex_color;

out vec4 color;
out vec3 FragPos;

void main() {
    color = vec4(vertex_color, 1.0);
    FragPos = vec3(vertex_position);
    gl_Position = vertex_position;
}