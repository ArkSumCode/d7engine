#version 330 core

in VS_OUTPUT {
    vec3 Color;
    float Opacity;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(IN.Color, IN.Opacity);
}