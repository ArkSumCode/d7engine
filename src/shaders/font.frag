#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
    vec4 Color;
} IN;

out vec4 Color;

uniform sampler2D tex_data;

void main()
{
    vec4 img = texture(tex_data, IN.TexCoord);
    if(img.a == 1.0) {
        img.r = IN.Color.r;
        img.g = IN.Color.g;
        img.b = IN.Color.b;
        img.a = 1.0;
    } else if(img.a != 0.0) {
        img.r = IN.Color.r;
        img.g = IN.Color.g;
        img.b = IN.Color.b;
        img.a = 0.5;
    } else {
       img.a = 0.0;
    }

    Color = img;
}