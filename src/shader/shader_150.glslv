#version 150 core

in vec4 vertex_position;
in vec3 vertex_color;
in vec3 vertex_normal;

uniform camera {
    mat4 camera_tranform;
};

uniform transform {
    mat4 transform_matrix;
};

out vec4 color;
out vec4 position;
out vec3 normal;

void main() {
    color = vec4(vertex_color, 1.0);
    normal = vertex_normal;
    position = vertex_position*camera_tranform;
    gl_Position = position;
}