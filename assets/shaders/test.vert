#version 410

in vec2 position;

flat out vec2 extra;

void main() {
  gl_Position = vec4(position, 0.0, 1.0);
  extra = position;
}