#version 410

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform vec2 mov;
uniform float sca;
uniform float rot;
uniform float rat;

void main() {
  mat2 scale = mat2(sca / rat, 0.0, 0.0, sca);
  mat2 rotation = mat2(cos(rot), -sin(rot), sin(rot), cos(rot));

  gl_Position = vec4(scale * rotation * position + mov, 0.0, 1.0);

  v_tex_coords = tex_coords;
}
