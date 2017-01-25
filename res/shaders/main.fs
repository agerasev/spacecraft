#version 130

in vec3 var_color;

out vec4 out_color;

void main() {
	out_color = vec4(var_color, 1.0);
}