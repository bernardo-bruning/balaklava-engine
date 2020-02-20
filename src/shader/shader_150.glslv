#version 150 core

in vec4 vertex_position;
in vec3 vertex_color;
in vec3 vertex_normal;

uniform viewport {
    mat4 viewport_tranform;
};

out vec4 color;
out vec4 position;
out vec3 normal;

void main() {
    color = vec4(vertex_color, 1.0);
    normal = vertex_normal;
    position = vertex_position*(viewport_tranform);
    gl_Position = position;
}