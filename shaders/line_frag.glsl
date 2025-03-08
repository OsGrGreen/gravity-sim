#version 330 core


in vec3 fragment_Normal;
in vec4 fragment_Position;

const vec3 u_light = vec3(0.0,0.0,1.0);
uniform int color_steps = 5;

const vec3 ambient_color = vec3(0.3, 0.2, 0.5);
const vec3 diffuse_color = vec3(0.6, 0.2, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

out vec4 color;

void main() {
    float ambient_strength = 0.3;
	vec3 ambient = ambient_strength * ambient_color;

	// diffuse
	vec3 normal = normalize(fragment_Normal);
    vec3 pos = vec3(fragment_Position);
	vec3 lightDir = normalize(u_light - pos);
	float diffuse = dot(normal, u_light);
	float diffuse_toon = max(ceil(diffuse * float(color_steps)) / float(color_steps), 0.0);

	vec3 toonColor = diffuse_toon * diffuse_color;

	color = vec4(toonColor, 1.0);
}