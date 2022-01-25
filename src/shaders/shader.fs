#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
// in int gl_PrimitiveID ;

// texture samplers
uniform sampler2D texture1;

void main()
{
	// linearly interpolate between both textures (80% container, 20% awesomeface)
	// FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.2);
	FragColor = texture(texture1, TexCoord);
	// float id = mod(float(gl_PrimitiveID), 3.0);
	// float val = (0.25 * id) + 0.25;
	// FragColor = vec4(val, val, val, 1.0);
}
