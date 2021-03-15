#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D tex_data;

void main()
{
    Color = texture(tex_data, IN.TexCoord);
}