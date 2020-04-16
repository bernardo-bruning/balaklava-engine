#version 150 core

in vec4 position;
in vec3 texture_region;
uniform mat4 transform;
out vec4 color_position;
out vec3 texture_position;

void main() {
    texture_position = texture_region;
    color_position = vec4(vec2(position)+1, 0.0, 1.0);
    gl_Position = transform*position;
}