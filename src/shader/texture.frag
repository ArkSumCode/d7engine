#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
    float Opacity;
} IN;

out vec4 Color;

uniform sampler2D tex_data;

void main()
{
    vec4 t = texture(tex_data, IN.TexCoord);
    t.a = IN.Opacity;
    Color = t;
}