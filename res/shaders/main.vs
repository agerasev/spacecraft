#version 130

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

in vec3 position;
in vec3 color;

out vec3 var_color;

void main() {
	var_color = color;
	gl_Position = proj*view*model*vec4(position, 1.0);
}