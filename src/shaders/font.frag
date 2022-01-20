#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D tex_data;

void main()
{
    vec4 img = texture(tex_data, IN.TexCoord);
    
    img.r = IN.Color.r;
    img.g = IN.Color.g;
    img.b = IN.Color.b;

    Color = img;
}