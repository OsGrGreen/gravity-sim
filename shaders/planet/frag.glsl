#version 330 core

in vec3 v_position;
in vec3 v_normal;
in vec2 v_tex_coords;


uniform float u_time;
uniform float radius;
uniform sampler2D tex;
uniform vec3 u_light = vec3(0.0,0.0,2.0);
uniform int color_steps = 5;

out vec4 color;

const vec3 ambient_color = vec3(0.1, 0.0, 0.2);
const vec3 diffuse_color = vec3(0.8, 0.5, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 0.0);


// Maybe improve this to not use an if
// Is possible with smoothstep
void main() {
    /*vec2 pos = v_tex_coords * 2.0 - 1.0;
    float distSq = dot(pos, pos);
    if (distSq > 0.5)
        discard;*/

	// diffuse
	vec3 normal = normalize(v_normal);
	vec3 lightDir = normalize(u_light - v_position);
	float diffuse = dot(normal, u_light);
	float diffuse_toon = max(ceil(diffuse * float(color_steps)) / float(color_steps), 0.0);

	vec4 toonColor = vec4(diffuse_toon * diffuse_color, 1.0);

    color = toonColor*texture(tex,v_tex_coords);
}
