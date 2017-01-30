#version 140

in vec2 v_tex_coord;
out vec4 target0;

uniform sampler2D tex2;

void main() {
    target0 = texture(tex2, v_tex_coord);
}