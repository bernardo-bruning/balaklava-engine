#version 150 core

in vec4 color;
in vec4 position;
in vec3 normal;

uniform light {
    vec4 light_position;
};

out vec4 target;

void main() {
    vec3 lightDir = normalize(vec3(light_position) - vec3(position));
    float diff = max(dot(normal, lightDir), 0.0);
    vec4 diffuse = diff * vec4(0.2);
    target = color * diffuse;
    //target = vec4(1.0, 0.0, 0.0, 1.0);
}
