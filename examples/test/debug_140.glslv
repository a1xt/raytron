#version 140

in vec3 a_Pos;
in vec4 a_Color;
out vec4 v_Color;

uniform mat4 view;
uniform mat4 proj;
uniform mat4 model;

void main() {
    v_Color = a_Color;
    gl_Position = proj * view * model * vec4(a_Pos, 1.0);
}