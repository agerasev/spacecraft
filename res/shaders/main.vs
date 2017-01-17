#version 130

uniform mat3 model;

in vec3 position;
in vec3 color;

out vec3 _color;

void main() {
	_color = color;
	gl_Position = vec4(model*position, 1.0);
}