#version 150 core

in vec4 color;
in vec3 position;
in vec3 normal;

uniform light {
    vec4 light_position;
};

out vec4 target;

void main() {
    vec3 lightDir = normalize(vec3(light_position) - position);
    float diff = max(dot(normal, lightDir), 0.0);
    vec4 diffuse = diff * vec4(0.2);
    target = color * diffuse;
}
