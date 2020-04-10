#version 150 core

in vec4 position;
uniform mat4 transform;
out vec4 color_position;

void main() {
    color_position = vec4(vec2(position)+1, 0.0, 1.0);
    gl_Position = transform*position;
}