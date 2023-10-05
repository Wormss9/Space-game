#version 410

flat in vec2 extra;

out vec4 color;

void main() { color = vec4(extra, 0.0, 1.0); }
