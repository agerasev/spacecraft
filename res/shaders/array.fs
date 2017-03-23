#version 130

in vec3 var_color;
in vec2 var_tex_pos;
in vec4 var_occlusion;

out vec4 out_color;

float sqr(in float t) {
	return t*t;
}

vec4 bilinear(in vec2 tp) {
	return vec4(tp.x*tp.y, tp.x*(1.0 - tp.y), (1.0 - tp.x)*(1.0 - tp.y), (1.0 - tp.x)*tp.y);
}

void main() {
	vec4 of = var_occlusion;
	vec2 tp = var_tex_pos;
	vec4 tf = bilinear(tp);
	float occl = dot(tf, of);
	out_color = vec4(var_color*occl, 1.0);
}
