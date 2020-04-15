#version 150 core
in vec4 color_position;
in vec3 texture_position;
out vec4 color;

uniform sampler2D sampler_texture;

void main() {
    color = texture(sampler_texture, vec2(texture_position));
}
