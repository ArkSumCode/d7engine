#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
    vec4 Color;
} IN;

out vec4 Color;

uniform sampler2D tex_data;

void main()
{
    Color = texture(tex_data, IN.TexCoord) * IN.Color;
}