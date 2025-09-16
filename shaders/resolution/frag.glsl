#version 330 core
in vec2 v_tex_coords;

out vec4 color;

const float outlineThickness = 1.0;
const vec4 outlineColor = vec4(0.0,0.0,0.0,1.0);

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords);
}