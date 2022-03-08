#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
    vec3 Color;
    float Opacity;
} IN;

out vec4 Color;

uniform sampler2D tex_data;

void main()
{
    vec4 t = texture(tex_data, IN.TexCoord);
    t.r = IN.Color.r;
    t.g = IN.Color.g;
    t.b = IN.Color.b;
    t.a = t.a * IN.Opacity;
    Color = t;
}