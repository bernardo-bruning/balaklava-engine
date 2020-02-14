#version 150 core

in vec4 v_Color;
in vec3 FragPos;

uniform Light {
    vec4 u_Pos;
    vec3 u_Color;
};

out vec4 Target0;

void main() {
    vec3 lightDir = normalize(vec3(u_Pos) - FragPos);
    float diff = max(dot(vec3(0.0, 0.0, 1.0), lightDir), 0.0);
    vec4 diffuse = diff * vec4(0.2);
    Target0 = v_Color * diffuse;
}
