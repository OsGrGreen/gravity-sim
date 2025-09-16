#version 450 core

layout(isolines) in;

out vec3 fragment_Normal;
out vec4 fragment_Position;


void BernsteinPolynomials(out float[4] b, out float[4] db, float t) {

	b[0] = pow(1.0 - t, 3);
	b[1] = 3.0 * pow(1.0 - t, 2) * t;
	b[2] = 3.0 * (1.0 - t) * pow(t,2);
	b[3] = pow(t, 3);

	//derivatives
	db[0] = -3.0 * pow(1.0 - t, 2);
	db[1] = -6.0 * (1.0 - t) * t + 3.0 * pow(1.0 - t,2);
	db[2] = -3.0 * pow(t, 2) + 6.0 * t * (1.0 - t);
	db[3] = 3.0 * pow(t, 2);
}

void main() {
	
	float t = gl_TessCoord.x;
	//To make the sums a little bit nicer

	vec4 p00 = gl_in[0].gl_Position;
	vec4 p01 = gl_in[1].gl_Position;
	vec4 p02 = gl_in[2].gl_Position;
	vec4 p03 = gl_in[3].gl_Position;

	//bernstein polynomials for interpolating in u and v directions
	float b[4], db[4];
	BernsteinPolynomials(b, db, t);

	//The double sum bt written in full
	fragment_Position = p00*b[0] + p01*b[1] + p02*b[2] + p03*b[3];

	vec3 dPos_dt = vec3(p00*db[0] + p01*db[1] + p02*db[2]+ p03*db[3]);
    vec3 out_point = vec3(fragment_Position);
	fragment_Normal = normalize(cross(dPos_dt, out_point));
	gl_Position = fragment_Position;
	
}