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
    Target0 = v_Color * vec4(lightDir,1.0);
}
