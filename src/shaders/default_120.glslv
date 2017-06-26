#version 130

attribute vec3 pos3;
attribute vec2 tex_coord;

varying vec2 v_tex_coord;

void main() {
    v_tex_coord = tex_coord;
    gl_Position = vec4(pos3, 1.0);
}