#version 150 core

in vec4 vertex_position;
in vec3 a_Color;

uniform Light {
    vec4 u_Pos;
    vec3 u_Color;
};

out vec4 v_Color;
out vec3 FragPos;

void main() {
    v_Color = vec4(a_Color, 1.0);
    FragPos = vec3(vertex_position);
    gl_Position = vertex_position;
}