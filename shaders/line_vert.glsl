#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec3 fragment_Normal;
out vec4 fragment_Position;

void main() {
    gl_Position = projection*view*model*vec4(position, 1.0);
    fragment_Normal = normal;
    fragment_Position = gl_Position;
}