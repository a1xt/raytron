#version 140

in vec3 pos3;
in vec2 tex_coord;

out vec2 v_tex_coord;

void main() {
    v_tex_coord = tex_coord;
    gl_Position = vec4(pos3, 1.0);
}