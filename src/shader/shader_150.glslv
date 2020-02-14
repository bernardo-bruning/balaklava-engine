#version 150 core

in vec4 a_Pos;
in vec3 a_Color;

uniform Light {
    vec4 u_Pos;
    vec3 u_Color;
};

out vec4 v_Color;
out vec3 FragPos;

void main() {
    v_Color = vec4(a_Color, 1.0);
    FragPos = vec3(a_Pos);
    gl_Position = a_Pos;
}