#version 330 core
out vec4 FragColor;

in vec2 TexCoord;

// texture sampler
uniform sampler2D texture1;
uniform float textureMix;

void main()
{
	// color from the texture
	vec4 texturedColor = texture(texture1, TexCoord);
	float id = mod(float(gl_PrimitiveID), 3.0);
	float val = (0.25 * id) + 0.25;
	// grey colored face
	vec4 greyColor = vec4(val, val, val, 1.0);
	// mix based on textureMix
	FragColor = mix(texturedColor, greyColor, textureMix);
}
