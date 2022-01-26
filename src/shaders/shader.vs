#version 330 core
layout (location = 0) in vec3 aPos;
// layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 transformation;

void main()
{
	// normalized model
	vec4 modelView = model * vec4(aPos, 1.0);
	// apply texture based on model z and y
	TexCoord = vec2((modelView.z / 2.0 + 0.5), (modelView.y / 2.0 + 0.5));
	gl_Position = projection * view * transformation * modelView ;
}