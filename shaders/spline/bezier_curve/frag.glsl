#version 450 core

in vec3 fragment_Normal;
in vec4 fragment_Position;

uniform vec3 u_light;
uniform float u_time;
uniform int color_steps = 5;
uniform vec3 object_color;

out vec4 color;


void main() {


	color = vec4(fragment_Normal*vec3(1.0,1.0,0.0),1.0);
}