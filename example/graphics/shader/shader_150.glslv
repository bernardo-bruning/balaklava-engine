#version 150 core

in vec4 vertex_position;

void main() {
    //color = vec4(1.0, 0.0, 0.0, 1.0);
    gl_Position = vertex_position;
}