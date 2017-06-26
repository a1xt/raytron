#version 120

varying vec2 v_tex_coord;

uniform sampler2D tex2;

void main() {
    gl_FragColor = texture2D(tex2, v_tex_coord);
}