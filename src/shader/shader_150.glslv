#version 150 core

in vec4 a_Pos;
in vec3 a_Color;

uniform Light {
    vec3 u_Color;
};

out vec4 v_Color;

void main() {
    v_Color = vec4(u_Color * vec3(1.0, 0.0, 0.0), 1.0);
    gl_Position = a_Pos;
}