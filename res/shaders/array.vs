#version 130

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

in vec3 position;
in vec3 color;
in vec2 tex_pos;
in vec4 occlusion;

out vec3 var_color;
out vec2 var_tex_pos;
out vec4 var_occlusion;

void main() {
	var_color = color;
	var_tex_pos = tex_pos;
	var_occlusion = occlusion;
	gl_Position = proj*view*model*vec4(position, 1.0);
}
