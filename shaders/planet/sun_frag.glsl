#version 330 core

in vec3 v_color;
in vec2 v_tex_coords;

uniform float u_time = 1.0;
uniform float radius;
uniform sampler2D tex;
uniform vec3 u_light = vec3(0.0,0.0,0.0);
uniform int color_steps = 5;
uniform float insideRadius = 1.0; 

out vec4 color;

const vec3 ambient_color = vec3(0.1, 0.0, 0.2);
const vec3 diffuse_color = vec3(0.8, 0.5, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 0.0);


void main() {

    vec4 toonColor = vec4(v_color,1.0);
    color = toonColor*texture(tex,v_tex_coords);
}

