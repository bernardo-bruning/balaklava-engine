#version 150 core

uniform sampler2D t_texture;
in vec4 color;
in vec4 position;
in vec3 normal;
in vec2 uv;

uniform light {
    vec4 light_position;
};

out vec4 target;

void main() {
    vec3 lightDir = normalize(vec3(light_position) - vec3(position));
    float diff = max(dot(normal, lightDir), 0.0);
    target = texture(t_texture, uv);
}
