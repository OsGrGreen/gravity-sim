#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float u_time;
uniform float slowdown = 0.05;

out vec3 v_color;
out vec2 v_tex_coords;


float rand(vec3 co) {
    return fract(sin(dot(co, vec3(12.9898,78.233,37.719))) * 43758.5453);
}

void main() {
    // world-space position
    vec4 world_pos = model * vec4(position, 1.0);

    // random-ish value based on the face (normal works as a seed)
    float r = rand(normal + floor(u_time * slowdown));
    float g = rand(normal.yzx + floor(u_time * 0.5 * slowdown));
    float b = rand(normal.zxy + floor(u_time * 0.25 * slowdown));

    // base sun color (yellowish) + variation
    v_color = vec3(1.0, 0.8, 0.2) + 0.3 * vec3(r, g, b);

    gl_Position = projection * view * world_pos;
    v_tex_coords = tex_coords;
}
