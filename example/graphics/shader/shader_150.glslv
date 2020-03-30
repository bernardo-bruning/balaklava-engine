#version 150 core

in vec4 vertex_position;
in vec3 vertex_color;
in vec3 vertex_normal;
in vec2 vertex_uv;

uniform camera {
    mat4 camera_tranform;
    mat4 camera_projection;
};

uniform transform {
    mat4 transform_matrix;
};

out vec4 color;
out vec4 position;
out vec3 normal;
out vec2 uv;

void main() {
    color = vec4(vertex_color, 1.0);
    normal = vertex_normal;
    position = position;
    gl_Position = camera_projection*camera_tranform*transform_matrix*vertex_position;
    uv = vertex_uv;
}