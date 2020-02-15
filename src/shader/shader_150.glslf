#version 150 core

in vec4 color;
in vec3 FragPos;

uniform Light {
    vec4 light_position;
};

out vec4 Target0;

void main() {
    vec3 lightDir = normalize(vec3(light_position) - FragPos);
    float diff = max(dot(vec3(0.0, 0.0, 1.0), lightDir), 0.0);
    vec4 diffuse = diff * vec4(0.2);
    Target0 = color * diffuse;
}
