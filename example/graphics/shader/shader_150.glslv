#version 150 core

in vec4 position;

void main() {
    //color = vec4(1.0, 0.0, 0.0, 1.0);
    gl_Position = position;
}