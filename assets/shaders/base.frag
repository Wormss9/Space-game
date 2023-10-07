#version 410

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() { color = texture(tex, v_tex_coords, 0.0); }
