#version 450 core

in vec3 fragment_Normal;
in vec4 fragment_Position;
in float frag_t;

uniform vec3 u_light;
uniform float u_time;
uniform int color_steps = 5;
uniform vec3 object_color;

uniform float u_dashSize = 0.05;
uniform float u_gapSize = 0.05;

out vec4 color;


void main() {
    float totalSize = u_dashSize + u_gapSize;

    float cyclePos = mod(frag_t, totalSize);

    if (cyclePos > u_dashSize) {
        discard;
    }

    color = vec4(1.0,1.0,1.0, 1.0);
}