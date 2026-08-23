#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

void main() {
    mat4 modelview = view * model;

    // world-space position of vertex
    vec4 world_pos = model * vec4(position, 1.0);
    v_position = world_pos.xyz;

    // world-space normal
    v_normal = normalize(mat3(transpose(inverse(model))) * normal);

    // final clip-space position
    gl_Position = projection * view * world_pos;
    v_tex_coords = tex_coords;
}